//! TURN-like relay server for WebRTC fallback.
//! Proxies encrypted packets between devices. Never decrypts — E2E maintained.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    tracing::info!("Borderless relay server v{}", env!("CARGO_PKG_VERSION"));
    // TODO: implement TURN-like relay. For now, use Coturn (see docker-compose.yml)
    Ok(())
}
