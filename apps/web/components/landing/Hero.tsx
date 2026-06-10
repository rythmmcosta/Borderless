import Link from 'next/link'

export default function Hero() {
  return (
    <section className="relative overflow-hidden bg-[#0a0a0a] py-24 md:py-36">
      {/* Glow */}
      <div
        aria-hidden
        className="pointer-events-none absolute left-1/2 top-0 h-[600px] w-[900px] -translate-x-1/2 -translate-y-1/3 rounded-full bg-primary/10 blur-[120px]"
      />

      <div className="relative mx-auto max-w-4xl px-4 text-center">
        <div className="mb-6 inline-flex items-center gap-2 rounded-full border border-primary/30 bg-primary/10 px-4 py-1.5 text-xs font-medium text-primary">
          <span className="h-1.5 w-1.5 animate-pulse-slow rounded-full bg-primary" />
          Open Source · Self-Hosted · Cross-Platform
        </div>

        <h1 className="animate-fade-up text-5xl font-bold leading-tight tracking-tight text-white md:text-7xl">
          One keyboard,
          <br />
          <span className="bg-gradient-to-r from-primary to-violet-400 bg-clip-text text-transparent">
            every device
          </span>
        </h1>

        <p className="mx-auto mt-6 max-w-2xl animate-fade-up text-lg leading-relaxed text-gray-400 [animation-delay:100ms]">
          Borderless lets you control Windows, macOS, Linux, and Android devices from a single keyboard and mouse.
          Clipboard sync, file drag-and-drop, and zero latency — all self-hosted.
        </p>

        <div className="mt-10 flex animate-fade-up flex-col items-center justify-center gap-4 [animation-delay:200ms] sm:flex-row">
          <Link
            href="/download"
            className="rounded-xl bg-primary px-8 py-3.5 text-base font-semibold text-white shadow-lg shadow-primary/25 transition-all hover:scale-105 hover:opacity-90"
          >
            Download Free
          </Link>
          <Link
            href="https://github.com/rythmmcosta/Borderless"
            target="_blank"
            rel="noreferrer"
            className="flex items-center gap-2 rounded-xl border border-white/10 bg-white/5 px-8 py-3.5 text-base font-semibold text-white transition-all hover:bg-white/10"
          >
            <svg viewBox="0 0 24 24" fill="currentColor" className="h-5 w-5">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" />
            </svg>
            View on GitHub
          </Link>
        </div>

        {/* Mock terminal preview */}
        <div className="mx-auto mt-16 max-w-2xl animate-fade-up overflow-hidden rounded-xl border border-white/10 bg-[#111] text-left shadow-2xl [animation-delay:300ms]">
          <div className="flex items-center gap-1.5 border-b border-white/5 px-4 py-3">
            <span className="h-3 w-3 rounded-full bg-red-500/70" />
            <span className="h-3 w-3 rounded-full bg-yellow-500/70" />
            <span className="h-3 w-3 rounded-full bg-green-500/70" />
            <span className="ml-3 text-xs text-gray-600">borderless — server</span>
          </div>
          <div className="space-y-1.5 px-4 py-4 font-mono text-xs text-gray-400">
            <p><span className="text-green-400">✓</span> Server started on <span className="text-white">0.0.0.0:8080</span></p>
            <p><span className="text-blue-400">→</span> Device connected: <span className="text-white">MacBook Pro</span> <span className="text-gray-600">(192.168.1.12)</span></p>
            <p><span className="text-blue-400">→</span> Device connected: <span className="text-white">Ubuntu Workstation</span> <span className="text-gray-600">(192.168.1.23)</span></p>
            <p><span className="text-yellow-400">⇄</span> Clipboard sync: <span className="text-gray-300">"Hello from macOS!"</span></p>
            <p className="text-gray-600">Latency: 2ms · Encrypted: AES-256 · Devices: 3</p>
          </div>
        </div>
      </div>
    </section>
  )
}
