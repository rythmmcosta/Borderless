const stats = [
  { value: '4',       label: 'Platforms',       sub: 'Win · Mac · Linux · Android' },
  { value: '<5ms',    label: 'Input Latency',    sub: 'On local network'            },
  { value: 'AES-256', label: 'Encryption',       sub: 'End-to-end, always'          },
  { value: 'MIT',     label: 'License',          sub: 'Free forever'                },
  { value: '0',       label: 'Cloud Required',   sub: '100% self-hosted'            },
]

export default function Stats() {
  return (
    <section className="border-y border-white/6 bg-surface/60 py-10">
      <div className="mx-auto max-w-6xl px-4">
        <div className="grid grid-cols-2 gap-6 sm:grid-cols-3 lg:grid-cols-5">
          {stats.map((s) => (
            <div key={s.label} className="text-center">
              <p className="text-2xl font-bold text-white md:text-3xl">{s.value}</p>
              <p className="mt-0.5 text-sm font-medium text-gray-300">{s.label}</p>
              <p className="mt-0.5 text-xs text-gray-600">{s.sub}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
