import Hero          from '@/components/landing/Hero'
import Stats         from '@/components/landing/Stats'
import Features      from '@/components/landing/Features'
import HowItWorks    from '@/components/landing/HowItWorks'
import ServerSection from '@/components/landing/ServerSection'
import Comparison    from '@/components/landing/Comparison'
import FAQ           from '@/components/landing/FAQ'
import Link          from 'next/link'
import DownloadCTA   from '@/components/download/DownloadCTA'

export const metadata = {
  title: 'Borderless — Open-Source KVM Switch & Clipboard Sync',
  description: 'Control every device from one keyboard and mouse. Open-source KVM switch, clipboard sync, and file transfer for Windows, macOS, Linux, Android, and headless servers.',
}

export default function LandingPage() {
  return (
    <>
      <Hero />
      <Stats />
      <Features />
      <HowItWorks />
      <ServerSection />
      <Comparison />
      <FAQ />

      {/* Final CTA */}
      <section className="border-t border-white/6 bg-surface/40 py-28">
        <div className="mx-auto max-w-3xl px-4 text-center">
          <h2 className="text-4xl font-bold text-white md:text-5xl">
            Ready to go borderless?
          </h2>
          <p className="mx-auto mt-5 mb-10 max-w-xl text-gray-400">
            Free forever. Self-hosted. No account required. Download for your platform and be up in minutes.
          </p>
          <DownloadCTA />
          <p className="mt-8 text-xs text-gray-600">
            MIT Licensed ·{' '}
            <Link href="https://github.com/rythmmcosta/Borderless" target="_blank" rel="noreferrer" className="hover:text-gray-400 transition-colors">Source on GitHub</Link>
            {' '}·{' '}
            <Link href="/releases" className="hover:text-gray-400 transition-colors">View Changelog</Link>
          </p>
        </div>
      </section>
    </>
  )
}
