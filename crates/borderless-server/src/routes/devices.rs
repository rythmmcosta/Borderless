use axum::{Router, Json, extract::{State, Path}, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{db, state::AppState, error::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/",    get(list_devices).post(register_device))
        .route("/:id", get(get_device).delete(delete_device))
}

#[derive(Deserialize)]
struct RegisterReq {
    name: String, platform: String, fingerprint: String, public_key: String,
    os_version: Option<String>, app_version: Option<String>,
}

#[derive(Serialize)]
struct DeviceResponse {
    id: Uuid, name: String, platform: String, os_version: Option<String>,
    app_version: Option<String>, fingerprint: String,
    is_online: bool, is_locked: bool, last_seen_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<db::devices::Device> for DeviceResponse {
    fn from(d: db::devices::Device) -> Self {
        Self { id: d.id, name: d.name, platform: d.platform, os_version: d.os_version,
               app_version: d.app_version, fingerprint: d.fingerprint,
               is_online: d.is_online, is_locked: d.is_locked, last_seen_at: d.last_seen_at }
    }
}

async fn list_devices(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<DeviceResponse>>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let devices = db::devices::list_for_user(&state.db, claims.sub).await?;
    Ok(Json(devices.into_iter().map(DeviceResponse::from).collect()))
}

async fn register_device(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RegisterReq>,
) -> Result<(StatusCode, Json<DeviceResponse>), ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let device = db::devices::register(
        &state.db, claims.sub, &req.name, &req.platform,
        &req.fingerprint, &req.public_key,
        req.os_version.as_deref(), req.app_version.as_deref(),
    ).await?;
    Ok((StatusCode::CREATED, Json(DeviceResponse::from(device))))
}

async fn get_device(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<DeviceResponse>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let device = db::devices::get_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;
    if device.user_id != claims.sub && claims.role != "admin" && claims.role != "super_admin" {
        return Err(ApiError::Forbidden);
    }
    Ok(Json(DeviceResponse::from(device)))
}

async fn delete_device(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    if !db::devices::delete(&state.db, id, claims.sub).await? {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
