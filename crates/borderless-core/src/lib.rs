//! # Borderless Core
//!
//! The shared Rust library compiled for every platform.
//! Provides: device identity, E2E crypto, input routing (KVM),
//! clipboard sync, file transfer, LAN discovery, and session management.

pub mod crypto;
pub mod error;
pub mod identity;
pub mod input;
pub mod clipboard;
pub mod discovery;
pub mod session;
pub mod protocol;
pub mod transfer;

pub use error::CoreError;
pub use identity::DeviceIdentity;
pub use session::{Session, SessionConfig, SessionState, SessionType};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top-level engine — one instance per running device.
/// Owns the state machine, discovery, and all active sessions.
pub struct BorderlessEngine {
    pub identity: DeviceIdentity,
    pub config:   EngineConfig,
    pub discovery: discovery::DiscoveryService,
    session_mgr:  session::SessionManager,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EngineConfig {
    /// Remote API base URL
    pub api_url:          String,
    /// Human-readable name shown to peers
    pub device_name:      String,
    /// Sync features
    pub clipboard_sync:   bool,
    pub file_transfer:    bool,
    pub notification_sync: bool,
    /// Port to listen on for peer connections
    pub listen_port:      u16,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            api_url:           "https://api.borderless.app/v1".into(),
            device_name:       hostname::get()
                                   .map(|h| h.to_string_lossy().to_string())
                                   .unwrap_or_else(|_| "My Device".into()),
            clipboard_sync:    true,
            file_transfer:     true,
            notification_sync: true,
            listen_port:       49152,
        }
    }
}

impl BorderlessEngine {
    pub async fn new(config: EngineConfig) -> Result<Self, CoreError> {
        let identity    = DeviceIdentity::load_or_create()?;
        let discovery   = discovery::DiscoveryService::new(&identity);
        let session_mgr = session::SessionManager::new(identity.clone(), config.listen_port);
        Ok(Self { identity, config, discovery, session_mgr })
    }

    /// Start LAN discovery + peer listener.
    pub async fn start(&mut self) -> Result<(), CoreError> {
        self.discovery.start().await?;
        self.session_mgr.start_listener().await?;
        tracing::info!(
            device_id = %self.identity.id,
            name       = %self.identity.display_name,
            "Borderless engine started"
        );
        Ok(())
    }

    /// Connect to a peer and return the live session handle.
    pub async fn connect(&mut self, peer_id: uuid::Uuid) -> Result<Arc<Session>, CoreError> {
        self.session_mgr.connect(peer_id).await
    }

    pub fn discovered_devices(&self) -> Vec<discovery::DiscoveredPeer> {
        self.discovery.peers()
    }

    pub fn stop(&mut self) {
        self.session_mgr.stop_all();
        self.discovery.stop();
        tracing::info!("Borderless engine stopped");
    }
}
