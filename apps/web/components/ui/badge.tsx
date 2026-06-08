import type { HTMLAttributes } from 'react'
import { clsx } from 'clsx'

type BadgeVariant = 'online' | 'offline' | 'neutral' | 'admin' | 'platform'

interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  variant?: BadgeVariant
}

const variantClasses: Record<BadgeVariant, string> = {
  online:   'bg-green-500/20 text-green-400',
  offline:  'bg-slate-500/20 text-slate-400',
  neutral:  'bg-indigo-500/20 text-indigo-300',
  admin:    'bg-orange-500/20 text-orange-300',
  platform: 'bg-purple-500/20 text-purple-300',
}

export function Badge({ variant = 'neutral', className, ...props }: BadgeProps) {
  return (
    <span
      className={clsx(
        'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium',
        variantClasses[variant],
        className,
      )}
      {...props}
    />
  )
}
