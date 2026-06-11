// Admin-only routes. All protected by require_admin middleware.

use axum::{
    Router, Extension, Json,
    routing::{get, post},
    extract::{State, Path, Query},
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{state::AppState, middleware::auth::AuthUser, error::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users",              get(list_all_users))
        .route("/users/:id",          get(get_user_detail))
        .route("/devices",            get(list_all_devices))
        .route("/devices/:id",        get(get_device_detail))
        .route("/devices/:id/lock",   post(lock_device))
        .route("/devices/:id/unlock", post(unlock_device))
        .route("/devices/:id/remote", post(init_remote_session))
        .route("/sessions",           get(list_all_sessions))
        .route("/sessions/:id",       post(terminate_session))
        .route("/audit",              get(get_audit_log))
        .route("/stats",              get(get_stats))
        .route_layer(axum::middleware::from_fn(crate::middleware::auth::require_admin))
}

async fn list_all_users(State(state): State<AppState>, Query(params): Query<PaginationParams>) -> Result<Json<PaginatedResponse<UserSummary>>, ApiError> {
    let offset = params.offset.unwrap_or(0); let limit = params.limit.unwrap_or(50).min(200);
    let users = sqlx::query_as!(UserSummary,
        r#"SELECT id, email, display_name, role, created_at, last_seen_at,
                  (SELECT COUNT(*) FROM devices WHERE user_id = u.id) as device_count
           FROM users u ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
        limit as i64, offset as i64).fetch_all(&state.db).await?;
    let total: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users").fetch_one(&state.db).await?.unwrap_or(0);
    Ok(Json(PaginatedResponse { items: users, total, offset, limit }))
}

async fn get_user_detail(State(state): State<AppState>, Path(user_id): Path<Uuid>) -> Result<Json<UserDetail>, ApiError> {
    let user = sqlx::query_as!(UserDetail,
        "SELECT id, email, display_name, role, mfa_enabled, created_at, last_seen_at FROM users WHERE id = $1",
        user_id).fetch_optional(&state.db).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(user))
}

async fn list_all_devices(State(state): State<AppState>, Query(params): Query<DeviceFilterParams>) -> Result<Json<Vec<DeviceSummary>>, ApiError> {
    let devices = sqlx::query_as!(DeviceSummary,
        r#"SELECT d.id, d.name, d.platform, d.os_version, d.app_version,
                  d.is_online, d.is_locked, d.last_ip::TEXT as last_ip, d.last_seen_at,
                  u.email as user_email, u.display_name as user_name
           FROM devices d JOIN users u ON u.id = d.user_id
           WHERE ($1::BOOL IS NULL OR d.is_online = $1) ORDER BY d.last_seen_at DESC NULLS LAST"#,
        params.online_only).fetch_all(&state.db).await?;
    Ok(Json(devices))
}

async fn get_device_detail(State(state): State<AppState>, Path(device_id): Path<Uuid>) -> Result<Json<DeviceDetail>, ApiError> {
    let device = sqlx::query_as!(DeviceDetail,
        r#"SELECT d.*, u.email as user_email FROM devices d JOIN users u ON u.id = d.user_id WHERE d.id = $1"#,
        device_id).fetch_optional(&state.db).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(device))
}

#[derive(Debug, Deserialize)]
pub struct LockRequest { pub reason: Option<String> }

async fn lock_device(State(state): State<AppState>, Extension(admin): Extension<AuthUser>, Path(device_id): Path<Uuid>, Json(req): Json<LockRequest>) -> Result<StatusCode, ApiError> {
    sqlx::query!("UPDATE devices SET device_token = gen_random_uuid()::TEXT WHERE id = $1", device_id).execute(&state.db).await?;
    let mut redis = state.redis.clone();
    let _: () = redis::cmd("PUBLISH").arg("device:lock").arg(device_id.to_string()).query_async(&mut redis).await.unwrap_or(());
    audit(&state, admin.id, "device.lock", device_id, serde_json::json!({ "reason": req.reason })).await;
    Ok(StatusCode::NO_CONTENT)
}

async fn unlock_device(State(state): State<AppState>, Extension(admin): Extension<AuthUser>, Path(device_id): Path<Uuid>) -> Result<StatusCode, ApiError> {
    sqlx::query!("UPDATE devices SET device_token = gen_random_uuid()::TEXT WHERE id = $1", device_id).execute(&state.db).await?;
    audit(&state, admin.id, "device.unlock", device_id, serde_json::json!({})).await;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteSessionRequest { pub session_type: String }

async fn init_remote_session(State(state): State<AppState>, Extension(admin): Extension<AuthUser>, Path(device_id): Path<Uuid>, Json(req): Json<RemoteSessionRequest>) -> Result<Json<serde_json::Value>, ApiError> {
    let session_id = Uuid::new_v4();
    let mut redis = state.redis.clone();
    let payload = serde_json::json!({ "session_id": session_id, "admin_id": admin.id, "session_type": req.session_type, "initiated_at": chrono::Utc::now() });
    let _: () = redis::cmd("PUBLISH").arg(format!("device:remote:{}", device_id)).arg(payload.to_string()).query_async(&mut redis).await.unwrap_or(());
    audit(&state, admin.id, "device.remote_session.init", device_id, serde_json::json!({ "session_type": req.session_type })).await;
    Ok(Json(serde_json::json!({ "session_id": session_id })))
}

async fn list_all_sessions(State(state): State<AppState>, Query(params): Query<PaginationParams>) -> Result<Json<Vec<SessionSummary>>, ApiError> {
    let sessions = sqlx::query_as!(SessionSummary,
        r#"SELECT s.id, s.session_type, s.transport, s.started_at, s.ended_at, s.bytes_sent, s.bytes_recv,
                  sd.name as source_name, td.name as target_name
           FROM sessions s JOIN devices sd ON sd.id = s.source_device_id JOIN devices td ON td.id = s.target_device_id
           ORDER BY s.started_at DESC LIMIT $1 OFFSET $2"#,
        params.limit.unwrap_or(50) as i64, params.offset.unwrap_or(0) as i64).fetch_all(&state.db).await?;
    Ok(Json(sessions))
}

async fn terminate_session(State(state): State<AppState>, Extension(admin): Extension<AuthUser>, Path(session_id): Path<Uuid>) -> Result<StatusCode, ApiError> {
    let mut redis = state.redis.clone();
    let _: () = redis::cmd("PUBLISH").arg("session:terminate").arg(session_id.to_string()).query_async(&mut redis).await.unwrap_or(());
    audit(&state, admin.id, "session.terminate", session_id, serde_json::json!({})).await;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub struct AuditFilterParams { pub offset: Option<i64>, pub limit: Option<i64>, pub action: Option<String>, pub actor_id: Option<Uuid>, pub from: Option<chrono::DateTime<chrono::Utc>>, pub to: Option<chrono::DateTime<chrono::Utc>> }

async fn get_audit_log(State(state): State<AppState>, Query(params): Query<AuditFilterParams>) -> Result<Json<Vec<AuditEntry>>, ApiError> {
    let entries = sqlx::query_as!(AuditEntry,
        r#"SELECT al.id, al.actor_id, al.action, al.resource_type, al.resource_id, al.ip_address::TEXT as ip_address, al.metadata, al.created_at, u.email as actor_email
           FROM audit_logs al LEFT JOIN users u ON u.id = al.actor_id
           WHERE ($1::TEXT IS NULL OR al.action LIKE $1) AND ($2::UUID IS NULL OR al.actor_id = $2)
             AND ($3::TIMESTAMPTZ IS NULL OR al.created_at >= $3) AND ($4::TIMESTAMPTZ IS NULL OR al.created_at <= $4)
           ORDER BY al.created_at DESC LIMIT $5 OFFSET $6"#,
        params.action.as_deref().map(|a| format!("{}%", a)), params.actor_id, params.from, params.to,
        params.limit.unwrap_or(100), params.offset.unwrap_or(0)).fetch_all(&state.db).await?;
    Ok(Json(entries))
}

async fn get_stats(State(state): State<AppState>) -> Result<Json<AdminStats>, ApiError> {
    let (total_users, online_devices, total_devices, sessions_today) = tokio::try_join!(
        sqlx::query_scalar!("SELECT COUNT(*) FROM users").fetch_one(&state.db),
        sqlx::query_scalar!("SELECT COUNT(*) FROM devices WHERE is_online = true").fetch_one(&state.db),
        sqlx::query_scalar!("SELECT COUNT(*) FROM devices").fetch_one(&state.db),
        sqlx::query_scalar!("SELECT COUNT(*) FROM sessions WHERE started_at > NOW() - INTERVAL '24 hours'").fetch_one(&state.db),
    )?;
    Ok(Json(AdminStats { total_users: total_users.unwrap_or(0), total_devices: total_devices.unwrap_or(0), online_devices: online_devices.unwrap_or(0), sessions_today: sessions_today.unwrap_or(0) }))
}

async fn audit(state: &AppState, actor_id: Uuid, action: &str, resource_id: Uuid, meta: serde_json::Value) {
    let _ = sqlx::query!("INSERT INTO audit_logs (actor_id, action, resource_type, resource_id, metadata) VALUES ($1, $2, 'admin', $3, $4)", actor_id, action, resource_id, meta).execute(&state.db).await;
}

#[derive(Debug, Deserialize)] pub struct PaginationParams { pub offset: Option<i64>, pub limit: Option<i64> }
#[derive(Debug, Deserialize)] pub struct DeviceFilterParams { pub online_only: Option<bool> }

#[derive(Debug, Serialize, sqlx::FromRow)] pub struct UserSummary { pub id: Uuid, pub email: String, pub display_name: Option<String>, pub role: String, pub created_at: chrono::DateTime<chrono::Utc>, pub last_seen_at: Option<chrono::DateTime<chrono::Utc>>, pub device_count: Option<i64> }
#[derive(Debug, Serialize, sqlx::FromRow)] pub struct UserDetail { pub id: Uuid, pub email: String, pub display_name: Option<String>, pub role: String, pub mfa_enabled: bool, pub created_at: chrono::DateTime<chrono::Utc>, pub last_seen_at: Option<chrono::DateTime<chrono::Utc>> }
#[derive(Debug, Serialize, sqlx::FromRow)] pub struct DeviceSummary { pub id: Uuid, pub name: String, pub platform: String, pub os_version: Option<String>, pub app_version: Option<String>, pub is_online: bool, pub is_locked: bool, pub last_ip: Option<String>, pub last_seen_at: Option<chrono::DateTime<chrono::Utc>>, pub user_email: String, pub user_name: Option<String> }
#[derive(Debug, Serialize, sqlx::FromRow)] pub struct DeviceDetail { pub id: Uuid, pub name: String, pub platform: String, pub is_online: bool, pub user_email: String }
#[derive(Debug, Serialize, sqlx::FromRow)] pub struct SessionSummary { pub id: Uuid, pub session_type: String, pub transport: String, pub started_at: chrono::DateTime<chrono::Utc>, pub ended_at: Option<chrono::DateTime<chrono::Utc>>, pub bytes_sent: i64, pub bytes_recv: i64, pub source_name: String, pub target_name: String }
#[derive(Debug, Serialize, sqlx::FromRow)] pub struct AuditEntry { pub id: Uuid, pub actor_id: Option<Uuid>, pub actor_email: Option<String>, pub action: String, pub resource_type: Option<String>, pub resource_id: Option<Uuid>, pub ip_address: Option<String>, pub metadata: serde_json::Value, pub created_at: chrono::DateTime<chrono::Utc> }
#[derive(Debug, Serialize)] pub struct AdminStats { pub total_users: i64, pub total_devices: i64, pub online_devices: i64, pub sessions_today: i64 }
#[derive(Debug, Serialize)] pub struct PaginatedResponse<T> { pub items: Vec<T>, pub total: i64, pub offset: i64, pub limit: i64 }
