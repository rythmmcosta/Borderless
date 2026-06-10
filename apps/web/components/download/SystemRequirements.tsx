const rows = [
  { platform: 'Windows', req: 'Windows 10 or later', arch: 'x64', net: 'LAN or internet' },
  { platform: 'macOS',   req: 'macOS 10.15 Catalina+', arch: 'Apple Silicon / Intel x64', net: 'LAN or internet' },
  { platform: 'Linux',   req: 'glibc 2.31+', arch: 'x64', net: 'LAN or internet' },
  { platform: 'Android', req: 'Android 5.0 (API 21)+', arch: 'ARMv7 / ARM64 / x86', net: 'LAN or internet' },
]

export default function SystemRequirements() {
  return (
    <div className="overflow-x-auto rounded-xl border border-white/10">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-white/10 bg-white/5">
            <th className="px-4 py-3 text-left font-semibold text-gray-300">Platform</th>
            <th className="px-4 py-3 text-left font-semibold text-gray-300">OS Version</th>
            <th className="px-4 py-3 text-left font-semibold text-gray-300">Architecture</th>
            <th className="px-4 py-3 text-left font-semibold text-gray-300">Network</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((r, i) => (
            <tr key={r.platform} className={i % 2 === 0 ? 'bg-transparent' : 'bg-white/[0.02]'}>
              <td className="px-4 py-3 font-medium text-white">{r.platform}</td>
              <td className="px-4 py-3 text-gray-400">{r.req}</td>
              <td className="px-4 py-3 text-gray-400">{r.arch}</td>
              <td className="px-4 py-3 text-gray-400">{r.net}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
