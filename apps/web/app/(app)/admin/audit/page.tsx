import { api } from '@/lib/api'
import { Table, THead, TBody, Th, Tr, Td } from '@/components/ui/table'
import type { AuditLog } from '@/lib/types'

export const metadata = { title: 'Audit Log' }

export default async function AdminAuditPage() {
  let logs: AuditLog[] = []
  try { logs = await api.getAuditLogs() } catch {}

  return (
    <div>
      <h2 className="text-xl font-semibold mb-4">Audit Log</h2>
      <Table>
        <THead><Tr><Th>Time</Th><Th>Actor</Th><Th>Action</Th><Th>Resource</Th><Th>IP</Th></Tr></THead>
        <TBody>
          {logs.map((l) => (
            <Tr key={l.id}>
              <Td className="text-xs text-gray-400 whitespace-nowrap">{new Date(l.created_at).toLocaleString()}</Td>
              <Td className="font-mono text-xs">{l.actor_id?.slice(0, 8) ?? 'system'}</Td>
              <Td>{l.action}</Td>
              <Td className="text-sm">{l.resource_type}{l.resource_id ? ` · ${l.resource_id.slice(0, 8)}` : ''}</Td>
              <Td className="text-xs text-gray-400">{l.ip_address ?? '—'}</Td>
            </Tr>
          ))}
          {logs.length === 0 && <Tr><Td colSpan={5} className="text-center text-gray-500 py-8">No audit entries</Td></Tr>}
        </TBody>
      </Table>
    </div>
  )
}
