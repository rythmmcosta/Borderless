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
  return { title: r ? `${r.version} — Borderless` : 'Release Not Found' }
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

  const platformLabel: Record<string, string> = {
    windows: 'Windows', macos: 'macOS', linux: 'Linux', android: 'Android',
  }

  return (
    <div className="min-h-screen bg-[#080810]">
      {/* Header */}
      <div className="border-b border-white/6 bg-surface/60 py-10">
        <div className="mx-auto max-w-3xl px-4">
          <Link href="/releases" className="mb-5 inline-flex items-center gap-1.5 text-sm text-gray-500 hover:text-gray-300 transition-colors">
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
            All releases
          </Link>

          <div className="flex flex-wrap items-start justify-between gap-4">
            <div>
              <div className="flex flex-wrap items-center gap-3 mb-2">
                <h1 className="font-mono text-3xl font-bold text-white">{release.version}</h1>
                {release.prerelease && (
                  <span className="rounded-full border border-amber-500/25 bg-amber-500/10 px-3 py-1 text-sm text-amber-400">
                    Pre-release
                  </span>
                )}
              </div>
              {release.name && release.name !== release.version && (
                <p className="text-gray-400 text-lg">{release.name}</p>
              )}
              <p className="mt-1.5 text-sm text-gray-600">{fmtDate(release.publishedAt)}</p>
            </div>
            <Link
              href={release.githubUrl}
              target="_blank"
              rel="noreferrer"
              className="flex items-center gap-2 rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm text-gray-300 hover:bg-white/10 hover:border-white/20 transition-all"
            >
              <svg viewBox="0 0 24 24" fill="currentColor" className="h-4 w-4 shrink-0">
                <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" />
              </svg>
              View on GitHub
            </Link>
          </div>
        </div>
      </div>

      <div className="mx-auto max-w-3xl px-4 py-10 space-y-10">
        {/* Downloads */}
        {release.assets.length > 0 && (
          <div>
            <h2 className="mb-4 text-lg font-semibold text-white flex items-center gap-2">
              <svg className="h-5 w-5 text-primary-light" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.75} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
              </svg>
              Downloads
              <span className="ml-auto text-sm font-normal text-gray-600">{release.assets.length} files</span>
            </h2>
            <div className="grid gap-3 sm:grid-cols-2">
              {(Object.entries(byPlatform) as [keyof typeof byPlatform, typeof release.assets][]).map(([platform, assets]) =>
                assets.length === 0 ? null : (
                  <div key={platform} className="rounded-2xl border border-white/6 bg-surface-1 p-4">
                    <div className="mb-3 flex items-center gap-2.5">
                      <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-white/5">
                        <PlatformIcon platform={platform} className="h-4 w-4 text-gray-300" />
                      </div>
                      <span className="text-sm font-semibold text-white">{platformLabel[platform] ?? platform}</span>
                      <span className="ml-auto text-xs text-gray-600">{assets.length} file{assets.length !== 1 ? 's' : ''}</span>
                    </div>
                    <div className="space-y-1.5">
                      {assets.map((a) => (
                        <a
                          key={a.url}
                          href={a.url}
                          className="group flex items-center justify-between rounded-xl border border-white/5 bg-white/3 px-3.5 py-2.5 text-xs hover:bg-white/8 hover:border-white/10 transition-all"
                        >
                          <span className="text-gray-300 group-hover:text-white transition-colors truncate mr-2">{a.label}</span>
                          <span className="text-gray-600 shrink-0">{fmtBytes(a.size)}</span>
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
            <h2 className="mb-4 text-lg font-semibold text-white flex items-center gap-2">
              <svg className="h-5 w-5 text-primary-light" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.75} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              Release Notes
            </h2>
            <div className="prose prose-invert prose-dark max-w-none rounded-2xl border border-white/6 bg-surface-1 p-6 md:p-8 prose-a:text-primary-light prose-code:text-gray-300 prose-pre:bg-[#0a0a12] prose-pre:border prose-pre:border-white/6 prose-headings:text-white prose-strong:text-white">
              <ReactMarkdown remarkPlugins={[remarkGfm]}>{release.notes}</ReactMarkdown>
            </div>
          </div>
        )}

        {/* Nav to other releases */}
        <div className="flex items-center justify-between pt-4 border-t border-white/6">
          <Link href="/releases" className="text-sm text-gray-500 hover:text-gray-300 transition-colors flex items-center gap-1.5">
            <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
            All releases
          </Link>
          <Link href="/download" className="text-sm text-primary-light hover:underline transition-colors">
            Download latest →
          </Link>
        </div>
      </div>
    </div>
  )
}
