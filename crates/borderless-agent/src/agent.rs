use anyhow::{Context, Result};
use uuid::Uuid;
use sha2::Digest;

use borderless_core::{BorderlessEngine, EngineConfig};

use crate::{api::Api, clipboard, config::AgentConfig};

/// Path where the registered device_id is persisted between restarts.
fn device_id_path() -> std::path::PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/var/lib"))
        .join("borderless-agent");
    std::fs::create_dir_all(&base).ok();
    base.join("device_id")
}

fn sha256_hex(s: &str) -> String {
    format!("{:x}", sha2::Sha256::digest(s.as_bytes()))
}

/// Load stored device_id, or register with the server and persist it.
async fn resolve_device_id(
    api: &Api, name: &str, fingerprint: &str, public_key: &str,
) -> Result<Uuid> {
    let path = device_id_path();

    // Try cached ID first
    if let Ok(s) = std::fs::read_to_string(&path) {
        if let Ok(id) = s.trim().parse::<Uuid>() {
            tracing::debug!("Using cached device_id {id}");
            return Ok(id);
        }
    }

    let (id, existed) = api.register_device(name, fingerprint, public_key).await
        .context("Device registration failed")?;
    if existed {
        tracing::info!("Re-using existing device registration");
    } else {
        tracing::info!("Registered new device");
    }
    std::fs::write(&path, id.to_string()).ok();
    Ok(id)
}

/// Main daemon: clipboard sync + KVM engine.
pub async fn run(cfg: AgentConfig) -> Result<()> {
    // ── Authenticate ──────────────────────────────────────────────────────────
    let mut api = Api::new(&cfg.agent.server_url);
    api.login(&cfg.auth.email, &cfg.auth.password).await
        .context("Authentication failed — check server_url, email, password in config")?;
    tracing::info!("Authenticated as {}", cfg.auth.email);

    // ── Initialise engine (identity + KVM session manager) ────────────────────
    let engine_cfg = EngineConfig {
        api_url:           cfg.agent.server_url.clone(),
        device_name:       cfg.agent.device_name.clone(),
        clipboard_sync:    cfg.features.clipboard_sync,
        file_transfer:     false,
        notification_sync: false,
        listen_port:       cfg.agent.listen_port.unwrap_or(49152),
    };
    let mut engine = BorderlessEngine::new(engine_cfg).await
        .context("Failed to initialise engine")?;

    // ── Register / resolve device_id ──────────────────────────────────────────
    let signing_key_hex = hex::encode(engine.identity.signing_key.verifying_key().to_bytes());
    let device_id = resolve_device_id(
        &api,
        &cfg.agent.device_name,
        &engine.identity.fingerprint,
        &signing_key_hex,
    ).await?;
    tracing::info!("Device ID: {device_id}");

    // ── Start engine (mDNS + TCP listener for KVM sessions) ──────────────────
    engine.start().await.context("Failed to start engine")?;
    tracing::info!(
        listen_port = cfg.agent.listen_port.unwrap_or(49152),
        "Engine started — accepting KVM sessions"
    );

    // ── Clipboard sync loop ───────────────────────────────────────────────────
    if cfg.features.clipboard_sync {
        clipboard_loop(api, device_id, cfg.features.poll_interval_secs.unwrap_or(3)).await?;
    } else {
        tokio::signal::ctrl_c().await?;
    }

    engine.stop();
    tracing::info!("Agent stopped");
    Ok(())
}

async fn clipboard_loop(api: Api, device_id: Uuid, poll_secs: u64) -> Result<()> {
    let interval = std::time::Duration::from_secs(poll_secs);
    let mut last_local_hash: Option<String> = None;
    let mut last_server_id:  Option<Uuid>   = None;

    tracing::info!("Clipboard sync active (poll every {poll_secs}s)");

    loop {
        tokio::time::sleep(interval).await;

        // ── Pull: server → local ─────────────────────────────────────────────
        match api.clipboard_history(5).await {
            Err(e) => tracing::warn!("Clipboard poll failed: {e}"),
            Ok(entries) => {
                if let Some(newest) = entries.first() {
                    let is_new   = last_server_id.map_or(true, |id| id != newest.id);
                    let from_other = newest.source_device_id.map_or(true, |id| id != device_id);

                    if is_new && from_other {
                        if let Some(text) = &newest.content {
                            if !text.is_empty() {
                                match clipboard::write(text) {
                                    Ok(_) => {
                                        tracing::info!(
                                            chars = text.len(),
                                            "Clipboard ← server ({})",
                                            newest.source_device_id
                                                .map(|u| u.to_string()).unwrap_or_default()
                                        );
                                        last_local_hash = Some(sha256_hex(text));
                                    }
                                    Err(e) => tracing::warn!("Clipboard write failed: {e}"),
                                }
                            }
                        }
                        last_server_id = Some(newest.id);
                    }
                }
            }
        }

        // ── Push: local → server ─────────────────────────────────────────────
        if let Some(text) = clipboard::read() {
            let h = sha256_hex(&text);
            if last_local_hash.as_deref() != Some(&h) {
                match api.sync_clipboard(device_id, &text, "text/plain").await {
                    Ok(_) => {
                        tracing::info!(chars = text.len(), "Clipboard → server");
                        last_local_hash = Some(h);
                    }
                    Err(e) => tracing::warn!("Clipboard push failed: {e}"),
                }
            }
        }
    }
}
