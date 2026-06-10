const features = [
  {
    icon: '🖱️',
    title: 'KVM Switch',
    desc: 'Move your cursor to the screen edge and instantly control any connected device — no physical switch needed.',
  },
  {
    icon: '📋',
    title: 'Clipboard Sync',
    desc: 'Copy on one device, paste on another. Text, images, and rich content sync in real time across your fleet.',
  },
  {
    icon: '📂',
    title: 'File Transfer',
    desc: 'Drag files between desktops as if they were on the same machine. Fast, encrypted, no cloud required.',
  },
  {
    icon: '🔐',
    title: 'Self-Hosted & Encrypted',
    desc: 'Your data never leaves your network. End-to-end encryption with your own server — no subscriptions.',
  },
  {
    icon: '📱',
    title: 'Android Support',
    desc: 'Use your phone as a second screen or control it from your desktop keyboard. First-class mobile integration.',
  },
  {
    icon: '⚡',
    title: 'Ultra-Low Latency',
    desc: 'Built in Rust with WebSocket signaling. Sub-5ms input latency on a local network — imperceptible lag.',
  },
]

export default function Features() {
  return (
    <section className="bg-[#0f0f0f] py-24">
      <div className="mx-auto max-w-6xl px-4">
        <div className="mb-12 text-center">
          <h2 className="text-3xl font-bold text-white md:text-4xl">
            Everything you need to work seamlessly
          </h2>
          <p className="mt-4 text-gray-400">
            A complete suite of tools to unify your multi-device workspace.
          </p>
        </div>

        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {features.map((f) => (
            <div
              key={f.title}
              className="group rounded-xl border border-white/5 bg-[#1a1a1a] p-6 transition-all hover:border-primary/30 hover:bg-[#1e1e2e]"
            >
              <div className="mb-3 text-3xl">{f.icon}</div>
              <h3 className="mb-2 font-semibold text-white">{f.title}</h3>
              <p className="text-sm leading-relaxed text-gray-400">{f.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
