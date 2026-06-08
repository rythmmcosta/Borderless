import type { HTMLAttributes } from 'react'
import { clsx } from 'clsx'

type CardProps = HTMLAttributes<HTMLDivElement>

export function Card({ className, ...props }: CardProps) {
  return (
    <div
      className={clsx('bg-surface-raised rounded-xl border border-white/5', className)}
      {...props}
    />
  )
}
