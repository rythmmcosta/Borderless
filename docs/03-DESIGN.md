# 03 — Design System & UI/UX

## Design Philosophy

Borderless lives in the background — it should feel **invisible when working perfectly**, and **clear and trustworthy** when you need to interact with it.

### Design Principles
1. **Minimal Interruption** — Never pull focus. Toasts over modals. System tray over windows.
2. **Instant Feedback** — Every action responds in < 100ms visually, even if the operation takes longer.
3. **Trust by Visibility** — Users always know which device their mouse controls. Never ambiguous.
4. **Progressive Disclosure** — Simple by default, powerful when you dig in.
5. **Platform Native** — Respect each OS's conventions. macOS menu bar. Windows system tray. Android bottom nav.

---

## Design Tokens

### Color Palette

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  BRAND COLORS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Primary Cyan        #00E5FF    ████  Active states, links, CTA
  Primary Dim         #0097A7    ████  Hover on cyan elements
  Purple Accent       #7C3AED    ████  Session active, special states
  Orange Accent       #F97316    ████  Warnings, Windows platform badge
  Green Accent        #10B981    ████  Success, online, connected
  Red Accent          #EF4444    ████  Error, offline, danger
  Yellow Accent       #F59E0B    ████  Caution, pending, admin badge

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  DARK THEME (Default)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Background          #0A0A0F    ████  App background
  Surface             #111118    ████  Cards, panels
  Surface Elevated    #16161F    ████  Modals, dropdowns
  Surface Border      #1E1E2E    ████  Dividers, outlines
  Text Primary        #E2E8F0    ████  Main text
  Text Secondary      #94A3B8    ████  Labels, metadata
  Text Muted          #64748B    ████  Placeholders, disabled
  Text Inverse        #0A0A0F    ████  Text on colored backgrounds

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  LIGHT THEME
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Background          #F8FAFC    ████  App background
  Surface             #FFFFFF    ████  Cards, panels
  Surface Elevated    #F1F5F9    ████  Modals, dropdowns
  Surface Border      #E2E8F0    ████  Dividers, outlines
  Text Primary        #0F172A    ████  Main text
  Text Secondary      #475569    ████  Labels, metadata
  Text Muted          #94A3B8    ████  Placeholders, disabled
```

### Typography

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  FONTS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Display    Syne 800          — App name, hero headlines
  Heading    Syne 700          — Section titles, card headers
  UI Label   IBM Plex Sans 500 — Buttons, nav items, labels
  Body       IBM Plex Sans 400 — Paragraphs, descriptions
  Mono       Space Mono 400    — Device IDs, code, metrics
  Mono Bold  Space Mono 700    — Stats, values, badges

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  TYPE SCALE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  xs      11px / 1.4   — Badges, captions, timestamps
  sm      13px / 1.5   — Secondary text, table cells
  base    14px / 1.7   — Default body text
  md      16px / 1.6   — Form inputs, prominent labels
  lg      18px / 1.5   — Card titles
  xl      22px / 1.3   — Section headers
  2xl     28px / 1.2   — Page titles
  3xl     36px / 1.1   — Hero text
  4xl     48px / 1.0   — Display text
```

### Spacing & Layout

```
Base unit: 4px

  xs    4px    — Tight internal padding
  sm    8px    — Component internal gaps
  md    12px   — Between related elements
  lg    16px   — Card padding (compact)
  xl    24px   — Card padding (standard)
  2xl   32px   — Section padding
  3xl   48px   — Page section separation
  4xl   64px   — Hero sections
  5xl   96px   — Major page sections

Layout Grid: 12 columns, 24px gutter, 32px margin
Max content width: 1200px
```

### Border Radius

```
  none    0px     — Borderless brand aesthetic (sharp)
  sm      2px     — Badges, tags
  md      4px     — Buttons, inputs
  lg      8px     — Cards
  xl      12px    — Large cards, modals
  full    9999px  — Pills, avatars
```

### Shadows

```
  sm      0 1px 3px rgba(0,0,0,0.4)       — Subtle elevation
  md      0 4px 12px rgba(0,0,0,0.5)      — Cards, dropdowns
  lg      0 12px 32px rgba(0,0,0,0.6)     — Modals, overlays
  glow    0 0 20px rgba(0,229,255,0.15)   — Active device indicator
  danger  0 0 20px rgba(239,68,68,0.2)    — Error states
```

---

## Component Library

### Status Indicator
```
╭─────────────────────────────────────╮
│  ● Connected    ○ Offline    ◌ Sync  │
╰─────────────────────────────────────╯

● #10B981 green pulse  = Online and active
● #F59E0B yellow pulse = Online, syncing
● #64748B grey static  = Offline
● #EF4444 red pulse    = Error / disconnected
```

### Device Card
```
╭──────────────────────────────────────────╮
│  🖥  MacBook Pro                    ● Online │
│      macOS 14.2 · Borderless 0.1.0          │
│      IP: 192.168.1.42  ·  Last: just now    │
│                                              │
│  [  Connect KVM  ]  [ View Screen ]  [···]  │
╰──────────────────────────────────────────────╯
```

### Toast Notifications
```
┌─────────────────────────────────────────────┐
│ ✓  Clipboard synced to MacBook Pro          │  ← Bottom-right
│    Text · 48 chars                          │     Auto-dismiss 3s
└─────────────────────────────────────────────┘

┌─────────────────────────────────────────────┐
│ ⚠  Connection lost to Desktop               │  ← Persistent until
│    Attempting reconnect...  [Dismiss]        │     resolved
└─────────────────────────────────────────────┘
```

---

## Screen Designs

### 1. Desktop App — Main Window

```
╔══════════════════════════════════════════════════════╗
║  BORDERLESS                              ─  □  ✕    ║
╠══════════════════════════════════════════════════════╣
║                                                       ║
║  Devices                           [+ Add Device]    ║
║  ─────────────────────────────────────────────────── ║
║                                                       ║
║  ┌─────────────────────────────────────────────────┐ ║
║  │ 🖥  This PC (Local)                      ● Here │ ║
║  │     Windows 11 · 1920×1080               ──────  │ ║
║  └─────────────────────────────────────────────────┘ ║
║                                                       ║
║  ┌─────────────────────────────────────────────────┐ ║
║  │ 💻  MacBook Pro                         ● Active │ ║
║  │     macOS 14.2 · 2560×1600                       │ ║
║  │     [■ KVM Active] [Clipboard ✓] [Files ✓]       │ ║
║  │     [  Disconnect  ]  [ Remote View ]  [ ··· ]   │ ║
║  └─────────────────────────────────────────────────┘ ║
║                                                       ║
║  ┌─────────────────────────────────────────────────┐ ║
║  │ 🐧  Ubuntu Server                        ○ Away  │ ║
║  │     Linux 22.04 · Last seen 2m ago               │ ║
║  │     [  Connect  ]                  [ ··· ]        │ ║
║  └─────────────────────────────────────────────────┘ ║
║                                                       ║
║  ─────────────────────────────────────────────────── ║
║  Layout      Clipboard     Settings      Admin        ║
╚══════════════════════════════════════════════════════╝
```

### 2. Desktop App — Device Layout Editor

```
╔══════════════════════════════════════════════════════╗
║  BORDERLESS › Layout Editor                          ║
╠══════════════════════════════════════════════════════╣
║                                                       ║
║  Drag devices to arrange their physical positions    ║
║                                                       ║
║  ┌─────────────────────────────────────────────────┐ ║
║  │                                                  │ ║
║  │   ┌──────────────┐    ┌──────────────────────┐  │ ║
║  │   │              │    │                      │  │ ║
║  │   │   This PC    │◄──►│    MacBook Pro       │  │ ║
║  │   │  Windows 11  │    │    macOS 14.2        │  │ ║
║  │   │  1920×1080   │    │    2560×1600         │  │ ║
║  │   │              │    │                      │  │ ║
║  │   └──────────────┘    └──────────────────────┘  │ ║
║  │         ▲                                        │ ║
║  │         │                                        │ ║
║  │   ┌──────────────┐                              │ ║
║  │   │ Ubuntu Server│                              │ ║
║  │   │  Linux 22.04 │                              │ ║
║  │   └──────────────┘                              │ ║
║  └─────────────────────────────────────────────────┘ ║
║                                                       ║
║  Edge Sensitivity: [────●────────] 5px               ║
║  Switch Hotkey:    [Ctrl+Alt+→]   [Change]            ║
║                                                       ║
║  [ Save Layout ]              [ Reset to Default ]   ║
╚══════════════════════════════════════════════════════╝
```

### 3. Web Dashboard — Admin Home

```
╔══════════════════════════════════════════════════════════════════╗
║  BORDERLESS ADMIN               [🔔 3]  [rythmmcosta ▼]         ║
╠══════╦═══════════════════════════════════════════════════════════╣
║      ║                                                           ║
║  📊  ║   Dashboard                                               ║
║      ║   ─────────────────────────────────────────────────────  ║
║  🖥  ║                                                           ║
║  Dvc ║   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐ ║
║      ║   │  Total   │  │  Online  │  │Sessions  │  │Errors  │ ║
║  👥  ║   │  Users   │  │ Devices  │  │  Today   │  │ Today  │ ║
║  Usr ║   │    47    │  │  23/31   │  │   142    │  │    2   │ ║
║      ║   └──────────┘  └──────────┘  └──────────┘  └────────┘ ║
║  📋  ║                                                           ║
║  Aud ║   Live Device Map                                         ║
║      ║   ┌─────────────────────────────────────────────────┐   ║
║  📊  ║   │  ●  user1-laptop   Win11   192.168.1.10  Active │   ║
║  Stat║   │  ●  user1-mac      macOS   192.168.1.11  Active │   ║
║      ║   │  ●  user2-desktop  Win10   10.0.0.5      Idle   │   ║
║  ⚙️  ║   │  ○  user3-linux    Ubuntu  —             Offline│   ║
║  Cfg ║   └─────────────────────────────────────────────────┘   ║
╚══════╩═══════════════════════════════════════════════════════════╝
```

### 4. Mobile App — Home (Flutter)

```
┌─────────────────────────┐
│ BORDERLESS          [⚙] │
│                          │
│  ● Connected  3 devices  │
│                          │
│  MY DEVICES              │
│  ──────────────────────  │
│                          │
│  ┌──────────────────────┐│
│  │ 🖥 Windows PC  ●    ││
│  │ KVM Active           ││
│  │ [Trackpad] [Manage]  ││
│  └──────────────────────┘│
│                          │
│  ┌──────────────────────┐│
│  │ 💻 MacBook Pro  ●   ││
│  │ Synced               ││
│  │ [Connect ] [Manage]  ││
│  └──────────────────────┘│
│                          │
│  [+ Add New Device]      │
│                          │
│ [Home] [Clipboard] [More]│
└─────────────────────────┘
```

### 5. Mobile App — Trackpad Mode

```
┌─────────────────────────┐
│ ← Controlling: Win PC   │
│                     [✕] │
│                          │
│  ┌──────────────────────┐│
│  │                      ││
│  │    TRACKPAD AREA     ││
│  │                      ││
│  │  Move finger here    ││
│  │  to move cursor on   ││
│  │  Windows PC          ││
│  │                      ││
│  │  Tap = Left Click    ││
│  │  2 fingers = Scroll  ││
│  │  Long press = Right  ││
│  └──────────────────────┘│
│                          │
│  [L CLICK] [R CLICK] [KB]│
└─────────────────────────┘
```

---

## System Tray / Menu Bar Design

### Windows System Tray (Right-click menu)
```
┌─────────────────────────────────┐
│  BORDERLESS v0.1.0              │
│  ─────────────────────────────  │
│  ● Controlling: MacBook Pro     │
│  ─────────────────────────────  │
│  Devices                      → │
│    ● This PC (local)            │
│    ● MacBook Pro ✓              │
│    ○ Ubuntu Server              │
│  ─────────────────────────────  │
│  Recall Cursor  Ctrl+Alt+R      │
│  Lock Cursor    Ctrl+Alt+L      │
│  ─────────────────────────────  │
│  Open Borderless                │
│  Settings                       │
│  ─────────────────────────────  │
│  Quit                           │
└─────────────────────────────────┘
```

### macOS Menu Bar
```
[●] Borderless
├── Status: Controlling MacBook Pro
├── ───────────────────────
├── Devices
│   ├── ● This Mac (local)
│   ├── ● Windows PC ✓
│   └── ○ Ubuntu Server
├── ───────────────────────
├── Recall Cursor    ⌃⌥R
├── Lock Cursor      ⌃⌥L
├── ───────────────────────
├── Open Borderless
├── Preferences...
├── ───────────────────────
└── Quit Borderless   ⌘Q
```

---

## Iconography

### App Icon
```
Shape:     Rounded square (iOS style on mobile, standard on desktop)
Symbol:    Two overlapping rectangles (screens) with arrows between them
Color:     Gradient — #00E5FF → #7C3AED (cyan to purple)
Dark bg:   #0A0A0F background
Sizes:     16, 32, 48, 64, 128, 256, 512, 1024px
```

---

## Animation & Motion

### Micro-interactions
```
Button hover:     translateY(-1px), 150ms ease
Button press:     translateY(0px), 80ms ease
Card hover:       border-color lightens, 200ms ease
Cursor crossing:  Flash indicator on edge, 100ms
Device switch:    Status badge fades, 300ms
```

### Screen Transitions
```
Route changes:    Fade in, 200ms ease
Modal open:       Scale 0.96→1.0 + fade, 250ms ease-out
Modal close:      Scale 1.0→0.96 + fade, 200ms ease-in
Toast in:         Slide up + fade, 300ms ease-out
Toast out:        Slide down + fade, 200ms ease-in
```

---

## Accessibility

- **WCAG 2.1 AA** compliance minimum
- All interactive elements keyboard navigable
- Screen reader support (ARIA labels on all custom components)
- Color contrast ratio ≥ 4.5:1 for body text
- Focus indicators visible and clear
- No reliance on color alone for status (use icons + text too)
- Reduce motion mode: disable non-essential animations
