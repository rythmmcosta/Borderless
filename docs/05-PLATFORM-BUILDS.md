# 05 — Platform Builds & Distribution

> How to build, sign, and distribute Borderless for every platform.

---

## Overview

| Platform | Format | Tool | Output |
|----------|--------|------|--------|
| Windows | `.exe` installer | NSIS / WiX | `borderless-setup-x64.exe` |
| Windows | `.msi` installer | WiX Toolset | `borderless-x64.msi` |
| macOS | `.dmg` | Tauri + create-dmg | `borderless-arm64.dmg` |
| macOS | `.pkg` | pkgbuild | `borderless.pkg` |
| Linux | `.deb` | dpkg-deb | `borderless_amd64.deb` |
| Linux | `.rpm` | rpmbuild | `borderless-x86_64.rpm` |
| Linux | `.AppImage` | AppImageKit | `borderless-x86_64.AppImage` |
| Linux | `.flatpak` | flatpak-builder | Flathub submission |
| Linux | `.snap` | snapcraft | Snapcraft store |
| Linux | `.iso` | mkisofs + casper | `borderless-live-amd64.iso` |
| Android | `.apk` | Flutter + Gradle | `borderless-release.apk` |
| Android | `.aab` | Flutter + Gradle | `borderless-release.aab` |
| iOS | `.ipa` | Xcode + Flutter | App Store submission |
| Docker | image | Docker | `ghcr.io/rythmmcosta/borderless` |

---

## 1. Windows

### Prerequisites
```powershell
winget install Rustlang.Rustup
winget install OpenJS.NodeJS.LTS
npm install -g pnpm
winget install Microsoft.VisualStudio.2022.BuildTools
pnpm add -g @tauri-apps/cli
```

### Build Commands
```powershell
cd apps/desktop
pnpm install
pnpm tauri dev      # Development
pnpm tauri build    # Production

# Output:
# .exe: apps/desktop/src-tauri/target/release/bundle/nsis/
# .msi: apps/desktop/src-tauri/target/release/bundle/msi/
```

### Code Signing (Windows)
```powershell
# Purchase certificate from Sectigo (~$200/yr) or SignPath (free for OSS)
# Store in GitHub Secrets:
# TAURI_SIGNING_PRIVATE_KEY = base64-encoded .pfx
# TAURI_SIGNING_PRIVATE_KEY_PASSWORD = cert password
```

---

## 2. macOS

### Prerequisites
```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add aarch64-apple-darwin x86_64-apple-darwin
brew install node pnpm
pnpm add -g @tauri-apps/cli
```

### Build Commands
```bash
pnpm tauri build --target aarch64-apple-darwin   # Apple Silicon
pnpm tauri build --target x86_64-apple-darwin    # Intel
pnpm tauri build --target universal-apple-darwin  # Universal
```

### Code Signing & Notarization
```bash
# Sign
codesign --force --deep --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --entitlements entitlements.plist --options runtime \
  apps/desktop/src-tauri/target/.../Borderless.app

# Notarize
xcrun notarytool submit borderless_arm64.dmg \
  --apple-id your@email.com --team-id TEAM_ID \
  --password @keychain:notarytool-password --wait

# Staple
xcrun stapler staple borderless_arm64.dmg
```

### macOS Entitlements (`entitlements.plist`)
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <!-- Required for global input capture (Accessibility) -->
  <key>com.apple.security.automation.apple-events</key><true/>
  <key>com.apple.security.network.client</key><true/>
  <key>com.apple.security.network.server</key><true/>
  <key>com.apple.security.network.multicast</key><true/>
</dict>
</plist>
```

---

## 3. Linux

### DEB Package (Ubuntu / Debian)
```bash
# Tauri automatically creates .deb:
cd apps/desktop && pnpm tauri build
# Output: target/release/bundle/deb/borderless_0.1.0_amd64.deb
```

### RPM Package (Fedora / CentOS)
```bash
sudo dnf install rpm-build rpmdevtools
rpmdev-setuptree
rpmbuild -ba ~/rpmbuild/SPECS/borderless.spec
```

### AppImage (Universal Linux)
```bash
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage
# Create AppDir, then:
ARCH=x86_64 ./appimagetool-x86_64.AppImage Borderless.AppDir borderless-x86_64.AppImage
```

### Flatpak
```yaml
app-id: app.borderless.desktop
runtime: org.freedesktop.Platform
runtime-version: '23.08'
sdk: org.freedesktop.Sdk
command: borderless
finish-args:
  - --share=network
  - --socket=wayland
  - --socket=fallback-x11
  - --device=all
  - --system-talk-name=org.freedesktop.Avahi
```

---

## 4. Linux ISO (Live USB)

A bootable Ubuntu-based live USB with Borderless pre-installed — useful for kiosk setups.

```bash
#!/bin/bash
# build-iso.sh — Builds a Borderless Live USB ISO
set -euo pipefail

ISO_NAME="borderless-live-amd64.iso"
WORK_DIR="/tmp/borderless-iso"

mkdir -p "$WORK_DIR"/{rootfs,iso/live,iso/boot/grub}

# Bootstrap Ubuntu 24.04 base, install packages, configure autostart...
# (see full script in deploy/build-iso.sh)

mksquashfs rootfs/ iso/live/filesystem.squashfs -comp xz
grub-mkrescue -o "$ISO_NAME" iso/ -- -volid "BORDERLESS_LIVE"

echo "✅ ISO built: $ISO_NAME"
echo "   Write to USB: sudo dd if=$ISO_NAME of=/dev/sdX bs=4M status=progress"
```

**ISO Features:**
- Ubuntu 24.04 LTS base, Borderless pre-installed and auto-started
- Openbox WM, Avahi mDNS, NetworkManager
- Auto-login as `borderless` user
- EFI + BIOS boot, ~800MB compressed

---

## 5. Android APK

```bash
cd apps/mobile

# Debug APK
flutter build apk --debug

# Release APK (signed)
flutter build apk --release

# App Bundle for Google Play
flutter build appbundle --release

# Split by ABI (smaller per-device downloads)
flutter build apk --split-per-abi
```

### Signing Setup
```bash
keytool -genkey -v -keystore borderless-release.jks \
  -keyalg RSA -keysize 2048 -validity 10000 -alias borderless
```

---

## 6. iOS IPA

```bash
# Must build on macOS with Xcode + Apple Developer account
cd apps/mobile
flutter build ios --release
# Then: Xcode → Product → Archive → Distribute App → App Store Connect
```

**Minimum:** iOS 15.0, iPhone 12+, iPad Air 4+

---

## 7. GitHub Actions Release Pipeline

```yaml
# .github/workflows/release.yml — triggered on git tag v*.*.*
jobs:
  build-matrix:
    strategy:
      matrix:
        include:
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: pnpm/action-setup@v3
      - name: Build desktop app
        run: cd apps/desktop && pnpm install && pnpm tauri build
      - uses: softprops/action-gh-release@v1
        with:
          files: apps/desktop/src-tauri/target/**/bundle/**
          draft: true

  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: subosito/flutter-action@v2
      - run: cd apps/mobile && flutter build apk --release --split-per-abi

  build-docker:
    runs-on: ubuntu-latest
    steps:
      - uses: docker/build-push-action@v5
        with:
          push: true
          tags: ghcr.io/rythmmcosta/borderless:${{ github.ref_name }}
```
