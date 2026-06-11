import { api } from '@/lib/api'
import { Table, THead, TBody, Th, Tr, Td } from '@/components/ui/table'
import type { AuditLog } from '@/lib/types'

export const metadata = { title: 'Audit Log' }

const actionColor = (action: string): string => {
  if (action.includes('delete') || action.includes('remove')) return 'text-red-400 bg-red-500/8 border-red-500/20'
  if (action.includes('create') || action.includes('register')) return 'text-emerald-400 bg-emerald-500/8 border-emerald-500/20'
  if (action.includes('login') || action.includes('auth')) return 'text-cyan-400 bg-cyan-500/8 border-cyan-500/20'
  if (action.includes('lock') || action.includes('suspend')) return 'text-amber-400 bg-amber-500/8 border-amber-500/20'
  return 'text-gray-400 bg-white/4 border-white/10'
}

export default async function AdminAuditPage() {
  let logs: AuditLog[] = []
  try { logs = await api.getAuditLogs() } catch {}

  const resourceTypes = [...new Set(logs.map((l) => l.resource_type))].slice(0, 5)

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-white">Audit Log</h2>
        <p className="mt-1 text-sm text-gray-500">{logs.length} entries · activity across all resources</p>
      </div>

      {resourceTypes.length > 0 && (
        <div className="flex flex-wrap gap-2">
          <span className="text-xs text-gray-600 self-center mr-1">Resources:</span>
          {resourceTypes.map((rt) => (
            <span key={rt} className="rounded-full border border-white/8 bg-white/4 px-2.5 py-0.5 text-xs text-gray-400 capitalize">
              {rt}
            </span>
          ))}
        </div>
      )}

      <Table>
        <THead>
          <Tr><Th>Time</Th><Th>Actor</Th><Th>Action</Th><Th>Resource</Th><Th>IP Address</Th></Tr>
        </THead>
        <TBody>
          {logs.map((l) => (
            <Tr key={l.id}>
              <Td className="whitespace-nowrap text-xs text-gray-500">
                {new Date(l.created_at).toLocaleString()}
              </Td>
              <Td>
                {l.actor_email
                  ? <span className="text-xs text-gray-400">{l.actor_email}</span>
                  : <span className="text-xs text-gray-600 italic">system</span>}
              </Td>
              <Td>
                <span className={`inline-flex items-center rounded-md border px-2 py-0.5 text-xs font-medium ${actionColor(l.action)}`}>
                  {l.action}
                </span>
              </Td>
              <Td>
                <span className="text-sm capitalize text-gray-400">{l.resource_type}</span>
                {l.resource_id && (
                  <span className="ml-1.5 font-mono text-xs text-gray-600">{l.resource_id.slice(0, 8)}…</span>
                )}
              </Td>
              <Td className="font-mono text-xs text-gray-500">{l.ip_address ?? '—'}</Td>
            </Tr>
          ))}
          {logs.length === 0 && (
            <Tr><Td colSpan={5} className="py-16 text-center text-gray-600">No audit entries yet.</Td></Tr>
          )}
        </TBody>
      </Table>
    </div>
  )
}
