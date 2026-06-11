import { cn } from '@/lib/utils'

export function Table({ children }: { children: React.ReactNode }) {
  return (
    <div className="overflow-x-auto rounded-xl border border-white/6">
      <table className="w-full text-sm">{children}</table>
    </div>
  )
}

export function THead({ children }: { children: React.ReactNode }) {
  return (
    <thead className="border-b border-white/6 bg-white/[0.02]">{children}</thead>
  )
}

export function TBody({ children }: { children: React.ReactNode }) {
  return <tbody className="divide-y divide-white/4">{children}</tbody>
}

export function Tr({ children, className }: { children: React.ReactNode; className?: string }) {
  return (
    <tr className={cn('hover:bg-white/[0.025] transition-colors', className)}>{children}</tr>
  )
}

export function Th({ children, className }: { children?: React.ReactNode; className?: string }) {
  return (
    <th className={cn('px-3 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider whitespace-nowrap sm:px-4', className)}>
      {children}
    </th>
  )
}

export function Td({
  children,
  className,
  colSpan,
}: {
  children?: React.ReactNode
  className?: string
  colSpan?: number
}) {
  return (
    <td className={cn('px-3 py-3 text-gray-300 sm:px-4', className)} colSpan={colSpan}>
      {children}
    </td>
  )
}
