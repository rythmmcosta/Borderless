//! macOS input capture via CGEventTap.
//! Requires "Accessibility" permission in System Preferences.

#![cfg(target_os = "macos")]
use tokio::sync::mpsc::Sender;
use super::super::{InputCapture, InputEvent};
use crate::CoreError;

pub struct MacosCapture;
impl MacosCapture { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl InputCapture for MacosCapture {
    async fn start(&self, _tx: Sender<InputEvent>) -> Result<(), CoreError> {
        // TODO: CGEventTapCreate with kCGSessionEventTap
        // Needs NSAccessibilityTrustedCheckWithPrompt check first
        Err(CoreError::InputCapture("macOS capture not yet implemented".into()))
    }
    fn stop(&self) {}
    fn set_suppress(&self, _: bool) {}
}
