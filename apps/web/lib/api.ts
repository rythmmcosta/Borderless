import type { User, Device, Session, AuditLog } from './types'

function getBase(): string {
  if (typeof window === 'undefined') {
    // Server-side (SSR / ISR) — connect directly to the Rust backend
    return process.env.INTERNAL_API_URL ?? 'http://localhost:8080'
  }
  // Client-side browser — goes through Nginx /api proxy
  return process.env.NEXT_PUBLIC_API_URL ?? '/api'
}

async function get<T>(path: string, opts?: RequestInit): Promise<T> {
  const base = getBase()
  const res = await fetch(`${base}${path}`, {
    next: { revalidate: 30 },
    headers: { 'Content-Type': 'application/json' },
    ...opts,
  })
  if (!res.ok) throw new Error(`${res.status} ${path}`)
  return res.json() as Promise<T>
}

export const api = {
  getDevices:       () => get<Device[]>('/devices'),
  getAdminDevices:  () => get<Device[]>('/admin/devices'),
  getAdminUsers:    () => get<User[]>('/admin/users'),
  getAdminSessions: () => get<Session[]>('/admin/sessions'),
  getAuditLogs:     () => get<AuditLog[]>('/admin/audit'),
}
