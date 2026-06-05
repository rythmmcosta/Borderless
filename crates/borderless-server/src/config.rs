use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host:                  String,
    pub port:                  u16,
    pub database_url:          String,
    pub redis_url:             String,
    pub jwt_secret:            String,
    pub jwt_expiry_secs:       u64,
    pub refresh_expiry_secs:   u64,
    pub admin_email:           String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            host:               std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port:               std::env::var("PORT").unwrap_or_else(|_| "8080".into()).parse()?,
            database_url:       std::env::var("DATABASE_URL")?,
            redis_url:          std::env::var("REDIS_URL")?,
            jwt_secret:         std::env::var("JWT_SECRET")?,
            jwt_expiry_secs:    std::env::var("JWT_EXPIRY_SECS").unwrap_or_else(|_| "900".into()).parse()?,
            refresh_expiry_secs: std::env::var("REFRESH_EXPIRY_SECS").unwrap_or_else(|_| "2592000".into()).parse()?,
            admin_email:        std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@borderless.local".into()),
        })
    }
}
