# 10 — Contributing & Repository Guide

## Repository Structure

```
borderless/
│
├── 📄 Cargo.toml               ← Rust workspace root
├── 📄 README.md                ← Project introduction
├── 📄 PHASE1_NEXT_STEPS.md     ← Immediate coding priorities
├── 📄 .env.example             ← Environment template
├── 📄 .gitignore
│
├── 📁 crates/                  ← All Rust crates
│   │
│   ├── 📦 borderless-core/     ← Shared engine (ALL platforms use this)
│   │   └── src/
│   │       ├── lib.rs          ← Public API + BorderlessEngine
│   │       ├── error.rs        ← Unified CoreError enum
│   │       ├── identity.rs     ← Device keypairs + hardware fingerprint
│   │       ├── crypto.rs       ← AES-256-GCM + helpers
│   │       ├── protocol.rs     ← Wire envelope + message types
│   │       ├── session.rs      ← Session state machine
│   │       ├── discovery.rs    ← mDNS-SD LAN discovery
│   │       ├── clipboard.rs    ← Clipboard monitor + sync
│   │       ├── transfer.rs     ← Chunked file transfer
│   │       └── input/
│   │           ├── mod.rs      ← InputEvent types + traits
│   │           ├── router.rs   ← DeviceLayout + edge routing
│   │           ├── capture/    ← OS input capture (per platform)
│   │           │   ├── windows.rs    ← WH_MOUSE_LL + WH_KEYBOARD_LL
│   │           │   ├── macos.rs      ← CGEventTap (TODO)
│   │           │   └── linux.rs      ← evdev + EVIOCGRAB (TODO)
│   │           └── inject/     ← OS input injection (per platform)
│   │               ├── windows.rs    ← SendInput
│   │               ├── macos.rs      ← CGEvent (TODO)
│   │               └── linux.rs      ← uinput (TODO)
│   │
│   ├── 📦 borderless-protocol/ ← Protobuf definitions
│   │   └── proto/borderless.proto
│   │
│   ├── 📦 borderless-server/   ← Axum REST API + WebSocket
│   │   └── src/
│   │       ├── main.rs         ← Server startup + router
│   │       ├── middleware/
│   │       │   └── auth.rs     ← JWT validation + admin check
│   │       └── routes/
│   │           ├── admin.rs    ← /admin/* (fleet management)
│   │           ├── auth.rs     ← /auth/* (login, register)
│   │           ├── devices.rs  ← /devices/*
│   │           └── health.rs   ← /health
│   │
│   ├── 📦 borderless-signaling/ ← WebRTC signaling server
│   │   └── src/main.rs         ← WS server + peer routing
│   │
│   └── 📦 borderless-relay/    ← TURN relay (fallback)
│       └── src/main.rs
│
├── 📁 apps/                    ← Platform applications
│   ├── 📱 desktop/             ← Tauri app (Windows + macOS + Linux)
│   ├── 📱 mobile/              ← Flutter app (Android + iOS)
│   └── 🌐 web/                 ← Next.js 14 web dashboard
│
├── 📁 migrations/              ← PostgreSQL SQL migrations
├── 📁 deploy/                  ← Deployment configuration
└── 📁 docs/                    ← This documentation
```

---

## Development Setup

### Prerequisites
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add x86_64-unknown-linux-gnu aarch64-apple-darwin

# Node.js 20+ + pnpm
npm install -g pnpm

# Flutter 3.19+
# https://docs.flutter.dev/get-started/install

# Docker + Docker Compose v2

# Optional tools
cargo install cargo-watch
cargo install sqlx-cli --no-default-features --features postgres
```

### First Time Setup
```bash
git clone https://github.com/rythmmcosta/Borderless
cd borderless

cp .env.example .env
# Edit .env: fill in JWT_SECRET (openssl rand -hex 32), ADMIN_EMAIL

docker compose -f deploy/docker-compose.yml up -d db redis

cargo build --all
cargo test --all

cargo run --package borderless-server
# → http://localhost:8080/v1/health should return 200
```

---

## Code Style & Standards

### Rust
```rust
// Format: rustfmt (enforced in CI)
cargo fmt --all

// Lint: clippy (warnings as errors in CI)
cargo clippy --all-targets -- -D warnings

// Patterns:
// - Use thiserror for library errors
// - Use anyhow for application errors
// - Async by default (tokio)
// - Tests in #[cfg(test)] modules at bottom of file
```

### TypeScript (React + Next.js)
```typescript
// Format: Prettier
// Lint: ESLint with TypeScript rules
// Conventions:
// - Functional components only (no class components)
// - Named exports (not default) for components
// - Use Tanstack Query for all server state
// - Use Zustand for local global state
// - Type everything — no 'any'
```

### Dart (Flutter)
```dart
// Format: dart format
// Lint: flutter analyze
// State: Riverpod
// Navigation: go_router
// Data classes: freezed
```

---

## Testing Strategy

### Unit Tests (Rust)
```rust
// Location: #[cfg(test)] block at bottom of each file
// Run: cargo test --package borderless-core

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn descriptive_test_name() {
        let key = SessionKey::from_bytes(&[0x42u8; 32]);
        let ct = key.encrypt(b"hello").unwrap();
        assert_eq!(key.decrypt(&ct).unwrap(), b"hello");
    }
}
```

### Integration Tests (API)
```rust
// Location: crates/borderless-server/tests/
// Run: cargo test --package borderless-server

#[sqlx::test]
async fn test_device_registration(pool: PgPool) {
    // Set up test state, make HTTP request, assert response + DB state
}
```

---

## Git Workflow

### Branch Naming
```
main          ← production-ready, protected
develop       ← integration branch
feature/*     ← new features (feature/windows-capture)
fix/*         ← bug fixes (fix/router-edge-detection)
docs/*        ← documentation only
```

### Commit Messages (Conventional Commits)
```
feat: add macOS input capture via CGEventTap
fix: correct cursor coordinate mapping on HiDPI displays
docs: update API reference for /admin/devices endpoint
refactor: extract HKDF into separate function
test: add router edge-crossing integration tests
chore: update cargo dependencies
```

### Pull Request Process
1. Create branch from `develop`
2. Write code + tests
3. `cargo fmt --all && cargo clippy --all-targets -- -D warnings`
4. `cargo test --all` — all tests pass
5. Open PR to `develop`
6. CI must pass (all platforms)
7. Review + merge

---

## Key Implementation TODOs

### Priority 1 — LAN KVM (Week 1-2)
```
[ ] crates/borderless-core/src/session.rs
    └── SessionManager::connect() — wire TCP transport
        - tokio::net::TcpStream::connect(ip, port)
        - ECDH handshake (send/receive HandshakePayload)
        - Derive session key
        - Spawn recv loop

[ ] crates/borderless-core/src/discovery.rs
    └── Uncomment mdns-sd code block in mdns_loop()

[ ] apps/desktop/src-tauri/src/commands.rs
    └── Tauri commands: start_kvm, stop_kvm, list_devices, get_layout
```

### Priority 2 — Tauri App (Week 2-3)
```
[ ] apps/desktop/ — Create with: pnpm create tauri-app
[ ] System tray with device switcher menu
[ ] Device layout drag-and-drop editor
[ ] Connection status indicator
[ ] Settings page
```

### Priority 3 — macOS + Linux Input (Week 3-4)
```
[ ] crates/borderless-core/src/input/capture/macos.rs
    └── CGEventTap implementation

[ ] crates/borderless-core/src/input/inject/macos.rs
    └── CGEventCreateMouseEvent / CGEventCreateKeyboardEvent

[ ] crates/borderless-core/src/input/capture/linux.rs
    └── evdev Device enumeration + EVIOCGRAB

[ ] crates/borderless-core/src/input/inject/linux.rs
    └── uinput virtual device creation + event writing
```

### Priority 4 — Cloud Backend (Week 4-6)
```
[ ] crates/borderless-server/src/routes/auth.rs — complete implementation
[ ] crates/borderless-server/src/routes/devices.rs — complete
[ ] crates/borderless-server/src/ws/mod.rs — signaling WebSocket
[ ] Deploy to VPS
```
