use axum::{Router, Json, extract::{State, Query}, http::StatusCode, routing::{get, post}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{db, state::AppState, error::ApiError};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_history).post(sync_entry))
}

#[derive(Deserialize)]
struct Pagination { limit: Option<i64>, offset: Option<i64> }

#[derive(Deserialize)]
struct SyncReq {
    content_type: String,
    content:      String,
    source_device_id: Option<Uuid>,
}

// DB row — includes raw bytes stored in content_encrypted
#[derive(sqlx::FromRow)]
struct HistoryRow {
    id:               Uuid,
    content_type:     String,
    content_hash:     String,
    content_size:     i32,
    content_encrypted: Option<Vec<u8>>,
    created_at:       chrono::DateTime<chrono::Utc>,
    source_device_id: Option<Uuid>,
}

// API response — decoded text included for text/* entries
#[derive(Debug, Serialize)]
struct HistoryItem {
    id:               Uuid,
    content_type:     String,
    content_hash:     String,
    content_size:     i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    content:          Option<String>,
    created_at:       chrono::DateTime<chrono::Utc>,
    source_device_id: Option<Uuid>,
}

fn row_to_item(r: HistoryRow) -> HistoryItem {
    let content = r.content_encrypted
        .filter(|_| r.content_type.starts_with("text/"))
        .and_then(|b| String::from_utf8(b).ok());
    HistoryItem {
        id: r.id, content_type: r.content_type, content_hash: r.content_hash,
        content_size: r.content_size, content, created_at: r.created_at,
        source_device_id: r.source_device_id,
    }
}

async fn list_history(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(p): Query<Pagination>,
) -> Result<Json<Vec<HistoryItem>>, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    let rows = sqlx::query_as::<_, HistoryRow>(
        "SELECT id, content_type, content_hash, content_size, content_encrypted, \
         created_at, source_device_id \
         FROM clipboard_history WHERE user_id = $1 AND expires_at > NOW() \
         ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    ).bind(claims.sub).bind(p.limit.unwrap_or(50)).bind(p.offset.unwrap_or(0))
    .fetch_all(&state.db).await?;
    Ok(Json(rows.into_iter().map(row_to_item).collect()))
}

async fn sync_entry(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<SyncReq>,
) -> Result<StatusCode, ApiError> {
    let claims = crate::middleware::auth::bearer_claims(&headers, &state.config.jwt_secret)?;
    if let Some(dev_id) = req.source_device_id {
        let dev = db::devices::get_by_id(&state.db, dev_id).await?.ok_or(ApiError::NotFound)?;
        if dev.user_id != claims.sub { return Err(ApiError::Forbidden); }
    }
    use sha2::Digest;
    let hash  = format!("{:x}", sha2::Sha256::digest(req.content.as_bytes()));
    let size  = req.content.len() as i32;
    // Store raw bytes for text content so agents can retrieve the actual text
    let bytes: Option<Vec<u8>> = if req.content_type.starts_with("text/") && size <= 65536 {
        Some(req.content.as_bytes().to_vec())
    } else {
        None
    };
    sqlx::query(
        "INSERT INTO clipboard_history \
         (user_id, source_device_id, content_type, content_hash, content_size, content_encrypted) \
         VALUES ($1, $2, $3, $4, $5, $6)",
    ).bind(claims.sub).bind(req.source_device_id).bind(&req.content_type)
     .bind(&hash).bind(size).bind(bytes).execute(&state.db).await?;
    // Pub/Sub push for real-time connected clients
    let mut redis = state.redis.clone();
    let payload = serde_json::json!({
        "user_id": claims.sub, "content_type": req.content_type, "content": req.content
    });
    let _: () = redis::cmd("PUBLISH").arg("clipboard:new")
        .arg(payload.to_string()).query_async(&mut redis).await.unwrap_or(());
    Ok(StatusCode::CREATED)
}
