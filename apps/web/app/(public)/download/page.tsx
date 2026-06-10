import { getLatestRelease } from '@/lib/releases'
import PlatformGrid from '@/components/download/PlatformGrid'
import SystemRequirements from '@/components/download/SystemRequirements'
import Link from 'next/link'

export const metadata = {
  title: 'Download',
  description: 'Download Borderless for Windows, macOS, Linux, and Android.',
}

export const revalidate = 300

export default async function DownloadPage() {
  const release = await getLatestRelease()

  return (
    <div className="mx-auto max-w-4xl px-4 py-16">
      <div className="mb-10 text-center">
        <h1 className="text-4xl font-bold text-white">Download Borderless</h1>
        {release ? (
          <p className="mt-3 text-gray-400">
            Latest release:{' '}
            <Link
              href={`/releases/${release.version}`}
              className="text-primary hover:underline"
            >
              {release.version}
            </Link>
            {' '}— {new Date(release.publishedAt).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })}
          </p>
        ) : (
          <p className="mt-3 text-gray-500">No releases published yet. Check back soon.</p>
        )}
      </div>

      <PlatformGrid allAssets={release?.assets ?? []} />

      <div className="mt-12">
        <h2 className="mb-4 text-xl font-semibold text-white">System Requirements</h2>
        <SystemRequirements />
      </div>

      <div className="mt-10 rounded-xl border border-white/5 bg-[#1a1a1a] p-6">
        <h2 className="mb-3 text-lg font-semibold text-white">Install via Docker (Server)</h2>
        <pre className="overflow-x-auto rounded-lg bg-[#0f0f0f] p-4 font-mono text-sm text-gray-300">
          <code>{`# Run the Borderless server
docker run -d \\
  -p 8080:8080 \\
  -e DATABASE_URL=postgres://... \\
  ghcr.io/rythmmcosta/borderless-server:latest`}</code>
        </pre>
        <p className="mt-3 text-sm text-gray-500">
          Full compose file and self-hosting guide on{' '}
          <Link
            href="https://github.com/rythmmcosta/Borderless"
            target="_blank"
            rel="noreferrer"
            className="text-primary hover:underline"
          >
            GitHub
          </Link>
          .
        </p>
      </div>

      <div className="mt-6 text-center">
        <Link href="/releases" className="text-sm text-gray-500 hover:text-gray-300">
          View all releases and changelogs →
        </Link>
      </div>
    </div>
  )
}
