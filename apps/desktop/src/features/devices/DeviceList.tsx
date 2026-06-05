import { useDevices } from "./useDevices";
import { DeviceCard } from "./DeviceCard";

export function DeviceList() {
  const { devices, loading, error, refresh, connect, disconnect } = useDevices();

  return (
    <div>
      <div style={{
        display:        "flex",
        alignItems:     "center",
        justifyContent: "space-between",
        marginBottom:   12,
      }}>
        <h2 style={{ fontSize: 16, fontWeight: 600 }}>
          Nearby Devices
          {devices.length > 0 && (
            <span style={{
              marginLeft: 8, fontSize: 12, fontWeight: 500,
              color: "#94a3b8",
            }}>
              {devices.length} found
            </span>
          )}
        </h2>
        <button className="btn-secondary" onClick={refresh} disabled={loading}>
          {loading ? "Scanning…" : "Refresh"}
        </button>
      </div>

      {error && (
        <div style={{
          padding: "8px 12px", marginBottom: 12,
          background: "#7f1d1d", borderRadius: 6,
          color: "#fca5a5", fontSize: 12,
        }}>
          {error}
        </div>
      )}

      {devices.length === 0 && !loading && (
        <div style={{
          display: "flex", flexDirection: "column", alignItems: "center",
          gap: 8, padding: "40px 0", color: "#475569",
        }}>
          <span style={{ fontSize: 36 }}>🔍</span>
          <p style={{ fontWeight: 500 }}>No devices found on LAN</p>
          <p style={{ fontSize: 12 }}>
            Make sure other machines are running Borderless on the same network.
          </p>
        </div>
      )}

      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {devices.map(device => (
          <DeviceCard
            key={device.id}
            device={device}
            onConnect={connect}
            onDisconnect={disconnect}
          />
        ))}
      </div>
    </div>
  );
}
