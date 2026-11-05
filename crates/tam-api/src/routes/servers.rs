use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;
use tam_core::models::server::{Environment, ServerNode};
use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct ServerQuery {
    pub environment: Option<String>,
}

pub async fn list_servers(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ServerQuery>,
) -> Result<Json<Vec<ServerNode>>, StatusCode> {
    let env_filter = query.environment.map(Environment::from);
    let servers = state
        .db
        .list_server_nodes(env_filter)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(servers))
}

pub async fn get_server(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ServerNode>, StatusCode> {
    let server = state
        .db
        .get_server_node(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(server))
}

