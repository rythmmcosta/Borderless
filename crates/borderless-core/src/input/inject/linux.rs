//! Linux input injection via uinput (kernel virtual device).
//! Requires /dev/uinput write access (add user to `input` group or use udev rules).

#![cfg(target_os = "linux")]

use std::sync::Mutex;
use evdev::{
    uinput::VirtualDeviceBuilder,
    AttributeSet, InputEvent, EventType,
    Key, RelativeAxisType,
};
use super::super::{Button, InputEvent as BLEvent, InputInject};
use crate::CoreError;

pub struct LinuxInject {
    device: Mutex<Option<evdev::uinput::VirtualDevice>>,
}

impl LinuxInject {
    pub fn new() -> Self {
        Self { device: Mutex::new(None) }
    }
}

#[async_trait::async_trait]
impl InputInject for LinuxInject {
    async fn inject(&self, event: BLEvent) -> Result<(), CoreError> {
        let mut lock = self.device.lock().map_err(|e| CoreError::InputInject(e.to_string()))?;
        if lock.is_none() {
            *lock = Some(create_virtual_device()?);
        }
        let dev = lock.as_mut().unwrap();
        inject_inner(dev, event)
    }

    fn screen_size(&self) -> (i32, i32) {
        use x11rb::connection::Connection;
        if let Ok((conn, n)) = x11rb::rust_connection::RustConnection::connect(None) {
            let s = &conn.setup().roots[n];
            return (s.width_in_pixels as i32, s.height_in_pixels as i32);
        }
        (1920, 1080)
    }
}

fn inject_inner(dev: &mut evdev::uinput::VirtualDevice, event: BLEvent) -> Result<(), CoreError> {
    let syn = InputEvent::new(EventType::SYNCHRONIZATION, 0, 0);

    match event {
        BLEvent::MouseMove(mv) => {
            let mut evs = vec![];
            if mv.dx != 0 {
                evs.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, mv.dx));
            }
            if mv.dy != 0 {
                evs.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, mv.dy));
            }
            if evs.is_empty() { return Ok(()); }
            evs.push(syn);
            dev.emit(&evs).map_err(|e| CoreError::InputInject(e.to_string()))?;
        }

        BLEvent::MouseButton(mb) => {
            let btn_code = mouse_button_code(&mb.button);
            let value    = if mb.pressed { 1 } else { 0 };
            dev.emit(&[
                InputEvent::new(EventType::KEY, btn_code, value),
                syn,
            ]).map_err(|e| CoreError::InputInject(e.to_string()))?;
        }

        BLEvent::MouseScroll(s) => {
            let mut evs = vec![];
            if s.delta_y != 0.0 {
                evs.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_WHEEL.0, s.delta_y.round() as i32));
            }
            if s.delta_x != 0.0 {
                evs.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_HWHEEL.0, s.delta_x.round() as i32));
            }
            if evs.is_empty() { return Ok(()); }
            evs.push(syn);
            dev.emit(&evs).map_err(|e| CoreError::InputInject(e.to_string()))?;
        }

        BLEvent::KeyPress(kp) => {
            let evdev_code = hid_to_evdev_key(kp.keycode) as u16;
            let value      = if kp.pressed { 1 } else { 0 };
            dev.emit(&[
                InputEvent::new(EventType::KEY, evdev_code, value),
                syn,
            ]).map_err(|e| CoreError::InputInject(e.to_string()))?;
        }

        BLEvent::TextInput(ti) => {
            // Best-effort: type each character as a keypress sequence
            for ch in ti.text.chars() {
                if let Some(code) = char_to_evdev_key(ch) {
                    dev.emit(&[
                        InputEvent::new(EventType::KEY, code, 1),
                        InputEvent::new(EventType::SYNCHRONIZATION, 0, 0),
                        InputEvent::new(EventType::KEY, code, 0),
                        InputEvent::new(EventType::SYNCHRONIZATION, 0, 0),
                    ]).map_err(|e| CoreError::InputInject(e.to_string()))?;
                }
            }
        }
    }

    Ok(())
}

fn create_virtual_device() -> Result<evdev::uinput::VirtualDevice, CoreError> {
    let mut keys = AttributeSet::<Key>::new();
    // Standard keyboard keys (ESC through F12)
    for code in 1u16..=88  { keys.insert(Key(code)); }
    // Extended modifier keys
    for code in [97u16, 100, 125, 126] { keys.insert(Key(code)); }
    // Mouse buttons: BTN_LEFT=0x110..BTN_TASK=0x117
    for code in 0x110u16..=0x117 { keys.insert(Key(code)); }

    let mut rel = AttributeSet::<RelativeAxisType>::new();
    rel.insert(RelativeAxisType::REL_X);
    rel.insert(RelativeAxisType::REL_Y);
    rel.insert(RelativeAxisType::REL_WHEEL);
    rel.insert(RelativeAxisType::REL_HWHEEL);

    VirtualDeviceBuilder::new()
        .map_err(|e| CoreError::InputInject(e.to_string()))?
        .name("Borderless Virtual Input")
        .with_keys(&keys)
        .map_err(|e| CoreError::InputInject(e.to_string()))?
        .with_relative_axes(&rel)
        .map_err(|e| CoreError::InputInject(e.to_string()))?
        .build()
        .map_err(|e| CoreError::InputInject(e.to_string()))
}

fn mouse_button_code(btn: &Button) -> u16 {
    match btn {
        Button::Left   => 0x110, // BTN_LEFT
        Button::Right  => 0x111, // BTN_RIGHT
        Button::Middle => 0x112, // BTN_MIDDLE
        Button::X1     => 0x113, // BTN_SIDE
        Button::X2     => 0x114, // BTN_EXTRA
    }
}

/// USB HID usage code → Linux input.h key code.
pub fn hid_to_evdev_key(hid: u32) -> u32 {
    match hid {
        0x04 => 30, 0x05 => 48, 0x06 => 46, 0x07 => 32, 0x08 => 18, // A B C D E
        0x09 => 33, 0x0A => 34, 0x0B => 35, 0x0C => 23, 0x0D => 36, // F G H I J
        0x0E => 37, 0x0F => 38, 0x10 => 50, 0x11 => 49, 0x12 => 24, // K L M N O
        0x13 => 25, 0x14 => 16, 0x15 => 19, 0x16 => 31, 0x17 => 20, // P Q R S T
        0x18 => 22, 0x19 => 47, 0x1A => 17, 0x1B => 45, 0x1C => 21, // U V W X Y
        0x1D => 44,                                                   // Z
        0x1E => 2,  0x1F => 3,  0x20 => 4,  0x21 => 5,  0x22 => 6,  // 1-5
        0x23 => 7,  0x24 => 8,  0x25 => 9,  0x26 => 10, 0x27 => 11, // 6-0
        0x28 => 28, 0x29 => 1,  0x2A => 14, 0x2B => 15, 0x2C => 57, // Ret Esc BS Tab Spc
        0x2D => 12, 0x2E => 13, 0x2F => 26, 0x30 => 27, 0x31 => 43, // - = [ ] \
        0x33 => 39, 0x34 => 40, 0x35 => 41, 0x36 => 51, 0x37 => 52, // ; ' ` , .
        0x38 => 53,                                                   // /
        0x3A => 59, 0x3B => 60, 0x3C => 61, 0x3D => 62, 0x3E => 63, // F1-F5
        0x3F => 64, 0x40 => 65, 0x41 => 66, 0x42 => 67, 0x43 => 68, // F6-F10
        0x44 => 87, 0x45 => 88,                                      // F11 F12
        0x50 => 105, 0x52 => 103, 0x4F => 106, 0x51 => 108,         // ← ↑ → ↓
        0x4A => 102, 0x4D => 107, 0x4B => 104, 0x4E => 109,         // Home End PgUp PgDn
        0x49 => 110, 0x4C => 111,                                    // Insert Delete
        0xE0 => 29,  0xE1 => 42,  0xE2 => 56,  0xE3 => 125,         // LCtrl LShft LAlt LMeta
        0xE4 => 97,  0xE5 => 54,  0xE6 => 100, 0xE7 => 126,         // RCtrl RShft RAlt RMeta
        _ => hid,
    }
}

fn char_to_evdev_key(ch: char) -> Option<u16> {
    match ch {
        'a'..='z' => Some(hid_to_evdev_key(0x04 + (ch as u32 - 'a' as u32)) as u16),
        'A'..='Z' => Some(hid_to_evdev_key(0x04 + (ch as u32 - 'A' as u32)) as u16),
        '0'       => Some(11),
        '1'..='9' => Some(2 + (ch as u16 - '1' as u16)),
        ' '       => Some(57),
        '\n'      => Some(28),
        '\t'      => Some(15),
        _         => None,
    }
}
