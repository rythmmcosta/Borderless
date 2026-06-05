#![cfg(target_os = "linux")]
use super::super::{InputEvent, InputInject};
use crate::CoreError;

pub struct LinuxInject;
impl LinuxInject { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl InputInject for LinuxInject {
    async fn inject(&self, _event: InputEvent) -> Result<(), CoreError> {
        // TODO: create uinput virtual device, write EV_KEY / EV_REL / EV_SYN events
        Err(CoreError::InputInject("Linux inject not yet implemented".into()))
    }
    fn screen_size(&self) -> (i32, i32) { (1920, 1080) }
}
