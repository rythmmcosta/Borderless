use sqlx::PgPool;
use uuid::Uuid;
pub async fn set_online(db: &PgPool, device_id: Uuid, online: bool, ip: Option<String>) {
    let _ = sqlx::query!("UPDATE devices SET is_online=$1, last_seen_at=NOW(), last_ip=$2::INET WHERE id=$3", online, ip, device_id).execute(db).await;
}
