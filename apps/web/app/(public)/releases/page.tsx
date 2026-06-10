import { getAllReleases, fmtDate } from '@/lib/releases'
import Link from 'next/link'

export const metadata = {
  title: 'Releases',
  description: 'Changelog and release history for Borderless.',
}

export const revalidate = 300

export default async function ReleasesPage() {
  const releases = await getAllReleases()

  return (
    <div className="mx-auto max-w-3xl px-4 py-16">
      <h1 className="mb-2 text-4xl font-bold text-white">Releases</h1>
      <p className="mb-10 text-gray-400">Full changelog for every version of Borderless.</p>

      {releases.length === 0 ? (
        <div className="rounded-xl border border-dashed border-white/10 py-16 text-center">
          <p className="text-gray-500">No releases yet.</p>
          <Link
            href="https://github.com/rythmmcosta/Borderless/releases"
            target="_blank"
            rel="noreferrer"
            className="mt-3 inline-block text-sm text-primary hover:underline"
          >
            Watch on GitHub →
          </Link>
        </div>
      ) : (
        <div className="space-y-4">
          {releases.map((r) => (
            <Link
              key={r.version}
              href={`/releases/${r.version}`}
              className="group block rounded-xl border border-white/5 bg-[#1a1a1a] p-6 transition-all hover:border-primary/30 hover:bg-[#1e1e2e]"
            >
              <div className="flex items-start justify-between gap-4">
                <div>
                  <div className="flex items-center gap-2">
                    <span className="font-mono text-lg font-semibold text-white">{r.version}</span>
                    {r.prerelease && (
                      <span className="rounded-full border border-yellow-500/30 bg-yellow-500/10 px-2 py-0.5 text-xs text-yellow-400">
                        Pre-release
                      </span>
                    )}
                  </div>
                  {r.name && r.name !== r.version && (
                    <p className="mt-0.5 text-sm text-gray-400">{r.name}</p>
                  )}
                  <p className="mt-2 text-xs text-gray-600">{fmtDate(r.publishedAt)}</p>
                </div>
                <div className="flex items-center gap-2 text-xs text-gray-500">
                  <span>{r.assets.length} assets</span>
                  <svg className="h-4 w-4 transition-transform group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                  </svg>
                </div>
              </div>
              {r.notes && (
                <p className="mt-3 line-clamp-2 text-sm text-gray-500">
                  {r.notes.replace(/#+\s/g, '').slice(0, 200)}
                </p>
              )}
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
