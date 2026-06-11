import { api } from '@/lib/api'
import { Badge } from '@/components/ui/badge'
import type { Device, Session } from '@/lib/types'

export const metadata = { title: 'Dashboard' }

function StatCard({ label, value, sub, color = 'text-white' }: {
  label: string; value: string | number; sub?: string; color?: string
}) {
  return (
    <div className="rounded-xl border border-white/6 bg-surface-1 p-5">
      <p className="text-xs font-medium uppercase tracking-wider text-gray-500">{label}</p>
      <p className={`mt-2 text-3xl font-bold ${color}`}>{value}</p>
      {sub && <p className="mt-1 text-xs text-gray-600">{sub}</p>}
    </div>
  )
}

const platformEmoji: Record<string, string> = {
  windows: '🖥️', macos: '💻', linux: '🐧', android: '📱',
}

export default async function DashboardPage() {
  let devices:  Device[]  = []
  let sessions: Session[] = []
  try { devices  = await api.getAdminDevices()  } catch {}
  try { sessions = await api.getAdminSessions() } catch {}

  const online  = devices.filter((d) => d.is_online).length
  const locked  = devices.filter((d) => d.is_locked).length
  const active  = sessions.filter((s) => !s.ended_at).length

  return (
    <div className="space-y-8">
      <div>
        <h2 className="text-2xl font-bold text-white">Fleet Overview</h2>
        <p className="mt-1 text-sm text-gray-500">Real-time status across all registered devices and sessions.</p>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-2 gap-4 lg:grid-cols-4">
        <StatCard label="Total Devices"   value={devices.length} sub="in fleet" />
        <StatCard label="Online"          value={online}  color="text-emerald-400" sub="reachable now" />
        <StatCard label="Active Sessions" value={active}  sub="KVM sessions" />
        <StatCard label="Locked"          value={locked}  color={locked > 0 ? 'text-amber-400' : 'text-white'} sub="access restricted" />
      </div>

      {/* Device grid */}
      <div>
        <h3 className="mb-4 text-xs font-semibold uppercase tracking-wider text-gray-500">Devices</h3>
        {devices.length === 0 ? (
          <div className="rounded-xl border border-dashed border-white/6 py-16 text-center text-sm text-gray-600">
            No devices registered yet.
          </div>
        ) : (
          <div className="grid gap-3 sm:grid-cols-2 xl:grid-cols-3">
            {devices.map((d) => (
              <div key={d.id} className="flex items-center gap-3 rounded-xl border border-white/6 bg-surface-1 p-4 transition-all hover:border-white/12">
                <div className={`flex h-10 w-10 shrink-0 items-center justify-center rounded-xl text-xl ${d.is_online ? 'bg-emerald-500/10' : 'bg-white/4'}`}>
                  {platformEmoji[d.platform] ?? '💻'}
                </div>
                <div className="min-w-0 flex-1">
                  <p className="truncate text-sm font-medium text-white">{d.name}</p>
                  <p className="text-xs capitalize text-gray-500">{d.platform}</p>
                </div>
                <div className="flex flex-col items-end gap-1 shrink-0">
                  <Badge variant={d.is_online ? 'success' : 'default'}>{d.is_online ? 'Online' : 'Offline'}</Badge>
                  {d.is_locked && <Badge variant="warning">Locked</Badge>}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Active sessions */}
      {sessions.length > 0 && (
        <div>
          <h3 className="mb-4 text-xs font-semibold uppercase tracking-wider text-gray-500">Live Sessions</h3>
          <div className="space-y-2">
            {sessions.filter((s) => !s.ended_at).slice(0, 5).map((s) => (
              <div key={s.id} className="flex items-center gap-4 rounded-xl border border-white/6 bg-surface-1 px-4 py-3">
                <div className="h-2 w-2 shrink-0 rounded-full bg-emerald-400 animate-pulse-slow" />
                <div className="flex-1 min-w-0">
                  <p className="text-sm text-white font-mono">{s.id.slice(0, 14)}…</p>
                  <p className="text-xs text-gray-500">{s.session_type} · {s.transport}</p>
                </div>
                <p className="text-xs text-gray-600 shrink-0">{new Date(s.started_at).toLocaleTimeString()}</p>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
