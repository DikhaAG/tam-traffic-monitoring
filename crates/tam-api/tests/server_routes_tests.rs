use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tam_api::app_state::AppState;
use tam_api::routes::create_router;
use tam_core::db::DatabasePool;
use tam_core::redis_client::RedisClient;
use tam_core::models::server::{Environment, ServerNode};
use chrono::Utc;
use std::sync::Arc;

#[tokio::test]
async fn test_health_and_servers_endpoints() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://tam_user:tam_secret_password@localhost:5433/tam_db".to_string());
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6380".to_string());

    let db = DatabasePool::connect(&db_url).await.expect("connect db");
    db.run_migrations().await.expect("migrations");

    let redis = RedisClient::connect(&redis_url).await.expect("connect redis");

    let state = Arc::new(AppState::new(db.clone(), redis, "test_token".to_string(), "test_jwt".to_string()));
    let app = create_router(state);

    // 1. Test /health
    let req = Request::builder().uri("/health").body(Body::empty()).unwrap();
    let res = tower::ServiceExt::oneshot(app.clone(), req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 2. Insert test server node
    let node = ServerNode {
        id: "srv-api-test-01".to_string(),
        hostname: "api-test-host".to_string(),
        ip_address: "10.0.1.5".to_string(),
        environment: Environment::Production,
        os_info: "Ubuntu 24.04".to_string(),
        cpu_cores: 4,
        total_memory_mb: 8192,
        disk_usage_percent: 20.0,
        status: "online".to_string(),
        last_heartbeat: Utc::now(),
    };
    db.upsert_server_node(&node).await.unwrap();

    // 3. Test GET /api/v1/servers
    let req = Request::builder().uri("/api/v1/servers?environment=production").body(Body::empty()).unwrap();
    let res = tower::ServiceExt::oneshot(app.clone(), req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let servers: Vec<ServerNode> = serde_json::from_slice(&body).unwrap();
    assert!(servers.iter().any(|s| s.id == "srv-api-test-01"));
}

