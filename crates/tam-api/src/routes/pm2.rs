use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use tam_core::models::pm2::{Pm2Command, Pm2ProcessMetrics};
use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct ProcessActionPayload {
    pub action: String, // restart, reload, stop, clear_logs
    pub issued_by: Option<String>,
}

#[derive(Serialize)]
pub struct ActionResponse {
    pub command_id: String,
    pub status: String,
    pub message: String,
}

pub async fn list_server_pm2(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Pm2ProcessMetrics>>, StatusCode> {
    let procs = state
        .db
        .get_latest_pm2_processes(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(procs))
}

pub async fn dispatch_pm2_action(
    State(state): State<Arc<AppState>>,
    Path((node_id, process_name)): Path<(String, String)>,
    Json(payload): Json<ProcessActionPayload>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let cmd_id = Uuid::new_v4().to_string();
    let cmd = Pm2Command {
        id: cmd_id.clone(),
        node_id: node_id.clone(),
        target_name: process_name.clone(),
        action: payload.action.clone(),
        issued_by: payload.issued_by.unwrap_or_else(|| "dashboard_user".to_string()),
        timestamp: Utc::now(),
        status: "dispatched".to_string(),
        output: None,
    };

    // 1. Audit log
    let _ = state.db.record_audit_log(&cmd).await;

    // 2. Dispatch to connected agent via WebSocket hub
    let dispatch_res = state.agent_hub.dispatch_command(&node_id, cmd).await;

    match dispatch_res {
        Ok(_) => Ok(Json(ActionResponse {
            command_id: cmd_id,
            status: "success".to_string(),
            message: format!("Command '{}' dispatched to agent for process '{}'", payload.action, process_name),
        })),
        Err(e) => Ok(Json(ActionResponse {
            command_id: cmd_id,
            status: "queued".to_string(),
            message: format!("Agent not currently online: {}", e),
        })),
    }
}

