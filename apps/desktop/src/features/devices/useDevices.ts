import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Device } from "./types";

export function useDevices(pollMs = 3000) {
  const [devices, setDevices] = useState<Device[]>([]);
  const [loading, setLoading] = useState(false);
  const [error,   setError]   = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const list = await invoke<Device[]>("list_devices");
      setDevices(list);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  const connect = useCallback(async (id: string) => {
    await invoke("connect_device", { deviceId: id });
    await refresh();
  }, [refresh]);

  const disconnect = useCallback(async (id: string) => {
    await invoke("disconnect_device", { deviceId: id });
    await refresh();
  }, [refresh]);

  useEffect(() => {
    refresh();
    const id = setInterval(refresh, pollMs);
    return () => clearInterval(id);
  }, [refresh, pollMs]);

  return { devices, loading, error, refresh, connect, disconnect };
}
