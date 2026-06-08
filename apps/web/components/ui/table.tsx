import { clsx } from 'clsx'

export function Table({ children }: { children: React.ReactNode }) {
  return (
    <div className="overflow-x-auto rounded-lg border border-white/5">
      <table className="w-full text-sm">{children}</table>
    </div>
  )
}

export function THead({ children }: { children: React.ReactNode }) {
  return <thead className="border-b border-white/10" style={{backgroundColor:'#1a1a1a'}}>{children}</thead>
}

export function TBody({ children }: { children: React.ReactNode }) {
  return <tbody className="divide-y divide-white/5">{children}</tbody>
}

export function Tr({ children }: { children: React.ReactNode }) {
  return <tr className="hover:bg-white/[0.02] transition-colors">{children}</tr>
}

export function Th({ children, className }: { children: React.ReactNode; className?: string }) {
  return (
    <th className={clsx('px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider', className)}>
      {children}
    </th>
  )
}

export function Td({
  children,
  className,
  colSpan,
}: {
  children: React.ReactNode
  className?: string
  colSpan?: number
}) {
  return (
    <td className={clsx('px-4 py-3', className)} colSpan={colSpan}>
      {children}
    </td>
  )
}
