import { useState } from "react";
import { Device } from "./types";

interface Props {
  device:     Device;
  onConnect:  (id: string) => Promise<void>;
  onDisconnect: (id: string) => Promise<void>;
}

const PLATFORM_ICON: Record<string, string> = {
  windows: "🫏",
  macos:   "🍎",
  linux:   "🐧",
  android: "🤖",
  ios:     "📱",
};

export function DeviceCard({ device, onConnect, onDisconnect }: Props) {
  const [busy, setBusy] = useState(false);

  const handleAction = async () => {
    setBusy(true);
    try {
      if (device.connected) {
        await onDisconnect(device.id);
      } else {
        await onConnect(device.id);
      }
    } finally {
      setBusy(false);
    }
  };

  const icon = PLATFORM_ICON[device.platform.toLowerCase()] ?? "💻";

  return (
    <div style={{
      display:        "flex",
      alignItems:     "center",
      justifyContent: "space-between",
      padding:        "12px 16px",
      background:     "#1e293b",
      borderRadius:   8,
      border:         device.connected ? "1px solid #6366f1" : "1px solid #334155",
    }}>
      <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
        <span style={{ fontSize: 24 }}>{icon}</span>
        <div>
          <div style={{ fontWeight: 600, fontSize: 14 }}>{device.name}</div>
          <div style={{ fontSize: 12, color: "#64748b" }}>
            {device.ip}:{device.port} · {device.platform}
          </div>
        </div>
      </div>

      <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
        {device.connected && (
          <span style={{
            fontSize: 11, fontWeight: 600, color: "#6366f1",
            background: "#1e1b4b", padding: "2px 8px", borderRadius: 12,
          }}>
            Connected
          </span>
        )}
        <button
          className={device.connected ? "btn-danger" : "btn-primary"}
          onClick={handleAction}
          disabled={busy}
        >
          {busy ? "…" : device.connected ? "Disconnect" : "Connect"}
        </button>
      </div>
    </div>
  );
}
