#![cfg(target_os = "macos")]
use super::super::{InputEvent, InputInject};
use crate::CoreError;

pub struct MacosInject;
impl MacosInject { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl InputInject for MacosInject {
    async fn inject(&self, _event: InputEvent) -> Result<(), CoreError> {
        // TODO: CGEventCreateMouseEvent / CGEventCreateKeyboardEvent
        Err(CoreError::InputInject("macOS inject not yet implemented".into()))
    }
    fn screen_size(&self) -> (i32, i32) { (1920, 1080) }
}
