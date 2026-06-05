# Phase 1 — Immediate Next Steps

## Priority order for making the first working demo

### 1. Wire the TCP transport (Session → LAN connect)
**File:** `crates/borderless-core/src/session.rs`  
Replace the placeholder in `SessionManager::connect()`:
```
1. Look up peer IP from DiscoveryService cache
2. tokio::net::TcpStream::connect(peer_ip:port)
3. Send HandshakePayload (our DH pubkey + device_id, signed)
4. Read peer's HandshakePayload, verify Ed25519 signature
5. derive_session_key(peer.dh_pubkey) → SessionKey
6. Confirm with encrypted "hello" ping/pong
7. Set session.state → Active
8. Spawn recv loop (decrypt → dispatch InputEvent etc)
```

### 2. Complete mDNS discovery loop
**File:** `crates/borderless-core/src/discovery.rs`  
Uncomment the `mdns-sd` code block and wire it up.

### 3. Build the Tauri desktop app
```bash
cd apps/desktop
npm create tauri-app@latest . -- --template react-ts
# Then add Tauri commands in src-tauri/src/commands.rs:
#   start_kvm_session, stop_session, list_devices, etc.
```

### 4. macOS input capture
**File:** `crates/borderless-core/src/input/capture/macos.rs`  
Use `CGEventTap` via `core-graphics` crate.  
Requires: Accessibility permission prompt.

### 5. Linux input capture + inject
**Files:** `capture/linux.rs`, `inject/linux.rs`  
Use `evdev` crate for capture + EVIOCGRAB.  
Use `uinput` for injection (create virtual device).

### 6. Clipboard — macOS + Linux
**File:** `crates/borderless-core/src/clipboard.rs`  
macOS: `NSPasteboard` via `objc2` crate.  
Linux: `x11-clipboard` or `wl-clipboard-rs` (Wayland).

## Running what works NOW (Windows)
```bash
# Start backend
cp .env.example .env
# edit .env — fill in DATABASE_URL, REDIS_URL, JWT_SECRET, ADMIN_EMAIL
docker compose -f deploy/docker-compose.yml up -d db redis

cargo run --package borderless-server

# Run tests
cargo test --package borderless-core
```

## Architecture reminder
```
InputCapture → InputRouter → local? → (pass through)
                           → remote? → Session::send_input() 
                                         → TCP → decrypt on target
                                              → InputInject::inject()
```
