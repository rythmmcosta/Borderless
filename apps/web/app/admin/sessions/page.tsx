import { api } from '@/lib/api'
import { Table, THead, TBody, Th, Tr, Td } from '@/components/ui/table'
import { Badge } from '@/components/ui/badge'
import type { Session } from '@/lib/types'

export default async function AdminSessionsPage() {
  let sessions: Session[] = []
  try { sessions = await api.getAdminSessions() } catch {}

  return (
    <div>
      <h2 className="text-xl font-semibold mb-4">Active Sessions</h2>
      <Table>
        <THead>
          <Tr><Th>ID</Th><Th>Type</Th><Th>Transport</Th><Th>Status</Th><Th>Started</Th></Tr>
        </THead>
        <TBody>
          {sessions.map((s) => (
            <Tr key={s.id}>
              <Td className="font-mono text-xs">{s.id.slice(0, 8)}</Td>
              <Td>{s.session_type}</Td>
              <Td>{s.transport}</Td>
              <Td>
                <Badge variant={s.status === 'active' ? 'success' : 'default'}>{s.status}</Badge>
              </Td>
              <Td className="text-gray-400 text-sm">{new Date(s.started_at).toLocaleString()}</Td>
            </Tr>
          ))}
          {sessions.length === 0 && (
            <Tr><Td colSpan={5} className="text-center text-gray-500 py-8">No active sessions</Td></Tr>
          )}
        </TBody>
      </Table>
    </div>
  )
}
