# 07 — API Reference

## Overview

- **Base URL:** `https://api.borderless.app/v1`
- **Auth:** `Authorization: Bearer <jwt>` on all authenticated endpoints
- **Device Auth:** `X-Device-Token: <device_token>` for device-only endpoints
- **Content-Type:** `application/json`
- **Errors:** `{ "error": "description" }` with appropriate HTTP status

### HTTP Status Codes
| Code | Meaning |
|------|--------|
| 200 | OK |
| 201 | Created |
| 204 | No Content |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 409 | Conflict |
| 429 | Too Many Requests |
| 500 | Internal Server Error |

---

## Authentication

### POST /auth/register
```json
{ "email": "user@example.com", "password": "SecurePassword123!", "display_name": "John Doe" }
```
**Response 201:**
```json
{
  "user": { "id": "uuid", "email": "user@example.com", "display_name": "John Doe" },
  "access_token": "eyJ...",
  "refresh_token": "uuid-opaque-token",
  "expires_in": 900
}
```

### POST /auth/login
```json
{ "email": "user@example.com", "password": "SecurePassword123!" }
```
**Response 200:** access_token + refresh_token + expires_in + user

### POST /auth/refresh
```json
{ "refresh_token": "uuid-opaque-token" }
```
Response 200: New token pair. Old refresh_token invalidated.

### POST /auth/logout
**Auth:** JWT — Response 204. All refresh tokens revoked.

### POST /auth/mfa/enable
**Auth:** JWT
**Response 200:**
```json
{
  "totp_uri": "otpauth://totp/Borderless:user@example.com?secret=BASE32SECRET&issuer=Borderless",
  "backup_codes": ["aaaaa-bbbbb", "ccccc-ddddd"]
}
```

### POST /auth/mfa/verify
```json
{ "code": "123456" }
```

---

## Users

### GET /users/me
**Auth:** JWT
```json
{
  "id": "uuid", "email": "user@example.com", "display_name": "John Doe",
  "role": "user", "mfa_enabled": false,
  "created_at": "2024-01-15T09:00:00Z", "last_seen_at": "2024-06-01T14:32:00Z"
}
```

### PUT /users/me
**Auth:** JWT — Update display_name, avatar_url.

---

## Devices

### GET /devices
**Auth:** JWT
```json
{
  "devices": [
    {
      "id": "uuid", "name": "My Windows PC", "platform": "windows",
      "os_version": "Windows 11 22H2", "app_version": "0.1.0",
      "is_online": true, "last_seen_at": "2024-06-01T14:32:00Z",
      "metadata": { "screen_width": 1920, "screen_height": 1080 }
    }
  ]
}
```

### POST /devices/register
**Auth:** JWT
```json
{
  "name": "My Windows PC", "fingerprint": "sha256hex...",
  "platform": "windows", "os_version": "Windows 11 22H2",
  "dh_pubkey_hex": "x25519-key-hex", "sign_pubkey_hex": "ed25519-key-hex",
  "metadata": { "screen_width": 1920, "screen_height": 1080 }
}
```
**Response 201:** `{ "device": {...}, "device_token": "opaque-token" }`

### DELETE /devices/:id
**Auth:** JWT — Response 204.

### POST /devices/pair
**Auth:** JWT + X-Device-Token
```json
{ "target_device_id": "uuid", "pin": "4823" }
```
**Response 200:** `{ "pair_id": "uuid", "status": "pending_confirmation" }`

### PUT /devices/pair/:id/confirm
**Auth:** JWT + X-Device-Token
```json
{ "accepted": true }
```

### PUT /devices/layout
**Auth:** JWT — Save device layout configuration.
```json
{
  "layout": {
    "connections": { "device_a_id": { "right": "device_b_id" } },
    "sizes": { "device_a_id": [1920, 1080], "device_b_id": [2560, 1600] }
  }
}
```

### POST /devices/:id/heartbeat
**Auth:** X-Device-Token — Call every 30 seconds.
```json
{ "ip": "192.168.1.10" }
```

---

## Sessions

### POST /sessions
**Auth:** JWT + X-Device-Token
```json
{ "target_device_id": "uuid", "session_type": "kvm", "transport": "lan" }
```
**Response 201:** `{ "session_id": "uuid" }`

### PUT /sessions/:id/end
**Auth:** X-Device-Token
```json
{ "bytes_sent": 1048576, "bytes_recv": 524288, "events_count": 15234 }
```

### GET /sessions
**Auth:** JWT — Lists sessions. Pagination: `?limit=50&offset=0`

---

## Clipboard

### GET /clipboard
**Auth:** JWT — `?limit=50&offset=0&type=text/plain`

### GET /clipboard/:id/content
**Auth:** JWT — Returns encrypted blob (`application/octet-stream`). Client decrypts locally.

### POST /clipboard
**Auth:** X-Device-Token
```json
{
  "content_type": "text/plain", "content_hash": "sha256hex",
  "content_encrypted": "base64-encrypted-bytes", "content_size": 48
}
```

### DELETE /clipboard/:id
**Auth:** JWT — Delete a specific item.

### DELETE /clipboard
**Auth:** JWT — Clear all clipboard history.

---

## File Transfers

### POST /transfers/init
**Auth:** X-Device-Token
```json
{
  "receiver_device_id": "uuid", "file_name": "report.pdf",
  "file_size": 2097152, "mime_type": "application/pdf",
  "sha256_hex": "abcdef...", "total_chunks": 32
}
```
**Response 201:** `{ "transfer_id": "uuid", "method": "p2p", "upload_url": null }`

### PUT /transfers/:id/status
**Auth:** X-Device-Token
```json
{ "status": "completed", "bytes_transferred": 2097152 }
```

### GET /transfers
**Auth:** JWT — List recent transfers.

---

## Notifications

### POST /notifications
**Auth:** X-Device-Token
```json
{
  "target_device_id": "uuid", "app_name": "Messages",
  "title": "New message from Alice", "body": "Hey, are you there?",
  "actions": [
    { "id": "reply", "label": "Reply", "type": "text_input" },
    { "id": "dismiss", "label": "Dismiss", "type": "button" }
  ]
}
```

### POST /notifications/:id/action
**Auth:** X-Device-Token
```json
{ "action_id": "reply", "text": "Yes, one moment!" }
```

---

## Admin API

All admin endpoints require `role: admin` or `role: super_admin`.

### GET /admin/stats
```json
{
  "total_users": 47, "total_devices": 89, "online_devices": 23,
  "sessions_today": 142, "data_transferred_gb": 12.4, "active_sessions": 8
}
```

### GET /admin/users
`?limit=50&offset=0&role=user&search=john`

### PUT /admin/users/:id/role
```json
{ "role": "admin" }
```

### PUT /admin/users/:id/disable
Disable user account (cannot login, devices disconnected).

### GET /admin/devices
`?online_only=true&platform=windows&limit=50`

### POST /admin/devices/:id/lock
```json
{ "reason": "Lost device" }
```

### POST /admin/devices/:id/unlock
Re-issue device token.

### POST /admin/devices/:id/remote
```json
{ "session_type": "view_only" }
```
**Response 200:**
```json
{ "session_id": "uuid", "webrtc_offer": "sdp...", "ice_servers": [...] }
```
The target device receives: "Admin is viewing your screen" with dismiss option.

### GET /admin/sessions
`?limit=100&device_id=uuid&from=2024-01-01&to=2024-06-01`

### DELETE /admin/sessions/:id
Force-terminate an active session.

### GET /admin/audit
`?limit=100&offset=0&action=device.lock&actor_id=uuid&from=2024-01-01`
**Response 200:**
```json
{
  "entries": [
    {
      "id": "uuid", "action": "admin.device.lock",
      "actor_email": "admin@example.com", "resource_type": "device",
      "resource_id": "uuid", "ip_address": "192.168.1.1",
      "metadata": { "reason": "Lost device" }, "created_at": "2024-06-01T14:32:00Z"
    }
  ],
  "total": 1024
}
```
Supports CSV export: `?format=csv`

---

## WebSocket API

### WS /ws/signal
WebRTC signaling channel.

**Auth message (first):**
```json
{ "type": "auth", "device_token": "opaque-token" }
```

**SDP Offer:**
```json
{ "type": "offer", "to": "target-device-uuid", "sdp": "v=0\r\no=- ...", "session_id": "uuid" }
```

**ICE candidates:**
```json
{ "type": "ice", "to": "target-device-uuid", "candidate": "candidate:...", "sdpMid": "0" }
```

**Presence events (server → client):**
```json
{ "type": "peer_online",  "device_id": "uuid" }
{ "type": "peer_offline", "device_id": "uuid" }
```

### WS /ws/events
Real-time event stream for web dashboard. **Auth:** `Authorization: Bearer <jwt>`

**Events:**
```json
{ "type": "device.online",  "device_id": "uuid", "ip": "..." }
{ "type": "device.offline", "device_id": "uuid" }
{ "type": "session.start",  "session_id": "uuid", "devices": ["a", "b"] }
{ "type": "session.end",    "session_id": "uuid" }
{ "type": "transfer.complete", "transfer_id": "uuid", "file_name": "..." }
```

### WS /ws/admin
Admin live dashboard stream. Requires `role: admin`.

---

## Rate Limits

| Endpoint | Limit |
|----------|-------|
| POST /auth/login | 10 req/min per IP |
| POST /auth/register | 5 req/min per IP |
| POST /devices/heartbeat | 4 req/min per device |
| POST /clipboard | 60 req/min per device |
| POST /notifications | 30 req/min per device |
| GET /admin/* | 100 req/min per user |
| All other | 200 req/min per user |

Rate limit headers on all responses:
```
X-RateLimit-Limit: 200
X-RateLimit-Remaining: 187
X-RateLimit-Reset: 1717250400
```
