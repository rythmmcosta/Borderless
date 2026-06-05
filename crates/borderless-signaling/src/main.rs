//! WebRTC Signaling Server.
//!
//! Relays SDP offer/answer and ICE candidates between two devices
//! so they can establish a direct WebRTC data channel.
//!
//! 1. Device connects via WebSocket (authenticated with device_token JWT)
//! 2. Device A sends { to: device_B_id, payload: <SDP offer> }
//! 3. Signaling server forwards to Device B's WebSocket
//! 4. After ICE negotiation, devices communicate P2P — server steps aside

use axum::{
    extract::{ ws::{Message, WebSocket, WebSocketUpgrade}, State },
    response::Response, routing::get, Router,
};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState { peers: Arc<RwLock<HashMap<Uuid, mpsc::Sender<String>>>> }

#[derive(Debug, Serialize, Deserialize)]
struct SignalingMessage { to: Uuid, payload: serde_json::Value, #[serde(default)] from: Option<Uuid> }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let state = AppState { peers: Arc::new(RwLock::new(HashMap::new())) };
    let port  = std::env::var("PORT").unwrap_or_else(|_| "8081".into());
    let app   = Router::new()
        .route("/signal", get(ws_handler))
        .route("/health", get(|| async { "ok" }))
        .with_state(state);
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!(%addr, "Signaling server started");
    axum::serve(tokio::net::TcpListener::bind(&addr).await?, app).await?;
    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::channel::<String>(64);
    let device_id = Uuid::new_v4(); // TODO: parse JWT from first message
    state.peers.write().await.insert(device_id, tx);
    tracing::info!(%device_id, "Peer connected to signaling");
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() { break; }
        }
    });
    while let Some(Ok(Message::Text(text))) = receiver.next().await {
        match serde_json::from_str::<SignalingMessage>(&text) {
            Ok(mut msg) => {
                msg.from = Some(device_id);
                let peers = state.peers.read().await;
                if let Some(dest_tx) = peers.get(&msg.to) {
                    let _ = dest_tx.try_send(serde_json::to_string(&msg).unwrap());
                } else { tracing::warn!(to = %msg.to, "Target device not connected"); }
            }
            Err(e) => tracing::warn!("Bad signaling message: {}", e),
        }
    }
    state.peers.write().await.remove(&device_id);
    send_task.abort();
    tracing::info!(%device_id, "Peer disconnected from signaling");
}
