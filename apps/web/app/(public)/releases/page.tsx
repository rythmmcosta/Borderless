import { getAllReleases, fmtDate } from '@/lib/releases'
import Link from 'next/link'

export const metadata = {
  title: 'Releases',
  description: 'Full changelog and release history for Borderless.',
}
export const revalidate = 300

export default async function ReleasesPage() {
  const releases = await getAllReleases()

  return (
    <div className="min-h-screen bg-[#080810]">
      <div className="border-b border-white/6 bg-surface/60 py-14">
        <div className="mx-auto max-w-3xl px-4 text-center">
          <h1 className="text-4xl font-bold text-white md:text-5xl">Changelog</h1>
          <p className="mt-3 text-gray-400">Every release, all in one place.</p>
        </div>
      </div>

      <div className="mx-auto max-w-3xl px-4 py-12">
        {releases.length === 0 ? (
          <div className="rounded-2xl border border-dashed border-white/8 py-20 text-center">
            <p className="text-gray-500">No releases published yet.</p>
            <Link
              href="https://github.com/rythmmcosta/Borderless/releases"
              target="_blank" rel="noreferrer"
              className="mt-3 inline-block text-sm text-primary-light hover:underline"
            >
              Watch on GitHub →
            </Link>
          </div>
        ) : (
          <div className="space-y-3">
            {releases.map((r, i) => (
              <Link
                key={r.version}
                href={`/releases/${r.version}`}
                className="group block rounded-2xl border border-white/6 bg-surface-1 p-6 transition-all duration-200 hover:border-primary/25 hover:bg-surface-2 hover:-translate-y-0.5"
              >
                <div className="flex items-start justify-between gap-4">
                  <div className="flex-1 min-w-0">
                    <div className="flex flex-wrap items-center gap-2.5 mb-1">
                      <span className="font-mono text-lg font-bold text-white">{r.version}</span>
                      {i === 0 && (
                        <span className="rounded-full bg-primary/15 border border-primary/25 px-2 py-0.5 text-xs font-medium text-primary-light">
                          Latest
                        </span>
                      )}
                      {r.prerelease && (
                        <span className="rounded-full border border-amber-500/25 bg-amber-500/10 px-2 py-0.5 text-xs text-amber-400">
                          Pre-release
                        </span>
                      )}
                    </div>
                    {r.name && r.name !== r.version && (
                      <p className="text-sm text-gray-400 mb-1">{r.name}</p>
                    )}
                    <p className="text-xs text-gray-600">{fmtDate(r.publishedAt)}</p>
                    {r.notes && (
                      <p className="mt-3 text-sm text-gray-500 line-clamp-2 leading-relaxed">
                        {r.notes.replace(/#{1,6}\s/g, '').slice(0, 220)}
                      </p>
                    )}
                  </div>
                  <div className="flex items-center gap-3 shrink-0 text-xs text-gray-600">
                    <span>{r.assets.length} assets</span>
                    <svg className="h-4 w-4 transition-transform group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                    </svg>
                  </div>
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
