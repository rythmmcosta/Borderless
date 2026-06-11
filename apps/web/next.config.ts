import type { NextConfig } from 'next'

const CSP = [
  "default-src 'self'",
  "script-src 'self' 'unsafe-inline'",   // Next.js hydration requires unsafe-inline
  "style-src 'self' 'unsafe-inline'",
  "img-src 'self' data: https:",
  "font-src 'self'",
  "connect-src 'self' https://api.github.com wss: ws:",
  "frame-ancestors 'none'",
].join('; ')

const nextConfig: NextConfig = {
  output: 'standalone',
  env: {
    NEXT_PUBLIC_SITE_URL: process.env.NEXT_PUBLIC_SITE_URL ?? 'https://borderless.myowncloud.tech',
    NEXT_PUBLIC_API_URL:  process.env.NEXT_PUBLIC_API_URL  ?? '/api',
    NEXT_PUBLIC_WS_URL:   process.env.NEXT_PUBLIC_WS_URL   ?? 'wss://borderless.myowncloud.tech',
  },
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          { key: 'X-Content-Type-Options',    value: 'nosniff' },
          { key: 'X-Frame-Options',           value: 'DENY' },
          { key: 'X-XSS-Protection',          value: '1; mode=block' },
          { key: 'Referrer-Policy',           value: 'strict-origin-when-cross-origin' },
          { key: 'Permissions-Policy',        value: 'camera=(), microphone=(), geolocation=()' },
          { key: 'Content-Security-Policy',   value: CSP },
        ],
      },
    ]
  },
}

export default nextConfig
