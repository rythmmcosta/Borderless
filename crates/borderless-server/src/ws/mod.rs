use axum::Router;
use crate::state::AppState;
pub fn routes() -> Router<AppState> {
    Router::new()
    // TODO: /signal — WebRTC signaling proxy
    // TODO: /events — SSE / WebSocket for real-time dashboard
}
