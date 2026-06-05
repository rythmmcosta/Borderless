//! Windows input capture via WH_MOUSE_LL + WH_KEYBOARD_LL global hooks.

#![cfg(target_os = "windows")]

use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::sync::mpsc::Sender;
use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use super::super::{Button, InputCapture, InputEvent, KeyPress, Modifiers, MouseButton, MouseMove, MouseScroll};
use crate::CoreError;

pub struct WindowsCapture { running: Arc<AtomicBool>, suppress: Arc<AtomicBool> }

impl WindowsCapture {
    pub fn new() -> Self { Self { running: Arc::new(AtomicBool::new(false)), suppress: Arc::new(AtomicBool::new(false)) } }
}

#[async_trait::async_trait]
impl InputCapture for WindowsCapture {
    async fn start(&self, tx: Sender<InputEvent>) -> Result<(), CoreError> {
        if self.running.swap(true, Ordering::SeqCst) { return Ok(()); }
        let running  = Arc::clone(&self.running);
        let suppress = Arc::clone(&self.suppress);
        std::thread::Builder::new().name("borderless-hook".into()).spawn(move || unsafe {
            HOOK_CTX.with(|ctx| { *ctx.borrow_mut() = Some(HookContext { tx, suppress }); });
            let mouse_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), None, 0).expect("WH_MOUSE_LL");
            let kb_hook   = SetWindowsHookExW(WH_KEYBOARD_LL, Some(kb_proc), None, 0).expect("WH_KEYBOARD_LL");
            let mut msg = MSG::default();
            while running.load(Ordering::SeqCst) {
                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    TranslateMessage(&msg); DispatchMessageW(&msg);
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            let _ = UnhookWindowsHookEx(mouse_hook);
            let _ = UnhookWindowsHookEx(kb_hook);
        }).map_err(|e| CoreError::InputCapture(e.to_string()))?;
        Ok(())
    }
    fn stop(&self) { self.running.store(false, Ordering::SeqCst); }
    fn set_suppress(&self, suppress: bool) { self.suppress.store(suppress, Ordering::SeqCst); }
}

struct HookContext { tx: Sender<InputEvent>, suppress: Arc<AtomicBool> }
thread_local! { static HOOK_CTX: std::cell::RefCell<Option<HookContext>> = std::cell::RefCell::new(None); }

fn send_event(event: InputEvent) { HOOK_CTX.with(|ctx| { if let Some(c) = ctx.borrow().as_ref() { let _ = c.tx.try_send(event); } }); }
fn is_suppressed() -> bool { HOOK_CTX.with(|ctx| ctx.borrow().as_ref().map(|c| c.suppress.load(Ordering::SeqCst)).unwrap_or(false)) }

unsafe extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let info = &*(lparam.0 as *const MSLLHOOKSTRUCT);
        let (sw, sh) = (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN));
        let event = match wparam.0 as u32 {
            WM_MOUSEMOVE    => Some(InputEvent::MouseMove(MouseMove { x: info.pt.x, y: info.pt.y, dx: 0, dy: 0, screen_w: sw, screen_h: sh })),
            WM_LBUTTONDOWN  => Some(InputEvent::MouseButton(MouseButton { button: Button::Left,   pressed: true,  x: info.pt.x, y: info.pt.y })),
            WM_LBUTTONUP    => Some(InputEvent::MouseButton(MouseButton { button: Button::Left,   pressed: false, x: info.pt.x, y: info.pt.y })),
            WM_RBUTTONDOWN  => Some(InputEvent::MouseButton(MouseButton { button: Button::Right,  pressed: true,  x: info.pt.x, y: info.pt.y })),
            WM_RBUTTONUP    => Some(InputEvent::MouseButton(MouseButton { button: Button::Right,  pressed: false, x: info.pt.x, y: info.pt.y })),
            WM_MBUTTONDOWN  => Some(InputEvent::MouseButton(MouseButton { button: Button::Middle, pressed: true,  x: info.pt.x, y: info.pt.y })),
            WM_MBUTTONUP    => Some(InputEvent::MouseButton(MouseButton { button: Button::Middle, pressed: false, x: info.pt.x, y: info.pt.y })),
            WM_MOUSEWHEEL   => { let d = (info.mouseData >> 16) as i16 as f32 / 120.0; Some(InputEvent::MouseScroll(MouseScroll { delta_x: 0.0, delta_y: d })) }
            WM_MOUSEHWHEEL  => { let d = (info.mouseData >> 16) as i16 as f32 / 120.0; Some(InputEvent::MouseScroll(MouseScroll { delta_x: d,   delta_y: 0.0 })) }
            _ => None,
        };
        if let Some(ev) = event {
            send_event(ev);
            if is_suppressed() { return LRESULT(1); }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

unsafe extern "system" fn kb_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let info    = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        let pressed = matches!(wparam.0 as u32, WM_KEYDOWN | WM_SYSKEYDOWN);
        let modifiers = Modifiers {
            ctrl:  (GetAsyncKeyState(VK_CONTROL.0 as i32) as u16 & 0x8000) != 0,
            shift: (GetAsyncKeyState(VK_SHIFT.0 as i32)   as u16 & 0x8000) != 0,
            alt:   (GetAsyncKeyState(VK_MENU.0 as i32)    as u16 & 0x8000) != 0,
            meta:  (GetAsyncKeyState(VK_LWIN.0 as i32) as u16 & 0x8000) != 0 || (GetAsyncKeyState(VK_RWIN.0 as i32) as u16 & 0x8000) != 0,
        };
        send_event(InputEvent::KeyPress(KeyPress { keycode: vk_to_hid(info.vkCode), modifiers, pressed }));
        if is_suppressed() { return LRESULT(1); }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

pub fn vk_to_hid(vk: u32) -> u32 {
    match vk {
        0x41..=0x5A => vk - 0x41 + 0x04,
        0x30 => 0x27, 0x31..=0x39 => vk - 0x31 + 0x1E,
        0x0D => 0x28, 0x1B => 0x29, 0x08 => 0x2A, 0x09 => 0x2B, 0x20 => 0x2C,
        0x70..=0x7B => vk - 0x70 + 0x3A,
        0x25 => 0x50, 0x26 => 0x52, 0x27 => 0x4F, 0x28 => 0x51,
        0x2D => 0x49, 0x2E => 0x4C, 0x24 => 0x4A, 0x23 => 0x4D,
        0x21 => 0x4B, 0x22 => 0x4E,
        0xA0 | 0xA1 => 0xE1, 0xA2 | 0xA3 => 0xE0, 0xA4 | 0xA5 => 0xE2, 0x5B | 0x5C => 0xE3,
        _ => vk,
    }
}
