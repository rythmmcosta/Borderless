import { api } from '@/lib/api'
import { Table, THead, TBody, Th, Tr, Td } from '@/components/ui/table'
import { Badge } from '@/components/ui/badge'
import type { User } from '@/lib/types'

export const metadata = { title: 'Users' }

export default async function AdminUsersPage() {
  let users: User[] = []
  try { users = await api.getAdminUsers() } catch {}

  const admins = users.filter((u) => u.role === 'admin').length
  const mfaEnabled = users.filter((u) => u.mfa_enabled).length

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-white">User Management</h2>
        <p className="mt-1 text-sm text-gray-500">{users.length} registered accounts</p>
      </div>

      <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
        {[
          { label: 'Total',   value: users.length, color: 'text-white' },
          { label: 'Admins',  value: admins,        color: admins > 0 ? 'text-amber-400' : 'text-white' },
          { label: 'MFA On',  value: mfaEnabled,    color: 'text-emerald-400' },
        ].map((s) => (
          <div key={s.label} className="rounded-xl border border-white/6 bg-surface-1 px-4 py-3">
            <p className="text-xs text-gray-500">{s.label}</p>
            <p className={`text-2xl font-bold ${s.color}`}>{s.value}</p>
          </div>
        ))}
      </div>

      <Table>
        <THead>
          <Tr><Th>User</Th><Th>Display Name</Th><Th>Role</Th><Th>MFA</Th><Th>Joined</Th><Th></Th></Tr>
        </THead>
        <TBody>
          {users.map((u) => (
            <Tr key={u.id}>
              <Td>
                <div className="flex items-center gap-2.5">
                  <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-primary/15 text-xs font-bold text-primary-light">
                    {(u.display_name ?? u.email).charAt(0).toUpperCase()}
                  </div>
                  <span className="text-sm text-white">{u.email}</span>
                </div>
              </Td>
              <Td className="text-gray-400">{u.display_name ?? '—'}</Td>
              <Td>
                <Badge variant={u.role === 'admin' ? 'warning' : 'default'}>{u.role}</Badge>
              </Td>
              <Td>
                {u.mfa_enabled
                  ? <span className="inline-flex items-center gap-1 text-xs text-emerald-400"><span className="h-1.5 w-1.5 rounded-full bg-emerald-400" />Enabled</span>
                  : <span className="text-xs text-gray-600">—</span>}
              </Td>
              <Td className="text-xs text-gray-500">{new Date(u.created_at).toLocaleDateString()}</Td>
              <Td>
                <div className="flex items-center gap-1.5 justify-end">
                  <button className="rounded-lg border border-white/8 bg-white/4 px-3 py-2 text-xs text-gray-400 hover:text-white hover:bg-white/8 transition-colors min-h-[36px]">
                    Edit
                  </button>
                  <button className="rounded-lg border border-red-500/20 bg-red-500/8 px-3 py-2 text-xs text-red-400 hover:bg-red-500/15 transition-colors min-h-[36px]">
                    Suspend
                  </button>
                </div>
              </Td>
            </Tr>
          ))}
          {users.length === 0 && (
            <Tr><Td colSpan={6} className="py-16 text-center text-gray-600">No users registered.</Td></Tr>
          )}
        </TBody>
      </Table>
    </div>
  )
}
