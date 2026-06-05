use axum::{Router, routing::get};
use crate::state::AppState;
pub fn routes() -> Router<AppState> { Router::new().route("/", get(|| async { "TODO: auth" })) }
