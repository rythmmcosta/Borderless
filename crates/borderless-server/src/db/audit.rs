use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
pub async fn log(db: &PgPool, actor_id: Option<Uuid>, device_id: Option<Uuid>, action: &str, resource_type: &str, resource_id: Option<Uuid>, metadata: serde_json::Value) {
    if let Err(e) = sqlx::query(
        "INSERT INTO audit_logs (actor_id, actor_device_id, action, resource_type, resource_id, metadata) \
         VALUES ($1, $2, $3, $4, $5, $6)",
    ).bind(actor_id).bind(device_id).bind(action).bind(resource_type).bind(resource_id).bind(metadata)
     .execute(db).await
    {
        tracing::warn!("Audit log write failed: {}", e);
    }
}
