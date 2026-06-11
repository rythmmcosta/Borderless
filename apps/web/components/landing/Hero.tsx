import Link from 'next/link'

export default function Hero() {
  return (
    <section className="relative overflow-hidden bg-[#080810] py-28 md:py-40">
      {/* Gradient glows */}
      <div aria-hidden className="pointer-events-none absolute inset-0">
        <div className="absolute left-1/2 top-[-10%] h-[300px] w-[500px] -translate-x-1/2 rounded-full bg-primary/10 blur-[100px] md:h-[600px] md:w-[900px] md:blur-[120px]" />
        <div className="absolute right-[-5%] bottom-[-10%] h-[200px] w-[300px] rounded-full bg-cyan-500/6 blur-[80px] md:h-[400px] md:w-[500px] md:blur-[100px]" />
      </div>

      {/* Grid overlay */}
      <div
        aria-hidden
        className="pointer-events-none absolute inset-0 opacity-[0.03]"
        style={{
          backgroundImage:
            'linear-gradient(rgba(255,255,255,0.5) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.5) 1px, transparent 1px)',
          backgroundSize: '80px 80px',
        }}
      />

      <div className="relative mx-auto max-w-5xl px-4 text-center">
        {/* Pill badge */}
        <div className="mb-8 inline-flex animate-fade-in items-center gap-2.5 rounded-full border border-primary/25 bg-primary/10 px-4 py-1.5 text-xs font-medium text-primary-light">
          <span className="h-1.5 w-1.5 animate-pulse-slow rounded-full bg-primary-light" />
          Open Source · Self-Hosted · No Subscription
        </div>

        {/* Headline */}
        <h1 className="animate-fade-up text-4xl font-bold leading-[1.08] tracking-tight text-white sm:text-5xl md:text-7xl lg:text-8xl">
          One keyboard.
          <br />
          <span className="text-gradient">Every device.</span>
        </h1>

        <p className="mx-auto mt-7 max-w-2xl animate-fade-up text-lg leading-relaxed text-gray-400 delay-100 md:text-xl">
          Borderless is an open-source software KVM switch — share your keyboard, mouse, and clipboard
          across Windows, macOS, Linux, Android, and headless servers. No hardware. No cloud. Just your network.
        </p>

        {/* CTAs */}
        <div className="mt-10 flex animate-fade-up flex-col items-center justify-center gap-3 delay-200 sm:flex-row">
          <Link
            href="/download"
            className="group inline-flex h-12 items-center gap-2.5 rounded-xl bg-primary px-7 text-[15px] font-semibold text-white shadow-glow-sm transition-all hover:shadow-glow-md hover:opacity-95 active:scale-[0.98]"
          >
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
            </svg>
            Download Free
          </Link>
          <Link
            href="https://github.com/rythmmcosta/Borderless"
            target="_blank" rel="noreferrer"
            className="inline-flex h-12 items-center gap-2.5 rounded-xl border border-white/10 bg-white/5 px-7 text-[15px] font-semibold text-white transition-all hover:bg-white/8 hover:border-white/20"
          >
            <svg viewBox="0 0 24 24" fill="currentColor" className="h-4 w-4">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" />
            </svg>
            View Source
          </Link>
        </div>

        {/* Platform pills */}
        <div className="mt-8 flex animate-fade-up flex-wrap justify-center gap-2 delay-300">
          {['Windows', 'macOS', 'Linux', 'Android', 'Proxmox', 'Servers'].map((p) => (
            <span key={p} className="rounded-full border border-white/8 bg-white/4 px-3 py-1 text-xs text-gray-400">
              {p}
            </span>
          ))}
        </div>

        {/* App preview */}
        <div className="mx-auto mt-14 max-w-2xl animate-fade-up delay-400 md:mt-16">
          <div className="overflow-hidden rounded-xl border border-white/8 bg-[#0e0e14] shadow-[0_20px_40px_rgba(0,0,0,0.5)] md:rounded-2xl md:shadow-[0_30px_60px_rgba(0,0,0,0.6)]">
            {/* Window chrome */}
            <div className="flex items-center gap-1.5 border-b border-white/6 bg-[#141420] px-3 py-2.5 md:px-4 md:py-3">
              <span className="h-2.5 w-2.5 rounded-full bg-red-500/70 md:h-3 md:w-3" />
              <span className="h-2.5 w-2.5 rounded-full bg-yellow-500/70 md:h-3 md:w-3" />
              <span className="h-2.5 w-2.5 rounded-full bg-green-500/70 md:h-3 md:w-3" />
              <span className="ml-auto text-[10px] text-gray-600 font-mono md:text-xs">borderless — server</span>
            </div>
            {/* Terminal */}
            <div className="space-y-2 px-3 py-3 font-mono text-[10px] text-left md:px-5 md:py-5 md:text-xs">
              <div className="flex items-center gap-2">
                <span className="text-emerald-400">✓</span>
                <span className="text-gray-400">Server listening on</span>
                <span className="text-white">0.0.0.0:8080</span>
                <span className="ml-auto text-gray-700">AES-256 ·</span>
                <span className="text-gray-700">encrypted</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-blue-400">→</span>
                <span className="text-gray-400">Device connected:</span>
                <span className="text-white">MacBook Pro</span>
                <span className="text-gray-600">(192.168.1.12)</span>
                <span className="ml-auto text-emerald-500/60 text-[10px]">ACTIVE</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-blue-400">→</span>
                <span className="text-gray-400">Device connected:</span>
                <span className="text-white">Ubuntu Desktop</span>
                <span className="text-gray-600">(192.168.1.23)</span>
                <span className="ml-auto text-emerald-500/60 text-[10px]">ACTIVE</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-blue-400">→</span>
                <span className="text-gray-400">Agent connected:</span>
                <span className="text-white">proxmox-01</span>
                <span className="text-gray-600">(192.168.1.5)</span>
                <span className="ml-auto text-cyan-500/60 text-[10px]">HEADLESS</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-violet-400">⇄</span>
                <span className="text-gray-400">Clipboard sync:</span>
                <span className="text-gray-300">&quot;docker compose up -d&quot;</span>
              </div>
              <div className="mt-1 text-gray-700 text-[10px]">
                Latency: 2ms · Devices: 4 · Uptime: 3d 14h
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
