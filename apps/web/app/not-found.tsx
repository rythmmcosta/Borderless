import Link from 'next/link'

export default function NotFound() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-[#080810] px-4 text-center">
      <div aria-hidden className="pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute left-1/2 top-1/3 h-[300px] w-[500px] -translate-x-1/2 -translate-y-1/2 rounded-full bg-primary/8 blur-[100px]" />
      </div>
      <div className="relative">
        <p className="text-8xl font-bold text-white/5">404</p>
        <div className="-mt-8">
          <h1 className="text-2xl font-bold text-white">Page not found</h1>
          <p className="mt-2 text-sm text-gray-500">The page you're looking for doesn't exist or has been moved.</p>
          <div className="mt-8 flex flex-wrap items-center justify-center gap-3">
            <Link
              href="/"
              className="inline-flex h-10 items-center gap-2 rounded-xl bg-primary px-5 text-sm font-semibold text-white shadow-glow-sm transition-all hover:opacity-90"
            >
              Go home
            </Link>
            <Link
              href="/download"
              className="inline-flex h-10 items-center gap-2 rounded-xl border border-white/10 bg-white/5 px-5 text-sm font-semibold text-white transition-all hover:bg-white/8"
            >
              Download
            </Link>
          </div>
        </div>
      </div>
    </div>
  )
}
