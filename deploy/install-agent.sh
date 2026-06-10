#!/usr/bin/env bash
# Borderless Agent installer for Debian/Ubuntu/CentOS/Arch Linux
# Usage: curl -sSL https://raw.githubusercontent.com/rythmmcosta/Borderless/main/deploy/install-agent.sh | sudo bash

set -euo pipefail

REPO="rythmmcosta/Borderless"
BINARY="borderless-agent"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/borderless-agent"
DATA_DIR="/var/lib/borderless-agent"

# ── Detect architecture ────────────────────────────────────────────────────────
ARCH=$(uname -m)
case $ARCH in
  x86_64)  TARGET="x86_64-unknown-linux-musl" ;;
  aarch64) TARGET="aarch64-unknown-linux-musl" ;;
  *) echo "Unsupported architecture: $ARCH" && exit 1 ;;
esac

# ── Fetch latest release ───────────────────────────────────────────────────────
echo "Fetching latest release from GitHub..."
RELEASE_URL="https://github.com/$REPO/releases/latest/download/${BINARY}-${TARGET}"

if command -v curl &>/dev/null; then
  curl -sSL -o "/tmp/$BINARY" "$RELEASE_URL"
elif command -v wget &>/dev/null; then
  wget -qO "/tmp/$BINARY" "$RELEASE_URL"
else
  echo "Error: curl or wget is required"
  exit 1
fi

# ── Install ────────────────────────────────────────────────────────────────────
chmod +x "/tmp/$BINARY"
mv "/tmp/$BINARY" "$INSTALL_DIR/$BINARY"
echo "Installed: $INSTALL_DIR/$BINARY"

# ── uinput group ──────────────────────────────────────────────────────────────
if ! getent group input &>/dev/null; then
  groupadd input
fi

# ── Create directories ─────────────────────────────────────────────────────────
mkdir -p "$CONFIG_DIR" "$DATA_DIR"
mkdir -p /run/borderless-agent

# ── Write default config if missing ───────────────────────────────────────────
if [ ! -f "$CONFIG_DIR/config.toml" ]; then
  HOSTNAME=$(cat /etc/hostname 2>/dev/null | tr -d '[:space:]' || echo "my-server")
  cat > "$CONFIG_DIR/config.toml" <<TOML
[agent]
server_url  = "https://borderless.myowncloud.tech/api"
device_name = "$HOSTNAME"

[auth]
email    = "admin@example.com"
password = "change-me"

[features]
clipboard_sync     = true
input_inject       = true
poll_interval_secs = 3
TOML
  echo "Created config: $CONFIG_DIR/config.toml"
  echo ""
  echo "  ⚠  Edit $CONFIG_DIR/config.toml and set your email/password."
fi

# ── Install & enable systemd service ──────────────────────────────────────────
"$INSTALL_DIR/$BINARY" install

echo ""
echo "Done! Next steps:"
echo "  1. Edit /etc/borderless-agent/config.toml"
echo "  2. sudo systemctl start borderless-agent"
echo "  3. sudo journalctl -u borderless-agent -f"
