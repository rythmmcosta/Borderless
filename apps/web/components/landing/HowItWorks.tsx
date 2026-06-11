const steps = [
  {
    n: '01',
    title: 'Deploy the server',
    desc:  'Run the Borderless server on any machine in your network — Docker or binary. Done in under a minute.',
    code:  'docker run -d -p 8080:8080 ghcr.io/rythmmcosta/borderless-server',
    color: 'from-violet-500/20 to-violet-500/5',
    border: 'border-violet-500/20',
  },
  {
    n: '02',
    title: 'Connect your devices',
    desc:  'Install Borderless on Windows, macOS, Linux, or Android. Point it at your server IP and connect.',
    code:  'Server: 192.168.1.10:8080  ✓ Connected',
    color: 'from-cyan-500/20 to-cyan-500/5',
    border: 'border-cyan-500/20',
  },
  {
    n: '03',
    title: 'Work without borders',
    desc:  'Your keyboard, mouse, clipboard, and files follow you seamlessly between every device and server.',
    code:  '4 devices online · Latency: 2ms · Encrypted',
    color: 'from-emerald-500/20 to-emerald-500/5',
    border: 'border-emerald-500/20',
  },
]

export default function HowItWorks() {
  return (
    <section className="py-28 bg-surface/40">
      <div className="mx-auto max-w-5xl px-4">
        <div className="mb-16 text-center">
          <p className="mb-3 text-sm font-medium uppercase tracking-widest text-primary-light">Setup</p>
          <h2 className="text-4xl font-bold text-white md:text-5xl">Up in three steps</h2>
          <p className="mt-4 text-gray-400">Self-host in minutes, no technical degree required.</p>
        </div>

        <div className="relative grid gap-6 md:grid-cols-3">
          {/* Connector line */}
          <div className="pointer-events-none absolute top-14 left-[calc(33%+2rem)] right-[calc(33%+2rem)] hidden h-px bg-gradient-to-r from-violet-500/30 via-cyan-500/30 to-emerald-500/30 md:block" />

          {steps.map((s, i) => (
            <div key={s.n} className={`relative rounded-2xl border ${s.border} bg-gradient-to-b ${s.color} p-6`}>
              <div className="mb-5 flex items-center gap-3">
                <span className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-white/10 bg-white/5 font-mono text-sm font-bold text-white">
                  {s.n}
                </span>
                {i < steps.length - 1 && (
                  <svg className="absolute right-4 top-9 hidden h-4 w-4 text-white/20 md:block" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                  </svg>
                )}
              </div>
              <h3 className="mb-2 font-semibold text-white">{s.title}</h3>
              <p className="mb-4 text-sm leading-relaxed text-gray-400">{s.desc}</p>
              <div className="rounded-lg bg-black/30 px-3 py-2.5 font-mono text-xs text-gray-300 break-all">
                {s.code}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
