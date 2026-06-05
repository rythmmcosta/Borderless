//! Linux input capture via evdev + EVIOCGRAB.
//! Opens /dev/input/event* devices, grabs exclusively, reads events.
//! Requires read permission on /dev/input/* (add user to `input` group).

#![cfg(target_os = "linux")]

use std::sync::{atomic::{AtomicBool, AtomicI32, Ordering}, Arc};
use tokio::sync::mpsc::Sender;
use evdev::{Device, InputEventKind, RelativeAxisType};
use super::super::{
    Button, InputCapture, InputEvent, KeyPress, Modifiers,
    MouseButton, MouseMove, MouseScroll,
};
use crate::CoreError;

pub struct LinuxCapture {
    running:  Arc<AtomicBool>,
    suppress: Arc<AtomicBool>,
}

impl LinuxCapture {
    pub fn new() -> Self {
        Self {
            running:  Arc::new(AtomicBool::new(false)),
            suppress: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl InputCapture for LinuxCapture {
    async fn start(&self, tx: Sender<InputEvent>) -> Result<(), CoreError> {
        if self.running.swap(true, Ordering::SeqCst) { return Ok(()); }

        let devices: Vec<Device> = evdev::enumerate()
            .filter_map(|(_, dev)| {
                let has_keys = dev.supported_keys()
                    .map_or(false, |k| k.iter().next().is_some());
                let has_rel = dev.supported_relative_axes()
                    .map_or(false, |r| r.iter().next().is_some());
                if has_keys || has_rel { Some(dev) } else { None }
            })
            .collect();

        if devices.is_empty() {
            return Err(CoreError::InputCapture("No input devices found in /dev/input — check permissions".into()));
        }

        // Shared cursor position across all device threads
        let cursor_x = Arc::new(AtomicI32::new(0));
        let cursor_y = Arc::new(AtomicI32::new(0));
        let (sw, sh) = get_screen_size();

        for mut device in devices {
            let tx_dev     = tx.clone();
            let running    = Arc::clone(&self.running);
            let suppress   = Arc::clone(&self.suppress);
            let cx         = Arc::clone(&cursor_x);
            let cy         = Arc::clone(&cursor_y);

            std::thread::Builder::new()
                .name("borderless-evdev".into())
                .spawn(move || {
                    if suppress.load(Ordering::SeqCst) {
                        let _ = device.grab();
                    }
                    device_loop(device, tx_dev, running, suppress, cx, cy, sw, sh);
                })
                .map_err(|e| CoreError::InputCapture(e.to_string()))?;
        }

        tracing::info!("Linux evdev capture started");
        Ok(())
    }

    fn stop(&self)                  { self.running.store(false, Ordering::SeqCst); }
    fn set_suppress(&self, s: bool) { self.suppress.store(s, Ordering::SeqCst); }
}

fn device_loop(
    mut device: Device,
    tx: Sender<InputEvent>,
    running: Arc<AtomicBool>,
    suppress: Arc<AtomicBool>,
    cursor_x: Arc<AtomicI32>,
    cursor_y: Arc<AtomicI32>,
    screen_w: i32,
    screen_h: i32,
) {
    use std::os::unix::io::AsRawFd;

    let fd = device.as_raw_fd();
    let mut grabbed = suppress.load(Ordering::SeqCst);

    loop {
        if !running.load(Ordering::SeqCst) { break; }

        // Dynamically grab/ungrab based on suppress state
        let want_grab = suppress.load(Ordering::SeqCst);
        if want_grab != grabbed {
            if want_grab { let _ = device.grab(); } else { let _ = device.ungrab(); }
            grabbed = want_grab;
        }

        // Poll with 50 ms timeout so we can check running/suppress periodically
        let ready = unsafe {
            let mut pfd = libc::pollfd {
                fd,
                events:  libc::POLLIN as libc::c_short,
                revents: 0,
            };
            libc::poll(&mut pfd as *mut _, 1 as libc::nfds_t, 50)
        };

        if ready <= 0 { continue; }

        match device.fetch_events() {
            Ok(events) => {
                for ev in events {
                    if let Some(input_ev) = translate(ev, &cursor_x, &cursor_y, screen_w, screen_h) {
                        let _ = tx.try_send(input_ev);
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }

    if grabbed { let _ = device.ungrab(); }
}

fn translate(
    ev: evdev::InputEvent,
    cx: &Arc<AtomicI32>,
    cy: &Arc<AtomicI32>,
    sw: i32,
    sh: i32,
) -> Option<InputEvent> {
    match ev.kind() {
        InputEventKind::Key(key) => {
            // Mouse buttons: BTN_LEFT=0x110, BTN_RIGHT=0x111, BTN_MIDDLE=0x112
            let code = key.code();
            if code >= 0x110 && code <= 0x114 {
                let btn = match code {
                    0x110 => Button::Left,
                    0x111 => Button::Right,
                    0x112 => Button::Middle,
                    0x113 => Button::X1,
                    _     => Button::X2,
                };
                let pressed = ev.value() != 0;
                let (x, y) = (cx.load(Ordering::Relaxed), cy.load(Ordering::Relaxed));
                return Some(InputEvent::MouseButton(MouseButton { button: btn, pressed, x, y }));
            }
            let pressed  = ev.value() != 0;
            let keycode  = evdev_key_to_hid(code);
            Some(InputEvent::KeyPress(KeyPress {
                keycode,
                modifiers: Modifiers::default(),
                pressed,
            }))
        }

        InputEventKind::RelAxis(axis) => {
            match axis {
                RelativeAxisType::REL_X => {
                    let new_x = (cx.fetch_add(ev.value(), Ordering::Relaxed) + ev.value())
                        .clamp(0, sw - 1);
                    cx.store(new_x, Ordering::Relaxed);
                    let y = cy.load(Ordering::Relaxed);
                    Some(InputEvent::MouseMove(MouseMove {
                        x: new_x, y, dx: ev.value(), dy: 0, screen_w: sw, screen_h: sh,
                    }))
                }
                RelativeAxisType::REL_Y => {
                    let new_y = (cy.fetch_add(ev.value(), Ordering::Relaxed) + ev.value())
                        .clamp(0, sh - 1);
                    cy.store(new_y, Ordering::Relaxed);
                    let x = cx.load(Ordering::Relaxed);
                    Some(InputEvent::MouseMove(MouseMove {
                        x, y: new_y, dx: 0, dy: ev.value(), screen_w: sw, screen_h: sh,
                    }))
                }
                RelativeAxisType::REL_WHEEL => {
                    Some(InputEvent::MouseScroll(MouseScroll { delta_x: 0.0, delta_y: ev.value() as f32 }))
                }
                RelativeAxisType::REL_HWHEEL => {
                    Some(InputEvent::MouseScroll(MouseScroll { delta_x: ev.value() as f32, delta_y: 0.0 }))
                }
                _ => None,
            }
        }

        _ => None,
    }
}

fn get_screen_size() -> (i32, i32) {
    // Try X11 first, fall back to sensible default
    use x11rb::connection::Connection;
    if let Ok((conn, screen_num)) = x11rb::rust_connection::RustConnection::connect(None) {
        let screen = &conn.setup().roots[screen_num];
        return (screen.width_in_pixels as i32, screen.height_in_pixels as i32);
    }
    (1920, 1080)
}

/// Linux input.h key code → USB HID usage code.
pub fn evdev_key_to_hid(code: u16) -> u32 {
    match code {
        16 => 0x14, 17 => 0x1A, 18 => 0x08, 19 => 0x15, 20 => 0x17, // Q W E R T
        21 => 0x1C, 22 => 0x18, 23 => 0x0C, 24 => 0x12, 25 => 0x13, // Y U I O P
        30 => 0x04, 31 => 0x16, 32 => 0x07, 33 => 0x09, 34 => 0x0A, // A S D F G
        35 => 0x0B, 36 => 0x0D, 37 => 0x0E, 38 => 0x0F,             // H J K L
        44 => 0x1D, 45 => 0x1B, 46 => 0x06, 47 => 0x19, 48 => 0x05, // Z X C V B
        49 => 0x11, 50 => 0x10,                                      // N M
        2  => 0x1E, 3  => 0x1F, 4  => 0x20, 5  => 0x21, 6  => 0x22, // 1 2 3 4 5
        7  => 0x23, 8  => 0x24, 9  => 0x25, 10 => 0x26, 11 => 0x27, // 6 7 8 9 0
        28 => 0x28, 1  => 0x29, 14 => 0x2A, 15 => 0x2B, 57 => 0x2C, // Ret Esc BS Tab Spc
        12 => 0x2D, 13 => 0x2E, 26 => 0x2F, 27 => 0x30, 43 => 0x31, // - = [ ] \
        39 => 0x33, 40 => 0x34, 41 => 0x35, 51 => 0x36, 52 => 0x37, // ; ' ` , .
        53 => 0x38,                                                   // /
        59 => 0x3A, 60 => 0x3B, 61 => 0x3C, 62 => 0x3D, 63 => 0x3E, // F1-F5
        64 => 0x3F, 65 => 0x40, 66 => 0x41, 67 => 0x42, 68 => 0x43, // F6-F10
        87 => 0x44, 88 => 0x45,                                      // F11 F12
        105 => 0x50, 103 => 0x52, 106 => 0x4F, 108 => 0x51,         // ← ↑ → ↓
        102 => 0x4A, 107 => 0x4D, 104 => 0x4B, 109 => 0x4E,         // Home End PgUp PgDn
        110 => 0x49, 111 => 0x4C,                                    // Insert Delete
        42  => 0xE1, 54  => 0xE5, 29  => 0xE0, 97  => 0xE4,         // LShft RShft LCtrl RCtrl
        56  => 0xE2, 100 => 0xE6, 125 => 0xE3, 126 => 0xE7,         // LAlt RAlt LMeta RMeta
        _ => code as u32,
    }
}
