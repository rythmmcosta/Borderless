import Link from 'next/link'

const sections = {
  Product: [
    { label: 'Download',   href: '/download'  },
    { label: 'Changelog',  href: '/releases'  },
    { label: 'Dashboard',  href: '/dashboard' },
  ],
  Platforms: [
    { label: 'Windows',   href: '/download#windows' },
    { label: 'macOS',     href: '/download#macos'   },
    { label: 'Linux',     href: '/download#linux'   },
    { label: 'Android',   href: '/download#android' },
    { label: 'Servers',   href: '/download#agent'   },
  ],
  Developers: [
    { label: 'GitHub',       href: 'https://github.com/rythmmcosta/Borderless' },
    { label: 'Issues',       href: 'https://github.com/rythmmcosta/Borderless/issues' },
    { label: 'Releases API', href: 'https://api.github.com/repos/rythmmcosta/Borderless/releases' },
  ],
}

const GH_ICON = (
  <svg viewBox="0 0 24 24" fill="currentColor" className="h-4 w-4">
    <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" />
  </svg>
)

export default function Footer() {
  return (
    <footer className="border-t border-white/6 bg-surface/60">
      <div className="mx-auto max-w-6xl px-4 py-14">
        <div className="grid grid-cols-2 gap-10 sm:grid-cols-4">
          {/* Brand */}
          <div className="col-span-2 sm:col-span-1">
            <Link href="/" className="flex items-center gap-2.5">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-primary to-violet-500 text-xs font-bold text-white">B</div>
              <span className="text-[15px] font-semibold text-white">Borderless</span>
            </Link>
            <p className="mt-3 text-xs leading-relaxed text-gray-500">
              Open-source KVM switch, clipboard sync, and file transfer for every platform.
            </p>
            <div className="mt-4 flex items-center gap-2">
              <Link
                href="https://github.com/rythmmcosta/Borderless"
                target="_blank" rel="noreferrer"
                className="flex items-center gap-1.5 rounded-md border border-white/8 bg-white/4 px-3 py-1.5 text-xs text-gray-400 hover:text-white hover:border-white/16 transition-colors"
              >
                {GH_ICON} GitHub
              </Link>
            </div>
          </div>

          {/* Nav sections */}
          {Object.entries(sections).map(([title, links]) => (
            <div key={title}>
              <h4 className="mb-3 text-xs font-semibold uppercase tracking-wider text-gray-500">{title}</h4>
              <ul className="space-y-2">
                {links.map((l) => (
                  <li key={l.href}>
                    <Link
                      href={l.href}
                      className="text-sm text-gray-500 transition-colors hover:text-gray-300"
                      {...(l.href.startsWith('http') ? { target: '_blank', rel: 'noreferrer' } : {})}
                    >
                      {l.label}
                    </Link>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        <div className="mt-12 flex flex-col items-center justify-between gap-3 border-t border-white/5 pt-6 sm:flex-row">
          <p className="text-xs text-gray-700">
            © {new Date().getFullYear()} Borderless. Released under the MIT License.
          </p>
          <p className="text-xs text-gray-700">
            Built with Rust · Flutter · Tauri · Next.js
          </p>
        </div>
      </div>
    </footer>
  )
}
