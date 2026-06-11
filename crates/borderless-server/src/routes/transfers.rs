use axum::{Router, Json, extract::{State, Path, Query}, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{db, state::AppState, error::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/",    get(list_transfers).post(initiate_transfer))
        .route("/:id", get(get_transfer))
}

#[derive(Deserialize)] struct Pagination { limit: Option<i64>, offset: Option<i64> }

#[derive(Deserialize)]
struct InitiateReq {
    sender_device_id: Uuid,
    receiver_device_id: Uuid,
    file_name: String,
    file_size: i64,
    mime_type: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct TransferInfo {
    id: Uuid, sender_device_id: Uuid, receiver_device_id: Uuid,
    file_name: String, file_size: i64, mime_type: Option<String>,
    status: String, bytes_transferred: i64, transfer_method: String,
    created_at: chrono::DateTime<chrono::Utc>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn list_transfers(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(p): Query<Pagination>,
) -> Result<Json<Vec<TransferInfo>>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let rows = sqlx::query_as::<_, TransferInfo>(
        "SELECT ft.id, ft.sender_device_id, ft.receiver_device_id, ft.file_name, ft.file_size, \
                ft.mime_type, ft.status::TEXT as status, ft.bytes_transferred, ft.transfer_method, \
                ft.created_at, ft.completed_at \
         FROM file_transfers ft \
         WHERE EXISTS ( \
             SELECT 1 FROM devices d WHERE d.user_id = $1 \
             AND (d.id = ft.sender_device_id OR d.id = ft.receiver_device_id) \
         ) \
         ORDER BY ft.created_at DESC LIMIT $2 OFFSET $3",
    ).bind(claims.sub).bind(p.limit.unwrap_or(20)).bind(p.offset.unwrap_or(0))
    .fetch_all(&state.db).await?;
    Ok(Json(rows))
}

async fn get_transfer(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<TransferInfo>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let row = sqlx::query_as::<_, TransferInfo>(
        "SELECT ft.id, ft.sender_device_id, ft.receiver_device_id, ft.file_name, ft.file_size, \
                ft.mime_type, ft.status::TEXT as status, ft.bytes_transferred, ft.transfer_method, \
                ft.created_at, ft.completed_at \
         FROM file_transfers ft \
         WHERE ft.id = $1 AND EXISTS ( \
             SELECT 1 FROM devices d WHERE d.user_id = $2 \
             AND (d.id = ft.sender_device_id OR d.id = ft.receiver_device_id) \
         )",
    ).bind(id).bind(claims.sub).fetch_optional(&state.db).await?
    .ok_or(ApiError::NotFound)?;
    Ok(Json(row))
}

async fn initiate_transfer(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<InitiateReq>,
) -> Result<(StatusCode, Json<TransferInfo>), ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let sender = db::devices::get_by_id(&state.db, req.sender_device_id).await?.ok_or(ApiError::NotFound)?;
    if sender.user_id != claims.sub { return Err(ApiError::Forbidden); }
    let row = sqlx::query_as::<_, TransferInfo>(
        "INSERT INTO file_transfers (sender_device_id, receiver_device_id, file_name, file_size, mime_type) \
         VALUES ($1, $2, $3, $4, $5) \
         RETURNING id, sender_device_id, receiver_device_id, file_name, file_size, mime_type, \
                   status::TEXT as status, bytes_transferred, transfer_method, created_at, completed_at",
    ).bind(req.sender_device_id).bind(req.receiver_device_id)
     .bind(&req.file_name).bind(req.file_size).bind(req.mime_type.as_deref())
     .fetch_one(&state.db).await?;
    Ok((StatusCode::CREATED, Json(row)))
}
