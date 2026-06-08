import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { api } from '@/lib/api'
import type { Device, AdminStats } from '@/lib/types'

async function getData() {
  const [stats, devices] = await Promise.allSettled([
    api.admin.getStats(),
    api.admin.listDevices(),
  ])
  return {
    stats: stats.status === 'fulfilled' ? stats.value : null,
    devices: devices.status === 'fulfilled' ? devices.value : [] as Device[],
  }
}

export default async function DashboardPage() {
  const { stats, devices } = await getData()

  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Fleet Overview</h1>

      {stats && <StatsGrid stats={stats} />}

      <h2 className="text-lg font-semibold mb-4 mt-8">All Devices</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
        {(devices as Device[]).map((d) => (
          <Card key={d.id} className="p-4">
            <div className="flex items-start justify-between">
              <div>
                <p className="font-semibold">{d.name}</p>
                <p className="text-sm text-slate-400 mt-0.5">{d.user_email}</p>
              </div>
              <Badge variant={d.is_online ? 'online' : 'offline'}>
                {d.is_online ? 'Online' : 'Offline'}
              </Badge>
            </div>
            <div className="mt-3 flex gap-2 flex-wrap">
              <Badge variant="platform">{d.platform}</Badge>
              {d.app_version && <Badge variant="neutral">v{d.app_version}</Badge>}
            </div>
          </Card>
        ))}
        {(devices as Device[]).length === 0 && (
          <p className="text-slate-500 col-span-full text-center py-16">No devices registered yet.</p>
        )}
      </div>
    </div>
  )
}

function StatsGrid({ stats }: { stats: AdminStats }) {
  return (
    <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
      {[
        { label: 'Total Users',    value: stats.total_users,    color: 'text-white' },
        { label: 'Total Devices',  value: stats.total_devices,  color: 'text-white' },
        { label: 'Online Now',     value: stats.online_devices, color: 'text-green-400' },
        { label: 'Sessions Today', value: stats.sessions_today, color: 'text-indigo-400' },
      ].map(({ label, value, color }) => (
        <Card key={label} className="p-4">
          <p className="text-sm text-slate-400">{label}</p>
          <p className={`text-3xl font-bold mt-1 ${color}`}>{value}</p>
        </Card>
      ))}
    </div>
  )
}
