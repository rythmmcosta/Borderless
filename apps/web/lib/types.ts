// User — matches UserSummary from GET /v1/admin/users (paginated items)
export interface User {
  id: string
  email: string
  display_name: string | null
  role: string
  mfa_enabled?: boolean      // only present in UserDetail (GET /v1/admin/users/:id)
  created_at: string
  last_seen_at: string | null
  device_count?: number | null
}

// Device — matches DeviceSummary from GET /v1/admin/devices
export interface Device {
  id: string
  name: string
  platform: string
  os_version: string | null
  app_version: string | null
  is_online: boolean
  is_locked: boolean
  last_ip?: string | null
  last_seen_at: string | null
  user_email?: string
  user_name?: string | null
  // kept for compatibility with non-admin device list
  user_id?: string
  fingerprint?: string
}

// Session — matches SessionSummary from GET /v1/admin/sessions
export interface Session {
  id: string
  session_type: string
  transport: string
  started_at: string
  ended_at: string | null
  bytes_sent?: number
  bytes_recv?: number
  source_name?: string
  target_name?: string
  // kept for compatibility
  status?: string
  initiator_device_id?: string
  target_device_id?: string
}

// AuditLog — matches AuditEntry from GET /v1/admin/audit
export interface AuditLog {
  id: string
  actor_id?: string | null
  actor_email?: string | null
  action: string
  resource_type: string
  resource_id: string | null
  ip_address: string | null
  metadata?: Record<string, unknown>
  created_at: string
}
