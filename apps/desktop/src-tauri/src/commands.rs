use tauri::State;
use serde::{Deserialize, Serialize};
use borderless_core::{BorderlessEngine, EngineConfig};
use crate::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    pub id:        String,
    pub name:      String,
    pub platform:  String,
    pub ip:        String,
    pub port:      u16,
    pub connected: bool,
}

#[derive(Serialize)]
pub struct EngineStatus {
    pub running:   bool,
    pub device_id: Option<String>,
    pub name:      Option<String>,
}

// ── Engine lifecycle ───────────────────────────────────────────────────

#[tauri::command]
pub async fn start_engine(state: State<'_, AppState>) -> Result<(), String> {
    let mut lock = state.engine.lock().await;
    if lock.is_some() { return Ok(()); }

    let config  = EngineConfig::default();
    let mut engine = BorderlessEngine::new(config)
        .await
        .map_err(|e| e.to_string())?;
    engine.start().await.map_err(|e| e.to_string())?;
    *lock = Some(engine);
    Ok(())
}

#[tauri::command]
pub async fn stop_engine(state: State<'_, AppState>) -> Result<(), String> {
    let mut lock = state.engine.lock().await;
    if let Some(mut engine) = lock.take() {
        engine.stop();
    }
    Ok(())
}

#[tauri::command]
pub async fn engine_status(state: State<'_, AppState>) -> Result<EngineStatus, String> {
    let lock = state.engine.lock().await;
    match lock.as_ref() {
        None => Ok(EngineStatus { running: false, device_id: None, name: None }),
        Some(engine) => Ok(EngineStatus {
            running:   true,
            device_id: Some(engine.identity.id.to_string()),
            name:      Some(engine.identity.display_name.clone()),
        }),
    }
}

// ── Peer discovery ──────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_devices(state: State<'_, AppState>) -> Result<Vec<DeviceInfo>, String> {
    let lock = state.engine.lock().await;
    let engine = match lock.as_ref() {
        Some(e) => e,
        None    => return Ok(vec![]),
    };

    let peers = engine.discovered_devices();
    let devices = peers
        .into_iter()
        .map(|p| DeviceInfo {
            id:        p.id.to_string(),
            name:      p.name,
            platform:  p.platform,
            ip:        p.ip.to_string(),
            port:      p.port,
            connected: false,
        })
        .collect();

    Ok(devices)
}

// ── Session control ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn connect_device(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    let mut lock = state.engine.lock().await;
    let engine = lock.as_mut().ok_or("Engine not running — call start_engine first")?;
    let id = uuid::Uuid::parse_str(&device_id).map_err(|e| e.to_string())?;
    engine.connect(id).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn disconnect_device(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    let lock = state.engine.lock().await;
    let engine = lock.as_ref().ok_or("Engine not running")?;
    let id = uuid::Uuid::parse_str(&device_id).map_err(|e| e.to_string())?;
    engine.disconnect(id).await;
    Ok(())
}
