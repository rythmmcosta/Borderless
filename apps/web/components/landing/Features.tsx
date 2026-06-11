const features = [
  {
    icon: (
      <svg className="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
      </svg>
    ),
    title: 'Software KVM Switch',
    desc:  'Move your cursor to the screen edge and instantly control any device — no hardware switch, no cables, no lag.',
    tag:   'Core',
    color: 'violet',
  },
  {
    icon: (
      <svg className="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
      </svg>
    ),
    title: 'Universal Clipboard',
    desc:  'Copy on macOS, paste on Windows. Text, images, and rich content sync instantly across every device in your fleet.',
    tag:   'Core',
    color: 'cyan',
  },
  {
    icon: (
      <svg className="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
      </svg>
    ),
    title: 'Drag-and-Drop Files',
    desc:  'Transfer files between machines by dragging them across screens. Fast, encrypted, no cloud upload needed.',
    tag:   'Core',
    color: 'emerald',
  },
  {
    icon: (
      <svg className="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
      </svg>
    ),
    title: 'End-to-End Encrypted',
    desc:  'Every keystroke, clipboard entry, and file is encrypted with AES-256-GCM. Your data never touches the internet.',
    tag:   'Security',
    color: 'amber',
  },
  {
    icon: (
      <svg className="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" />
      </svg>
    ),
    title: 'Android Support',
    desc:  'Use your phone as a second screen or type on it from your desktop keyboard. Full-featured Android app included.',
    tag:   'Mobile',
    color: 'green',
  },
  {
    icon: (
      <svg className="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
      </svg>
    ),
    title: 'Headless Server Agent',
    desc:  'Run the lightweight agent on Proxmox hosts, aaPanel servers, or any bare-metal Linux — no desktop environment needed.',
    tag:   'Servers',
    color: 'blue',
  },
]

const colors: Record<string, string> = {
  violet:  'text-violet-400  bg-violet-400/10  border-violet-400/20',
  cyan:    'text-cyan-400    bg-cyan-400/10    border-cyan-400/20',
  emerald: 'text-emerald-400 bg-emerald-400/10 border-emerald-400/20',
  amber:   'text-amber-400   bg-amber-400/10   border-amber-400/20',
  green:   'text-green-400   bg-green-400/10   border-green-400/20',
  blue:    'text-blue-400    bg-blue-400/10    border-blue-400/20',
}

export default function Features() {
  return (
    <section className="py-28 bg-[#080810]">
      <div className="mx-auto max-w-6xl px-4">
        <div className="mb-16 text-center">
          <p className="mb-3 text-sm font-medium uppercase tracking-widest text-primary-light">Features</p>
          <h2 className="text-4xl font-bold text-white md:text-5xl">
            Everything in one place
          </h2>
          <p className="mt-4 text-gray-400 max-w-xl mx-auto">
            A complete productivity suite for multi-device workspaces — from desktop to data center.
          </p>
        </div>

        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {features.map((f, i) => (
            <div
              key={f.title}
              className="group relative overflow-hidden rounded-2xl border border-white/6 bg-surface-1 p-6 transition-all duration-300 hover:border-white/12 hover:-translate-y-1 hover:shadow-card-hover"
              style={{ animationDelay: `${i * 80}ms` }}
            >
              {/* Subtle glow on hover */}
              <div className="pointer-events-none absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                style={{ background: 'radial-gradient(circle at 30% 30%, rgba(124,58,237,0.05), transparent 60%)' }} />

              <div className={`mb-4 inline-flex items-center justify-center h-11 w-11 rounded-xl border ${colors[f.color]}`}>
                {f.icon}
              </div>

              <div className="mb-2 flex items-center gap-2">
                <h3 className="font-semibold text-white">{f.title}</h3>
                <span className={`text-[10px] font-medium px-1.5 py-0.5 rounded border ${colors[f.color]}`}>{f.tag}</span>
              </div>
              <p className="text-sm leading-relaxed text-gray-400">{f.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
