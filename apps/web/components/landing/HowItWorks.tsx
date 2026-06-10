const steps = [
  {
    n: '01',
    title: 'Deploy the server',
    desc: 'Run the Borderless server binary or Docker container on any machine in your network. It takes under a minute.',
    code: 'docker run -p 8080:8080 ghcr.io/rythmmcosta/borderless-server',
  },
  {
    n: '02',
    title: 'Install on your devices',
    desc: 'Download the Borderless app for Windows, macOS, Linux, or Android. Enter your server address and connect.',
    code: 'Server: 192.168.1.10:8080',
  },
  {
    n: '03',
    title: 'Work without limits',
    desc: 'Seamlessly move between devices. Your keyboard, mouse, clipboard, and files follow you everywhere.',
    code: '✓ 3 devices online · Latency: 2ms',
  },
]

export default function HowItWorks() {
  return (
    <section className="bg-[#0a0a0a] py-24">
      <div className="mx-auto max-w-5xl px-4">
        <div className="mb-12 text-center">
          <h2 className="text-3xl font-bold text-white md:text-4xl">How it works</h2>
          <p className="mt-4 text-gray-400">Up and running in three steps.</p>
        </div>

        <div className="grid gap-6 md:grid-cols-3">
          {steps.map((s) => (
            <div key={s.n} className="relative rounded-xl border border-white/5 bg-[#1a1a1a] p-6">
              <span className="mb-4 block text-5xl font-bold text-white/5">{s.n}</span>
              <h3 className="mb-2 font-semibold text-white">{s.title}</h3>
              <p className="mb-4 text-sm leading-relaxed text-gray-400">{s.desc}</p>
              <div className="rounded-lg bg-[#0f0f0f] px-3 py-2 font-mono text-xs text-gray-400">
                {s.code}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
