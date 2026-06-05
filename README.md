# Borderless

> A modern, open-source, cross-platform alternative to Mouse Without Borders.
> Share your mouse, keyboard, clipboard, files, and notifications across all your devices — seamlessly.

[![CI](https://github.com/borderless-app/borderless/actions/workflows/ci.yml/badge.svg)](https://github.com/borderless-app/borderless/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/core-Rust-orange)](https://www.rust-lang.org/)

---

## What Is Borderless?

Borderless lets you control multiple computers with a single mouse and keyboard — move the cursor to the edge of one screen and it seamlessly appears on the next machine. It also syncs your clipboard, forwards notifications, and lets you drag-and-drop files between machines.

Works on **Windows, macOS, Linux** (desktop), **Android, iOS** (mobile), and a **Web dashboard**.

Supports both **LAN** (zero-latency, direct) and **Internet** (WebRTC P2P with TURN relay fallback).

---

## Features

| Feature | Status |
|---------|--------|
| Mouse/keyboard KVM (like Mouse Without Borders) | 🚧 Phase 1 |
| LAN device discovery (mDNS) | 🚧 Phase 1 |
| Internet connectivity (WebRTC P2P) | 🚧 Phase 2 |
| Clipboard sync (text, images, HTML) | 🚧 Phase 3 |
| File drag-and-drop transfer | 🚧 Phase 3 |
| Notification forwarding | 🚧 Phase 3 |
| Remote desktop control | 🚧 Phase 4 |
| Web admin dashboard | 🚧 Phase 4 |
| Mobile apps (Android/iOS) | 🚧 Phase 3 |
| Cloud sync (settings, layout) | 🚧 Phase 5 |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Client Apps                                                 │
│  Tauri (Win/Mac/Linux)  Flutter (Android/iOS)  Next.js Web  │
└───────────────────────────┬─────────────────────────────────┘
                            │ FFI / Tauri Commands
┌───────────────────────────▼─────────────────────────────────┐
│  borderless-core (Rust)                                      │
│  Identity · Crypto · Input · Clipboard · Transfer           │
└───────────────────────────┬─────────────────────────────────┘
                            │ WebRTC / QUIC
┌───────────────────────────▼─────────────────────────────────┐
│  borderless-server (Axum)  +  borderless-signaling           │
│  REST API · WebSocket · TURN relay                          │
└───────────────────────────┬─────────────────────────────────┘
                            │
              PostgreSQL · Redis · Supabase
```

---

## Quick Start (Development)

### Prerequisites
- Rust 1.75+
- Node.js 20+
- pnpm 8+
- Docker + Docker Compose
- Flutter 3.19+ (for mobile)

### 1. Clone and setup

```bash
git clone https://github.com/rythmmcosta/Borderless
cd Borderless
cp .env.example .env
# Edit .env — set ADMIN_EMAIL and JWT_SECRET
```

### 2. Start backend services

```bash
cd deploy
docker compose up -d db redis coturn
```

### 3. Run the API server

```bash
cargo run --package borderless-server
```

### 4. Run the desktop app (dev mode)

```bash
cd apps/desktop
pnpm install
pnpm tauri dev
```

### 5. Run the web dashboard

```bash
cd apps/web
pnpm install
pnpm dev
# Open http://localhost:3000
```

---

## Repository Structure

```
borderless/
├── crates/
│   ├── borderless-core/       # Shared Rust library (all platforms)
│   ├── borderless-protocol/   # Protobuf message definitions
│   ├── borderless-server/     # Axum REST + WebSocket API
│   ├── borderless-signaling/  # WebRTC signaling server
│   └── borderless-relay/      # TURN relay server
├── apps/
│   ├── desktop/               # Tauri (Windows + macOS + Linux)
│   ├── mobile/                # Flutter (Android + iOS)
│   └── web/                   # Next.js 14 admin + user dashboard
├── migrations/                # PostgreSQL migrations
├── deploy/                    # Docker, Kubernetes, Helm
└── docs/                      # Documentation
```

---

## Security

- **E2E Encrypted**: All device-to-device traffic is AES-256-GCM encrypted. The server never sees plaintext.
- **Zero Trust**: Every device must be explicitly paired and trusted.
- **Consent**: Remote control sessions show a visible indicator on the target device.
- **Audit Log**: All admin actions are recorded in an append-only audit log.

See [SECURITY.md](docs/08-SECURITY.md) for the full security model.

---

## License

MIT — see [LICENSE](LICENSE)
