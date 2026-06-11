import { getLatestRelease } from '@/lib/releases'
import PlatformGrid from '@/components/download/PlatformGrid'
import SystemRequirements from '@/components/download/SystemRequirements'
import Link from 'next/link'

export const metadata = {
  title: 'Download',
  description: 'Download Borderless for Windows, macOS, Linux, Android, and Linux servers.',
}
export const revalidate = 300

const installCommands = [
  { label: 'Linux Server / Proxmox (one-line install)', cmd: 'curl -sSL https://github.com/rythmmcosta/Borderless/raw/main/deploy/install-agent.sh | sudo bash' },
  { label: 'Docker (self-host the server)', cmd: 'docker run -d -p 8080:8080 -e DATABASE_URL=postgres://... ghcr.io/rythmmcosta/borderless-server:latest' },
]

export default async function DownloadPage() {
  const release = await getLatestRelease()

  return (
    <div className="min-h-screen bg-[#080810]">
      {/* Header */}
      <div className="border-b border-white/6 bg-surface/60 py-14">
        <div className="mx-auto max-w-4xl px-4 text-center">
          <h1 className="text-4xl font-bold text-white md:text-5xl">Download Borderless</h1>
          {release ? (
            <p className="mt-3 text-gray-400">
              Latest:{' '}
              <Link href={`/releases/${release.version}`} className="text-primary-light hover:underline font-mono">
                {release.version}
              </Link>
              {' '}— {new Date(release.publishedAt).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })}
            </p>
          ) : (
            <p className="mt-3 text-gray-500">No release published yet — check back soon.</p>
          )}
        </div>
      </div>

      <div className="mx-auto max-w-4xl px-4 py-12 space-y-12">
        {/* Platform download grid */}
        <PlatformGrid allAssets={release?.assets ?? []} />

        {/* Headless agent section */}
        <div id="agent" className="rounded-2xl border border-cyan-500/15 bg-gradient-to-br from-cyan-500/5 to-surface-1 p-6">
          <div className="mb-3 flex items-center gap-2">
            <span className="inline-flex h-7 w-7 items-center justify-center rounded-lg bg-cyan-500/15 text-cyan-400">
              <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2" />
              </svg>
            </span>
            <h2 className="text-lg font-semibold text-white">Linux Server Agent</h2>
            <span className="rounded-full border border-cyan-500/25 bg-cyan-500/10 px-2 py-0.5 text-[10px] font-medium text-cyan-400">Proxmox · aaPanel · Bare Metal</span>
          </div>
          <p className="mb-5 text-sm text-gray-400">
            No desktop environment needed. The agent provides clipboard sync and keyboard/mouse injection via Linux uinput on any headless server.
          </p>
          <div className="space-y-3">
            {installCommands.map((c) => (
              <div key={c.label}>
                <p className="mb-1.5 text-xs text-gray-500">{c.label}</p>
                <div className="flex items-center gap-2 rounded-lg bg-black/40 px-4 py-3 font-mono text-xs text-gray-300 overflow-x-auto border border-white/6">
                  <span className="shrink-0 text-gray-600">$</span>
                  <span>{c.cmd}</span>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* System requirements */}
        <div>
          <h2 className="mb-4 text-xl font-semibold text-white">System Requirements</h2>
          <SystemRequirements />
        </div>

        {/* Self-host server */}
        <div className="rounded-2xl border border-white/6 bg-surface-1 p-6">
          <h2 className="mb-2 text-lg font-semibold text-white">Self-Host the Server</h2>
          <p className="mb-4 text-sm text-gray-400">
            The Borderless server is required for all clients to connect. Run it on any machine in your network.
          </p>
          <pre className="overflow-x-auto rounded-xl bg-[#080810] p-5 text-xs text-gray-300 border border-white/6">
            <code>{`# docker-compose.yml (minimal)
services:
  server:
    image: ghcr.io/rythmmcosta/borderless-server:latest
    ports: ["8080:8080"]
    environment:
      DATABASE_URL: postgres://admin:<STRONG_DB_PASSWORD>@db:5432/borderless
      JWT_SECRET: <RUN: openssl rand -hex 32>
  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: borderless
      POSTGRES_USER: admin
      POSTGRES_PASSWORD: <STRONG_DB_PASSWORD>`}</code>
          </pre>
          <p className="mt-3 text-xs text-gray-600">
            Full compose with Redis, TURN server, and web dashboard:{' '}
            <Link href="https://github.com/rythmmcosta/Borderless/blob/main/deploy/docker-compose.yml" target="_blank" rel="noreferrer" className="text-primary-light hover:underline">
              deploy/docker-compose.yml
            </Link>
          </p>
        </div>

        <div className="text-center">
          <Link href="/releases" className="text-sm text-gray-500 hover:text-gray-300 transition-colors">
            View all releases and changelogs →
          </Link>
        </div>
      </div>
    </div>
  )
}
