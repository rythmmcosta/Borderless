use sqlx::PgPool;
use uuid::Uuid;
pub async fn set_last_seen(db: &PgPool, user_id: Uuid) {
    let _ = sqlx::query!("UPDATE users SET last_seen_at = NOW() WHERE id = $1", user_id).execute(db).await;
}
