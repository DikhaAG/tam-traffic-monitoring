pub mod servers;
pub mod pm2;
pub mod metrics;
pub mod alerts;
pub mod logs;

use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use crate::app_state::AppState;
use crate::ws::{agent_ws_handler, dashboard_ws_handler};

async fn root_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "service": "TAM Telemetry & PM2 Monitoring API",
        "version": env!("CARGO_PKG_VERSION"),
        "status": "online",
        "endpoints": {
            "health": "/health",
            "servers": "/api/v1/servers",
            "metrics": "/api/v1/metrics/overview",
            "alerts": "/api/v1/alerts",
            "ws_agent": "/ws/agent",
            "ws_dashboard": "/ws/live"
        }
    }))
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/health", get(|| async { "OK" }))
        // WebSocket hubs
        .route("/ws/agent", get(agent_ws_handler))
        .route("/ws/live", get(dashboard_ws_handler))
        // Server & PM2 management
        .route("/api/v1/servers", get(servers::list_servers))
        .route("/api/v1/servers/{id}", get(servers::get_server))
        .route("/api/v1/servers/{id}/pm2", get(pm2::list_server_pm2))
        .route("/api/v1/servers/{id}/pm2/{process_name}/action", post(pm2::dispatch_pm2_action))
        .route("/api/v1/servers/{id}/pm2/{process_name}/logs", get(logs::get_process_logs))
        // Traffic & System metrics
        .route("/api/v1/metrics/overview", get(metrics::get_metrics_overview))
        .route("/api/v1/alerts", get(alerts::list_alerts))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

