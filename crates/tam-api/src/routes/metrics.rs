use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use tam_core::models::traffic::AggregatedMetricsSnapshot;
use crate::app_state::AppState;

pub async fn get_metrics_overview(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AggregatedMetricsSnapshot>, StatusCode> {
    let mut agg = state.aggregator.lock().await;
    let snapshot = agg.snapshot();
    Ok(Json(snapshot))
}

