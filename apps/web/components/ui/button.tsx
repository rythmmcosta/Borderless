import { cn } from '@/lib/utils'
import { ButtonHTMLAttributes, forwardRef } from 'react'

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'outline' | 'danger'
  size?:    'sm' | 'md' | 'lg'
}

const variants: Record<NonNullable<ButtonProps['variant']>, string> = {
  primary:   'bg-primary text-white shadow-glow-sm hover:opacity-90 hover:shadow-glow-md active:scale-[0.98]',
  secondary: 'bg-white/8 text-white border border-white/10 hover:bg-white/12 hover:border-white/20',
  ghost:     'text-gray-400 hover:text-white hover:bg-white/5',
  outline:   'border border-white/10 text-white hover:border-primary/50 hover:bg-primary/5',
  danger:    'bg-red-600/15 text-red-400 border border-red-500/30 hover:bg-red-600/25',
}

const sizes: Record<NonNullable<ButtonProps['size']>, string> = {
  sm: 'h-7  px-3 text-xs  rounded-md  gap-1.5',
  md: 'h-9  px-4 text-sm  rounded-lg  gap-2',
  lg: 'h-11 px-6 text-base rounded-xl gap-2.5',
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ variant = 'primary', size = 'md', className, children, ...props }, ref) => (
    <button
      ref={ref}
      className={cn(
        'inline-flex items-center justify-center font-medium transition-all duration-150 disabled:opacity-40 disabled:pointer-events-none select-none',
        variants[variant],
        sizes[size],
        className,
      )}
      {...props}
    >
      {children}
    </button>
  ),
)
Button.displayName = 'Button'
