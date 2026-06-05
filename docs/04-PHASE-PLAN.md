# 04 — Phase-by-Phase Development Plan

## Overview

| Phase | Name | Duration | Milestone |
|-------|------|----------|-----------|
| 1 | Foundation & Core Engine | Weeks 1–8 | LAN KVM working on Windows |
| 2 | Cloud Backend & Internet | Weeks 9–14 | Internet KVM + auth |
| 3 | Sync Features + Mobile | Weeks 15–22 | Clipboard + files + mobile |
| 4 | Remote Control + Admin | Weeks 23–30 | Full admin dashboard |
| 5 | Polish + Cloud Sync | Weeks 31–38 | All features, all platforms |
| 6 | Security + Launch | Weeks 39–48 | Public release |

**Total:** ~12 months to MVP, ~18 months to full launch

---

## Phase 1 — Foundation & Core Engine (Weeks 1–8)

### Goal
A working Windows-to-Windows KVM on LAN. Mouse crosses screen edge → appears on the other PC.

### Deliverables
- [x] Rust workspace structure
- [x] `borderless-core` crate with all modules scaffolded
- [x] Device identity (Ed25519 + X25519, hardware fingerprint)
- [x] AES-256-GCM crypto with tests
- [x] Input event types (mouse, keyboard, scroll)
- [x] Input router with device layout and edge-crossing logic
- [x] Windows input capture (WH_MOUSE_LL + WH_KEYBOARD_LL)
- [x] Windows input injection (SendInput)
- [x] mDNS-SD discovery scaffold
- [x] Session state machine
- [x] File transfer engine (chunked + SHA-256)
- [x] Protocol envelope format
- [ ] **TCP transport wired into session** ← IN PROGRESS
- [ ] Complete mDNS-SD loop (uncomment code)
- [ ] Tauri desktop app shell (Windows build)
- [ ] Device pairing UI (PIN-based)
- [ ] Device layout drag-and-drop editor
- [ ] System tray integration (Windows)
- [ ] macOS input capture (CGEventTap)
- [ ] Linux input capture (evdev + EVIOCGRAB)
- [ ] Clipboard sync Windows ↔ Windows (text)

### TCP Transport (Priority #1)
```rust
// In session.rs SessionManager::connect():
// 1. Get peer's IP from DiscoveryService cache
// 2. tokio::net::TcpStream::connect(peer_ip, LISTEN_PORT).await
// 3. Send HandshakePayload (DH pubkey + signed device_id)
// 4. Receive peer HandshakePayload, verify Ed25519 signature
// 5. derive_session_key(peer_dh_pubkey) → SessionKey
// 6. Encrypted ping/pong to confirm session
// 7. state → Active, spawn recv_loop
```

### Phase 1 Success Criteria
- ✅ Move cursor from Windows PC A to Windows PC B via LAN
- ✅ Keyboard input follows cursor to correct machine
- ✅ Devices auto-discover each other via mDNS
- ✅ Pairing requires explicit user action (PIN)
- ✅ `cargo test` passes all unit tests

---

## Phase 2 — Cloud Backend & Internet KVM (Weeks 9–14)

### Goal
Connect two computers on different networks via WebRTC P2P.

### Deliverables
- [ ] Complete Axum REST API (all route implementations)
- [ ] User registration + login (email/password + Google OAuth)
- [ ] JWT auth + refresh token rotation
- [ ] Device registration with cloud API
- [ ] WebRTC signaling server (SDP offer/answer relay)
- [ ] WebRTC data channel in Rust (`webrtc-rs` crate)
- [ ] STUN/TURN integration (Coturn)
- [ ] Internet KVM via WebRTC (P2P first, TURN fallback)
- [ ] PostgreSQL all migrations running
- [ ] Redis sessions + device presence
- [ ] Docker Compose complete (all services)
- [ ] Deploy to VPS (Hetzner ~€8/mo)
- [ ] HTTPS with Let’s Encrypt
- [ ] GitHub Actions CI passing on all 3 platforms

### WebRTC Step Plan
```toml
# Add to borderless-core Cargo.toml:
webrtc = "0.9"
```
```rust
let ice_servers = vec![
    RTCIceServer { urls: vec!["stun:stun.l.google.com:19302".into()], ..Default::default() },
    RTCIceServer { urls: vec!["turn:your-server.com:3478".into()], username: "borderless".into(), .. },
];
```

---

## Phase 3 — Sync Features + Mobile (Weeks 15–22)

### Clipboard Sync
- [ ] macOS clipboard read/write (NSPasteboard via objc2)
- [ ] Linux clipboard (x11-clipboard + wl-clipboard-rs)
- [ ] Image + HTML clipboard sync
- [ ] Cloud clipboard history (Supabase Storage, encrypted)

### File Transfer
- [ ] Drag-and-drop file from one screen to another
- [ ] Transfer progress UI, pause/resume/cancel
- [ ] Large file support (tested to 10GB)

### Notification Sync
- [ ] Windows notification capture (COM/WNS)
- [ ] macOS notification capture
- [ ] Android notification listener service
- [ ] iOS notification extension

### Flutter Mobile App
- [ ] dart:ffi bridge to borderless-core.so / .dylib
- [ ] Device pairing (QR code scan)
- [ ] Trackpad mode (touch → mouse events)
- [ ] Clipboard viewer + sync
- [ ] Android APK + iOS IPA

---

## Phase 4 — Remote Control + Admin Dashboard (Weeks 23–30)

### Screen Capture Engine
- [ ] Windows: DXGI Desktop Duplication API
- [ ] macOS: ScreenCaptureKit
- [ ] Linux: PipeWire / X11
- [ ] H.264 encoding, WebRTC video track

### Next.js Admin Dashboard
- [ ] Live device list (Supabase Realtime)
- [ ] View device screen (live WebRTC stream)
- [ ] Remote control device (visible indicator on target)
- [ ] Lock/unlock device (revoke token via Redis)
- [ ] User management, session history, audit log, analytics

---

## Phase 5 — Polish + Cloud Sync (Weeks 31–38)
- [ ] Settings sync across devices (Supabase Realtime)
- [ ] Multi-user organizations + role management
- [ ] 2FA/Passkeys (WebAuthn)
- [ ] Full platform parity (all 6 targets)
- [ ] CPU < 0.5% idle, input latency < 5ms LAN
- [ ] Auto-update (Tauri updater)

---

## Phase 6 — Security Audit + Launch (Weeks 39–48)
- [ ] External penetration test
- [ ] Signed binaries (Authenticode + Apple notarization)
- [ ] Linux packaging: DEB, RPM, AppImage, Flatpak, AUR
- [ ] Google Play + Apple App Store submissions
- [ ] Docs site, Discord, HackerNews launch

---

## Resource Requirements

```
Solo developer estimate:
  Phase 1:  8 weeks full-time
  Phase 2:  6 weeks full-time
  Phase 3:  8 weeks full-time
  Phase 4:  8 weeks full-time
  Phase 5:  8 weeks full-time
  Phase 6: 10 weeks (part-time + community)
  Total: ~12 months solo
```

```
Infrastructure (self-hosted MVP):
  VPS Hetzner CX21:   €5/month  — API + Signaling
  VPS Hetzner CX11:   €3/month  — Coturn TURN relay
  Supabase Free:      $0/month  — Up to 500MB DB
  Domain:             $15/year
  Total:              ~€8/month
```
