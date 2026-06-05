# 08 — Security Model

## Philosophy

Borderless handles keyboard input, mouse movement, clipboard content (which may contain passwords), screen capture, and file transfers. **The security bar must be extremely high.**

Core principles:
1. **E2E by default** — server never sees plaintext device data
2. **Zero trust** — no implicit trust, everything authenticated
3. **Least privilege** — components only access what they need
4. **Transparency** — all admin actions visible in audit log, users notified of remote sessions
5. **Defense in depth** — multiple independent security layers

---

## Cryptography

### Key Exchange
```
Algorithm: X25519 (Elliptic Curve Diffie-Hellman)
Key size:  255 bits
Purpose:   Derive shared session key between two devices

Process:
  1. Device A generates StaticSecret_A, PublicKey_A = from(Secret_A)
  2. Device B generates StaticSecret_B, PublicKey_B = from(Secret_B)
  3. During handshake: A sends PublicKey_A (signed), B sends PublicKey_B (signed)
  4. A computes: SharedSecret = Secret_A.dh(PublicKey_B)
  5. B computes: SharedSecret = Secret_B.dh(PublicKey_A)
  6. Both get: SessionKey = HKDF-SHA256(SharedSecret, "borderless-v1-session-key")
  
Result: Server never knows SharedSecret or SessionKey
```

### Session Encryption
```
Algorithm: AES-256-GCM (Authenticated Encryption)
Key size:  256 bits (32 bytes from HKDF output)
Nonce:     96 bits (12 bytes), randomly generated per message
Auth tag:  128 bits (16 bytes)
Purpose:   Encrypt + authenticate every wire message

Wire format:
  [12-byte random nonce][ciphertext][16-byte auth tag]
  
Security properties:
  - Confidentiality: AES-256 (quantum-resistant to 2^128)
  - Integrity: GCM auth tag detects any modification
  - Authenticity: Combined with device signing below
```

### Device Identity Signing
```
Algorithm: Ed25519 (Edwards-curve Digital Signature Algorithm)
Key size:  256 bits
Purpose:   Prove "I am this device" during handshake

During handshake:
  1. Device A signs: Signature_A = Ed25519.sign(PublicKey_A_dh, SigningKey_A)
  2. Device B verifies: Ed25519.verify(PublicKey_A_dh, Signature_A, VerifyingKey_A)
  3. VerifyingKey_A retrieved from server (or cached from previous pair)
  
This prevents MITM: attacker can't forge a DH key with a valid signature
```

### Password Hashing
```
Algorithm: Argon2id
Parameters: m=65536 (64MB), t=3 iterations, p=4 threads
Purpose:    Hash user passwords before storage

Never store: plaintext passwords
Never log:   passwords, tokens, session keys
```

### JWT Tokens
```
Algorithm: HS256 (HMAC-SHA256)
Secret:    32+ random bytes from env var
Expiry:    15 minutes (access token)
Refresh:   30 days (single-use rotation)

Claims:
  sub:   user UUID
  email: user email (for quick display)
  role:  user | admin | super_admin
  iat:   issued at (seconds)
  exp:   expires at (seconds)

Device tokens:
  Format:  UUID v4 (opaque, 128-bit random)
  Storage: sha256(token) in database (never raw)
  Rotation: on device lock/unlock only
```

---

## Authentication & Authorization

### Authentication Flow
```
1. User submits email + password
2. Server: argon2id.verify(password, stored_hash)
3. Server issues: access_token (15min) + refresh_token (30 days)
4. Client stores: access_token in memory, refresh_token in secure storage
5. Client auto-refreshes when access_token expires (silent refresh)
6. On refresh: old refresh_token invalidated, new pair issued (rotation)
```

### Authorization Model
```
Roles:
  super_admin  → everything (including modifying other admins)
  admin        → view + control all users/devices in system
  user         → own devices only

Enforcement:
  1. JWT middleware: validates token, extracts role
  2. Admin middleware: checks role == admin || super_admin
  3. DB queries: WHERE user_id = $1 (prevents horizontal privilege escalation)
  4. Supabase RLS: secondary enforcement layer

Device auth:
  Devices use device_token (separate from user JWT)
  Device can only update its own record, post its own events
  Device cannot read other devices' data
```

---

## Network Security

### Transport Security
```
All API traffic:    HTTPS (TLS 1.3 minimum)
WebSocket:         WSS only
TURN relay:        TLS + DTLS
LAN connections:   TCP with application-layer E2E encryption
                   (no TLS needed — already AES-256-GCM)

TLS configuration (Nginx):
  ssl_protocols TLSv1.3;
  ssl_ciphers 'TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256';
  ssl_prefer_server_ciphers off;
  ssl_session_timeout 1d;
  ssl_session_cache shared:SSL:10m;
  add_header Strict-Transport-Security "max-age=31536000" always;
```

### Rate Limiting
```
Implementation: Redis sliding window
Limits per endpoint: see API docs
Response on limit:   429 Too Many Requests
Headers:             X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset
Ban policy:          Repeated limit violations → temporary IP ban (1hr)
```

### Firewall Rules (Production)
```
Inbound allowed:
  443/tcp  — HTTPS API + WSS
  80/tcp   — HTTP (redirect to HTTPS only)
  3478/udp — TURN/STUN
  3478/tcp — TURN/STUN
  5349/udp — TURN TLS
  5349/tcp — TURN TLS

Inbound blocked:
  Everything else (including database port 5432, redis 6379)

PostgreSQL + Redis: internal network only, no public access
```

---

## Data Security

### Data at Rest
```
Database encryption:
  PostgreSQL: Encrypted at OS level (LUKS / managed DB encryption)
  Sensitive fields encrypted at application level:
    - mfa_secret: AES-256-CBC with server key
    - clipboard content: E2E encrypted, server cannot decrypt

Key management:
  JWT_SECRET: environment variable, rotated periodically
  DB password: environment variable or secrets manager
  TURN secret: separate env var
  
Never in code:
  No secrets in source code
  No secrets in Docker images
  No secrets in logs
```

### Clipboard Security
```
Clipboard content is NEVER stored in plaintext on the server.
It is encrypted client-side before upload:

Client flow:
  1. Capture clipboard change
  2. session_key.encrypt(content) → encrypted_blob
  3. POST /clipboard { content_encrypted: base64(encrypted_blob) }
  4. Server stores encrypted_blob (cannot decrypt)
  5. Other devices fetch encrypted_blob
  6. session_key.decrypt(encrypted_blob) → plaintext (client side only)

Admin cannot read clipboard contents (no session key access)
```

---

## Admin Security

### Remote Session Transparency
```
When admin starts remote view/control:

1. Target device receives WebSocket push:
   { type: "admin_remote_start", admin_email: "...", session_type: "view_only" }

2. Target device UI shows permanent banner:
   ┌─────────────────────────────────────────────┐
   │ 👁 admin@example.com is viewing your screen │
   │ [End Remote Session]                         │
   └─────────────────────────────────────────────┘

3. Admin session recorded in audit_log immediately

4. User CAN dismiss view-only session

5. Remote control shows additional banner:
   ┌─────────────────────────────────────────────┐
   │ 🕹 admin@example.com is controlling your PC  │
   │ [Take Back Control]  [End Remote Session]    │
   └─────────────────────────────────────────────┘
```

### Audit Log Integrity
```sql
-- Audit log is protected at database level:
CREATE RULE no_update_audit AS ON UPDATE TO audit_logs DO INSTEAD NOTHING;
CREATE RULE no_delete_audit AS ON DELETE TO audit_logs DO INSTEAD NOTHING;

-- Even super_admin cannot delete audit entries via application
-- Only direct DB access (which requires server credentials) can remove entries
```

---

## Threat Model

| Threat | Impact | Likelihood | Mitigation |
|--------|--------|------------|------------|
| Network MITM (LAN) | Critical | Medium | E2E encryption — server sees ciphertext only |
| Network MITM (Internet) | Critical | Low | TLS + E2E encryption + certificate pinning |
| Compromised relay server | High | Low | E2E encryption — relay sees ciphertext only |
| Malicious device pairing | Critical | Medium | PIN + explicit confirmation + Ed25519 signature |
| JWT theft | High | Low | 15min expiry + rotation + device binding |
| Brute force login | High | Medium | Argon2id + rate limiting + account lockout |
| SQL injection | Critical | Low | SQLx parameterized queries (no string concat) |
| XSS in web dashboard | High | Medium | CSP headers + React auto-escaping |
| CSRF | Medium | Low | SameSite cookie flag + CORS policy |
| Privilege escalation | High | Low | Role checks + RLS policies (dual enforcement) |
| Malicious file transfer | Medium | Low | MIME validation + size limits + AV hook point |
| Clipboard exfiltration | Medium | Low | E2E encrypted + TTL + user controls |
| Admin abuse | Medium | Low | Audit log + user notification + consent prompts |
| Supply chain attack | High | Low | cargo-audit + Dependabot + signed releases |
| Credential stuffing | High | Medium | Rate limiting + 2FA support + breach detection |

---

## Security Checklist (Pre-Launch)

### Code
- [ ] `cargo audit` — no known vulnerabilities
- [ ] `npm audit` — no critical vulnerabilities  
- [ ] No secrets in git history
- [ ] No `unwrap()` in production paths — all errors handled
- [ ] SAST scan (CodeQL via GitHub Actions)

### Infrastructure
- [ ] All services behind firewall (no direct DB/Redis access)
- [ ] TLS 1.3 minimum, no TLS 1.0/1.1
- [ ] HSTS header set
- [ ] CSP header configured
- [ ] Secrets in environment variables (not hardcoded)
- [ ] Automatic security updates enabled
- [ ] Backups encrypted

### Application
- [ ] All inputs validated and sanitized
- [ ] File upload: MIME type validated, size limited
- [ ] Rate limiting active on all auth endpoints
- [ ] JWT expiry enforced (not just presence)
- [ ] Admin operations all audit-logged
- [ ] Remote sessions notify target device

### External
- [ ] External penetration test commissioned
- [ ] Responsible disclosure policy published
- [ ] CVE process defined
- [ ] Security contact in README

---

## Vulnerability Disclosure

Found a security vulnerability?

**Email:** security@borderless.app  

**Process:**
1. Report privately via email
2. We confirm receipt within 48 hours
3. We investigate and develop fix (typically 7-14 days)
4. We release patch and credit you (if desired)
5. CVE requested if applicable
