import { clsx } from 'clsx'
import type { ButtonHTMLAttributes } from 'react'

type Variant = 'default' | 'outline' | 'ghost' | 'destructive'

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant
}

const variants: Record<Variant, string> = {
  default:     'bg-purple-600 hover:bg-purple-700 text-white',
  outline:     'border border-white/10 hover:bg-white/5',
  ghost:       'hover:bg-white/5',
  destructive: 'bg-red-600 hover:bg-red-700 text-white',
}

export function Button({ variant = 'default', className, ...props }: ButtonProps) {
  return (
    <button
      {...props}
      className={clsx(
        'px-4 py-2 rounded-md text-sm font-medium transition-colors disabled:opacity-50',
        variants[variant],
        className
      )}
    />
  )
}
