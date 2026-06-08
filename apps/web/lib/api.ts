import type { User, Device, Session, AuditLog } from './types'

const BASE = process.env.NEXT_PUBLIC_API_URL ?? 'http://localhost:8080/v1'

async function get<T>(path: string): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    next: { revalidate: 30 },
    headers: { 'Content-Type': 'application/json' },
  })
  if (!res.ok) throw new Error(`${res.status} ${path}`)
  return res.json() as Promise<T>
}

export const api = {
  getDevices:        () => get<Device[]>('/devices'),
  getAdminDevices:   () => get<Device[]>('/admin/devices'),
  getAdminUsers:     () => get<User[]>('/admin/users'),
  getAdminSessions:  () => get<Session[]>('/admin/sessions'),
  getAuditLogs:      () => get<AuditLog[]>('/admin/audit'),
}
