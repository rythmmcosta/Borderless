//! Windows input injection via SendInput.

#![cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use super::super::{Button, InputEvent, InputInject, MouseButton};
use crate::CoreError;

pub struct WindowsInject;
impl WindowsInject { pub fn new() -> Self { Self } }

#[async_trait::async_trait]
impl InputInject for WindowsInject {
    async fn inject(&self, event: InputEvent) -> Result<(), CoreError> { unsafe { inject_inner(event) } }
    fn screen_size(&self) -> (i32, i32) { unsafe { (GetSystemMetrics(SM_CXVIRTUALSCREEN), GetSystemMetrics(SM_CYVIRTUALSCREEN)) } }
}

unsafe fn inject_inner(event: InputEvent) -> Result<(), CoreError> {
    match event {
        InputEvent::MouseMove(mv) => {
            let (sw, sh) = (GetSystemMetrics(SM_CXVIRTUALSCREEN), GetSystemMetrics(SM_CYVIRTUALSCREEN));
            let ax = (mv.x as f64 / sw as f64 * 65535.0).round() as i32;
            let ay = (mv.y as f64 / sh as f64 * 65535.0).round() as i32;
            send_input(&[INPUT { r#type: INPUT_MOUSE, Anonymous: INPUT_0 { mi: MOUSEINPUT { dx: ax, dy: ay, mouseData: 0, dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK, time: 0, dwExtraInfo: 0 } } }]);
        }
        InputEvent::MouseButton(mb) => {
            let flags = button_flags(&mb);
            send_input(&[INPUT { r#type: INPUT_MOUSE, Anonymous: INPUT_0 { mi: MOUSEINPUT { dx: 0, dy: 0, mouseData: 0, dwFlags: flags, time: 0, dwExtraInfo: 0 } } }]);
        }
        InputEvent::MouseScroll(scroll) => {
            let data = (scroll.delta_y * 120.0).round() as i32 as u32;
            send_input(&[INPUT { r#type: INPUT_MOUSE, Anonymous: INPUT_0 { mi: MOUSEINPUT { dx: 0, dy: 0, mouseData: data, dwFlags: MOUSEEVENTF_WHEEL, time: 0, dwExtraInfo: 0 } } }]);
        }
        InputEvent::KeyPress(kp) => {
            let vk = hid_to_vk(kp.keycode);
            let flags = if kp.pressed { KEYEVENTF_SCANCODE } else { KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP };
            send_input(&[INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(vk as u16), wScan: 0, dwFlags: flags, time: 0, dwExtraInfo: 0 } } }]);
        }
        InputEvent::TextInput(ti) => {
            for ch in ti.text.encode_utf16() {
                let down = INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0), wScan: ch, dwFlags: KEYEVENTF_UNICODE, time: 0, dwExtraInfo: 0 } } };
                let up   = INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0), wScan: ch, dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 } } };
                send_input(&[down, up]);
            }
        }
    }
    Ok(())
}

unsafe fn send_input(inputs: &[INPUT]) { SendInput(inputs, std::mem::size_of::<INPUT>() as i32); }

fn button_flags(mb: &MouseButton) -> MOUSE_EVENT_FLAGS {
    match (&mb.button, mb.pressed) {
        (Button::Left,   true)  => MOUSEEVENTF_LEFTDOWN,
        (Button::Left,   false) => MOUSEEVENTF_LEFTUP,
        (Button::Right,  true)  => MOUSEEVENTF_RIGHTDOWN,
        (Button::Right,  false) => MOUSEEVENTF_RIGHTUP,
        (Button::Middle, true)  => MOUSEEVENTF_MIDDLEDOWN,
        (Button::Middle, false) => MOUSEEVENTF_MIDDLEUP,
        _ => MOUSE_EVENT_FLAGS(0),
    }
}

fn hid_to_vk(hid: u32) -> u32 {
    match hid {
        0x04..=0x1D => hid - 0x04 + 0x41,
        0x27 => 0x30, 0x1E..=0x26 => hid - 0x1E + 0x31,
        0x28 => 0x0D, 0x29 => 0x1B, 0x2A => 0x08, 0x2B => 0x09, 0x2C => 0x20,
        0x3A..=0x45 => hid - 0x3A + 0x70,
        0x50 => 0x25, 0x52 => 0x26, 0x4F => 0x27, 0x51 => 0x28,
        0x49 => 0x2D, 0x4C => 0x2E, 0x4A => 0x24, 0x4D => 0x23,
        0x4B => 0x21, 0x4E => 0x22,
        _ => hid,
    }
}
