//! Session lifecycle management.
//!
//! State machine: Connecting → Handshaking → Active → Disconnecting → Closed

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::{CoreError, identity::DeviceIdentity, crypto::SessionKey, input::InputEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType { Kvm, RemoteControl, ViewOnly }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub session_type:     SessionType,
    pub clipboard_sync:   bool,
    pub file_transfer:    bool,
    pub notification_sync: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self { session_type: SessionType::Kvm, clipboard_sync: true, file_transfer: true, notification_sync: true }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState { Connecting, Handshaking, Active, Disconnecting, Closed, Failed(String) }

pub struct Session {
    pub id:             Uuid,
    pub peer_device_id: Uuid,
    pub config:         SessionConfig,
    pub state:          Arc<RwLock<SessionState>>,
    pub session_key:    SessionKey,
    send_tx:            mpsc::Sender<Vec<u8>>,
}

impl Session {
    pub async fn current_state(&self) -> SessionState { self.state.read().await.clone() }

    pub fn is_active(&self) -> bool {
        matches!(self.state.try_read().map(|s| s.clone()), Ok(SessionState::Active))
    }

    pub async fn send_input(&self, event: &InputEvent) -> Result<(), CoreError> {
        let payload   = serde_json::to_vec(event)?;
        let encrypted = self.session_key.encrypt(&payload)?;
        self.send_tx.send(encrypted).await.map_err(|_| CoreError::NotConnected(self.peer_device_id))
    }

    pub async fn send_raw(&self, payload: &[u8]) -> Result<(), CoreError> {
        let encrypted = self.session_key.encrypt(payload)?;
        self.send_tx.send(encrypted).await.map_err(|_| CoreError::NotConnected(self.peer_device_id))
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CoreError> { self.session_key.decrypt(data) }

    pub async fn close(&self) { *self.state.write().await = SessionState::Disconnecting; }
}

pub struct SessionManager {
    identity:    DeviceIdentity,
    listen_port: u16,
    sessions:    Arc<RwLock<HashMap<Uuid, Arc<Session>>>>,
}

impl SessionManager {
    pub fn new(identity: DeviceIdentity, listen_port: u16) -> Self {
        Self { identity, listen_port, sessions: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn start_listener(&self) -> Result<(), CoreError> {
        let addr = format!("0.0.0.0:{}", self.listen_port);
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
                                // TODO: handshake, ECDH, create Session, spawn recv loop
                                let _ = stream;
                            }
                            Err(e) => tracing::warn!("Accept error: {}", e),
                        }
                    }
                }
                Err(e) => tracing::error!("Failed to bind peer listener on {}: {}", addr, e),
            }
        });
        Ok(())
    }

    pub async fn connect(&self, peer_id: Uuid) -> Result<Arc<Session>, CoreError> {
        if let Some(existing) = self.sessions.read().await.get(&peer_id) {
            if existing.is_active() { return Ok(Arc::clone(existing)); }
        }
        tracing::info!(%peer_id, "Initiating session");
        // TODO Phase 1: look up peer IP from discovery cache, open TCP stream, perform handshake:
        //   1. Send our DH public key + device ID (signed)
        //   2. Receive peer DH public key, verify signature
        //   3. Derive shared session key via ECDH + HKDF
        //   4. Confirm with encrypted ping/pong
        let (tx, _rx) = mpsc::channel(256);
        let session = Arc::new(Session {
            id:             Uuid::new_v4(),
            peer_device_id: peer_id,
            config:         SessionConfig::default(),
            state:          Arc::new(RwLock::new(SessionState::Connecting)),
            session_key:    SessionKey::from_bytes(&[0u8; 32]),
            send_tx:        tx,
        });
        self.sessions.write().await.insert(peer_id, Arc::clone(&session));
        Err(CoreError::Session("TCP transport not yet wired — LAN handshake coming in Phase 1".into()))
    }

    pub fn stop_all(&self) {
        if let Ok(sessions) = self.sessions.try_read() {
            for session in sessions.values() {
                let state = Arc::clone(&session.state);
                tokio::spawn(async move { *state.write().await = SessionState::Disconnecting; });
            }
        }
    }

    pub async fn active_sessions(&self) -> Vec<Arc<Session>> {
        self.sessions.read().await.values().filter(|s| s.is_active()).cloned().collect()
    }
}
