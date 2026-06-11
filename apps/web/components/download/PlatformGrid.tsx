'use client'

import { useEffect, useState } from 'react'
import Link from 'next/link'
import { PlatformIcon } from './PlatformIcons'
import type { Platform, PlatformAsset } from '@/lib/releases'
import { fmtBytes } from '@/lib/releases'

type PlatformSection = {
  platform: Platform
  label: string
  description: string
  requirements: string
  color: string
  assets: PlatformAsset[]
}

function detectPlatform(): Platform {
  if (typeof navigator === 'undefined') return 'windows'
  const ua = navigator.userAgent.toLowerCase()
  if (/android/.test(ua)) return 'android'
  if (/iphone|ipad|mac os x/.test(ua) && /mac/.test(ua)) return 'macos'
  if (/linux/.test(ua)) return 'linux'
  return 'windows'
}

function AssetButton({ asset }: { asset: PlatformAsset }) {
  return (
    <a
      href={asset.url}
      className="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 px-4 py-3 text-sm transition-all hover:border-white/20 hover:bg-white/10"
    >
      <span className="font-medium text-white">{asset.label}</span>
      <div className="flex items-center gap-3 text-gray-400">
        <span className="text-xs">{fmtBytes(asset.size)}</span>
        <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
        </svg>
      </div>
    </a>
  )
}

export default function PlatformGrid({ allAssets }: { allAssets: PlatformAsset[] }) {
  const [active, setActive] = useState<Platform>('windows')

  useEffect(() => {
    setActive(detectPlatform())
  }, [])

  const platforms: PlatformSection[] = [
    {
      platform: 'windows',
      label: 'Windows',
      description: 'Windows 10 and later (64-bit)',
      requirements: 'Windows 10 or later, x64',
      color: '#00adef',
      assets: allAssets.filter((a) => a.platform === 'windows'),
    },
    {
      platform: 'macos',
      label: 'macOS',
      description: 'macOS 10.15 Catalina and later',
      requirements: 'macOS 10.15+, Apple Silicon or Intel',
      color: '#a0a0a0',
      assets: allAssets.filter((a) => a.platform === 'macos'),
    },
    {
      platform: 'linux',
      label: 'Linux',
      description: 'AppImage, Debian/Ubuntu, RPM',
      requirements: 'glibc 2.31+, WebKit2GTK 4.1',
      color: '#fcc624',
      assets: allAssets.filter((a) => a.platform === 'linux'),
    },
    {
      platform: 'android',
      label: 'Android',
      description: 'Android 5.0 (API 21) and later',
      requirements: 'Android 5.0+, ARMv7/ARM64/x86',
      color: '#3ddc84',
      assets: allAssets.filter((a) => a.platform === 'android'),
    },
  ]

  const current = platforms.find((p) => p.platform === active)!

  return (
    <div>
      {/* Platform tabs */}
      <div className="mb-8 flex flex-wrap gap-2">
        {platforms.map((p) => (
          <button
            key={p.platform}
            id={p.platform}
            onClick={() => setActive(p.platform)}
            className={`flex items-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-all min-h-[44px] ${
              active === p.platform
                ? 'border-primary bg-primary/10 text-white'
                : 'border-white/10 bg-white/5 text-gray-400 hover:border-white/20 hover:text-white'
            }`}
          >
            <PlatformIcon platform={p.platform} className="h-4 w-4" />
            {p.label}
            {active === p.platform && (
              <span className="ml-1 rounded-sm bg-primary/30 px-1.5 py-0.5 text-xs text-primary-light">
                Detected
              </span>
            )}
          </button>
        ))}
      </div>

      {/* Active platform panel */}
      <div className="rounded-xl border border-white/10 bg-[#1a1a1a] p-6">
        <div className="mb-6 flex items-start gap-4">
          <div className="flex h-14 w-14 items-center justify-center rounded-xl bg-white/5">
            <PlatformIcon platform={current.platform} className="h-7 w-7 text-gray-200" />
          </div>
          <div>
            <h3 className="text-xl font-semibold text-white">{current.label}</h3>
            <p className="mt-0.5 text-sm text-gray-400">{current.description}</p>
            <p className="mt-1 text-xs text-gray-600">Requires: {current.requirements}</p>
          </div>
        </div>

        {current.assets.length > 0 ? (
          <div className="space-y-2">
            {current.assets.map((a) => (
              <AssetButton key={a.url} asset={a} />
            ))}
          </div>
        ) : (
          <div className="rounded-lg border border-dashed border-white/10 px-6 py-10 text-center">
            <p className="text-gray-500">No release assets available yet.</p>
            <p className="mt-1 text-xs text-gray-600">Check back after the first release is published.</p>
            <Link
              href="https://github.com/rythmmcosta/Borderless/releases"
              target="_blank"
              rel="noreferrer"
              className="mt-4 inline-block text-sm text-primary hover:underline"
            >
              View on GitHub Releases →
            </Link>
          </div>
        )}
      </div>
    </div>
  )
}
