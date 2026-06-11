'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useState, useEffect } from 'react'
import { cn } from '@/lib/utils'

const links = [
  { href: '/',         label: 'Home'      },
  { href: '/download', label: 'Download'  },
  { href: '/releases', label: 'Changelog' },
]

const GH_ICON = (
  <svg viewBox="0 0 24 24" fill="currentColor" className="h-4 w-4">
    <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" />
  </svg>
)

export default function Navbar() {
  const pathname = usePathname()
  const [open, setOpen]         = useState(false)
  const [scrolled, setScrolled] = useState(false)

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 20)
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => window.removeEventListener('scroll', onScroll)
  }, [])

  return (
    <header
      className={cn(
        'sticky top-0 z-50 transition-all duration-300',
        scrolled
          ? 'border-b border-white/8 bg-[#080810]/90 backdrop-blur-xl shadow-[0_1px_0_rgba(255,255,255,0.04)]'
          : 'border-b border-transparent bg-transparent',
      )}
    >
      <div className="mx-auto flex h-16 max-w-6xl items-center justify-between px-4">
        {/* Logo */}
        <Link href="/" className="flex items-center gap-2.5 group">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-primary to-violet-500 text-xs font-bold text-white shadow-glow-sm group-hover:shadow-glow-md transition-shadow">
            B
          </div>
          <span className="text-[15px] font-semibold tracking-tight text-white">Borderless</span>
        </Link>

        {/* Desktop nav */}
        <nav className="hidden items-center gap-0.5 md:flex">
          {links.map((l) => (
            <Link
              key={l.href}
              href={l.href}
              className={cn(
                'rounded-md px-3.5 py-2 text-sm transition-colors',
                pathname === l.href
                  ? 'text-white font-medium bg-white/8'
                  : 'text-gray-400 hover:text-white hover:bg-white/5',
              )}
            >
              {l.label}
            </Link>
          ))}
        </nav>

        {/* Desktop actions */}
        <div className="hidden items-center gap-2 md:flex">
          <Link
            href="https://github.com/rythmmcosta/Borderless"
            target="_blank" rel="noreferrer"
            className="flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
          >
            {GH_ICON}
            <span>GitHub</span>
          </Link>
          <Link
            href="/download"
            className="rounded-lg bg-primary px-4 py-2 text-sm font-medium text-white shadow-glow-sm hover:opacity-90 hover:shadow-glow-md transition-all"
          >
            Download
          </Link>
        </div>

        {/* Mobile hamburger */}
        <button
          className="flex h-9 w-9 flex-col items-center justify-center gap-[5px] rounded-lg hover:bg-white/5 md:hidden"
          onClick={() => setOpen(!open)}
          aria-label="Menu"
        >
          <span className={cn('h-[1.5px] w-5 bg-gray-300 transition-all duration-200', open && 'translate-y-[6.5px] rotate-45')} />
          <span className={cn('h-[1.5px] w-5 bg-gray-300 transition-all duration-200', open && 'opacity-0')} />
          <span className={cn('h-[1.5px] w-5 bg-gray-300 transition-all duration-200', open && '-translate-y-[6.5px] -rotate-45')} />
        </button>
      </div>

      {/* Mobile menu */}
      <div className={cn(
        'overflow-hidden transition-all duration-300 md:hidden',
        open ? 'max-h-64' : 'max-h-0',
      )}>
        <div className="border-t border-white/6 bg-[#080810]/95 px-4 py-3 space-y-1">
          {links.map((l) => (
            <Link
              key={l.href}
              href={l.href}
              onClick={() => setOpen(false)}
              className={cn(
                'block rounded-lg px-3 py-2 text-sm transition-colors',
                pathname === l.href ? 'text-white bg-white/8' : 'text-gray-400 hover:text-white',
              )}
            >
              {l.label}
            </Link>
          ))}
          <Link
            href="/download"
            onClick={() => setOpen(false)}
            className="block mt-2 rounded-lg bg-primary px-3 py-2 text-center text-sm font-medium text-white"
          >
            Download
          </Link>
        </div>
      </div>
    </header>
  )
}
