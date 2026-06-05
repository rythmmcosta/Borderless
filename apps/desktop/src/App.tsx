import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DeviceList } from "./features/devices/DeviceList";

interface EngineStatus {
  running:   boolean;
  device_id: string | null;
  name:      string | null;
}

export default function App() {
  const [status, setStatus]   = useState<EngineStatus>({ running: false, device_id: null, name: null });
  const [loading, setLoading] = useState(false);
  const [error,   setError]   = useState<string | null>(null);

  const refreshStatus = useCallback(async () => {
    try {
      const s = await invoke<EngineStatus>("engine_status");
      setStatus(s);
    } catch (e) {
      setError(String(e));
    }
  }, []);

  useEffect(() => {
    refreshStatus();
    const id = setInterval(refreshStatus, 5000);
    return () => clearInterval(id);
  }, [refreshStatus]);

  const handleStart = async () => {
    setLoading(true);
    setError(null);
    try {
      await invoke("start_engine");
      await refreshStatus();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const handleStop = async () => {
    setLoading(true);
    try {
      await invoke("stop_engine");
      await refreshStatus();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      <Header
        status={status}
        loading={loading}
        onStart={handleStart}
        onStop={handleStop}
      />

      {error && (
        <div style={{
          margin: "0 16px",
          padding: "8px 12px",
          background: "#7f1d1d",
          borderRadius: 6,
          color: "#fca5a5",
          fontSize: 12,
        }}>
          {error}
        </div>
      )}

      <main style={{ flex: 1, overflow: "auto", padding: "16px" }}>
        {status.running
          ? <DeviceList />
          : (
            <div style={{
              display: "flex", flexDirection: "column", alignItems: "center",
              justifyContent: "center", height: "100%", gap: 16, color: "#64748b",
            }}>
              <div style={{ fontSize: 48 }}>⚡</div>
              <p style={{ fontSize: 16, fontWeight: 500 }}>Borderless is not running</p>
              <p style={{ fontSize: 13 }}>Click <strong>Start</strong> to begin LAN discovery and enable KVM sharing.</p>
              <button className="btn-primary" onClick={handleStart} disabled={loading}>
                {loading ? "Starting…" : "Start Borderless"}
              </button>
            </div>
          )
        }
      </main>
    </div>
  );
}

function Header({ status, loading, onStart, onStop }: {
  status:  EngineStatus;
  loading: boolean;
  onStart: () => void;
  onStop:  () => void;
}) {
  return (
    <header style={{
      display: "flex", alignItems: "center", justifyContent: "space-between",
      padding: "12px 16px",
      background: "#1e293b",
      borderBottom: "1px solid #334155",
    }}>
      <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
        <div style={{
          width: 8, height: 8, borderRadius: "50%",
          background: status.running ? "#22c55e" : "#475569",
          boxShadow: status.running ? "0 0 6px #22c55e" : "none",
        }} />
        <span style={{ fontWeight: 600, fontSize: 15 }}>Borderless</span>
        {status.name && (
          <span style={{ fontSize: 12, color: "#94a3b8" }}>— {status.name}</span>
        )}
      </div>

      <div style={{ display: "flex", gap: 8 }}>
        {status.running
          ? <button className="btn-danger"    onClick={onStop}  disabled={loading}>Stop</button>
          : <button className="btn-primary"   onClick={onStart} disabled={loading}>Start</button>
        }
      </div>
    </header>
  );
}
