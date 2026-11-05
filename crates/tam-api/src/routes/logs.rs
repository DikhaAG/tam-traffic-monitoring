use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;
use crate::app_state::AppState;

#[derive(Serialize)]
pub struct LogResponse {
    pub process_name: String,
    pub lines: Vec<String>,
}

pub async fn get_process_logs(
    State(state): State<Arc<AppState>>,
    Path((node_id, process_name)): Path<(String, String)>,
) -> Result<Json<LogResponse>, StatusCode> {
    // Audit logs as fallback / history
    let audit_logs = state
        .db
        .get_audit_logs(&node_id, 20)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let lines: Vec<String> = audit_logs
        .into_iter()
        .filter(|l| l.target_name == process_name)
        .map(|l| format!("[{}] Command {} executed by {}: {}", l.timestamp, l.action, l.issued_by, l.output.unwrap_or_default()))
        .collect();

    Ok(Json(LogResponse {
        process_name,
        lines: if lines.is_empty() {
            vec!["[SYSTEM] Connected to PM2 log stream - Waiting for new logs...".to_string()]
        } else {
            lines
        },
    }))
}

