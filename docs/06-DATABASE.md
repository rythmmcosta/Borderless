# 06 — Database Schema

## Overview

Borderless uses **PostgreSQL 16** as the primary database, managed through Supabase (cloud) or a self-hosted instance.

**Key design decisions:**
- UUID primary keys everywhere (no sequential IDs — prevents enumeration)
- `TIMESTAMPTZ` for all timestamps (UTC everywhere)
- JSONB for flexible metadata/settings fields
- Immutable audit log (no UPDATE/DELETE via SQL rules)
- Row-Level Security (RLS) via Supabase policies

---

## Entity Relationship Diagram

```
organizations
     │ 1
     │
     │ ∞
   users ────────────────────────────────────────────┐
     │ 1                                        │
     │                                          │
     │ ∞                                        │
  devices ─────────────────┐               │
     │ 1              1 ◄─── device_pairs      │
     │                    ─── device_pairs      │
     │ ∞                                        │
  sessions ──────────── source_device           │
     │                  target_device           │
     │ 1                initiated_by ───────────┘
     │
     ∞
 (sessions link to clipboard_history, file_transfers, notifications)

audit_logs ◄── actor_id (users) + actor_device_id (devices)

clipboard_history ◄── user_id + source_device_id
file_transfers    ◄── sender_device_id + receiver_device_id
notifications     ◄── source_device_id + target_device_id
refresh_tokens    ◄── user_id
```

---

## Tables

### `organizations`
```sql
CREATE TABLE organizations (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        TEXT NOT NULL,
    slug        TEXT UNIQUE NOT NULL,      -- URL-safe org identifier
    owner_id    UUID REFERENCES users(id) ON DELETE SET NULL,
    plan        TEXT NOT NULL DEFAULT 'free',  -- 'free' | 'pro' | 'enterprise'
    max_devices INTEGER NOT NULL DEFAULT 10,
    settings    JSONB NOT NULL DEFAULT '{}',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- settings JSONB structure:
{
  "allow_remote_control": true,
  "require_consent": true,
  "session_timeout_minutes": 60,
  "clipboard_sync": true,
  "file_transfer_max_mb": 500,
  "ip_allowlist": [],
  "notification_sync": true
}
```

### `users`
```sql
CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email         TEXT UNIQUE NOT NULL,
    password_hash TEXT,                     -- null = OAuth-only user
    display_name  TEXT,
    avatar_url    TEXT,
    role          user_role NOT NULL DEFAULT 'user',
    org_id        UUID REFERENCES organizations(id) ON DELETE SET NULL,
    mfa_enabled   BOOLEAN NOT NULL DEFAULT FALSE,
    mfa_secret    TEXT,                     -- TOTP secret (encrypted at app level)
    public_key    TEXT,                     -- Ed25519 public key hex
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at  TIMESTAMPTZ,
    is_active     BOOLEAN NOT NULL DEFAULT TRUE
);

-- role enum:
CREATE TYPE user_role AS ENUM ('user', 'admin', 'super_admin');
```

### `refresh_tokens`
```sql
CREATE TABLE refresh_tokens (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash  TEXT NOT NULL UNIQUE,      -- sha256(raw_token) stored, never raw
    expires_at  TIMESTAMPTZ NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    used_at     TIMESTAMPTZ,               -- set when used (single-use rotation)
    revoked     BOOLEAN NOT NULL DEFAULT FALSE,
    user_agent  TEXT,
    ip_address  INET
);
```

### `devices`
```sql
CREATE TABLE devices (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    org_id       UUID REFERENCES organizations(id),
    name         TEXT NOT NULL,
    fingerprint  TEXT UNIQUE NOT NULL,     -- hardware fingerprint (SHA256 hex)
    platform     device_platform NOT NULL,
    os_version   TEXT,                     -- "Windows 11 22H2"
    app_version  TEXT,                     -- "0.1.0"
    public_key   TEXT NOT NULL,            -- X25519 DH public key (hex)
    sign_pubkey  TEXT NOT NULL,            -- Ed25519 verifying key (hex)
    device_token TEXT UNIQUE NOT NULL DEFAULT gen_random_uuid()::TEXT,
    is_online    BOOLEAN NOT NULL DEFAULT FALSE,
    is_locked    BOOLEAN NOT NULL DEFAULT FALSE,
    last_ip      INET,
    last_seen_at TIMESTAMPTZ,
    trusted_at   TIMESTAMPTZ,
    metadata     JSONB NOT NULL DEFAULT '{}'
);

-- platform enum:
CREATE TYPE device_platform AS ENUM (
    'windows', 'macos', 'linux', 'android', 'ios', 'web'
);

-- metadata JSONB structure:
{
  "screen_width": 1920,
  "screen_height": 1080,
  "screen_count": 2,
  "cpu_arch": "x86_64",
  "hostname": "my-desktop"
}
```

### `device_pairs`
```sql
CREATE TABLE device_pairs (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    device_a_id       UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    device_b_id       UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    layout_position   JSONB NOT NULL DEFAULT '{}',
    kvm_enabled       BOOLEAN NOT NULL DEFAULT TRUE,
    clipboard_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    file_enabled      BOOLEAN NOT NULL DEFAULT TRUE,
    notif_enabled     BOOLEAN NOT NULL DEFAULT TRUE,
    paired_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(device_a_id, device_b_id),
    CHECK(device_a_id < device_b_id)  -- canonical ordering prevents duplicates
);

-- layout_position JSONB structure:
{
  "device_a": { "edge": "right", "neighbour": "device_b_id" },
  "device_b": { "edge": "left",  "neighbour": "device_a_id" },
  "device_a_screen": { "w": 1920, "h": 1080 },
  "device_b_screen": { "w": 2560, "h": 1600 }
}
```

### `sessions`
```sql
CREATE TABLE sessions (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_device_id UUID NOT NULL REFERENCES devices(id),
    target_device_id UUID NOT NULL REFERENCES devices(id),
    initiated_by     UUID REFERENCES users(id),
    session_type     session_type NOT NULL,
    transport        transport_type NOT NULL DEFAULT 'p2p',
    started_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at         TIMESTAMPTZ,
    duration_secs    INTEGER GENERATED ALWAYS AS (
        EXTRACT(EPOCH FROM (ended_at - started_at))::INTEGER
    ) STORED,
    bytes_sent       BIGINT NOT NULL DEFAULT 0,
    bytes_recv       BIGINT NOT NULL DEFAULT 0,
    events_count     BIGINT NOT NULL DEFAULT 0,
    CHECK(source_device_id != target_device_id)
);

CREATE TYPE session_type AS ENUM ('kvm', 'remote_control', 'view_only');
CREATE TYPE transport_type AS ENUM ('lan', 'p2p', 'relay');
```

### `clipboard_history`
```sql
CREATE TABLE clipboard_history (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id          UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    source_device_id UUID REFERENCES devices(id) ON DELETE SET NULL,
    content_type     TEXT NOT NULL,        -- "text/plain" | "text/html" | "image/png"
    content_hash     TEXT NOT NULL,        -- SHA256 for deduplication
    content_encrypted BYTEA,              -- AES-256-GCM encrypted content
    content_size     INTEGER NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at       TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '7 days'
);
```

### `file_transfers`
```sql
CREATE TABLE file_transfers (
    id                 UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sender_device_id   UUID NOT NULL REFERENCES devices(id),
    receiver_device_id UUID NOT NULL REFERENCES devices(id),
    initiated_by       UUID REFERENCES users(id),
    file_name          TEXT NOT NULL,
    file_size          BIGINT NOT NULL,
    mime_type          TEXT,
    sha256_hex         TEXT NOT NULL,
    status             transfer_status NOT NULL DEFAULT 'pending',
    bytes_transferred  BIGINT NOT NULL DEFAULT 0,
    storage_path       TEXT,              -- Supabase Storage path (relay transfers)
    transfer_method    TEXT NOT NULL DEFAULT 'p2p',
    error_message      TEXT,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at       TIMESTAMPTZ
);

CREATE TYPE transfer_status AS ENUM (
    'pending', 'accepted', 'transferring', 'verifying',
    'completed', 'failed', 'cancelled', 'rejected'
);
```

### `notifications`
```sql
CREATE TABLE notifications (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_device_id UUID NOT NULL REFERENCES devices(id),
    target_device_id UUID NOT NULL REFERENCES devices(id),
    app_name         TEXT NOT NULL,
    app_icon_url     TEXT,
    title            TEXT NOT NULL,
    body             TEXT,
    image_url        TEXT,
    actions          JSONB NOT NULL DEFAULT '[]',
    status           notification_status NOT NULL DEFAULT 'pending',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_at     TIMESTAMPTZ,
    dismissed_at     TIMESTAMPTZ
);

CREATE TYPE notification_status AS ENUM ('pending', 'delivered', 'dismissed', 'expired');

-- actions JSONB structure:
[
  { "id": "reply", "label": "Reply", "type": "text_input" },
  { "id": "dismiss", "label": "Dismiss", "type": "button" }
]
```

### `audit_logs` (Append-Only)
```sql
CREATE TABLE audit_logs (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    actor_id        UUID REFERENCES users(id) ON DELETE SET NULL,
    actor_device_id UUID REFERENCES devices(id) ON DELETE SET NULL,
    org_id          UUID REFERENCES organizations(id) ON DELETE SET NULL,
    action          TEXT NOT NULL,
    resource_type   TEXT,
    resource_id     UUID,
    ip_address      INET,
    user_agent      TEXT,
    metadata        JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- IMMUTABLE: These rules prevent any modification or deletion
CREATE RULE no_update_audit AS ON UPDATE TO audit_logs DO INSTEAD NOTHING;
CREATE RULE no_delete_audit AS ON DELETE TO audit_logs DO INSTEAD NOTHING;
```

---

## Indexes

```sql
-- Devices
CREATE INDEX idx_devices_user     ON devices(user_id);
CREATE INDEX idx_devices_org      ON devices(org_id);
CREATE INDEX idx_devices_online   ON devices(is_online) WHERE is_online = TRUE;
CREATE INDEX idx_devices_fp       ON devices(fingerprint);
CREATE INDEX idx_devices_token    ON devices(device_token);

-- Sessions
CREATE INDEX idx_sessions_src     ON sessions(source_device_id, started_at DESC);
CREATE INDEX idx_sessions_tgt     ON sessions(target_device_id, started_at DESC);
CREATE INDEX idx_sessions_active  ON sessions(ended_at) WHERE ended_at IS NULL;

-- Clipboard
CREATE INDEX idx_clipboard_user   ON clipboard_history(user_id, created_at DESC);
CREATE INDEX idx_clipboard_hash   ON clipboard_history(user_id, content_hash);
CREATE INDEX idx_clipboard_expiry ON clipboard_history(expires_at);

-- Transfers
CREATE INDEX idx_transfers_sender ON file_transfers(sender_device_id, created_at DESC);
CREATE INDEX idx_transfers_recv   ON file_transfers(receiver_device_id, created_at DESC);
CREATE INDEX idx_transfers_status ON file_transfers(status) WHERE status != 'completed';

-- Notifications
CREATE INDEX idx_notifs_target    ON notifications(target_device_id, status);

-- Audit
CREATE INDEX idx_audit_actor      ON audit_logs(actor_id, created_at DESC);
CREATE INDEX idx_audit_org        ON audit_logs(org_id, created_at DESC);
CREATE INDEX idx_audit_action     ON audit_logs(action, created_at DESC);
CREATE INDEX idx_audit_resource   ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_created    ON audit_logs(created_at DESC);

-- Refresh tokens
CREATE INDEX idx_refresh_user     ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_expiry   ON refresh_tokens(expires_at)
    WHERE revoked = FALSE;
```

---

## Supabase Row-Level Security Policies

```sql
-- Enable RLS on all tables
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE devices ENABLE ROW LEVEL SECURITY;
ALTER TABLE sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE clipboard_history ENABLE ROW LEVEL SECURITY;
ALTER TABLE file_transfers ENABLE ROW LEVEL SECURITY;
ALTER TABLE notifications ENABLE ROW LEVEL SECURITY;

-- Users: see only themselves (admin sees all via service role)
CREATE POLICY "users_self" ON users
    FOR SELECT USING (auth.uid() = id);
CREATE POLICY "users_update_self" ON users
    FOR UPDATE USING (auth.uid() = id);

-- Devices: see only own devices
CREATE POLICY "devices_own" ON devices
    FOR ALL USING (auth.uid() = user_id);

-- Sessions: see sessions involving own devices
CREATE POLICY "sessions_own" ON sessions
    FOR SELECT USING (
        source_device_id IN (SELECT id FROM devices WHERE user_id = auth.uid())
        OR
        target_device_id IN (SELECT id FROM devices WHERE user_id = auth.uid())
    );

-- Clipboard: see only own
CREATE POLICY "clipboard_own" ON clipboard_history
    FOR ALL USING (auth.uid() = user_id);

-- Admin bypass: use service role key in backend, bypasses RLS
-- All admin operations go through borderless-server with service role
```

---

## Migrations

```
migrations/
├── 001_initial.sql        -- Core schema (enums, tables, indexes)
├── 002_rls_policies.sql   -- Row-level security
├── 003_functions.sql      -- Utility functions (cleanup, etc)
└── 004_seed.sql           -- Development seed data
```

### Running Migrations
```bash
# Development (Docker)
docker compose -f deploy/docker-compose.yml up -d db
cargo run --package borderless-server  # runs sqlx migrate! on startup

# Manual
sqlx migrate run --database-url postgresql://postgres:postgres@localhost:5432/borderless

# Rollback
sqlx migrate revert --database-url ...
```

---

## Maintenance Queries

```sql
-- Cleanup expired clipboard (run daily)
DELETE FROM clipboard_history WHERE expires_at < NOW();

-- Mark devices offline if not seen in 2 minutes
UPDATE devices SET is_online = FALSE
WHERE is_online = TRUE AND last_seen_at < NOW() - INTERVAL '2 minutes';

-- Close orphaned sessions
UPDATE sessions SET ended_at = NOW()
WHERE ended_at IS NULL AND started_at < NOW() - INTERVAL '24 hours';

-- Archive old audit logs (> 1 year) to cold storage
-- (Do NOT delete — export to S3 first)
COPY (SELECT * FROM audit_logs WHERE created_at < NOW() - INTERVAL '1 year')
TO '/backups/audit_archive.csv' CSV HEADER;
```
