import { cn } from '@/lib/utils'

interface CardProps {
  className?: string
  children: React.ReactNode
  hover?: boolean
  glow?: boolean
}

export function Card({ className, children, hover = false, glow = false }: CardProps) {
  return (
    <div
      className={cn(
        'rounded-xl border border-white/6 bg-surface-1',
        hover && 'transition-all duration-200 hover:border-primary/30 hover:shadow-card-hover hover:-translate-y-0.5',
        glow  && 'shadow-glow-sm',
        className,
      )}
    >
      {children}
    </div>
  )
}

export function CardHeader({ className, children }: { className?: string; children: React.ReactNode }) {
  return <div className={cn('p-5 pb-0', className)}>{children}</div>
}

export function CardContent({ className, children }: { className?: string; children: React.ReactNode }) {
  return <div className={cn('p-5', className)}>{children}</div>
}

export function CardTitle({ className, children }: { className?: string; children: React.ReactNode }) {
  return <h3 className={cn('text-base font-semibold text-white', className)}>{children}</h3>
}

export function CardDescription({ className, children }: { className?: string; children: React.ReactNode }) {
  return <p className={cn('text-sm text-gray-400 mt-1', className)}>{children}</p>
}
