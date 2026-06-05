//! Input subsystem — the core of Mouse Without Borders behaviour.

pub mod capture;
pub mod inject;
pub mod router;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputEvent {
    MouseMove(MouseMove),
    MouseButton(MouseButton),
    MouseScroll(MouseScroll),
    KeyPress(KeyPress),
    TextInput(TextInput),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MouseMove { pub x: i32, pub y: i32, pub dx: i32, pub dy: i32, pub screen_w: i32, pub screen_h: i32 }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MouseButton { pub button: Button, pub pressed: bool, pub x: i32, pub y: i32 }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Button { Left, Right, Middle, X1, X2 }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MouseScroll { pub delta_x: f32, pub delta_y: f32 }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPress { pub keycode: u32, pub modifiers: Modifiers, pub pressed: bool }

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Modifiers { pub ctrl: bool, pub shift: bool, pub alt: bool, pub meta: bool }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextInput { pub text: String }

#[async_trait::async_trait]
pub trait InputCapture: Send + Sync {
    async fn start(&self, tx: tokio::sync::mpsc::Sender<InputEvent>) -> Result<(), crate::CoreError>;
    fn stop(&self);
    fn set_suppress(&self, suppress: bool);
}

#[async_trait::async_trait]
pub trait InputInject: Send + Sync {
    async fn inject(&self, event: InputEvent) -> Result<(), crate::CoreError>;
    fn screen_size(&self) -> (i32, i32);
}

#[cfg(target_os = "windows")] pub use capture::windows::WindowsCapture as PlatformCapture;
#[cfg(target_os = "windows")] pub use inject::windows::WindowsInject   as PlatformInject;
#[cfg(target_os = "macos")]   pub use capture::macos::MacosCapture as PlatformCapture;
#[cfg(target_os = "macos")]   pub use inject::macos::MacosInject   as PlatformInject;
#[cfg(target_os = "linux")]   pub use capture::linux::LinuxCapture as PlatformCapture;
#[cfg(target_os = "linux")]   pub use inject::linux::LinuxInject   as PlatformInject;
