import { Badge } from '@/components/ui/badge'
import { api } from '@/lib/api'
import type { User, PaginatedResponse } from '@/lib/types'

export default async function AdminUsersPage() {
  let data: PaginatedResponse<User> = { items: [], total: 0, offset: 0, limit: 50 }
  try { data = await api.admin.listUsers() } catch { /* show empty */ }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">User Management</h1>
        <Badge variant="neutral">{data.total} users</Badge>
      </div>

      <div className="overflow-x-auto rounded-xl border border-white/5">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-white/5 bg-surface-raised">
              {['Email', 'Display Name', 'Role', 'Devices', 'Created', 'Last Seen'].map((h) => (
                <th key={h} className="text-left px-4 py-3 text-slate-400 font-medium">{h}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {data.items.map((u) => (
              <tr key={u.id} className="border-b border-white/5 hover:bg-white/[0.02] transition-colors">
                <td className="px-4 py-3 font-medium">{u.email}</td>
                <td className="px-4 py-3 text-slate-300">{u.display_name ?? '—'}</td>
                <td className="px-4 py-3">
                  <Badge variant={u.role === 'admin' || u.role === 'super_admin' ? 'admin' : 'neutral'}>
                    {u.role}
                  </Badge>
                </td>
                <td className="px-4 py-3 text-center text-slate-300">{u.device_count ?? 0}</td>
                <td className="px-4 py-3 text-slate-400 text-xs">
                  {new Date(u.created_at).toLocaleDateString()}
                </td>
                <td className="px-4 py-3 text-slate-400 text-xs">
                  {u.last_seen_at ? new Date(u.last_seen_at).toLocaleString() : 'Never'}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {data.items.length === 0 && (
          <div className="text-center py-16 text-slate-500">No users registered.</div>
        )}
      </div>
    </div>
  )
}
