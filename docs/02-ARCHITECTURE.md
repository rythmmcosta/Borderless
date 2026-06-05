# 02 — System Architecture

## High-Level Overview

```
╔══════════════════════════════════════════════════════════════════╗
║                        CLIENT LAYER                              ║
║                                                                  ║
║  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────────┐  ║
║  │  Tauri   │  │  Tauri   │  │  Tauri   │  │   Next.js 14   │  ║
║  │ Windows  │  │  macOS   │  │  Linux   │  │  Web Dashboard │  ║
║  └────┬────┘  └────┬────┘  └────┬────┘  └───────┬────────┘  ║
║       │              │              │                │            ║
║  ┌────┴──────────────┴──────────────┴────┐           │            ║
║  │       Flutter Android + iOS           │           │            ║
║  └────────────────────────────────────────┘           │            ║
╚══════════════════════════════════════════════════════╪═══════════╝
                                                       │
╔══════════════════════════════════════════════════════╪═══════════╗
║                      CORE ENGINE                      │           ║
║                                                       │           ║
║  ┌────────────────────────────────────────────────┐ │           ║
║  │              borderless-core (Rust)              │ │           ║
║  │                                                  │ │           ║
║  │  identity  │  crypto  │  input   │  clipboard   │ │           ║
║  │  session   │  protocol│  transfer│  discovery   │ │           ║
║  └───────────────────────┬────────────────────────┘ │           ║
╚════════════════════════╪══════════════════════════════════════════╝
                           │
╔════════════════════════╪══════════════════════════════════════════╗
║                 COMMUNICATION LAYER                              ║
║                                                                  ║
║  ┌─────────────┐  ┌──────────────┐  ┌───────────────────────┐  ║
║  │ QUIC/TCP    │  │ WebRTC P2P   │  │   TURN Relay (coturn) │  ║
║  │  LAN Fast   │  │  Internet    │  │   Fallback             │  ║
║  │  Path       │  │  Direct      │  │                        │  ║
║  └─────────────┘  └──────┬───────┘  └───────────────────────┘  ║
║                          │                                       ║
║  ┌───────────────────────┴───────────────────────┐              ║
║  │        WebSocket Signaling Server              │              ║
║  │        (borderless-signaling, Rust)            │              ║
║  └───────────────────────────────────────────────┘              ║
╚══════════════════════════════════════════════════════════════════╝
                           │
╔════════════════════════╪══════════════════════════════════════════╗
║                   BACKEND SERVICES                               ║
║                                                                  ║
║  ┌──────────────────┐  ┌────────────┐  ┌────────────────────┐  ║
║  │ borderless-server│  │  Supabase  │  │    Redis 7         │  ║
║  │    (Axum REST)   │  │  Auth +    │  │  Sessions + PubSub │  ║
║  │    Port 8080     │  │  Storage   │  │  + Rate Limiting   │  ║
║  └────────┬─────────┘  └─────┬──────┘  └────────────────────┘  ║
║           │                  │                                   ║
║  ┌────────┴──────────────────┴────────────────────────────┐ ║
║  │               PostgreSQL 16                                 │ ║
║  │   users │ devices │ sessions │ clipboard │ audit_logs      │ ║
║  └────────────────────────────────────────────────────────┘   ║
╚══════════════════════════════════════════════════════════════════╝
```

---

## Technology Stack

### Core Engine
| Technology | Version | Purpose |
|-----------|---------|-------|
| **Rust** | 1.75+ | All performance-critical code |
| **Tokio** | 1.35 | Async runtime |
| **AES-256-GCM** | aes-gcm 0.10 | Symmetric encryption |
| **X25519** | x25519-dalek 2.0 | Key exchange |
| **Ed25519** | ed25519-dalek 2.0 | Device signing |
| **HKDF-SHA256** | hkdf 0.12 | Key derivation |
| **mDNS-SD** | mdns-sd 0.7 | LAN discovery |
| **Protobuf** | prost 0.12 | Wire protocol |

### Desktop Application
| Technology | Version | Purpose |
|-----------|---------|-------|
| **Tauri v2** | 2.x | Desktop app shell (Win/Mac/Linux) |
| **React 18** | 18.x | UI framework |
| **TypeScript** | 5.x | Type-safe frontend |
| **Tailwind CSS** | 3.x | Styling |
| **Tanstack Query** | 5.x | Data fetching + caching |
| **Zustand** | 4.x | Global state management |
| **React Router** | 6.x | Client-side routing |

### Mobile Application
| Technology | Version | Purpose |
|-----------|---------|-------|
| **Flutter** | 3.19+ | Cross-platform mobile (iOS + Android) |
| **Dart** | 3.x | Mobile app language |
| **Riverpod** | 2.x | State management |
| **dart:ffi** | built-in | Bridge to Rust core |
| **go_router** | 13.x | Navigation |
| **flutter_secure_storage** | 9.x | Secure key storage |

### Web Dashboard
| Technology | Version | Purpose |
|-----------|---------|-------|
| **Next.js 14** | 14.x | React framework, App Router |
| **TypeScript** | 5.x | Type safety |
| **Tailwind CSS** | 3.x | Styling |
| **shadcn/ui** | Latest | Component library |
| **Supabase JS** | 2.x | Realtime + Auth |
| **Recharts** | 2.x | Analytics charts |
| **Tanstack Table** | 8.x | Data tables |

### Backend Services
| Technology | Version | Purpose |
|-----------|---------|-------|
| **Axum** | 0.7 | REST API framework |
| **SQLx** | 0.7 | Async PostgreSQL |
| **Redis** | 7.x | Sessions, PubSub, cache |
| **PostgreSQL** | 16.x | Primary database |
| **Supabase** | Latest | Auth, storage, realtime |
| **Coturn** | Latest | STUN/TURN relay |
| **jsonwebtoken** | 9.x | JWT auth |
| **argon2** | 0.5 | Password hashing |

### Infrastructure
| Technology | Purpose |
|-----------|-------|
| **Docker + Compose** | Local development |
| **Kubernetes** | Production orchestration |
| **Helm** | K8s package management |
| **GitHub Actions** | CI/CD |
| **Nginx** | Reverse proxy / TLS termination |

---

## Data Flow Diagrams

### KVM Input Flow (Mouse Move)

```
User moves mouse to right edge on Machine A
           │
           ▼
  WindowsCapture::mouse_proc() (WH_MOUSE_LL hook)
           │
           ▼  InputEvent::MouseMove { x: 1919, y: 540, screen_w: 1920, screen_h: 1080 }
  InputRouter::route(event)
           │
           ├─── x >= screen_w - 2 ──► Edge::Right detected
           │
           ▼
  Check DeviceLayout: Machine_A.Right = Machine_B
           │
           ▼
  Switch active_device → Machine_B
  Calculate entry point: entry_x=1, entry_y=720 (scaled)
  Return: Routing::SwitchTo { target: machine_b_id, entry_x:1, entry_y:720 }
           │
           ▼
  Tauri command handler receives SwitchTo
           │
           ▼
  Session::send_input(CursorWarp { x:1, y:720 })
           │
           ▼  [AES-256-GCM encrypted payload over TCP/WebRTC]
           │
           ▼
  Machine B receives Envelope { msg_type: CursorWarp, payload: [encrypted] }
           │
           ▼
  Session::decrypt(payload) → CursorWarp { x:1, y:720 }
           │
           ▼
  WindowsInject::inject(MouseMove { x:1, y:720 })
           │
           ▼
  SendInput(MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE)
           │
           ▼
  Cursor appears on Machine B at left edge, position 720px down
```

### Device Pairing Flow

```
User opens Borderless on Machine A        User opens Borderless on Machine B
           │                                           │
           ▼                                           ▼
  DiscoveryService starts mDNS             DiscoveryService starts mDNS
  Advertises: _borderless._tcp.local.      Advertises: _borderless._tcp.local.
           │                                           │
           └──────────────┬────────────────────────┘
                          │  mDNS resolution
                          ▼
              Both devices see each other
              in "Discovered Devices" list
                          │
                          ▼
              User clicks "Pair" on Machine A
                          │
                          ▼
              Machine A shows PIN: 4823
                          │
                          ▼
              User enters 4823 on Machine B
                          │
                          ▼
              TCP connection established
                          │
                          ▼
              ECDH Handshake:
              A → B: { dh_pubkey_A, device_id_A, sig(dh_pubkey_A) }
              B → A: { dh_pubkey_B, device_id_B, sig(dh_pubkey_B) }
                          │
                          ▼
              Both sides: derive_session_key(peer_dh_pubkey)
              → Same 32-byte AES key (ECDH + HKDF-SHA256)
                          │
                          ▼
              Encrypted "hello" ping/pong to confirm
                          │
                          ▼
              Pair stored in DB, device_pairs record created
                          │
                          ▼
              ✅ Devices paired — KVM active
```

### Internet Connection Flow (WebRTC)

```
Machine A (home)                Server                Machine B (office)
     │                            │                         │
     │── WS connect ───────────►│                         │
     │   (device_token JWT)       │                         │
     │                            │◄─── WS connect ─────────│
     │                            │     (device_token JWT)  │
     │                            │                         │
     │── Signal: SDP Offer ──────►│── Forward to B ────────►│
     │                            │                         │
     │                            │◄─── SDP Answer ─────────│
     │◄─── Forward to A ──────────│                         │
     │                            │                         │
     │──── ICE Candidates ───────►│──── Forward ───────────►│
     │◄─── ICE Candidates ────────│◄─── Forward ────────────│
     │                            │                         │
     │◄═══════ WebRTC DataChannel (P2P, E2E encrypted) ════►│
     │         Server no longer involved                     │
     │                            │                         │
     │  [if P2P fails]            │                         │
     │◄══════ TURN Relay ═════════╪════════════════════►│
                                  │
                        (still E2E encrypted —
                         relay sees ciphertext only)
```

---

## Module Dependency Graph

```
borderless-protocol
        │
        ▼
borderless-core  ◄─────────────────────────────────────────┐
   │                                                        │
   ├── identity.rs      (Device keypairs, fingerprint)      │
   ├── crypto.rs        (AES-256-GCM, HKDF)                │
   ├── input/           (Capture, inject, router)            │
   ├── clipboard.rs     (Monitor, sync)                     │
   ├── session.rs       (State machine, encrypted channels)  │
   ├── discovery.rs     (mDNS-SD)                           │
   ├── transfer.rs      (Chunked file transfer)              │
   └── protocol.rs      (Wire format)                       │
                                                            │
         ┌──────────────────────────────────────────────────┘
         │
         ├─► apps/desktop/src-tauri/  (Tauri commands → borderless-core via FFI)
         ├─► apps/mobile/             (Flutter dart:ffi → borderless-core cdylib)
         └─► crates/borderless-server/ (uses protocol types)

borderless-server
   ├── routes/admin.rs   (Admin fleet management)
   ├── routes/auth.rs    (JWT + OAuth)
   ├── routes/devices.rs (Device registry)
   ├── middleware/auth.rs (JWT validation, admin check)
   └── db/               (SQLx queries)

borderless-signaling
   └── WebRTC signaling (SDP offer/answer, ICE candidates)
```

---

## Security Architecture

See [08-SECURITY.md](./08-SECURITY.md) for full details.

### Key principles:
1. **E2E Encrypted** — session key derived via ECDH, server never has it
2. **Zero Trust** — every device explicitly paired, every action authenticated
3. **Append-only audit** — all admin actions logged, cannot be deleted
4. **Minimal surface** — server handles routing only, never plaintext

---

## Performance Architecture

### LAN Fast Path (< 5ms target)
```
TCP/QUIC direct connection (no relay)
→ Binary MessagePack serialization
→ No cloud round-trip
→ Local network only
```

### Internet Path (< 30ms target)
```
WebRTC DataChannel (ICE/STUN resolved)
→ Direct P2P after signaling
→ Binary encoding
→ Adaptive rate control
```

### Relay Path (< 80ms target)
```
TURN relay (coturn)
→ Used when P2P fails (strict NAT, firewalls)
→ Encrypted blob forwarding
→ Bandwidth throttled per session
```
