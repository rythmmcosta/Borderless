import { clsx } from 'clsx'

export function Card({ className, children }: { className?: string; children: React.ReactNode }) {
  return (
    <div
      className={clsx('rounded-lg p-4 border border-white/5', className)}
      style={{backgroundColor:'#1a1a1a'}}
    >
      {children}
    </div>
  )
}
