//! macOS input capture via CGEventTap.
//! Requires "Accessibility" permission in System Preferences → Privacy.

#![cfg(target_os = "macos")]

use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::sync::mpsc::Sender;
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventType, EventField,
};
use core_graphics::display::CGDisplay;
use core_foundation::runloop::{CFRunLoop, kCFRunLoopDefaultMode};
use super::super::{
    Button, InputCapture, InputEvent, KeyPress, Modifiers,
    MouseButton, MouseMove, MouseScroll,
};
use crate::CoreError;

pub struct MacosCapture {
    running:  Arc<AtomicBool>,
    suppress: Arc<AtomicBool>,
}

impl MacosCapture {
    pub fn new() -> Self {
        Self {
            running:  Arc::new(AtomicBool::new(false)),
            suppress: Arc::new(AtomicBool::new(false)),
        }
    }
}

struct TapContext { tx: Sender<InputEvent>, suppress: Arc<AtomicBool> }
thread_local! {
    static TAP_CTX: std::cell::RefCell<Option<TapContext>> = std::cell::RefCell::new(None);
}

fn send_ev(ev: InputEvent) {
    TAP_CTX.with(|c| {
        if let Some(ctx) = c.borrow().as_ref() { let _ = ctx.tx.try_send(ev); }
    });
}

fn suppressed() -> bool {
    TAP_CTX.with(|c| {
        c.borrow().as_ref().map(|ctx| ctx.suppress.load(Ordering::SeqCst)).unwrap_or(false)
    })
}

#[async_trait::async_trait]
impl InputCapture for MacosCapture {
    async fn start(&self, tx: Sender<InputEvent>) -> Result<(), CoreError> {
        if self.running.swap(true, Ordering::SeqCst) { return Ok(()); }
        let running  = Arc::clone(&self.running);
        let suppress = Arc::clone(&self.suppress);
        std::thread::Builder::new()
            .name("borderless-tap".into())
            .spawn(move || {
                TAP_CTX.with(|c| { *c.borrow_mut() = Some(TapContext { tx, suppress }); });
                tap_thread(running);
            })
            .map_err(|e| CoreError::InputCapture(e.to_string()))?;
        Ok(())
    }

    fn stop(&self)                  { self.running.store(false, Ordering::SeqCst); }
    fn set_suppress(&self, s: bool) { self.suppress.store(s, Ordering::SeqCst); }
}

// CGEventType values as bits for the mask
const fn cg_bit(t: CGEventType) -> u64 { 1u64 << (t as u64) }

fn tap_thread(running: Arc<AtomicBool>) {
    let mask = cg_bit(CGEventType::MouseMoved)
        | cg_bit(CGEventType::LeftMouseDown)   | cg_bit(CGEventType::LeftMouseUp)
        | cg_bit(CGEventType::RightMouseDown)  | cg_bit(CGEventType::RightMouseUp)
        | cg_bit(CGEventType::OtherMouseDown)  | cg_bit(CGEventType::OtherMouseUp)
        | cg_bit(CGEventType::LeftMouseDragged)| cg_bit(CGEventType::RightMouseDragged)
        | cg_bit(CGEventType::ScrollWheel)
        | cg_bit(CGEventType::KeyDown)         | cg_bit(CGEventType::KeyUp)
        | cg_bit(CGEventType::FlagsChanged);

    let tap = match CGEventTap::new(
        CGEventTapLocation::Session,
        CGEventTapPlacement::HeadInsert,
        CGEventTapOptions::Default,
        mask,
        handle_cg_event,
    ) {
        Ok(t)  => t,
        Err(_) => {
            tracing::error!("CGEventTap::new failed — grant Accessibility in System Preferences");
            return;
        }
    };

    let source = tap.mach_port.create_runloop_source(0);
    let rl = CFRunLoop::get_current();
    unsafe { rl.add_source(&source, kCFRunLoopDefaultMode); }
    tap.enable();

    while running.load(Ordering::SeqCst) {
        unsafe { CFRunLoop::run_in_mode(kCFRunLoopDefaultMode, 0.1, false); }
    }
    unsafe { rl.remove_source(&source, kCFRunLoopDefaultMode); }
}

fn handle_cg_event(
    _proxy: core_graphics::event::CGEventTapProxy,
    event_type: CGEventType,
    event: &CGEvent,
) -> Option<CGEvent> {
    let main = CGDisplay::main();
    let (sw, sh) = (main.pixels_wide() as i32, main.pixels_high() as i32);
    let loc  = event.location();
    let (ex, ey) = (loc.x as i32, loc.y as i32);

    // Raw flags: kCGEventFlagMaskShift=0x2_0000, Control=0x4_0000,
    //            Alternate=0x8_0000, Command=0x10_0000
    let flags = event.get_flags().bits();
    let mods = Modifiers {
        ctrl:  flags & 0x0004_0000 != 0,
        shift: flags & 0x0002_0000 != 0,
        alt:   flags & 0x0008_0000 != 0,
        meta:  flags & 0x0010_0000 != 0,
    };

    let ev = match event_type {
        CGEventType::MouseMoved
        | CGEventType::LeftMouseDragged
        | CGEventType::RightMouseDragged => Some(InputEvent::MouseMove(
            MouseMove { x: ex, y: ey, dx: 0, dy: 0, screen_w: sw, screen_h: sh },
        )),

        CGEventType::LeftMouseDown  => Some(InputEvent::MouseButton(MouseButton { button: Button::Left,   pressed: true,  x: ex, y: ey })),
        CGEventType::LeftMouseUp    => Some(InputEvent::MouseButton(MouseButton { button: Button::Left,   pressed: false, x: ex, y: ey })),
        CGEventType::RightMouseDown => Some(InputEvent::MouseButton(MouseButton { button: Button::Right,  pressed: true,  x: ex, y: ey })),
        CGEventType::RightMouseUp   => Some(InputEvent::MouseButton(MouseButton { button: Button::Right,  pressed: false, x: ex, y: ey })),

        CGEventType::OtherMouseDown | CGEventType::OtherMouseUp => {
            let btn_num = event.get_integer_value_field(EventField(0)) as u32; // kCGMouseEventButtonNumber
            let btn = match btn_num { 2 => Button::Middle, 3 => Button::X1, _ => Button::X2 };
            let pressed = event_type == CGEventType::OtherMouseDown;
            Some(InputEvent::MouseButton(MouseButton { button: btn, pressed, x: ex, y: ey }))
        }

        CGEventType::ScrollWheel => {
            // kCGScrollWheelEventDeltaAxis1=11 (vertical), Axis2=12 (horizontal)
            let dy = event.get_integer_value_field(EventField(11)) as f32;
            let dx = event.get_integer_value_field(EventField(12)) as f32;
            Some(InputEvent::MouseScroll(MouseScroll { delta_x: dx, delta_y: dy }))
        }

        CGEventType::KeyDown | CGEventType::KeyUp => {
            // kCGKeyboardEventKeycode = 9
            let cg_code = event.get_integer_value_field(EventField(9)) as u32;
            Some(InputEvent::KeyPress(KeyPress {
                keycode:   cg_keycode_to_hid(cg_code),
                modifiers: mods,
                pressed:   event_type == CGEventType::KeyDown,
            }))
        }

        _ => None,
    };

    if let Some(input_ev) = ev {
        send_ev(input_ev);
        if suppressed() { return None; } // None = consume event
    }

    Some(event.clone())
}

/// macOS Carbon virtual keycode → USB HID usage code.
pub fn cg_keycode_to_hid(cg: u32) -> u32 {
    match cg {
        0x00 => 0x04, 0x0B => 0x05, 0x08 => 0x06, 0x02 => 0x07, // A B C D
        0x0E => 0x08, 0x03 => 0x09, 0x05 => 0x0A, 0x04 => 0x0B, // E F G H
        0x22 => 0x0C, 0x26 => 0x0D, 0x28 => 0x0E, 0x25 => 0x0F, // I J K L
        0x2E => 0x10, 0x2D => 0x11, 0x1F => 0x12, 0x23 => 0x13, // M N O P
        0x0C => 0x14, 0x0F => 0x15, 0x01 => 0x16, 0x11 => 0x17, // Q R S T
        0x20 => 0x18, 0x09 => 0x19, 0x0D => 0x1A, 0x07 => 0x1B, // U V W X
        0x10 => 0x1C, 0x06 => 0x1D,                              // Y Z
        0x12 => 0x1E, 0x13 => 0x1F, 0x14 => 0x20, 0x15 => 0x21, // 1 2 3 4
        0x17 => 0x22, 0x16 => 0x23, 0x1A => 0x24, 0x1C => 0x25, // 5 6 7 8
        0x19 => 0x26, 0x1D => 0x27,                              // 9 0
        0x24 => 0x28, 0x35 => 0x29, 0x33 => 0x2A, 0x30 => 0x2B, // Ret Esc BS Tab
        0x31 => 0x2C, 0x1B => 0x2D, 0x18 => 0x2E,               // Space - =
        0x21 => 0x2F, 0x1E => 0x30, 0x2A => 0x31, 0x29 => 0x33, // [ ] \ ;
        0x27 => 0x34, 0x32 => 0x35, 0x2B => 0x36, 0x2F => 0x37, // ' ` , .
        0x2C => 0x38,                                             // /
        0x7A => 0x3A, 0x78 => 0x3B, 0x63 => 0x3C, 0x76 => 0x3D, // F1-F4
        0x60 => 0x3E, 0x61 => 0x3F, 0x62 => 0x40, 0x64 => 0x41, // F5-F8
        0x65 => 0x42, 0x6D => 0x43, 0x67 => 0x44, 0x6F => 0x45, // F9-F12
        0x7B => 0x50, 0x7E => 0x52, 0x7C => 0x4F, 0x7D => 0x51, // ← ↑ → ↓
        _ => cg,
    }
}
