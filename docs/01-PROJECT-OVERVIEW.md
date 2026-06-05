# 01 — Project Overview

## What Is Borderless?

Borderless is a **free, open-source, cross-platform device synchronization and KVM (Keyboard-Video-Mouse) software** that lets you seamlessly control multiple computers, phones, and tablets using a single mouse and keyboard.

---

## The Problem

Modern professionals use multiple computers daily — Windows workstation, MacBook, Linux server, Android tablet, iPhone. Current tools:
- **Mouse Without Borders**: Windows-only, discontinued
- **Synergy**: Paid, complex, security concerns
- **ShareMouse**: Paid, closed-source
- None work across all 6 platforms simultaneously
- None provide cloud sync across networks
- None have a proper admin dashboard

---

## The Solution

```
┌────────────────────────────────────────────────────────────┐
│                     BORDERLESS                           │
│  One mouse. One keyboard. All your devices.             │
│  Windows ←──→ macOS ←──→ Linux ←──→ Android ←──→ iOS   │
│                         ↕                               │
│              Web Dashboard (Admin + User)               │
└────────────────────────────────────────────────────────────┘
```

### Core Features

| Feature | Description |
|---------|-------------|
| **Mouse/Keyboard KVM** | Move cursor to screen edge → instantly controls next machine |
| **Clipboard Sync** | Copy on one device, paste on any other (text, images, HTML) |
| **File Transfer** | Drag files between screens — appears on the other machine |
| **Notification Sync** | Phone notifications appear on your desktop |
| **LAN Mode** | Ultra-low latency on the same network (< 5ms) |
| **Internet Mode** | WebRTC P2P across different networks |
| **Cloud Relay** | TURN server fallback when P2P fails |
| **Admin Dashboard** | Web UI to manage all devices, sessions, audit logs |
| **Remote Desktop** | Full screen control of any paired device |
| **Offline First** | LAN mode works with no internet |

---

## How It Compares

| Feature | Borderless | Mouse Without Borders | Synergy | ShareMouse |
|---------|-----------|----------------------|---------|------------|
| Windows | ✅ | ✅ | ✅ | ✅ |
| macOS | ✅ | ❌ | ✅ | ✅ |
| Linux | ✅ | ❌ | ✅ | ❌ |
| Android | ✅ | ❌ | ❌ | ❌ |
| iOS | ✅ | ❌ | ❌ | ❌ |
| Web App | ✅ | ❌ | ❌ | ❌ |
| Internet (non-LAN) | ✅ | ❌ | ✅ (paid) | ❌ |
| Cloud Sync | ✅ | ❌ | ❌ | ❌ |
| Admin Dashboard | ✅ | ❌ | ❌ | ❌ |
| Open Source | ✅ | ❌ | ❌ | ❌ |
| Free | ✅ | ✅ | Freemium | Freemium |
| E2E Encrypted | ✅ | ❌ | ❌ | ❌ |
| Notification Sync | ✅ | ❌ | ❌ | ❌ |

---

## Technical Vision

### Performance Targets
- **Input latency (LAN):** < 5ms
- **Input latency (Internet P2P):** < 30ms
- **Input latency (TURN relay):** < 80ms
- **Clipboard sync time:** < 500ms
- **File transfer speed (LAN):** > 100 MB/s
- **CPU usage (idle):** < 0.5%
- **Memory footprint:** < 50 MB

---

## Project Status

| Component | Status | Phase |
|-----------|--------|-------|
| Core Rust engine | 🟡 In Progress | Phase 1 |
| Windows input capture | 🟡 In Progress | Phase 1 |
| macOS input capture | 🔴 Not Started | Phase 1 |
| Linux input capture | 🔴 Not Started | Phase 1 |
| LAN discovery | 🟡 Scaffold | Phase 1 |
| TCP session transport | 🔴 Not Started | Phase 1 |
| Cloud backend (Axum) | 🟡 Scaffold | Phase 2 |
| WebRTC P2P | 🔴 Not Started | Phase 2 |
| Clipboard sync | 🟡 Windows only | Phase 3 |
| File transfer | ✅ Engine done | Phase 3 |
| Tauri desktop app | 🔴 Not Started | Phase 1 |
| Flutter mobile app | 🔴 Not Started | Phase 3 |
| Next.js web dashboard | 🔴 Not Started | Phase 4 |
| Admin panel | 🟡 API scaffold | Phase 4 |
