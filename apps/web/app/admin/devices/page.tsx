import { api } from '@/lib/api'
import { Table, THead, TBody, Th, Tr, Td } from '@/components/ui/table'
import { Badge } from '@/components/ui/badge'
import type { Device } from '@/lib/types'

export default async function AdminDevicesPage() {
  let devices: Device[] = []
  try { devices = await api.getAdminDevices() } catch {}

  return (
    <div>
      <h2 className="text-xl font-semibold mb-4">Device Management</h2>
      <Table>
        <THead>
          <Tr><Th>Name</Th><Th>Platform</Th><Th>Owner</Th><Th>Status</Th><Th>Last Seen</Th></Tr>
        </THead>
        <TBody>
          {devices.map((d) => (
            <Tr key={d.id}>
              <Td>{d.name}</Td>
              <Td>{d.platform}</Td>
              <Td className="font-mono text-xs">{d.user_id.slice(0, 8)}</Td>
              <Td>
                <Badge variant={d.is_online ? 'success' : 'default'}>
                  {d.is_online ? 'Online' : 'Offline'}
                </Badge>
                {d.is_locked && <Badge variant="warning" className="ml-1">Locked</Badge>}
              </Td>
              <Td className="text-gray-400 text-sm">
                {d.last_seen_at ? new Date(d.last_seen_at).toLocaleString() : '—'}
              </Td>
            </Tr>
          ))}
          {devices.length === 0 && (
            <Tr><Td colSpan={5} className="text-center text-gray-500 py-8">No devices</Td></Tr>
          )}
        </TBody>
      </Table>
    </div>
  )
}
