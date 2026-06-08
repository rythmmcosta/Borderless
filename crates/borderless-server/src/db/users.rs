use sqlx::PgPool;
use uuid::Uuid;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id:            Uuid,
    pub email:         String,
    pub display_name:  Option<String>,
    pub role:          String,
    pub password_hash: Option<String>,
    pub mfa_enabled:   bool,
    pub created_at:    DateTime<Utc>,
    pub last_seen_at:  Option<DateTime<Utc>>,
}

pub async fn create(
    db: &PgPool, email: &str, display_name: Option<&str>,
    password_hash: &str, role: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (email, display_name, password_hash, role) \
         VALUES ($1, $2, $3, $4::user_role) \
         RETURNING id, email, display_name, role::TEXT as role, \
                   password_hash, mfa_enabled, created_at, last_seen_at",
    ).bind(email).bind(display_name).bind(password_hash).bind(role)
    .fetch_one(db).await
}

pub async fn get_by_email(db: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, display_name, role::TEXT as role, \
                password_hash, mfa_enabled, created_at, last_seen_at \
         FROM users WHERE email = $1",
    ).bind(email).fetch_optional(db).await
}

pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, display_name, role::TEXT as role, \
                password_hash, mfa_enabled, created_at, last_seen_at \
         FROM users WHERE id = $1",
    ).bind(id).fetch_optional(db).await
}

pub async fn set_last_seen(db: &PgPool, user_id: Uuid) {
    let _ = sqlx::query("UPDATE users SET last_seen_at = NOW() WHERE id = $1")
        .bind(user_id).execute(db).await;
}

pub async fn store_refresh_token(
    db: &PgPool, user_id: Uuid, token: &str, expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    ).bind(user_id).bind(token).bind(expires_at)
    .execute(db).await.map(|_| ())
}

pub async fn verify_refresh_token(db: &PgPool, token: &str) -> Result<Option<Uuid>, sqlx::Error> {
    let row: Option<(Uuid,)> = sqlx::query_as::<_, (Uuid,)>(
        "DELETE FROM refresh_tokens \
         WHERE token_hash = $1 AND expires_at > NOW() AND revoked = false \
         RETURNING user_id",
    ).bind(token).fetch_optional(db).await?;
    Ok(row.map(|(uid,)| uid))
}

pub async fn revoke_refresh_token(db: &PgPool, token: &str) {
    let _ = sqlx::query("DELETE FROM refresh_tokens WHERE token_hash = $1")
        .bind(token).execute(db).await;
}
