//! Session lifecycle management.
//!
//! A `Session` is one encrypted connection between two devices.
//! It can carry: KVM input events, clipboard changes, file chunks,
//! screen frames, and notifications.
//!
//! State machine:
//!   Connecting → Handshaking → Active → Disconnecting → Closed

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use x25519_dalek::PublicKey as X25519Public;

use crate::{CoreError, identity::DeviceIdentity, crypto::SessionKey, input::InputEvent};
use crate::protocol::HandshakePayload;

// ── Session config ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    /// Mouse + keyboard + clipboard sharing (Mouse Without Borders mode)
    Kvm,
    /// Full screen stream with remote input
    RemoteControl,
    /// Screen stream, no input (view only)
    ViewOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub session_type:      SessionType,
    pub clipboard_sync:    bool,
    pub file_transfer:     bool,
    pub notification_sync: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            session_type:      SessionType::Kvm,
            clipboard_sync:    true,
            file_transfer:     true,
            notification_sync: true,
        }
    }
}

// ── Session state ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Connecting,
    Handshaking,
    Active,
    Disconnecting,
    Closed,
    Failed(String),
}

// ── Session ───────────────────────────────────────────────────────

/// A live encrypted session with a peer device.
pub struct Session {
    pub id:             Uuid,
    pub peer_device_id: Uuid,
    pub config:         SessionConfig,
    pub state:          Arc<RwLock<SessionState>>,
    pub session_key:    SessionKey,
    /// Enqueue encrypted packets for the send loop
    send_tx:            mpsc::Sender<Vec<u8>>,
}

impl Session {
    pub async fn current_state(&self) -> SessionState {
        self.state.read().await.clone()
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.state.try_read().map(|s| s.clone()),
            Ok(SessionState::Active)
        )
    }

    /// Encrypt and enqueue an input event for transmission to peer.
    pub async fn send_input(&self, event: &InputEvent) -> Result<(), CoreError> {
        let payload   = serde_json::to_vec(event)?;
        let encrypted = self.session_key.encrypt(&payload)?;
        self.send_tx
            .send(encrypted)
            .await
            .map_err(|_| CoreError::NotConnected(self.peer_device_id))
    }

    /// Encrypt and enqueue any raw payload.
    pub async fn send_raw(&self, payload: &[u8]) -> Result<(), CoreError> {
        let encrypted = self.session_key.encrypt(payload)?;
        self.send_tx
            .send(encrypted)
            .await
            .map_err(|_| CoreError::NotConnected(self.peer_device_id))
    }

    /// Decrypt a packet received from the peer.
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CoreError> {
        self.session_key.decrypt(data)
    }

    pub async fn close(&self) {
        *self.state.write().await = SessionState::Disconnecting;
    }
}

// ── Wire framing ──────────────────────────────────────────────────
//
// Each packet: [ 4-byte big-endian length ] [ data ]

async fn send_framed(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    data: &[u8],
) -> Result<(), CoreError> {
    let len = (data.len() as u32).to_be_bytes();
    writer.write_all(&len).await?;
    writer.write_all(data).await?;
    Ok(())
}

async fn recv_framed(
    reader: &mut tokio::net::tcp::OwnedReadHalf,
) -> Result<Vec<u8>, CoreError> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len > 64 * 1024 * 1024 {
        return Err(CoreError::Protocol("Frame too large".into()));
    }
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    Ok(buf)
}

// ── Handshake helpers ─────────────────────────────────────────────

fn build_handshake(identity: &DeviceIdentity) -> HandshakePayload {
    let dh_pub    = identity.dh_public();
    let dh_bytes  = dh_pub.as_bytes();
    let sig       = identity.sign(dh_bytes);
    HandshakePayload {
        device_id:       identity.id,
        display_name:    identity.display_name.clone(),
        platform:        std::env::consts::OS.to_string(),
        version:         env!("CARGO_PKG_VERSION").to_string(),
        dh_pubkey_hex:   hex::encode(dh_bytes),
        sign_pubkey_hex: hex::encode(identity.verifying_key().as_bytes()),
        signature_hex:   hex::encode(sig.to_bytes()),
    }
}

/// Verify the Ed25519 signature on a HandshakePayload and return the peer's device ID.
fn verify_handshake(payload: &HandshakePayload) -> Result<Uuid, CoreError> {
    let sign_arr: [u8; 32] = hex::decode(&payload.sign_pubkey_hex)
        .map_err(|e| CoreError::Auth(format!("sign_pubkey_hex decode: {e}")))?
        .try_into()
        .map_err(|_| CoreError::Auth("sign pubkey must be 32 bytes".into()))?;
    let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&sign_arr)
        .map_err(|e| CoreError::Auth(format!("invalid verifying key: {e}")))?;

    let dh_bytes = hex::decode(&payload.dh_pubkey_hex)
        .map_err(|e| CoreError::Auth(format!("dh_pubkey_hex decode: {e}")))?;

    let sig_arr: [u8; 64] = hex::decode(&payload.signature_hex)
        .map_err(|e| CoreError::Auth(format!("signature_hex decode: {e}")))?
        .try_into()
        .map_err(|_| CoreError::Auth("signature must be 64 bytes".into()))?;
    let signature = ed25519_dalek::Signature::from_bytes(&sig_arr);

    verifying_key
        .verify_strict(&dh_bytes, &signature)
        .map_err(|e| CoreError::Auth(format!("handshake signature invalid: {e}")))?;

    Ok(payload.device_id)
}

fn derive_key(identity: &DeviceIdentity, peer_hs: &HandshakePayload) -> Result<SessionKey, CoreError> {
    let dh_arr: [u8; 32] = hex::decode(&peer_hs.dh_pubkey_hex)
        .map_err(|e| CoreError::Session(format!("dh_pubkey_hex decode: {e}")))?
        .try_into()
        .map_err(|_| CoreError::Session("DH pubkey must be 32 bytes".into()))?;
    let peer_pub  = X25519Public::from(dh_arr);
    let key_bytes = identity.derive_session_key(&peer_pub);
    Ok(SessionKey::from_bytes(&key_bytes))
}

// ── Background I/O loops ──────────────────────────────────────────

async fn send_loop(
    mut writer: tokio::net::tcp::OwnedWriteHalf,
    mut rx:     mpsc::Receiver<Vec<u8>>,
) {
    while let Some(packet) = rx.recv().await {
        let len = (packet.len() as u32).to_be_bytes();
        if writer.write_all(&len).await.is_err() { break; }
        if writer.write_all(&packet).await.is_err() { break; }
    }
    tracing::debug!("send loop ended");
}

async fn recv_loop(
    mut reader:  tokio::net::tcp::OwnedReadHalf,
    session_key: SessionKey,
    state:       Arc<RwLock<SessionState>>,
    peer_id:     Uuid,
) {
    loop {
        let mut len_buf = [0u8; 4];
        if reader.read_exact(&mut len_buf).await.is_err() { break; }
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > 64 * 1024 * 1024 { break; }
        let mut buf = vec![0u8; len];
        if reader.read_exact(&mut buf).await.is_err() { break; }

        match session_key.decrypt(&buf) {
            Ok(plaintext) => tracing::trace!(%peer_id, bytes = plaintext.len(), "recv"),
            Err(e)        => tracing::warn!(%peer_id, "decrypt failed: {e}"),
        }
    }
    *state.write().await = SessionState::Closed;
    tracing::info!(%peer_id, "session closed");
}

// ── Incoming connection handler ───────────────────────────────────

async fn handle_incoming(
    stream:   TcpStream,
    peer_addr: SocketAddr,
    identity: DeviceIdentity,
    sessions: Arc<RwLock<HashMap<Uuid, Arc<Session>>>>,
) -> Result<Uuid, CoreError> {
    let (mut reader, mut writer) = stream.into_split();

    // Listener receives first, then replies
    let peer_bytes = recv_framed(&mut reader).await?;
    let peer_hs: HandshakePayload = serde_json::from_slice(&peer_bytes)?;
    let peer_id = verify_handshake(&peer_hs)?;

    let our_bytes = serde_json::to_vec(&build_handshake(&identity))?;
    send_framed(&mut writer, &our_bytes).await?;

    let session_key = derive_key(&identity, &peer_hs)?;
    tracing::info!(%peer_id, %peer_addr, "incoming handshake complete");

    let (tx, rx) = mpsc::channel(256);
    let state    = Arc::new(RwLock::new(SessionState::Active));
    let session  = Arc::new(Session {
        id:             Uuid::new_v4(),
        peer_device_id: peer_id,
        config:         SessionConfig::default(),
        state:          Arc::clone(&state),
        session_key:    session_key.clone(),
        send_tx:        tx,
    });

    tokio::spawn(send_loop(writer, rx));
    tokio::spawn(recv_loop(reader, session_key, Arc::clone(&state), peer_id));

    sessions.write().await.insert(peer_id, Arc::clone(&session));
    Ok(peer_id)
}

// ── Session manager ───────────────────────────────────────────────

pub struct SessionManager {
    identity:    DeviceIdentity,
    listen_port: u16,
    sessions:    Arc<RwLock<HashMap<Uuid, Arc<Session>>>>,
}

impl SessionManager {
    pub fn new(identity: DeviceIdentity, listen_port: u16) -> Self {
        Self {
            identity,
            listen_port,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Bind a TCP listener for incoming peer connections.
    pub async fn start_listener(&self) -> Result<(), CoreError> {
        let addr     = format!("0.0.0.0:{}", self.listen_port);
        let sessions = Arc::clone(&self.sessions);
        let identity = self.identity.clone();

        tokio::spawn(async move {
            match tokio::net::TcpListener::bind(&addr).await {
                Ok(listener) => {
                    tracing::info!(%addr, "Peer listener started");
                    loop {
                        match listener.accept().await {
                            Ok((stream, peer_addr)) => {
                                tracing::info!(%peer_addr, "Incoming peer connection");
                                let s = Arc::clone(&sessions);
                                let id = identity.clone();
                                tokio::spawn(async move {
                                    if let Err(e) = handle_incoming(stream, peer_addr, id, s).await {
                                        tracing::warn!(%peer_addr, "Handshake failed: {e}");
                                    }
                                });
                            }
                            Err(e) => tracing::warn!("Accept error: {e}"),
                        }
                    }
                }
                Err(e) => tracing::error!("Failed to bind peer listener on {addr}: {e}"),
            }
        });

        Ok(())
    }

    /// Initiate a connection to a peer device over TCP.
    ///
    /// `peer_addr` is resolved externally (from LAN discovery or the cloud signaling API).
    /// Phase 2 will add WebRTC for internet-routed connections.
    pub async fn connect(&self, peer_id: Uuid, peer_addr: SocketAddr) -> Result<Arc<Session>, CoreError> {
        // Return existing active session if available
        if let Some(existing) = self.sessions.read().await.get(&peer_id) {
            if existing.is_active() {
                return Ok(Arc::clone(existing));
            }
        }

        tracing::info!(%peer_id, %peer_addr, "Initiating TCP connection");
        let stream = TcpStream::connect(peer_addr).await
            .map_err(|e| CoreError::Transport(format!("TCP connect to {peer_addr}: {e}")))?;

        let (mut reader, mut writer) = stream.into_split();

        // Initiator sends first, then receives
        let our_bytes = serde_json::to_vec(&build_handshake(&self.identity))?;
        send_framed(&mut writer, &our_bytes).await?;

        let peer_bytes = recv_framed(&mut reader).await?;
        let peer_hs: HandshakePayload = serde_json::from_slice(&peer_bytes)?;
        let actual_peer_id = verify_handshake(&peer_hs)?;

        if actual_peer_id != peer_id {
            return Err(CoreError::Auth(format!(
                "Peer ID mismatch: expected {peer_id}, got {actual_peer_id}"
            )));
        }

        let session_key = derive_key(&self.identity, &peer_hs)?;
        tracing::info!(%peer_id, "Handshake complete — session active");

        let (tx, rx) = mpsc::channel(256);
        let state    = Arc::new(RwLock::new(SessionState::Active));
        let session  = Arc::new(Session {
            id:             Uuid::new_v4(),
            peer_device_id: peer_id,
            config:         SessionConfig::default(),
            state:          Arc::clone(&state),
            session_key:    session_key.clone(),
            send_tx:        tx,
        });

        tokio::spawn(send_loop(writer, rx));
        tokio::spawn(recv_loop(reader, session_key, Arc::clone(&state), peer_id));

        self.sessions.write().await.insert(peer_id, Arc::clone(&session));
        Ok(session)
    }

    pub fn stop_all(&self) {
        if let Ok(sessions) = self.sessions.try_read() {
            for session in sessions.values() {
                let state = Arc::clone(&session.state);
                tokio::spawn(async move {
                    *state.write().await = SessionState::Disconnecting;
                });
            }
        }
    }

    pub async fn disconnect(&self, peer_id: Uuid) {
        if let Some(session) = self.sessions.read().await.get(&peer_id) {
            session.close().await;
        }
    }

    pub async fn active_sessions(&self) -> Vec<Arc<Session>> {
        self.sessions
            .read()
            .await
            .values()
            .filter(|s| s.is_active())
            .cloned()
            .collect()
    }
}
