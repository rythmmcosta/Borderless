import { api } from '@/lib/api'
import type { AuditEntry } from '@/lib/types'

export default async function AdminAuditPage() {
  let entries: AuditEntry[] = []
  try { entries = await api.admin.getAuditLog() } catch { /* show empty */ }

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Audit Log</h1>
      <div className="overflow-x-auto rounded-xl border border-white/5">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-white/5 bg-surface-raised">
              {['Time', 'Actor', 'Action', 'Resource', 'IP'].map((h) => (
                <th key={h} className="text-left px-4 py-3 text-slate-400 font-medium">{h}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {entries.map((e) => (
              <tr key={e.id} className="border-b border-white/5 hover:bg-white/[0.02] transition-colors">
                <td className="px-4 py-3 text-xs text-slate-400 whitespace-nowrap">
                  {new Date(e.created_at).toLocaleString()}
                </td>
                <td className="px-4 py-3 text-slate-300">{e.actor_email ?? 'System'}</td>
                <td className="px-4 py-3 font-mono text-xs text-indigo-300">{e.action}</td>
                <td className="px-4 py-3 text-xs text-slate-400">
                  {e.resource_type
                    ? `${e.resource_type} / ${e.resource_id?.slice(0, 8)}…`
                    : '—'}
                </td>
                <td className="px-4 py-3 text-xs text-slate-500">{e.ip_address ?? '—'}</td>
              </tr>
            ))}
          </tbody>
        </table>
        {entries.length === 0 && (
          <div className="text-center py-16 text-slate-500">No audit entries found.</div>
        )}
      </div>
    </div>
  )
}
