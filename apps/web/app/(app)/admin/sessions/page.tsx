import { api } from '@/lib/api'
import { Table, THead, TBody, Th, Tr, Td } from '@/components/ui/table'
import { Badge } from '@/components/ui/badge'
import type { Session } from '@/lib/types'

export const metadata = { title: 'Sessions' }

const transportColor: Record<string, string> = {
  webrtc: 'text-cyan-400',
  relay:  'text-amber-400',
  direct: 'text-emerald-400',
}

export default async function AdminSessionsPage() {
  let sessions: Session[] = []
  try { sessions = await api.getAdminSessions() } catch {}

  // Derive status from ended_at — backend SessionSummary doesn't include a status column
  const active    = sessions.filter((s) => !s.ended_at).length
  const webrtc    = sessions.filter((s) => s.transport === 'webrtc').length
  const relayed   = sessions.filter((s) => s.transport === 'relay').length

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-white">Sessions</h2>
        <p className="mt-1 text-sm text-gray-500">{sessions.length} total · {active} active</p>
      </div>

      <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
        {[
          { label: 'Active',   value: active,   color: active > 0 ? 'text-emerald-400' : 'text-white' },
          { label: 'WebRTC',   value: webrtc,   color: 'text-cyan-400' },
          { label: 'Relayed',  value: relayed,  color: relayed > 0 ? 'text-amber-400' : 'text-white' },
        ].map((s) => (
          <div key={s.label} className="rounded-xl border border-white/6 bg-surface-1 px-4 py-3">
            <p className="text-xs text-gray-500">{s.label}</p>
            <p className={`text-2xl font-bold ${s.color}`}>{s.value}</p>
          </div>
        ))}
      </div>

      <Table>
        <THead>
          <Tr><Th>Session ID</Th><Th>Type</Th><Th>Transport</Th><Th>Status</Th><Th>Started</Th><Th></Th></Tr>
        </THead>
        <TBody>
          {sessions.map((s) => {
            const isActive = !s.ended_at
            return (
              <Tr key={s.id}>
                <Td>
                  <div className="flex items-center gap-2">
                    {isActive && (
                      <span className="h-1.5 w-1.5 shrink-0 rounded-full bg-emerald-400 animate-pulse" />
                    )}
                    <span className="font-mono text-xs text-gray-300">{s.id.slice(0, 14)}…</span>
                  </div>
                </Td>
                <Td className="text-sm capitalize text-gray-400">
                  {s.session_type}
                  {(s.source_name || s.target_name) && (
                    <p className="text-xs text-gray-600 mt-0.5">{s.source_name} → {s.target_name}</p>
                  )}
                </Td>
                <Td>
                  <span className={`text-xs font-medium ${transportColor[s.transport] ?? 'text-gray-400'}`}>
                    {s.transport}
                  </span>
                </Td>
                <Td>
                  <Badge variant={isActive ? 'success' : 'default'}>{isActive ? 'active' : 'ended'}</Badge>
                </Td>
                <Td className="text-xs text-gray-500">{new Date(s.started_at).toLocaleString()}</Td>
                <Td>
                  {isActive && (
                    <button className="rounded-lg border border-red-500/20 bg-red-500/8 px-3 py-2 text-xs text-red-400 hover:bg-red-500/15 transition-colors min-h-[36px]">
                      Terminate
                    </button>
                  )}
                </Td>
              </Tr>
            )
          })}
          {sessions.length === 0 && (
            <Tr><Td colSpan={6} className="py-16 text-center text-gray-600">No sessions recorded.</Td></Tr>
          )}
        </TBody>
      </Table>
    </div>
  )
}
