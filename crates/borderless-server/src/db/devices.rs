use sqlx::PgPool;
use uuid::Uuid;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Device {
    pub id:           Uuid,
    pub user_id:      Uuid,
    pub name:         String,
    pub platform:     String,
    pub os_version:   Option<String>,
    pub app_version:  Option<String>,
    pub fingerprint:  String,
    pub public_key:   String,
    pub is_online:    bool,
    pub is_locked:    bool,
    pub last_seen_at: Option<DateTime<Utc>>,
}

pub async fn register(
    db: &PgPool, user_id: Uuid, name: &str, platform: &str,
    fingerprint: &str, public_key: &str,
    os_version: Option<&str>, app_version: Option<&str>,
) -> Result<Device, sqlx::Error> {
    sqlx::query_as::<_, Device>(
        "INSERT INTO devices (user_id, name, platform, fingerprint, public_key, os_version, app_version) \
         VALUES ($1, $2, $3::device_platform, $4, $5, $6, $7) \
         ON CONFLICT (fingerprint) DO UPDATE SET \
             name = EXCLUDED.name, app_version = EXCLUDED.app_version, last_seen_at = NOW() \
         RETURNING id, user_id, name, platform::TEXT as platform, os_version, app_version, \
                   fingerprint, public_key, is_online, is_locked, last_seen_at",
    ).bind(user_id).bind(name).bind(platform).bind(fingerprint).bind(public_key)
     .bind(os_version).bind(app_version)
     .fetch_one(db).await
}

pub async fn list_for_user(db: &PgPool, user_id: Uuid) -> Result<Vec<Device>, sqlx::Error> {
    sqlx::query_as::<_, Device>(
        "SELECT id, user_id, name, platform::TEXT as platform, os_version, app_version, \
                fingerprint, public_key, is_online, is_locked, last_seen_at \
         FROM devices WHERE user_id = $1 ORDER BY last_seen_at DESC NULLS LAST",
    ).bind(user_id).fetch_all(db).await
}

pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<Option<Device>, sqlx::Error> {
    sqlx::query_as::<_, Device>(
        "SELECT id, user_id, name, platform::TEXT as platform, os_version, app_version, \
                fingerprint, public_key, is_online, is_locked, last_seen_at \
         FROM devices WHERE id = $1",
    ).bind(id).fetch_optional(db).await
}

pub async fn delete(db: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("DELETE FROM devices WHERE id = $1 AND user_id = $2")
        .bind(id).bind(user_id).execute(db).await?;
    Ok(r.rows_affected() > 0)
}

#[allow(dead_code)]
pub async fn set_online(db: &PgPool, device_id: Uuid, online: bool, ip: Option<String>) {
    let _ = sqlx::query(
        "UPDATE devices SET is_online = $1, last_seen_at = NOW(), last_ip = $2::INET WHERE id = $3",
    ).bind(online).bind(ip).bind(device_id).execute(db).await;
}
