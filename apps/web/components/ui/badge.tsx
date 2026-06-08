import { clsx } from 'clsx'

type Variant = 'default' | 'success' | 'warning' | 'destructive'

const variants: Record<Variant, string> = {
  default:     'bg-white/10 text-gray-300',
  success:     'bg-green-500/20 text-green-400',
  warning:     'bg-orange-500/20 text-orange-400',
  destructive: 'bg-red-500/20 text-red-400',
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
      className={clsx(
        'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium',
        variants[variant],
        className
      )}
    >
      {children}
    </span>
  )
}
