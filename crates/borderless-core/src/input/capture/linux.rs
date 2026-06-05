//! Linux input capture via evdev + EVIOCGRAB.
//! Opens /dev/input/ devices, grabs exclusively, reads events.

#![cfg(target_os = "linux")]
use tokio::sync::mpsc::Sender;
use super::super::{InputCapture, InputEvent};
use crate::CoreError;

pub struct LinuxCapture;
impl LinuxCapture { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl InputCapture for LinuxCapture {
    async fn start(&self, _tx: Sender<InputEvent>) -> Result<(), CoreError> {
        // TODO:
        // 1. Enumerate /dev/input/event* devices
        // 2. Filter to keyboard (EV_KEY) and mouse (EV_REL / EV_ABS)
        // 3. Open each, call ioctl EVIOCGRAB for exclusive access
        // 4. Spawn async task reading evdev::Device::fetch_events()
        // 5. Translate evdev::Key codes to HID via lookup table
        Err(CoreError::InputCapture("Linux capture not yet implemented".into()))
    }
    fn stop(&self) {}
    fn set_suppress(&self, _: bool) {}
}
