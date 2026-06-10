import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  output: 'standalone',
  env: {
    NEXT_PUBLIC_SITE_URL: process.env.NEXT_PUBLIC_SITE_URL ?? 'https://borderless.myowncloud.tech',
    NEXT_PUBLIC_API_URL:  process.env.NEXT_PUBLIC_API_URL  ?? '/api',
    NEXT_PUBLIC_WS_URL:   process.env.NEXT_PUBLIC_WS_URL   ?? 'wss://borderless.myowncloud.tech',
  },
}

export default nextConfig
