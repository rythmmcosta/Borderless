export interface Device {
  id: string
  user_id: string
  name: string
  platform: string
  os_version?: string
  app_version?: string
  fingerprint: string
  is_online: boolean
  is_locked: boolean
  last_seen_at?: string
  user_email?: string
  user_name?: string
}

export interface User {
  id: string
  email: string
  display_name?: string
  role: string
  mfa_enabled: boolean
  created_at: string
  last_seen_at?: string
  device_count?: number
}

export interface Session {
  id: string
  session_type: string
  transport: string
  started_at: string
  ended_at?: string
  bytes_sent: number
  bytes_recv: number
  source_name: string
  target_name: string
}

export interface AuditEntry {
  id: string
  action: string
  resource_type?: string
  resource_id?: string
  ip_address?: string
  metadata: Record<string, unknown>
  created_at: string
  actor_email?: string
}

export interface AdminStats {
  total_users: number
  total_devices: number
  online_devices: number
  sessions_today: number
}

export interface PaginatedResponse<T> {
  items: T[]
  total: number
  offset: number
  limit: number
}
