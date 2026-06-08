use axum::{Router, Json, extract::{State, Path, Query}, http::StatusCode, routing::{get, patch, post}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{db, state::AppState, error::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/",          get(list_notifications).post(send_notification))
        .route("/:id/read",  patch(mark_read))
}

#[derive(Deserialize)] struct Pagination { limit: Option<i64>, offset: Option<i64> }

#[derive(Deserialize)]
struct SendReq {
    source_device_id: Uuid, target_device_id: Uuid,
    app_name: String, title: String,
    body: Option<String>, icon_url: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct NotifInfo {
    id: Uuid, source_device_id: Uuid, target_device_id: Uuid,
    app_name: String, title: String,
    body: Option<String>, icon_url: Option<String>,
    status: String, created_at: chrono::DateTime<chrono::Utc>,
}

async fn list_notifications(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(p): Query<Pagination>,
) -> Result<Json<Vec<NotifInfo>>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let rows = sqlx::query_as::<_, NotifInfo>(
        "SELECT n.id, n.source_device_id, n.target_device_id, n.app_name, n.title, n.body, \
                n.icon_url, n.status::TEXT as status, n.created_at \
         FROM notifications n \
         WHERE EXISTS ( \
             SELECT 1 FROM devices d WHERE d.user_id = $1 \
             AND (d.id = n.source_device_id OR d.id = n.target_device_id) \
         ) \
         ORDER BY n.created_at DESC LIMIT $2 OFFSET $3",
    ).bind(claims.sub).bind(p.limit.unwrap_or(20)).bind(p.offset.unwrap_or(0))
    .fetch_all(&state.db).await?;
    Ok(Json(rows))
}

async fn send_notification(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<SendReq>,
) -> Result<(StatusCode, Json<NotifInfo>), ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let dev = db::devices::get_by_id(&state.db, req.source_device_id).await?.ok_or(ApiError::NotFound)?;
    if dev.user_id != claims.sub { return Err(ApiError::Forbidden); }
    let row = sqlx::query_as::<_, NotifInfo>(
        "INSERT INTO notifications (source_device_id, target_device_id, app_name, title, body, icon_url) \
         VALUES ($1, $2, $3, $4, $5, $6) \
         RETURNING id, source_device_id, target_device_id, app_name, title, body, icon_url, \
                   status::TEXT as status, created_at",
    ).bind(req.source_device_id).bind(req.target_device_id)
     .bind(&req.app_name).bind(&req.title).bind(req.body.as_deref()).bind(req.icon_url.as_deref())
     .fetch_one(&state.db).await?;
    let mut redis = state.redis.clone();
    let payload = serde_json::json!({
        "notification_id": row.id,
        "target_device_id": req.target_device_id,
        "app_name": req.app_name, "title": req.title, "body": req.body,
    });
    let _: () = redis::cmd("PUBLISH")
        .arg(format!("notif:{}", req.target_device_id))
        .arg(payload.to_string()).query_async(&mut redis).await.unwrap_or(());
    Ok((StatusCode::CREATED, Json(row)))
}

async fn mark_read(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let r = sqlx::query(
        "UPDATE notifications SET status = 'dismissed'::notification_status \
         WHERE id = $1 AND EXISTS ( \
             SELECT 1 FROM devices d WHERE d.user_id = $2 AND d.id = notifications.target_device_id \
         )",
    ).bind(id).bind(claims.sub).execute(&state.db).await?;
    if r.rows_affected() == 0 { return Err(ApiError::NotFound); }
    Ok(StatusCode::NO_CONTENT)
}
