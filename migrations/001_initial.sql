-- migrations/001_initial.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TYPE user_role AS ENUM ('user', 'admin', 'super_admin');
CREATE TYPE device_platform AS ENUM ('windows', 'macos', 'linux', 'android', 'ios', 'web');
CREATE TYPE session_type AS ENUM ('kvm', 'remote_control', 'view_only');
CREATE TYPE transport_type AS ENUM ('lan', 'p2p', 'relay');
CREATE TYPE transfer_status AS ENUM ('pending', 'transferring', 'completed', 'failed', 'cancelled');
CREATE TYPE notification_status AS ENUM ('pending', 'delivered', 'dismissed');

CREATE TABLE users (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email        TEXT UNIQUE NOT NULL,
    password_hash TEXT,
    display_name TEXT,
    avatar_url   TEXT,
    role         user_role NOT NULL DEFAULT 'user',
    mfa_enabled  BOOLEAN NOT NULL DEFAULT FALSE,
    mfa_secret   TEXT,
    public_key   TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ,
    is_active    BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE refresh_tokens (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    used_at    TIMESTAMPTZ,
    revoked    BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE devices (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name         TEXT NOT NULL,
    fingerprint  TEXT UNIQUE NOT NULL,
    platform     device_platform NOT NULL,
    os_version   TEXT,
    app_version  TEXT,
    public_key   TEXT NOT NULL,
    device_token TEXT UNIQUE NOT NULL DEFAULT gen_random_uuid()::TEXT,
    is_online    BOOLEAN NOT NULL DEFAULT FALSE,
    last_ip      INET,
    last_seen_at TIMESTAMPTZ,
    trusted_at   TIMESTAMPTZ,
    is_locked    BOOLEAN NOT NULL DEFAULT FALSE,
    metadata     JSONB NOT NULL DEFAULT '{}'
);

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
    CHECK(device_a_id != device_b_id)
);

CREATE TABLE sessions (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_device_id UUID NOT NULL REFERENCES devices(id),
    target_device_id UUID NOT NULL REFERENCES devices(id),
    initiated_by     UUID REFERENCES users(id),
    session_type     session_type NOT NULL,
    transport        transport_type NOT NULL DEFAULT 'p2p',
    started_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at         TIMESTAMPTZ,
    bytes_sent       BIGINT NOT NULL DEFAULT 0,
    bytes_recv       BIGINT NOT NULL DEFAULT 0,
    CHECK(source_device_id != target_device_id)
);

CREATE TABLE clipboard_history (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id          UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    source_device_id UUID REFERENCES devices(id) ON DELETE SET NULL,
    content_type     TEXT NOT NULL,
    content_hash     TEXT NOT NULL,
    content_encrypted BYTEA,
    content_size     INTEGER NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at       TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '7 days'
);

CREATE TABLE file_transfers (
    id                 UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sender_device_id   UUID NOT NULL REFERENCES devices(id),
    receiver_device_id UUID NOT NULL REFERENCES devices(id),
    file_name          TEXT NOT NULL,
    file_size          BIGINT NOT NULL,
    mime_type          TEXT,
    status             transfer_status NOT NULL DEFAULT 'pending',
    bytes_transferred  BIGINT NOT NULL DEFAULT 0,
    storage_path       TEXT,
    transfer_method    TEXT NOT NULL DEFAULT 'p2p',
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at       TIMESTAMPTZ
);

CREATE TABLE notifications (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_device_id UUID NOT NULL REFERENCES devices(id),
    target_device_id UUID NOT NULL REFERENCES devices(id),
    app_name         TEXT NOT NULL,
    title            TEXT NOT NULL,
    body             TEXT,
    icon_url         TEXT,
    actions          JSONB NOT NULL DEFAULT '[]',
    status           notification_status NOT NULL DEFAULT 'pending',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE audit_logs (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    actor_id        UUID REFERENCES users(id) ON DELETE SET NULL,
    actor_device_id UUID REFERENCES devices(id) ON DELETE SET NULL,
    action          TEXT NOT NULL,
    resource_type   TEXT,
    resource_id     UUID,
    ip_address      INET,
    metadata        JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Immutable audit log (WORM)
CREATE RULE no_update_audit AS ON UPDATE TO audit_logs DO INSTEAD NOTHING;
CREATE RULE no_delete_audit AS ON DELETE TO audit_logs DO INSTEAD NOTHING;

CREATE INDEX idx_devices_user_id    ON devices(user_id);
CREATE INDEX idx_devices_online     ON devices(is_online) WHERE is_online = TRUE;
CREATE INDEX idx_sessions_devices   ON sessions(source_device_id, target_device_id);
CREATE INDEX idx_sessions_started   ON sessions(started_at DESC);
CREATE INDEX idx_clipboard_user     ON clipboard_history(user_id, created_at DESC);
CREATE INDEX idx_clipboard_expires  ON clipboard_history(expires_at);
CREATE INDEX idx_transfers_sender   ON file_transfers(sender_device_id, created_at DESC);
CREATE INDEX idx_notifications_tgt  ON notifications(target_device_id, status);
CREATE INDEX idx_audit_actor        ON audit_logs(actor_id, created_at DESC);
CREATE INDEX idx_audit_action       ON audit_logs(action, created_at DESC);
CREATE INDEX idx_audit_created      ON audit_logs(created_at DESC);

CREATE OR REPLACE FUNCTION cleanup_expired_clipboard() RETURNS void AS $$
BEGIN DELETE FROM clipboard_history WHERE expires_at < NOW(); END;
$$ LANGUAGE plpgsql;
