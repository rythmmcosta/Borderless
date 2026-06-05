//! Unified error type for borderless-core.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Identity error: {0}")]
    Identity(String),

    #[error("Input capture error: {0}")]
    InputCapture(String),

    #[error("Input injection error: {0}")]
    InputInject(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Discovery error: {0}")]
    Discovery(String),

    #[error("Transfer error: {0}")]
    Transfer(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Not connected to peer {0}")]
    NotConnected(uuid::Uuid),

    #[error("Peer not found: {0}")]
    PeerNotFound(uuid::Uuid),

    #[error("Auth error: {0}")]
    Auth(String),

    #[error("Clipboard error: {0}")]
    Clipboard(String),
}

impl From<aes_gcm::Error> for CoreError {
    fn from(e: aes_gcm::Error) -> Self {
        CoreError::Crypto(e.to_string())
    }
}
