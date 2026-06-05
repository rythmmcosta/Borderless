//! macOS input injection via CGEvent.

#![cfg(target_os = "macos")]

use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, EventField};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::display::CGDisplay;
use core_graphics::geometry::CGPoint;
use super::super::{Button, InputEvent, InputInject, MouseButton};
use crate::CoreError;

pub struct MacosInject;
impl MacosInject { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl InputInject for MacosInject {
    async fn inject(&self, event: InputEvent) -> Result<(), CoreError> {
        inject_inner(event).map_err(|e| CoreError::InputInject(e))
    }

    fn screen_size(&self) -> (i32, i32) {
        let d = CGDisplay::main();
        (d.pixels_wide() as i32, d.pixels_high() as i32)
    }
}

fn inject_inner(event: InputEvent) -> Result<(), String> {
    let src = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        .map_err(|_| "CGEventSource::new failed")?;

    match event {
        InputEvent::MouseMove(mv) => {
            let pt = CGPoint { x: mv.x as f64, y: mv.y as f64 };
            let ev = CGEvent::new_mouse_event(src, CGEventType::MouseMoved, pt, CGMouseButton::Left)
                .map_err(|_| "new_mouse_event failed")?;
            ev.post(CGEventTapLocation::HIDEventTap);
        }

        InputEvent::MouseButton(mb) => {
            let pt = CGPoint { x: mb.x as f64, y: mb.y as f64 };
            let (ev_down, ev_up, cg_btn) = match mb.button {
                Button::Left   => (CGEventType::LeftMouseDown,   CGEventType::LeftMouseUp,   CGMouseButton::Left),
                Button::Right  => (CGEventType::RightMouseDown,  CGEventType::RightMouseUp,  CGMouseButton::Right),
                Button::Middle => (CGEventType::OtherMouseDown,  CGEventType::OtherMouseUp,  CGMouseButton::Center),
                Button::X1     => (CGEventType::OtherMouseDown,  CGEventType::OtherMouseUp,  CGMouseButton::Center),
                Button::X2     => (CGEventType::OtherMouseDown,  CGEventType::OtherMouseUp,  CGMouseButton::Center),
            };
            let ev_type = if mb.pressed { ev_down } else { ev_up };
            let ev = CGEvent::new_mouse_event(src, ev_type, pt, cg_btn)
                .map_err(|_| "new_mouse_event failed")?;
            if matches!(mb.button, Button::Middle | Button::X1 | Button::X2) {
                let btn_num: i64 = match mb.button { Button::Middle => 2, Button::X1 => 3, _ => 4 };
                ev.set_integer_value_field(EventField(0), btn_num);
            }
            ev.post(CGEventTapLocation::HIDEventTap);
        }

        InputEvent::MouseScroll(s) => {
            // kCGScrollEventUnitLine = 1; create scroll event with line deltas
            let ev = CGEvent::new_scroll_event(
                src,
                core_graphics::event::ScrollEventUnit::Line,
                2,
                s.delta_y as i32,
                s.delta_x as i32,
                0,
            ).map_err(|_| "new_scroll_event failed")?;
            ev.post(CGEventTapLocation::HIDEventTap);
        }

        InputEvent::KeyPress(kp) => {
            let cg_code = hid_to_cg_keycode(kp.keycode) as u16;
            let ev = CGEvent::new_keyboard_event(src, cg_code, kp.pressed)
                .map_err(|_| "new_keyboard_event failed")?;
            ev.post(CGEventTapLocation::HIDEventTap);
        }

        InputEvent::TextInput(ti) => {
            // Post individual Unicode keypresses for each char
            let src2 = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
                .map_err(|_| "CGEventSource::new failed")?;
            if let Ok(ev) = CGEvent::new_keyboard_event(src2, 0, true) {
                let chars: Vec<u16> = ti.text.encode_utf16().collect();
                ev.set_string_from_utf16_unchecked(&chars);
                ev.post(CGEventTapLocation::HIDEventTap);
            }
        }
    }

    Ok(())
}

/// USB HID usage code → macOS Carbon virtual keycode.
pub fn hid_to_cg_keycode(hid: u32) -> u32 {
    match hid {
        0x04 => 0x00, 0x05 => 0x0B, 0x06 => 0x08, 0x07 => 0x02, // A B C D
        0x08 => 0x0E, 0x09 => 0x03, 0x0A => 0x05, 0x0B => 0x04, // E F G H
        0x0C => 0x22, 0x0D => 0x26, 0x0E => 0x28, 0x0F => 0x25, // I J K L
        0x10 => 0x2E, 0x11 => 0x2D, 0x12 => 0x1F, 0x13 => 0x23, // M N O P
        0x14 => 0x0C, 0x15 => 0x0F, 0x16 => 0x01, 0x17 => 0x11, // Q R S T
        0x18 => 0x20, 0x19 => 0x09, 0x1A => 0x0D, 0x1B => 0x07, // U V W X
        0x1C => 0x10, 0x1D => 0x06,                              // Y Z
        0x1E => 0x12, 0x1F => 0x13, 0x20 => 0x14, 0x21 => 0x15, // 1 2 3 4
        0x22 => 0x17, 0x23 => 0x16, 0x24 => 0x1A, 0x25 => 0x1C, // 5 6 7 8
        0x26 => 0x19, 0x27 => 0x1D,                              // 9 0
        0x28 => 0x24, 0x29 => 0x35, 0x2A => 0x33, 0x2B => 0x30, // Ret Esc BS Tab
        0x2C => 0x31, 0x2D => 0x1B, 0x2E => 0x18,               // Space - =
        0x2F => 0x21, 0x30 => 0x1E, 0x31 => 0x2A, 0x33 => 0x29, // [ ] \ ;
        0x34 => 0x27, 0x35 => 0x32, 0x36 => 0x2B, 0x37 => 0x2F, // ' ` , .
        0x38 => 0x2C,                                             // /
        0x3A => 0x7A, 0x3B => 0x78, 0x3C => 0x63, 0x3D => 0x76, // F1-F4
        0x3E => 0x60, 0x3F => 0x61, 0x40 => 0x62, 0x41 => 0x64, // F5-F8
        0x42 => 0x65, 0x43 => 0x6D, 0x44 => 0x67, 0x45 => 0x6F, // F9-F12
        0x50 => 0x7B, 0x52 => 0x7E, 0x4F => 0x7C, 0x51 => 0x7D, // ← ↑ → ↓
        _ => hid,
    }
}
