'use client'
import Link from 'next/link'
import { usePathname } from 'next/navigation'

const NAV = [
  { href: '/dashboard',      label: 'Dashboard',  icon: '📊' },
  { href: '/admin/devices',  label: 'Devices',    icon: '🖥️' },
  { href: '/admin/users',    label: 'Users',      icon: '👥' },
  { href: '/admin/sessions', label: 'Sessions',   icon: '🔗' },
  { href: '/admin/audit',    label: 'Audit Log',  icon: '📋' },
]

export function Sidebar() {
  const pathname = usePathname()
  return (
    <aside className="w-56 bg-surface-raised border-r border-white/5 flex flex-col flex-shrink-0">
      <div className="px-5 py-5 border-b border-white/5">
        <span className="text-lg font-bold text-brand">Borderless</span>
        <p className="text-xs text-slate-500 mt-0.5">Admin Dashboard</p>
      </div>
      <nav className="flex-1 py-4 space-y-0.5 px-2">
        {NAV.map(({ href, label, icon }) => {
          const active = pathname === href || pathname.startsWith(href + '/')
          return (
            <Link
              key={href}
              href={href}
              className={`flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-colors ${
                active
                  ? 'bg-brand/20 text-brand font-medium'
                  : 'text-slate-400 hover:text-white hover:bg-white/5'
              }`}
            >
              <span className="text-base">{icon}</span>
              <span>{label}</span>
            </Link>
          )
        })}
      </nav>
      <div className="px-5 py-4 border-t border-white/5 text-xs text-slate-500">
        Borderless v0.1.0
      </div>
    </aside>
  )
}
