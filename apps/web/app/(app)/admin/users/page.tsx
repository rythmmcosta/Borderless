import { api } from '@/lib/api'
import { Table, THead, TBody, Th, Tr, Td } from '@/components/ui/table'
import { Badge } from '@/components/ui/badge'
import type { User } from '@/lib/types'

export const metadata = { title: 'Users' }

export default async function AdminUsersPage() {
  let users: User[] = []
  try { users = await api.getAdminUsers() } catch {}

  return (
    <div>
      <h2 className="text-xl font-semibold mb-4">User Management</h2>
      <Table>
        <THead><Tr><Th>Email</Th><Th>Display Name</Th><Th>Role</Th><Th>MFA</Th><Th>Joined</Th></Tr></THead>
        <TBody>
          {users.map((u) => (
            <Tr key={u.id}>
              <Td>{u.email}</Td>
              <Td>{u.display_name ?? '—'}</Td>
              <Td><Badge variant={u.role === 'admin' ? 'warning' : 'default'}>{u.role}</Badge></Td>
              <Td>{u.mfa_enabled ? '✓' : '—'}</Td>
              <Td className="text-gray-400 text-sm">{new Date(u.created_at).toLocaleDateString()}</Td>
            </Tr>
          ))}
          {users.length === 0 && <Tr><Td colSpan={5} className="text-center text-gray-500 py-8">No users</Td></Tr>}
        </TBody>
      </Table>
    </div>
  )
}
