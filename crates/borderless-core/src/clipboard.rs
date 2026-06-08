//! Clipboard synchronization engine.
//! Polls local clipboard every 250ms, encrypts changes, sends to paired devices.

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, Duration};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::CoreError;
use crate::crypto::sha256_hex;

const POLL_MS:        u64   = 250;
const MAX_SYNC_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text(String),
    Html { html: String, text: Option<String> },
    Image { data: Vec<u8>, mime: String },
    FilePaths(Vec<String>),
}

impl ClipboardContent {
    pub fn hash(&self) -> String {
        match self {
            Self::Text(t)           => sha256_hex(t.as_bytes()),
            Self::Html { html, .. } => sha256_hex(html.as_bytes()),
            Self::Image { data, .. }=> sha256_hex(data),
            Self::FilePaths(f)      => sha256_hex(f.join("\0").as_bytes()),
        }
    }
    pub fn byte_len(&self) -> usize {
        match self {
            Self::Text(t)           => t.len(),
            Self::Html { html, .. } => html.len(),
            Self::Image { data, .. }=> data.len(),
            Self::FilePaths(f)      => f.iter().map(|p| p.len()).sum(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardChange {
    pub source_device_id: Uuid,
    pub content:          ClipboardContent,
    pub timestamp:        chrono::DateTime<chrono::Utc>,
}

pub struct ClipboardMonitor {
    device_id: Uuid,
    last_hash: Arc<Mutex<Option<String>>>,
    change_tx: mpsc::Sender<ClipboardChange>,
}

impl ClipboardMonitor {
    pub fn new(device_id: Uuid) -> (Self, mpsc::Receiver<ClipboardChange>) {
        let (tx, rx) = mpsc::channel(64);
        (Self { device_id, last_hash: Arc::new(Mutex::new(None)), change_tx: tx }, rx)
    }

    pub fn start_polling(&self) {
        let last_hash = Arc::clone(&self.last_hash);
        let tx        = self.change_tx.clone();
        let device_id = self.device_id;
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_millis(POLL_MS));
            loop {
                tick.tick().await;
                match platform_read() {
                    Ok(content) => {
                        if content.byte_len() > MAX_SYNC_BYTES { continue; }
                        let hash = content.hash();
                        let mut last = last_hash.lock().await;
                        if last.as_deref() != Some(&hash) {
                            *last = Some(hash);
                            if tx.send(ClipboardChange { source_device_id: device_id, content, timestamp: chrono::Utc::now() }).await.is_err() { break; }
                        }
                    }
                    Err(e) => tracing::trace!("Clipboard read: {}", e),
                }
            }
        });
    }

    pub async fn apply_remote(&self, change: &ClipboardChange) -> Result<(), CoreError> {
        if change.source_device_id == self.device_id { return Ok(()); }
        *self.last_hash.lock().await = Some(change.content.hash());
        platform_write(&change.content)
    }
}

#[cfg(target_os = "windows")]
fn platform_read() -> Result<ClipboardContent, CoreError> {
    use clipboard_win::get_clipboard_string;
    Ok(ClipboardContent::Text(get_clipboard_string().map_err(|e| CoreError::Clipboard(e.to_string()))?))
}
#[cfg(target_os = "windows")]
fn platform_write(content: &ClipboardContent) -> Result<(), CoreError> {
    use clipboard_win::set_clipboard_string;
    match content {
        ClipboardContent::Text(t) => set_clipboard_string(t).map_err(|e| CoreError::Clipboard(e.to_string())),
        _ => Err(CoreError::Clipboard("Non-text not yet supported on Windows".into())),
    }
}

#[cfg(target_os = "linux")]
fn platform_read() -> Result<ClipboardContent, CoreError> {
    let mut cb = arboard::Clipboard::new().map_err(|e| CoreError::Clipboard(e.to_string()))?;
    let text = cb.get_text().map_err(|e| CoreError::Clipboard(e.to_string()))?;
    Ok(ClipboardContent::Text(text))
}
#[cfg(target_os = "linux")]
fn platform_write(content: &ClipboardContent) -> Result<(), CoreError> {
    match content {
        ClipboardContent::Text(t) => {
            let mut cb = arboard::Clipboard::new().map_err(|e| CoreError::Clipboard(e.to_string()))?;
            cb.set_text(t.clone()).map_err(|e| CoreError::Clipboard(e.to_string()))
        }
        _ => Err(CoreError::Clipboard("Only text clipboard supported on Linux".into())),
    }
}

#[cfg(target_os = "macos")]
fn platform_read() -> Result<ClipboardContent, CoreError> {
    let mut cb = arboard::Clipboard::new().map_err(|e| CoreError::Clipboard(e.to_string()))?;
    let text = cb.get_text().map_err(|e| CoreError::Clipboard(e.to_string()))?;
    Ok(ClipboardContent::Text(text))
}
#[cfg(target_os = "macos")]
fn platform_write(content: &ClipboardContent) -> Result<(), CoreError> {
    match content {
        ClipboardContent::Text(t) => {
            let mut cb = arboard::Clipboard::new().map_err(|e| CoreError::Clipboard(e.to_string()))?;
            cb.set_text(t.clone()).map_err(|e| CoreError::Clipboard(e.to_string()))
        }
        _ => Err(CoreError::Clipboard("Only text clipboard supported on macOS".into())),
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn platform_read() -> Result<ClipboardContent, CoreError> { Err(CoreError::Clipboard("Clipboard not available".into())) }
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn platform_write(_: &ClipboardContent) -> Result<(), CoreError> { Err(CoreError::Clipboard("Clipboard not available".into())) }
