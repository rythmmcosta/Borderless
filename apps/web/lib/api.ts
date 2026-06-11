import { cookies } from 'next/headers'
import type { User, Device, Session, AuditLog } from './types'

function getBase(): string {
  const root = typeof window === 'undefined'
    ? (process.env.INTERNAL_API_URL ?? 'http://localhost:8080')
    : (process.env.NEXT_PUBLIC_API_URL ?? '/api')
  return `${root}/v1`
}

async function get<T>(path: string, opts?: RequestInit): Promise<T> {
  const base = getBase()
  const headers: Record<string, string> = { 'Content-Type': 'application/json' }

  if (typeof window === 'undefined') {
    const token = (await cookies()).get('borderless_token')?.value
    if (token) headers.Authorization = `Bearer ${token}`
  }

  const res = await fetch(`${base}${path}`, {
    next: { revalidate: 30 },
    headers,
    ...opts,
  })
  if (res.status === 401) throw new Error('Unauthorized')
  if (!res.ok) throw new Error(`${res.status} ${path}`)
  return res.json() as Promise<T>
}

export const api = {
  getDevices:       () => get<Device[]>('/devices'),
  getAdminDevices:  () => get<Device[]>('/admin/devices'),
  getAdminUsers:    () => get<{ items: User[] }>('/admin/users').then((r) => r.items),
  getAdminSessions: () => get<Session[]>('/admin/sessions'),
  getAuditLogs:     () => get<AuditLog[]>('/admin/audit'),
}
