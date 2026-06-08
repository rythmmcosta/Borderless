import type { Device, User, Session, AuditEntry, AdminStats, PaginatedResponse } from './types'

const API_URL = process.env.API_URL ?? 'http://localhost:8080/v1'

async function fetchJson<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_URL}${path}`, {
    ...init,
    headers: { 'Content-Type': 'application/json', ...(init?.headers ?? {}) },
    cache: 'no-store',
  })
  if (!res.ok) throw new Error(`API ${path} returned ${res.status}`)
  return res.json() as Promise<T>
}

export const api = {
  admin: {
    getStats:    () => fetchJson<AdminStats>('/admin/stats'),
    listDevices: () => fetchJson<Device[]>('/admin/devices'),
    listUsers:   () => fetchJson<PaginatedResponse<User>>('/admin/users'),
    listSessions:() => fetchJson<Session[]>('/admin/sessions'),
    getAuditLog: () => fetchJson<AuditEntry[]>('/admin/audit'),
    lockDevice:   (id: string) => fetchJson(`/admin/devices/${id}/lock`,   { method: 'POST', body: '{}' }),
    unlockDevice: (id: string) => fetchJson(`/admin/devices/${id}/unlock`, { method: 'POST' }),
    terminateSession: (id: string) => fetchJson(`/admin/sessions/${id}`,   { method: 'POST' }),
  },
  auth: {
    me:       () => fetchJson('/auth/me'),
    login:    (email: string, password: string) =>
      fetchJson('/auth/login',    { method: 'POST', body: JSON.stringify({ email, password }) }),
    register: (email: string, password: string) =>
      fetchJson('/auth/register', { method: 'POST', body: JSON.stringify({ email, password }) }),
  },
  devices: {
    list: () => fetchJson<Device[]>('/devices'),
  },
}
