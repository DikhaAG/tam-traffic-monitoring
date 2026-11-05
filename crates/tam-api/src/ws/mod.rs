pub mod agent_hub;
pub mod dashboard_hub;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::sync::Arc;
use crate::app_state::AppState;
use tam_agent::ws_client::AgentMessage;

#[derive(Deserialize)]
pub struct AgentWsQuery {
    pub token: Option<String>,
    pub node_id: Option<String>,
}

pub async fn agent_ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<AgentWsQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let expected_token = state.agent_token.clone();
    let node_id = query.node_id.unwrap_or_else(|| "unknown-node".to_string());

    if query.token.as_deref() != Some(&expected_token) {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    }

    ws.on_upgrade(move |socket| handle_agent_socket(socket, state, node_id))
}

async fn handle_agent_socket(socket: WebSocket, state: Arc<AppState>, node_id: String) {
    let (mut sender, mut receiver) = socket.split();
    let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel(20);

    state.agent_hub.register_agent(&node_id, cmd_tx).await;
    tracing::info!("Server node '{}' registered on WebSocket hub", node_id);

    let mut send_task = tokio::spawn(async move {
        while let Some(cmd) = cmd_rx.recv().await {
            let server_msg = tam_agent::ws_client::ServerCommandMessage::ExecuteCommand(cmd);
            if let Ok(json) = serde_json::to_string(&server_msg) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    let state_clone = state.clone();
    let node_id_clone = node_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(msg) = serde_json::from_str::<AgentMessage>(&text) {
                match msg {
                    AgentMessage::Heartbeat(node) => {
                        let _ = state_clone.db.upsert_server_node(&node).await;
                        state_clone.dashboard_hub.broadcast(&text);
                    }
                    AgentMessage::Pm2Metrics(procs) => {
                        let _ = state_clone.db.save_pm2_snapshot(&node_id_clone, &procs).await;
                        state_clone.dashboard_hub.broadcast(&text);
                    }
                    AgentMessage::CommandResponse { id, status, output } => {
                        tracing::info!("Received command response for {}: {} - {}", id, status, output);
                        state_clone.dashboard_hub.broadcast(&text);
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {},
        _ = (&mut recv_task) => {},
    }

    state.agent_hub.unregister_agent(&node_id).await;
    tracing::info!("Server node '{}' disconnected from WebSocket hub", node_id);
}

pub async fn dashboard_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_dashboard_socket(socket, state))
}

async fn handle_dashboard_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.dashboard_hub.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Ping(payload))) = receiver.next().await {
            tracing::trace!("Received ping with payload len: {}", payload.len());
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {},
        _ = (&mut recv_task) => {},
    }
}

