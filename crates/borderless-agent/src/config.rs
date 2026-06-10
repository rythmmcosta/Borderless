use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentConfig {
    pub agent:    AgentSection,
    pub auth:     AuthSection,
    #[serde(default)]
    pub features: Features,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentSection {
    /// Base URL of the Borderless REST API (no trailing slash)
    pub server_url:  String,
    /// Human-readable name shown in the device list
    pub device_name: String,
    /// TCP port for incoming KVM peer connections
    pub listen_port: Option<u16>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthSection {
    pub email:    String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Features {
    /// Sync clipboard with other Borderless devices
    pub clipboard_sync:      bool,
    /// Inject keyboard/mouse events via uinput (requires /dev/uinput access)
    pub input_inject:        bool,
    /// How often to poll the server for new clipboard entries (seconds)
    pub poll_interval_secs:  Option<u64>,
}

impl Default for Features {
    fn default() -> Self {
        Self { clipboard_sync: true, input_inject: true, poll_interval_secs: Some(3) }
    }
}

/// Search order: CLI arg → /etc/borderless-agent/config.toml → ~/.config/borderless-agent/config.toml
pub fn default_path() -> PathBuf {
    let sys = PathBuf::from("/etc/borderless-agent/config.toml");
    if sys.exists() { return sys; }
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("borderless-agent/config.toml")
}

pub fn load(path: Option<PathBuf>) -> Result<AgentConfig> {
    let path = path.unwrap_or_else(default_path);
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("Cannot read config: {}", path.display()))?;
    toml::from_str(&raw).with_context(|| format!("Invalid TOML in {}", path.display()))
}
