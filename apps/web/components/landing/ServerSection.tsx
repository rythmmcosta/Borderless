export default function ServerSection() {
  return (
    <section className="py-28 bg-[#080810]">
      <div className="mx-auto max-w-6xl px-4">
        <div className="overflow-hidden rounded-3xl border border-cyan-500/15 bg-gradient-to-br from-cyan-500/5 via-surface-1 to-surface-1">
          <div className="grid gap-0 lg:grid-cols-2">
            {/* Text side */}
            <div className="p-8 md:p-12">
              <div className="mb-4 inline-flex items-center gap-2 rounded-full border border-cyan-500/25 bg-cyan-500/10 px-3 py-1 text-xs font-medium text-cyan-400">
                <span className="h-1.5 w-1.5 rounded-full bg-cyan-400 animate-pulse-slow" />
                For Proxmox · aaPanel · Bare Metal
              </div>

              <h2 className="mt-4 text-3xl font-bold text-white md:text-4xl">
                Headless servers, <br />
                <span className="text-gradient">fully covered</span>
              </h2>

              <p className="mt-4 text-gray-400 leading-relaxed">
                No display? No problem. The <code className="rounded bg-white/8 px-1.5 py-0.5 text-sm text-cyan-300">borderless-agent</code> daemon runs on any Linux server without a desktop environment, connecting it to your KVM fleet.
              </p>

              <ul className="mt-6 space-y-3">
                {[
                  { icon: '📋', text: 'Clipboard sync — copy on desktop, paste via SSH' },
                  { icon: '⌨️', text: 'Keyboard input injection via uinput (no X11 needed)' },
                  { icon: '🔄', text: 'Runs as a systemd service, starts on boot' },
                  { icon: '📦', text: 'Static binary, no runtime dependencies' },
                ].map((item) => (
                  <li key={item.text} className="flex items-start gap-3 text-sm text-gray-300">
                    <span className="mt-0.5 text-base shrink-0">{item.icon}</span>
                    <span>{item.text}</span>
                  </li>
                ))}
              </ul>

              <div className="mt-8 flex flex-wrap gap-3">
                <a
                  href="/download"
                  className="inline-flex items-center gap-2 rounded-lg bg-cyan-500/15 border border-cyan-500/30 px-4 py-2.5 text-sm font-medium text-cyan-300 hover:bg-cyan-500/25 transition-colors"
                >
                  <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                  </svg>
                  Download Agent
                </a>
                <a
                  href="https://github.com/rythmmcosta/Borderless"
                  target="_blank" rel="noreferrer"
                  className="inline-flex items-center gap-2 rounded-lg border border-white/10 px-4 py-2.5 text-sm font-medium text-gray-400 hover:text-white hover:border-white/20 transition-colors"
                >
                  Install Guide →
                </a>
              </div>
            </div>

            {/* Terminal side */}
            <div className="flex items-center border-t border-cyan-500/10 p-8 md:p-10 lg:border-l lg:border-t-0">
              <div className="w-full overflow-hidden rounded-xl border border-white/8 bg-[#080810]">
                <div className="flex items-center gap-1.5 border-b border-white/6 bg-[#141420] px-4 py-2.5">
                  <span className="h-2.5 w-2.5 rounded-full bg-red-500/60" />
                  <span className="h-2.5 w-2.5 rounded-full bg-yellow-500/60" />
                  <span className="h-2.5 w-2.5 rounded-full bg-green-500/60" />
                  <span className="ml-auto text-[10px] text-gray-600 font-mono">proxmox-01</span>
                </div>
                <div className="p-4 font-mono text-xs space-y-2">
                  <div><span className="text-gray-600">$</span> <span className="text-gray-300">curl -sSL .../install-agent.sh | sudo bash</span></div>
                  <div className="text-gray-500 ml-2">Installing borderless-agent v0.1.0...</div>
                  <div className="text-gray-500 ml-2">✓ Binary installed to /usr/local/bin/</div>
                  <div className="text-gray-500 ml-2">✓ Systemd service enabled</div>
                  <div className="text-gray-500 ml-2">⚠  Edit /etc/borderless-agent/config.toml</div>
                  <div className="mt-2"><span className="text-gray-600">$</span> <span className="text-gray-300">sudo systemctl start borderless-agent</span></div>
                  <div className="text-emerald-400/80 ml-2">✓ Agent running — 3 devices in fleet</div>
                  <div className="mt-2"><span className="text-gray-600">$</span> <span className="text-gray-300">borderless-agent paste</span></div>
                  <div className="text-cyan-300/80 ml-2">docker compose up -d --build</div>
                  <div className="mt-2 text-gray-600 text-[10px]">clipboard synced from MacBook Pro · 2s ago</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
