use anyhow::{bail, Context, Result};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Api {
    client: Client,
    base:   String,
    pub token: Option<String>,
}

// ── Auth ─────────────────────────────────────────────────────────────────────

#[derive(Serialize)] struct LoginReq<'a> { email: &'a str, password: &'a str }
#[derive(Deserialize)] struct LoginResp { access_token: String }

// ── Devices ───────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct RegisterReq<'a> {
    name:        &'a str,
    platform:    &'static str,
    fingerprint: &'a str,
    public_key:  &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version:  Option<String>,
    app_version: Option<&'static str>,
}

#[derive(Deserialize)]
pub struct DeviceResp { pub id: Uuid, pub name: String }

// ── Clipboard ─────────────────────────────────────────────────────────────────

#[derive(Deserialize, Debug)]
pub struct ClipboardEntry {
    pub id:               Uuid,
    pub content_type:     String,
    pub content_hash:     String,
    pub content:          Option<String>,
    pub created_at:       chrono::DateTime<chrono::Utc>,
    pub source_device_id: Option<Uuid>,
}

#[derive(Serialize)]
struct ClipboardSyncReq<'a> {
    content_type:     &'a str,
    content:          &'a str,
    source_device_id: Option<Uuid>,
}

// ── Implementation ────────────────────────────────────────────────────────────

impl Api {
    pub fn new(base: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect("reqwest client");
        Self { client, base: base.trim_end_matches('/').to_owned(), token: None }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/v1{}", self.base, path)
    }

    fn bearer(&self) -> Result<String> {
        self.token.as_ref()
            .map(|t| format!("Bearer {t}"))
            .context("Not authenticated — call login() first")
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<()> {
        let r: LoginResp = self.client
            .post(self.url("/auth/login"))
            .json(&LoginReq { email, password })
            .send().await?
            .error_for_status()?
            .json().await?;
        self.token = Some(r.access_token);
        Ok(())
    }

    /// Register device. Returns (device_id, already_existed).
    pub async fn register_device(
        &self, name: &str, fingerprint: &str, public_key: &str,
    ) -> Result<(Uuid, bool)> {
        let os = std::fs::read_to_string("/etc/os-release").ok()
            .and_then(|s| s.lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .map(|l| l.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_owned()));

        let resp = self.client
            .post(self.url("/devices/"))
            .header("Authorization", self.bearer()?)
            .json(&RegisterReq {
                name, platform: "linux", fingerprint, public_key,
                os_version: os, app_version: Some(env!("CARGO_PKG_VERSION")),
            })
            .send().await?;

        if resp.status() == StatusCode::CONFLICT {
            // Device already registered: find it by listing and matching fingerprint
            let devices: Vec<DeviceResp> = self.client
                .get(self.url("/devices/"))
                .header("Authorization", self.bearer()?)
                .send().await?.error_for_status()?.json().await?;
            if let Some(d) = devices.into_iter().find(|d| d.name == name) {
                return Ok((d.id, true));
            }
            bail!("Device conflict but could not find existing device");
        }

        let d: DeviceResp = resp.error_for_status()?.json().await?;
        Ok((d.id, false))
    }

    pub async fn clipboard_history(&self, limit: i64) -> Result<Vec<ClipboardEntry>> {
        let entries: Vec<ClipboardEntry> = self.client
            .get(self.url(&format!("/clipboard/?limit={limit}")))
            .header("Authorization", self.bearer()?)
            .send().await?.error_for_status()?.json().await?;
        Ok(entries)
    }

    pub async fn sync_clipboard(
        &self, device_id: Uuid, content: &str, content_type: &str,
    ) -> Result<()> {
        self.client
            .post(self.url("/clipboard/"))
            .header("Authorization", self.bearer()?)
            .json(&ClipboardSyncReq { content_type, content, source_device_id: Some(device_id) })
            .send().await?.error_for_status()?;
        Ok(())
    }
}
