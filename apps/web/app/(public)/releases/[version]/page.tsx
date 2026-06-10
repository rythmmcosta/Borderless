import { getReleaseByVersion, getAllReleases, fmtDate, fmtBytes } from '@/lib/releases'
import { PlatformIcon } from '@/components/download/PlatformIcons'
import Link from 'next/link'
import { notFound } from 'next/navigation'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'

export const revalidate = 300

export async function generateStaticParams() {
  const releases = await getAllReleases()
  return releases.map((r) => ({ version: r.version }))
}

export async function generateMetadata({ params }: { params: { version: string } }) {
  const r = await getReleaseByVersion(params.version)
  return { title: r ? `${r.version} Release Notes` : 'Release Not Found' }
}

export default async function ReleasePage({ params }: { params: { version: string } }) {
  const release = await getReleaseByVersion(params.version)
  if (!release) notFound()

  const byPlatform = {
    windows: release.assets.filter((a) => a.platform === 'windows'),
    macos:   release.assets.filter((a) => a.platform === 'macos'),
    linux:   release.assets.filter((a) => a.platform === 'linux'),
    android: release.assets.filter((a) => a.platform === 'android'),
  } as const

  return (
    <div className="mx-auto max-w-3xl px-4 py-16">
      <Link href="/releases" className="mb-8 inline-flex items-center gap-1 text-sm text-gray-500 hover:text-gray-300">
        <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
        </svg>
        All releases
      </Link>

      <div className="mb-8 flex flex-wrap items-start justify-between gap-4">
        <div>
          <div className="flex items-center gap-3">
            <h1 className="font-mono text-3xl font-bold text-white">{release.version}</h1>
            {release.prerelease && (
              <span className="rounded-full border border-yellow-500/30 bg-yellow-500/10 px-3 py-1 text-sm text-yellow-400">
                Pre-release
              </span>
            )}
          </div>
          {release.name && release.name !== release.version && (
            <p className="mt-1 text-gray-400">{release.name}</p>
          )}
          <p className="mt-2 text-sm text-gray-600">{fmtDate(release.publishedAt)}</p>
        </div>
        <Link
          href={release.githubUrl}
          target="_blank"
          rel="noreferrer"
          className="flex items-center gap-2 rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm text-gray-300 hover:bg-white/10"
        >
          <svg viewBox="0 0 24 24" fill="currentColor" className="h-4 w-4">
            <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" />
          </svg>
          View on GitHub
        </Link>
      </div>

      {/* Downloads */}
      {release.assets.length > 0 && (
        <div className="mb-10">
          <h2 className="mb-4 text-lg font-semibold text-white">Downloads</h2>
          <div className="grid gap-3 sm:grid-cols-2">
            {(Object.entries(byPlatform) as [keyof typeof byPlatform, typeof release.assets][]).map(([platform, assets]) =>
              assets.length === 0 ? null : (
                <div key={platform} className="rounded-xl border border-white/5 bg-[#1a1a1a] p-4">
                  <div className="mb-3 flex items-center gap-2">
                    <PlatformIcon platform={platform} className="h-4 w-4 text-gray-300" />
                    <span className="text-sm font-medium capitalize text-white">{platform}</span>
                  </div>
                  <div className="space-y-1.5">
                    {assets.map((a) => (
                      <a
                        key={a.url}
                        href={a.url}
                        className="flex items-center justify-between rounded-lg border border-white/5 bg-white/5 px-3 py-2 text-xs hover:bg-white/10"
                      >
                        <span className="text-gray-300">{a.label}</span>
                        <span className="text-gray-600">{fmtBytes(a.size)}</span>
                      </a>
                    ))}
                  </div>
                </div>
              )
            )}
          </div>
        </div>
      )}

      {/* Release notes */}
      {release.notes && (
        <div>
          <h2 className="mb-4 text-lg font-semibold text-white">Release Notes</h2>
          <div className="prose prose-invert prose-dark max-w-none rounded-xl border border-white/5 bg-[#1a1a1a] p-6 prose-a:text-primary prose-code:text-gray-300 prose-pre:bg-[#0f0f0f]">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{release.notes}</ReactMarkdown>
          </div>
        </div>
      )}
    </div>
  )
}
