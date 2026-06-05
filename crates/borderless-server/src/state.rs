use std::sync::Arc;
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState { pub db: PgPool, pub redis: ConnectionManager, pub config: Arc<Config> }

impl AppState {
    pub async fn new(cfg: &Config) -> anyhow::Result<Self> {
        let db = PgPool::connect(&cfg.database_url).await?;
        tracing::info!("Connected to PostgreSQL");
        let redis_client = redis::Client::open(cfg.redis_url.as_str())?;
        let redis = ConnectionManager::new(redis_client).await?;
        tracing::info!("Connected to Redis");
        Ok(Self { db, redis, config: Arc::new(cfg.clone()) })
    }
}
