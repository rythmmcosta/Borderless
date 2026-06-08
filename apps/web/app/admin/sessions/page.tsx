import { Badge } from '@/components/ui/badge'
import { api } from '@/lib/api'
import type { Session } from '@/lib/types'

export default async function AdminSessionsPage() {
  let sessions: Session[] = []
  try { sessions = await api.admin.listSessions() } catch { /* show empty */ }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Sessions</h1>
        <Badge variant="neutral">{sessions.length}</Badge>
      </div>

      <div className="overflow-x-auto rounded-xl border border-white/5">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-white/5 bg-surface-raised">
              {['ID', 'Type', 'Transport', 'Source', 'Target', 'Started', 'Status'].map((h) => (
                <th key={h} className="text-left px-4 py-3 text-slate-400 font-medium">{h}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {sessions.map((s) => (
              <tr key={s.id} className="border-b border-white/5 hover:bg-white/[0.02] transition-colors">
                <td className="px-4 py-3 text-xs text-slate-500 font-mono">{s.id.slice(0, 8)}…</td>
                <td className="px-4 py-3"><Badge variant="neutral">{s.session_type}</Badge></td>
                <td className="px-4 py-3"><Badge variant="platform">{s.transport}</Badge></td>
                <td className="px-4 py-3 text-slate-300">{s.source_name}</td>
                <td className="px-4 py-3 text-slate-300">{s.target_name}</td>
                <td className="px-4 py-3 text-slate-400 text-xs">
                  {new Date(s.started_at).toLocaleString()}
                </td>
                <td className="px-4 py-3">
                  <Badge variant={s.ended_at ? 'offline' : 'online'}>
                    {s.ended_at ? 'Ended' : 'Active'}
                  </Badge>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {sessions.length === 0 && (
          <div className="text-center py-16 text-slate-500">No sessions found.</div>
        )}
      </div>
    </div>
  )
}
