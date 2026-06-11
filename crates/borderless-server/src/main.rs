mod config;
mod db;
mod error;
mod middleware;
mod routes;
mod state;
mod ws;

use axum::Router;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "borderless_server=debug,tower_http=info,sqlx=warn".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg   = config::Config::from_env()?;
    let state = AppState::new(&cfg).await?;

    tracing::info!("Running migrations…");
    sqlx::migrate!("../../migrations").run(&state.db).await?;
    tracing::info!("Migrations complete");

    let allowed_origin = cfg.allowed_origin.clone();
    let cors = CorsLayer::new()
        .allow_origin(
            allowed_origin
                .parse::<axum::http::HeaderValue>()
                .unwrap_or_else(|_| "http://localhost:3000".parse().unwrap()),
        )
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
        ])
        .allow_headers([axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        .nest("/v1/health",        routes::health::routes())
        .nest("/v1/auth",          routes::auth::routes())
        .nest("/v1/devices",       routes::devices::routes())
        .nest("/v1/clipboard",     routes::clipboard::routes())
        .nest("/v1/transfers",     routes::transfers::routes())
        .nest("/v1/notifications", routes::notifications::routes())
        .nest("/v1/sessions",      routes::sessions::routes())
        .nest("/v1/admin",         routes::admin::routes())
        .nest("/v1/ws",            ws::routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .with_state(state);

    let addr = format!("{}:{}", cfg.host, cfg.port);
    tracing::info!(%addr, "Borderless API server started");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
