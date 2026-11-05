use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;
use tam_core::models::alert::AlertEvent;
use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct AlertQuery {
    pub resolved: Option<bool>,
    pub limit: Option<i64>,
}

pub async fn list_alerts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AlertQuery>,
) -> Result<Json<Vec<AlertEvent>>, StatusCode> {
    let limit = query.limit.unwrap_or(50);
    let alerts = state
        .db
        .list_alert_incidents(query.resolved, limit)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(alerts))
}

