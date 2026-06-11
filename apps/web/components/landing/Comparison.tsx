const rows = [
  { feature: 'Price',              borderless: 'Free',    synergy: '$29/year', mwb: 'Free (Windows)' },
  { feature: 'Open Source',        borderless: true,      synergy: false,      mwb: false  },
  { feature: 'Self-Hosted',        borderless: true,      synergy: false,      mwb: false  },
  { feature: 'Windows',            borderless: true,      synergy: true,       mwb: true   },
  { feature: 'macOS',              borderless: true,      synergy: true,       mwb: false  },
  { feature: 'Linux',              borderless: true,      synergy: true,       mwb: false  },
  { feature: 'Android',            borderless: true,      synergy: false,      mwb: false  },
  { feature: 'Headless Servers',   borderless: true,      synergy: false,      mwb: false  },
  { feature: 'File Transfer',      borderless: true,      synergy: true,       mwb: false  },
  { feature: 'End-to-End Encrypt', borderless: true,      synergy: true,       mwb: false  },
  { feature: 'Cloud Account',      borderless: 'None',    synergy: 'Required', mwb: 'None' },
]

function Cell({ val }: { val: boolean | string }) {
  if (typeof val === 'boolean') {
    return val
      ? <span className="text-emerald-400 font-medium text-lg">✓</span>
      : <span className="text-gray-700 text-lg">—</span>
  }
  return <span className="text-sm text-gray-300">{val}</span>
}

export default function Comparison() {
  return (
    <section className="py-28 bg-surface/40">
      <div className="mx-auto max-w-4xl px-4">
        <div className="mb-14 text-center">
          <p className="mb-3 text-sm font-medium uppercase tracking-widest text-primary-light">Compare</p>
          <h2 className="text-4xl font-bold text-white md:text-5xl">Why Borderless?</h2>
          <p className="mt-4 text-gray-400">See how we stack up against popular alternatives.</p>
        </div>

        <div className="overflow-x-auto rounded-2xl border border-white/6">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-white/8">
                <th className="py-4 pl-5 text-left text-xs font-medium uppercase tracking-wide text-gray-500">Feature</th>
                <th className="py-4 px-4 text-center">
                  <div className="inline-flex flex-col items-center gap-1">
                    <span className="flex h-7 w-7 items-center justify-center rounded-lg bg-primary text-xs font-bold text-white">B</span>
                    <span className="text-xs font-semibold text-white">Borderless</span>
                  </div>
                </th>
                <th className="py-4 px-4 text-center">
                  <div className="inline-flex flex-col items-center gap-1">
                    <span className="flex h-7 w-7 items-center justify-center rounded-lg bg-white/10 text-xs font-bold text-gray-300">S</span>
                    <span className="text-xs font-medium text-gray-400">Synergy</span>
                  </div>
                </th>
                <th className="py-4 px-4 text-center">
                  <div className="inline-flex flex-col items-center gap-1">
                    <span className="flex h-7 w-7 items-center justify-center rounded-lg bg-white/10 text-xs font-bold text-gray-300">M</span>
                    <span className="text-xs font-medium text-gray-400 whitespace-nowrap">Mouse w/o Borders</span>
                  </div>
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-white/4">
              {rows.map((r) => (
                <tr key={r.feature} className="hover:bg-white/[0.02] transition-colors">
                  <td className="py-3.5 pl-5 text-gray-300">{r.feature}</td>
                  <td className="py-3.5 px-4 text-center bg-primary/5">
                    <Cell val={r.borderless} />
                  </td>
                  <td className="py-3.5 px-4 text-center">
                    <Cell val={r.synergy} />
                  </td>
                  <td className="py-3.5 px-4 text-center">
                    <Cell val={r.mwb} />
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </section>
  )
}
