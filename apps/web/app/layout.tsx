import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: {
    default: 'Borderless — Open-Source KVM Switch & Sync',
    template: '%s | Borderless',
  },
  description:
    'Control every device from anywhere. Open-source KVM switch, clipboard sync and file transfer for Windows, macOS, Linux and Android.',
  metadataBase: new URL(process.env.NEXT_PUBLIC_SITE_URL ?? 'https://borderless.myowncloud.tech'),
  openGraph: {
    siteName: 'Borderless',
    type: 'website',
  },
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="dark">
      <body className="bg-[#0a0a0a] text-white antialiased">{children}</body>
    </html>
  )
}
