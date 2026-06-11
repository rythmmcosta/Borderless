import { cn } from '@/lib/utils'

type Variant = 'default' | 'success' | 'warning' | 'destructive' | 'info' | 'purple'

const variants: Record<Variant, string> = {
  default:     'bg-white/8 text-gray-300 border-white/10',
  success:     'bg-emerald-500/15 text-emerald-400 border-emerald-500/25',
  warning:     'bg-amber-500/15 text-amber-400 border-amber-500/25',
  destructive: 'bg-red-500/15 text-red-400 border-red-500/25',
  info:        'bg-cyan-500/15 text-cyan-400 border-cyan-500/25',
  purple:      'bg-violet-500/15 text-violet-400 border-violet-500/25',
}

export function Badge({
  variant = 'default',
  className,
  children,
}: {
  variant?: Variant
  className?: string
  children: React.ReactNode
}) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-xs font-medium border',
        variants[variant],
        className,
      )}
    >
      {children}
    </span>
  )
}
