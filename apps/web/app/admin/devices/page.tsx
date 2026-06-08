import { Badge } from '@/components/ui/badge'
import { api } from '@/lib/api'
import type { Device } from '@/lib/types'

export default async function AdminDevicesPage() {
  let devices: Device[] = []
  try { devices = await api.admin.listDevices() } catch { /* show empty */ }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Device Management</h1>
        <Badge variant="neutral">{devices.length} devices</Badge>
      </div>

      <div className="overflow-x-auto rounded-xl border border-white/5">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-white/5 bg-surface-raised">
              {['Device', 'User', 'Platform', 'Status', 'Last Seen'].map((h) => (
                <th key={h} className="text-left px-4 py-3 text-slate-400 font-medium">{h}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {devices.map((d) => (
              <tr key={d.id} className="border-b border-white/5 hover:bg-white/[0.02] transition-colors">
                <td className="px-4 py-3">
                  <p className="font-medium">{d.name}</p>
                  <p className="text-xs text-slate-500 font-mono">{d.id.slice(0, 8)}…</p>
                </td>
                <td className="px-4 py-3 text-slate-300">{d.user_email}</td>
                <td className="px-4 py-3"><Badge variant="platform">{d.platform}</Badge></td>
                <td className="px-4 py-3">
                  <Badge variant={d.is_online ? 'online' : 'offline'}>
                    {d.is_online ? 'Online' : 'Offline'}
                  </Badge>
                </td>
                <td className="px-4 py-3 text-slate-400 text-xs">
                  {d.last_seen_at ? new Date(d.last_seen_at).toLocaleString() : '—'}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {devices.length === 0 && (
          <div className="text-center py-16 text-slate-500">No devices registered.</div>
        )}
      </div>
    </div>
  )
}
