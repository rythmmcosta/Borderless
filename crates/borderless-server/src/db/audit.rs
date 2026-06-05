use sqlx::PgPool;
use uuid::Uuid;
pub async fn log(db: &PgPool, actor_id: Option<Uuid>, device_id: Option<Uuid>, action: &str, resource_type: &str, resource_id: Option<Uuid>, metadata: serde_json::Value) {
    if let Err(e) = sqlx::query!(
        "INSERT INTO audit_logs (actor_id, actor_device_id, action, resource_type, resource_id, metadata) VALUES ($1, $2, $3, $4, $5, $6)",
        actor_id, device_id, action, resource_type, resource_id, metadata
    ).execute(db).await { tracing::warn!("Audit log write failed: {}", e); }
}
