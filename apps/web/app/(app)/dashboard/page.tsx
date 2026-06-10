import { api } from '@/lib/api'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import type { Device } from '@/lib/types'

export const metadata = { title: 'Dashboard' }

export default async function DashboardPage() {
  let devices: Device[] = []
  try { devices = await api.getDevices() } catch {}

  const online = devices.filter((d) => d.is_online).length

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Fleet Overview</h1>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
        <Card>
          <div className="text-4xl font-bold text-primary">{devices.length}</div>
          <div className="text-sm text-gray-400 mt-1">Total Devices</div>
        </Card>
        <Card>
          <div className="text-4xl font-bold text-green-400">{online}</div>
          <div className="text-sm text-gray-400 mt-1">Online Now</div>
        </Card>
        <Card>
          <div className="text-4xl font-bold text-gray-400">{devices.length - online}</div>
          <div className="text-sm text-gray-400 mt-1">Offline</div>
        </Card>
      </div>
      <h2 className="text-lg font-semibold mb-3">Devices</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
        {devices.map((d) => (
          <Card key={d.id}>
            <div className="flex items-center justify-between">
              <span className="font-medium">{d.name}</span>
              <Badge variant={d.is_online ? 'success' : 'default'}>
                {d.is_online ? 'Online' : 'Offline'}
              </Badge>
            </div>
            <div className="text-sm text-gray-400 mt-1">{d.platform}</div>
          </Card>
        ))}
        {devices.length === 0 && (
          <div className="col-span-3 text-center text-gray-500 py-12">No devices registered yet</div>
        )}
      </div>
    </div>
  )
}
