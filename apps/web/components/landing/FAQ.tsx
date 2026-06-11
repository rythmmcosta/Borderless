'use client'

import { useState } from 'react'
import { cn } from '@/lib/utils'

const faqs = [
  {
    q: 'Is Borderless really free?',
    a: 'Yes. Borderless is MIT-licensed open source software. It will always be free to use, self-host, and modify. No premium tiers, no feature gates.',
  },
  {
    q: 'Do I need an internet connection?',
    a: 'No. Borderless works entirely on your local network. The server runs on any machine in your LAN. Internet is only used to download the software itself.',
  },
  {
    q: 'Can it work across subnets or over VPN?',
    a: 'Yes. As long as your devices can reach the Borderless server IP, KVM sessions work. A WireGuard or Tailscale VPN between sites works perfectly.',
  },
  {
    q: 'How is it different from TeamViewer or AnyDesk?',
    a: 'Those are remote support tools — you see a remote screen in a window. Borderless is a KVM switch — your mouse cursor moves off one physical screen and onto another. It is meant for machines you already have monitors connected to (or headless servers you manage via SSH).',
  },
  {
    q: 'Can it run on my Proxmox or aaPanel server?',
    a: 'Yes. The borderless-agent daemon runs on any Linux server without a desktop environment. It uses the Linux uinput kernel interface for keyboard/mouse injection and syncs clipboard over REST — no X11, no display required.',
  },
  {
    q: 'How secure is it?',
    a: 'All data between devices is encrypted with AES-256-GCM and authenticated with Ed25519 signatures. Device identity is established via X25519 ECDH key exchange. Your server handles routing, but cannot read payload content.',
  },
  {
    q: 'Do I need an Android or Apple developer account?',
    a: 'No. You can sideload the Android APK directly from the GitHub Releases page without a Play Store account. iOS requires an Apple Developer account for distribution, but Android works with "Unknown sources" enabled.',
  },
  {
    q: 'Can I contribute to the project?',
    a: 'Absolutely. The full source is on GitHub. PRs, issues, and feature requests are welcome. The codebase is Rust (server + agent), Flutter (Android/iOS), Tauri+React (desktop), and Next.js (web dashboard).',
  },
]

export default function FAQ() {
  const [open, setOpen] = useState<number | null>(null)

  return (
    <section className="py-28 bg-[#080810]">
      <div className="mx-auto max-w-2xl px-4">
        <div className="mb-14 text-center">
          <p className="mb-3 text-sm font-medium uppercase tracking-widest text-primary-light">FAQ</p>
          <h2 className="text-4xl font-bold text-white md:text-5xl">Common questions</h2>
        </div>

        <div className="space-y-2">
          {faqs.map((faq, i) => (
            <div
              key={i}
              className={cn(
                'rounded-xl border transition-all duration-200',
                open === i ? 'border-primary/30 bg-primary/5' : 'border-white/6 bg-surface-1',
              )}
            >
              <button
                className="flex w-full items-center justify-between px-5 py-4 text-left"
                onClick={() => setOpen(open === i ? null : i)}
              >
                <span className={cn('text-sm font-medium transition-colors', open === i ? 'text-white' : 'text-gray-300')}>
                  {faq.q}
                </span>
                <svg
                  className={cn('h-4 w-4 shrink-0 text-gray-500 transition-transform duration-200', open === i && 'rotate-180')}
                  fill="none" stroke="currentColor" viewBox="0 0 24 24"
                >
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                </svg>
              </button>
              <div className={cn('overflow-hidden transition-all duration-200', open === i ? 'max-h-60' : 'max-h-0')}>
                <p className="px-5 pb-5 text-sm leading-relaxed text-gray-400">{faq.a}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
