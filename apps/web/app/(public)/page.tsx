import Hero from '@/components/landing/Hero'
import Features from '@/components/landing/Features'
import HowItWorks from '@/components/landing/HowItWorks'
import DownloadCTA from '@/components/download/DownloadCTA'
import Link from 'next/link'

export const metadata = {
  title: 'Borderless — Open-Source KVM Switch & Sync',
}

export default function LandingPage() {
  return (
    <>
      <Hero />
      <Features />
      <HowItWorks />

      {/* Download banner */}
      <section className="bg-[#0f0f0f] py-24">
        <div className="mx-auto max-w-3xl px-4 text-center">
          <h2 className="text-3xl font-bold text-white md:text-4xl">
            Ready to go borderless?
          </h2>
          <p className="mt-4 mb-10 text-gray-400">
            Free, open-source, and self-hosted. Download for your platform and get started in minutes.
          </p>
          <DownloadCTA />
          <p className="mt-6 text-sm text-gray-600">
            MIT License ·{' '}
            <Link href="https://github.com/rythmmcosta/Borderless" target="_blank" rel="noreferrer" className="text-gray-400 hover:text-white">
              Source on GitHub
            </Link>
            {' '}·{' '}
            <Link href="/releases" className="text-gray-400 hover:text-white">
              Changelog
            </Link>
          </p>
        </div>
      </section>
    </>
  )
}
