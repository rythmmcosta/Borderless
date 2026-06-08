use axum::{Router, Json, extract::{State, Path, Query}, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{state::AppState, error::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/",    get(list_sessions))
        .route("/:id", get(get_session).delete(terminate_session))
}

#[derive(Deserialize)] struct Pagination { limit: Option<i64>, offset: Option<i64> }

#[derive(Debug, Serialize, sqlx::FromRow)]
struct SessionInfo {
    id: Uuid, session_type: String, transport: String,
    started_at: chrono::DateTime<chrono::Utc>,
    ended_at: Option<chrono::DateTime<chrono::Utc>>,
    bytes_sent: i64, bytes_recv: i64,
    source_device_id: Uuid, target_device_id: Uuid,
}

async fn list_sessions(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(p): Query<Pagination>,
) -> Result<Json<Vec<SessionInfo>>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let rows = sqlx::query_as::<_, SessionInfo>(
        "SELECT s.id, s.session_type::TEXT as session_type, s.transport::TEXT as transport, \
                s.started_at, s.ended_at, s.bytes_sent, s.bytes_recv, \
                s.source_device_id, s.target_device_id \
         FROM sessions s \
         WHERE EXISTS ( \
             SELECT 1 FROM devices d WHERE d.user_id = $1 \
             AND (d.id = s.source_device_id OR d.id = s.target_device_id) \
         ) \
         ORDER BY s.started_at DESC LIMIT $2 OFFSET $3",
    ).bind(claims.sub).bind(p.limit.unwrap_or(20)).bind(p.offset.unwrap_or(0))
    .fetch_all(&state.db).await?;
    Ok(Json(rows))
}

async fn get_session(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<SessionInfo>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let row = sqlx::query_as::<_, SessionInfo>(
        "SELECT s.id, s.session_type::TEXT as session_type, s.transport::TEXT as transport, \
                s.started_at, s.ended_at, s.bytes_sent, s.bytes_recv, \
                s.source_device_id, s.target_device_id \
         FROM sessions s \
         WHERE s.id = $1 AND EXISTS ( \
             SELECT 1 FROM devices d WHERE d.user_id = $2 \
             AND (d.id = s.source_device_id OR d.id = s.target_device_id) \
         )",
    ).bind(id).bind(claims.sub).fetch_optional(&state.db).await?
    .ok_or(ApiError::NotFound)?;
    Ok(Json(row))
}

async fn terminate_session(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let owned: bool = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS ( \
             SELECT 1 FROM sessions s \
             JOIN devices d ON d.id = s.source_device_id OR d.id = s.target_device_id \
             WHERE s.id = $1 AND d.user_id = $2 \
         )",
    ).bind(id).bind(claims.sub).fetch_one(&state.db).await?;
    if !owned { return Err(ApiError::NotFound); }
    let mut redis = state.redis.clone();
    let _: () = redis::cmd("PUBLISH").arg("session:terminate").arg(id.to_string())
        .query_async(&mut redis).await.unwrap_or(());
    Ok(StatusCode::NO_CONTENT)
}
