export default function AdminLayout({ children }: { children: React.ReactNode }) {
  return (
    <div>
      <div className="flex items-center gap-2 mb-6">
        <span className="px-2 py-0.5 text-xs rounded bg-orange-500/20 text-orange-400 font-medium">ADMIN</span>
        <h1 className="text-xl font-bold">Administration</h1>
      </div>
      {children}
    </div>
  )
}
