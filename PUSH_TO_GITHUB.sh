#!/bin/bash
# ─────────────────────────────────────────────────────────
#  Borderless — Push to GitHub
#  Run this script ONCE from your local machine after 
#  extracting the zip file.
# ─────────────────────────────────────────────────────────
set -e

echo "🚀 Setting up Borderless repo for GitHub..."

# Initialize git if not already done
if [ ! -d ".git" ]; then
  git init
  git branch -M main
fi

# Stage everything
git add .

# Initial commit
git commit -m "feat: Phase 1 foundation — Rust core engine, input routing, crypto, sessions

WHAT'S IN THIS COMMIT:
======================
Core Engine (borderless-core)
  - DeviceIdentity  — Ed25519 signing + X25519 ECDH, hardware fingerprint, persist to disk
  - SessionKey      — AES-256-GCM encrypt/decrypt with prepended nonce
  - InputRouter     — Mouse Without Borders edge-crossing logic, device layout grid
  - WindowsCapture  — SetWindowsHookExW (WH_MOUSE_LL + WH_KEYBOARD_LL) global hooks
  - WindowsInject   — SendInput absolute mouse + keyboard + unicode text injection
  - ClipboardMonitor— Poll-based clipboard change detection + cross-platform sync
  - DiscoveryService— mDNS-SD LAN device discovery scaffold
  - Session         — State machine + encrypted send/receive channels
  - Transfer        — Chunked file transfer with SHA-256 integrity verification
  - Protocol        — Wire envelope format + message types

Backend (borderless-server)
  - Axum REST API server
  - JWT auth middleware + admin role check
  - Admin routes: fleet view, device lock, remote session init, audit log, stats
  - PostgreSQL schema (9 tables, immutable audit log)
  - Redis session store

Infra
  - Docker Compose (Postgres + Redis + Coturn TURN + API + Web)
  - GitHub Actions CI (Rust clippy + tests on Win/Mac/Linux, Flutter, Next.js)
  - GitHub Actions Release (signed Tauri binaries, Docker image)

Tests: 14 unit tests covering crypto, router, transfer, identity

Phase 1 TODO (next steps):
  - Wire TCP transport into session.rs connect()
  - Implement mDNS-SD loop in discovery.rs
  - macOS CGEventTap capture + inject
  - Linux evdev capture + uinput inject
  - Tauri desktop app shell
  - Clipboard platform impls (macOS/Linux)
"

# Set remote origin
git remote remove origin 2>/dev/null || true
git remote add origin https://github.com/rythmmcosta/Borderless.git

echo ""
echo "✅ Ready to push! Run:"
echo "   git push -u origin main"
echo ""
echo "If you want to use SSH instead:"
echo "   git remote set-url origin git@github.com:rythmmcosta/Borderless.git"
echo "   git push -u origin main"
