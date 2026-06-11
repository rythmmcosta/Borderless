import { api } from '@/lib/api'
import { Table, THead, TBody, Th, Tr, Td } from '@/components/ui/table'
import { Badge } from '@/components/ui/badge'
import type { Device } from '@/lib/types'

export const metadata = { title: 'Devices' }

const platformEmoji: Record<string, string> = {
  windows: '🖥️', macos: '💻', linux: '🐧', android: '📱',
}

export default async function AdminDevicesPage() {
  let devices: Device[] = []
  try { devices = await api.getAdminDevices() } catch {}

  const online = devices.filter((d) => d.is_online).length
  const locked = devices.filter((d) => d.is_locked).length

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-white">Device Management</h2>
        <p className="mt-1 text-sm text-gray-500">{devices.length} devices · {online} online</p>
      </div>

      <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
        {[
          { label: 'Total',  value: devices.length, color: 'text-white' },
          { label: 'Online', value: online,          color: 'text-emerald-400' },
          { label: 'Locked', value: locked,          color: locked > 0 ? 'text-amber-400' : 'text-white' },
        ].map((s) => (
          <div key={s.label} className="rounded-xl border border-white/6 bg-surface-1 px-4 py-3">
            <p className="text-xs text-gray-500">{s.label}</p>
            <p className={`text-2xl font-bold ${s.color}`}>{s.value}</p>
          </div>
        ))}
      </div>

      <Table>
        <THead>
          <Tr><Th>Device</Th><Th>Platform</Th><Th>Owner</Th><Th>Status</Th><Th>Last Seen</Th><Th></Th></Tr>
        </THead>
        <TBody>
          {devices.map((d) => (
            <Tr key={d.id}>
              <Td>
                <div className="flex items-center gap-2.5">
                  <span className="text-base">{platformEmoji[d.platform] ?? '💻'}</span>
                  <span className="font-medium text-white">{d.name}</span>
                </div>
              </Td>
              <Td className="capitalize">{d.platform}</Td>
              <Td className="text-xs text-gray-500">{d.user_email ?? '—'}</Td>
              <Td>
                <div className="flex items-center gap-1.5">
                  <Badge variant={d.is_online ? 'success' : 'default'}>{d.is_online ? 'Online' : 'Offline'}</Badge>
                  {d.is_locked && <Badge variant="warning">Locked</Badge>}
                </div>
              </Td>
              <Td className="text-xs text-gray-500">{d.last_seen_at ? new Date(d.last_seen_at).toLocaleString() : '—'}</Td>
              <Td>
                <div className="flex items-center gap-1.5 justify-end">
                  <button className="rounded-lg border border-white/8 bg-white/4 px-3 py-2 text-xs text-gray-400 hover:text-white hover:bg-white/8 transition-colors min-h-[36px]">
                    {d.is_locked ? 'Unlock' : 'Lock'}
                  </button>
                  <button className="rounded-lg border border-red-500/20 bg-red-500/8 px-3 py-2 text-xs text-red-400 hover:bg-red-500/15 transition-colors min-h-[36px]">
                    Remove
                  </button>
                </div>
              </Td>
            </Tr>
          ))}
          {devices.length === 0 && <Tr><Td colSpan={6} className="py-16 text-center text-gray-600">No devices registered.</Td></Tr>}
        </TBody>
      </Table>
    </div>
  )
}
