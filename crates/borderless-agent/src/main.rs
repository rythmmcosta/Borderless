mod agent;
mod api;
mod clipboard;
mod config;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name    = "borderless-agent",
    version = env!("CARGO_PKG_VERSION"),
    about   = "Headless Borderless agent — clipboard sync + KVM for Linux servers",
)]
struct Cli {
    /// Path to config file (default: /etc/borderless-agent/config.toml)
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run as daemon (default if no subcommand given)
    Run,

    /// Print the latest clipboard content from the Borderless server
    Paste,

    /// Push text to the Borderless clipboard (reads stdin if no argument given)
    Copy {
        /// Text to copy (optional; reads from stdin if omitted)
        text: Option<String>,
    },

    /// Show agent version and config path
    Status,

    /// Write the systemd service file and enable the agent on boot (requires root)
    Install,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "borderless_agent=info".into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.cmd.unwrap_or(Cmd::Run) {
        Cmd::Run     => cmd_run(cli.config).await,
        Cmd::Paste   => cmd_paste(cli.config).await,
        Cmd::Copy { text } => cmd_copy(cli.config, text).await,
        Cmd::Status  => cmd_status(cli.config),
        Cmd::Install => cmd_install(),
    }
}

// ── run ───────────────────────────────────────────────────────────────────────

async fn cmd_run(config_path: Option<PathBuf>) -> Result<()> {
    let cfg = config::load(config_path)?;
    tracing::info!(device = %cfg.agent.device_name, server = %cfg.agent.server_url, "Starting agent");
    agent::run(cfg).await
}

// ── paste ─────────────────────────────────────────────────────────────────────

async fn cmd_paste(config_path: Option<PathBuf>) -> Result<()> {
    let cfg = config::load(config_path)?;
    let mut api = api::Api::new(&cfg.agent.server_url);
    api.login(&cfg.auth.email, &cfg.auth.password).await
        .context("Authentication failed")?;

    let entries = api.clipboard_history(1).await?;
    match entries.into_iter().next().and_then(|e| e.content) {
        Some(text) => print!("{text}"),
        None       => eprintln!("No clipboard content available on server"),
    }
    Ok(())
}

// ── copy ──────────────────────────────────────────────────────────────────────

async fn cmd_copy(config_path: Option<PathBuf>, text: Option<String>) -> Result<()> {
    let cfg = config::load(config_path)?;

    // Read text from argument or stdin
    let content = match text {
        Some(t) => t,
        None => {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    if content.is_empty() {
        anyhow::bail!("Nothing to copy");
    }

    let mut api = api::Api::new(&cfg.agent.server_url);
    api.login(&cfg.auth.email, &cfg.auth.password).await
        .context("Authentication failed")?;

    // Use a placeholder device_id when copying from CLI (not running as daemon)
    let cached = config::default_path()
        .parent().map(|p| p.join("device_id"))
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| s.trim().parse::<uuid::Uuid>().ok());

    let device_id = cached.unwrap_or_else(uuid::Uuid::new_v4);
    api.sync_clipboard(device_id, &content, "text/plain").await?;

    // Also write to local clipboard
    if let Err(e) = clipboard::write(&content) {
        tracing::debug!("Local clipboard write skipped: {e}");
    }

    eprintln!("Copied {} characters to Borderless clipboard", content.len());
    Ok(())
}

// ── status ────────────────────────────────────────────────────────────────────

fn cmd_status(config_path: Option<PathBuf>) -> Result<()> {
    let path = config_path.unwrap_or_else(config::default_path);
    println!("borderless-agent v{}", env!("CARGO_PKG_VERSION"));
    println!("Config: {}", path.display());

    match config::load(Some(path)) {
        Ok(cfg) => {
            println!("Server:  {}", cfg.agent.server_url);
            println!("Device:  {}", cfg.agent.device_name);
            println!("Clip:    {}", cfg.features.clipboard_sync);
            println!("Input:   {}", cfg.features.input_inject);

            // Show cached device_id if present
            let id_path = dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("/var/lib"))
                .join("borderless-agent/device_id");
            if let Ok(id) = std::fs::read_to_string(id_path) {
                println!("ID:      {}", id.trim());
            }
        }
        Err(e) => println!("Config error: {e}"),
    }
    Ok(())
}

// ── install ───────────────────────────────────────────────────────────────────

fn cmd_install() -> Result<()> {
    let exe = std::env::current_exe()
        .context("Cannot determine current executable path")?
        .display().to_string();

    // Write systemd unit
    let unit = format!(
        r#"[Unit]
Description=Borderless Agent — headless KVM + clipboard sync
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart={exe} run
Restart=on-failure
RestartSec=10
StandardOutput=journal
StandardError=journal

# Allow uinput for keyboard/mouse injection
SupplementaryGroups=input

[Install]
WantedBy=multi-user.target
"#,
    );

    std::fs::create_dir_all("/etc/systemd/system")?;
    std::fs::write("/etc/systemd/system/borderless-agent.service", &unit)?;
    println!("Wrote /etc/systemd/system/borderless-agent.service");

    // Write default config if missing
    let cfg_dir = PathBuf::from("/etc/borderless-agent");
    std::fs::create_dir_all(&cfg_dir)?;
    let cfg_path = cfg_dir.join("config.toml");
    if !cfg_path.exists() {
        let hostname = std::fs::read_to_string("/etc/hostname")
            .unwrap_or_else(|_| "my-server".into())
            .trim().to_owned();
        let template = format!(
            r#"[agent]
server_url  = "https://borderless.myowncloud.tech/api"
device_name = "{hostname}"

[auth]
email    = "admin@example.com"
password = "change-me"

[features]
clipboard_sync     = true
input_inject       = true
poll_interval_secs = 3
"#,
        );
        std::fs::write(&cfg_path, template)?;
        println!("Wrote default config to {}", cfg_path.display());
        println!("⚠  Edit {} and fill in your credentials before starting!", cfg_path.display());
    }

    // Reload & enable
    for cmd in &[
        "systemctl daemon-reload",
        "systemctl enable borderless-agent",
    ] {
        let status = std::process::Command::new("sh")
            .args(["-c", cmd]).status()?;
        if status.success() {
            println!("✓ {cmd}");
        } else {
            eprintln!("✗ {cmd} failed");
        }
    }

    println!("\nStart now:  systemctl start borderless-agent");
    println!("View logs:  journalctl -u borderless-agent -f");
    Ok(())
}
