'use client'
import Link from 'next/link'
import { usePathname } from 'next/navigation'

const nav = [
  { href: '/dashboard',       label: 'Dashboard',  icon: '◈' },
  { href: '/admin/devices',   label: 'Devices',    icon: '⊞' },
  { href: '/admin/users',     label: 'Users',      icon: '○' },
  { href: '/admin/sessions',  label: 'Sessions',   icon: '⚡' },
  { href: '/admin/audit',     label: 'Audit Log',  icon: '⊟' },
]

export function Sidebar() {
  const path = usePathname()
  return (
    <aside className="w-56 shrink-0 border-r border-white/5 flex flex-col" style={{backgroundColor:'#1a1a1a'}}>
      <div className="p-4 border-b border-white/5">
        <span className="font-bold text-lg tracking-tight">Borderless</span>
      </div>
      <nav className="flex-1 p-2 space-y-0.5">
        {nav.map((item) => {
          const active = path.startsWith(item.href)
          return (
            <Link
              key={item.href}
              href={item.href}
              className={[
                'flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-colors',
                active
                  ? 'text-white font-medium'
                  : 'text-gray-400 hover:text-white',
              ].join(' ')}
              style={active ? {backgroundColor:'rgba(103,80,164,0.2)',color:'#9d84d6'} : {}}
            >
              <span className="text-base">{item.icon}</span>
              {item.label}
            </Link>
          )
        })}
      </nav>
      <div className="p-3 border-t border-white/5 text-xs" style={{color:'#444'}}>v0.1.0</div>
    </aside>
  )
}
