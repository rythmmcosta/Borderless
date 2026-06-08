export interface User {
  id: string
  email: string
  display_name: string | null
  role: string
  mfa_enabled: boolean
  created_at: string
  last_seen_at: string | null
}

export interface Device {
  id: string
  user_id: string
  name: string
  platform: string
  os_version: string | null
  app_version: string | null
  fingerprint: string
  is_online: boolean
  is_locked: boolean
  last_seen_at: string | null
}

export interface Session {
  id: string
  initiator_device_id: string
  target_device_id: string
  session_type: string
  transport: string
  status: string
  started_at: string
  ended_at: string | null
}

export interface AuditLog {
  id: string
  actor_id: string | null
  action: string
  resource_type: string
  resource_id: string | null
  ip_address: string | null
  created_at: string
}
