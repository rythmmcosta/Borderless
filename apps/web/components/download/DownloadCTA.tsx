import Link from 'next/link'
import { PlatformIcon } from './PlatformIcons'
import type { Platform } from '@/lib/releases'

const platforms: { p: Platform; label: string; href: string }[] = [
  { p: 'windows', label: 'Windows',  href: '/download#windows' },
  { p: 'macos',   label: 'macOS',    href: '/download#macos'   },
  { p: 'linux',   label: 'Linux',    href: '/download#linux'   },
  { p: 'android', label: 'Android',  href: '/download#android' },
]

export default function DownloadCTA() {
  return (
    <div className="flex flex-wrap justify-center gap-3">
      {platforms.map(({ p, label, href }) => (
        <Link
          key={p}
          href={href}
          className="flex items-center gap-2 rounded-lg border border-white/10 bg-white/5 px-5 py-3 text-sm font-medium text-white transition-all hover:border-white/20 hover:bg-white/10"
        >
          <PlatformIcon platform={p} className="h-5 w-5 text-gray-300" />
          {label}
        </Link>
      ))}
    </div>
  )
}
