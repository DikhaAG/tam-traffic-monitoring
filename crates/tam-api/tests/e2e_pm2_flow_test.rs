use axum::http::StatusCode;
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use tam_agent::ws_client::{AgentMessage, ServerCommandMessage};
use tam_api::app_state::AppState;
use tam_api::routes::create_router;
use tam_core::db::DatabasePool;
use tam_core::models::alert::AlertEvent;
use tam_core::models::pm2::Pm2ProcessMetrics;
use tam_core::models::server::{Environment, ServerNode};
use tam_core::redis_client::RedisClient;
use tam_core::telegram::TelegramAlertDispatcher;

#[tokio::test]
async fn test_e2e_pm2_flow() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://tam_user:tam_secret_password@localhost:5433/tam_db".to_string());
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6380".to_string());

    let db = DatabasePool::connect(&db_url).await.expect("connect db");
    db.run_migrations().await.expect("migrations");

    let redis = RedisClient::connect(&redis_url).await.expect("connect redis");

    let agent_token = "e2e_agent_secret_token".to_string();
    let state = Arc::new(AppState::new(
        db.clone(),
        redis,
        agent_token.clone(),
        "e2e_jwt".to_string(),
    ));

    let app = create_router(state.clone());

    // Bind to ephemeral port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind listener");
    let addr = listener.local_addr().expect("local addr");
    let server_task = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("serve axum");
    });

    let node_id = "srv-e2e-node-01";

    // 1. Connect Dashboard Live Stream WS (/ws/live)
    let dashboard_url = format!("ws://{}/ws/live", addr);
    let (dashboard_ws, _) = connect_async(&dashboard_url)
        .await
        .expect("connect dashboard ws");
    let (_, mut dashboard_read) = dashboard_ws.split();

    // 2. Connect Agent WS (/ws/agent)
    let agent_url = format!(
        "ws://{}/ws/agent?token={}&node_id={}",
        addr, agent_token, node_id
    );
    let (agent_ws, _) = connect_async(&agent_url)
        .await
        .expect("connect agent ws");
    let (mut agent_write, mut agent_read) = agent_ws.split();

    // 3. Agent sends Heartbeat
    let test_node = ServerNode {
        id: node_id.to_string(),
        hostname: "ubuntu-e2e-srv".to_string(),
        ip_address: "192.168.1.100".to_string(),
        environment: Environment::Production,
        os_info: "Ubuntu 24.04 LTS".to_string(),
        cpu_cores: 8,
        total_memory_mb: 16384,
        disk_usage_percent: 42.5,
        status: "online".to_string(),
        last_heartbeat: Utc::now(),
    };

    let hb_msg = AgentMessage::Heartbeat(test_node.clone());
    let hb_json = serde_json::to_string(&hb_msg).unwrap();
    agent_write
        .send(Message::Text(hb_json.clone().into()))
        .await
        .expect("send heartbeat");

    // Verify Dashboard receives heartbeat broadcast
    let dash_msg = tokio::time::timeout(Duration::from_secs(3), dashboard_read.next())
        .await
        .expect("dash recv timeout")
        .expect("dash msg")
        .expect("dash ok");
    let dash_text = dash_msg.into_text().unwrap();
    assert!(dash_text.contains("Heartbeat"));
    assert!(dash_text.contains("ubuntu-e2e-srv"));

    // 4. Agent sends PM2 Telemetry (including a crashed process)
    let crashed_proc = Pm2ProcessMetrics {
        process_id: 1,
        name: "payment-worker".to_string(),
        status: "errored".to_string(),
        cpu_percent: 0.0,
        memory_bytes: 42_000_000,
        restart_count: 15,
        uptime_seconds: 0,
        git_branch: Some("main".to_string()),
        git_commit: Some("c4f1a23".to_string()),
        git_commit_msg: Some("fix: payment webhook".to_string()),
        node_version: Some("v20.18.0".to_string()),
    };
    let metrics_msg = AgentMessage::Pm2Metrics(vec![crashed_proc.clone()]);
    let metrics_json = serde_json::to_string(&metrics_msg).unwrap();
    agent_write
        .send(Message::Text(metrics_json.into()))
        .await
        .expect("send metrics");

    // Verify Dashboard receives PM2 telemetry broadcast
    let dash_msg2 = tokio::time::timeout(Duration::from_secs(3), dashboard_read.next())
        .await
        .expect("dash recv timeout 2")
        .expect("dash msg 2")
        .expect("dash ok 2");
    let dash_text2 = dash_msg2.into_text().unwrap();
    assert!(dash_text2.contains("payment-worker"));
    assert!(dash_text2.contains("errored"));

    // 5. Test Remote PM2 Command Dispatch via HTTP API: POST /api/v1/servers/:id/pm2/:name/action
    let http_client = reqwest::Client::new();
    let action_url = format!(
        "http://{}/api/v1/servers/{}/pm2/payment-worker/action",
        addr, node_id
    );
    let action_res = http_client
        .post(&action_url)
        .json(&json!({
            "action": "restart",
            "issued_by": "qa-automation"
        }))
        .send()
        .await
        .expect("dispatch action");

    assert_eq!(action_res.status(), StatusCode::OK);
    let action_body: serde_json::Value = action_res.json().await.expect("json body");
    assert_eq!(action_body["status"], "success");
    let cmd_id = action_body["command_id"].as_str().unwrap().to_string();

    // 6. Agent receives Command over WebSocket
    let agent_cmd_msg = tokio::time::timeout(Duration::from_secs(3), agent_read.next())
        .await
        .expect("agent recv timeout")
        .expect("agent msg")
        .expect("agent ok");
    let agent_cmd_text = agent_cmd_msg.into_text().unwrap();
    let parsed_cmd: ServerCommandMessage =
        serde_json::from_str(&agent_cmd_text).expect("parse server command");

    match parsed_cmd {
        ServerCommandMessage::ExecuteCommand(cmd) => {
            assert_eq!(cmd.id, cmd_id);
            assert_eq!(cmd.target_name, "payment-worker");
            assert_eq!(cmd.action, "restart");
            assert_eq!(cmd.issued_by, "qa-automation");

            // Agent responds with success output
            let resp_msg = AgentMessage::CommandResponse {
                id: cmd.id,
                status: "success".to_string(),
                output: "PM2 process payment-worker restarted successfully [pid: 4092]".to_string(),
            };
            agent_write
                .send(Message::Text(serde_json::to_string(&resp_msg).unwrap().into()))
                .await
                .expect("send response");
        }
    }

    // 7. Verify Dashboard receives Command Response
    let dash_msg3 = tokio::time::timeout(Duration::from_secs(3), dashboard_read.next())
        .await
        .expect("dash recv timeout 3")
        .expect("dash msg 3")
        .expect("dash ok 3");
    let dash_text3 = dash_msg3.into_text().unwrap();
    assert!(dash_text3.contains("CommandResponse"));
    assert!(dash_text3.contains("payment-worker restarted successfully"));

    // 8. Verify Telegram Alert formatting for PM2 crash
    let alert = AlertEvent {
        id: "alert-pm2-e2e".to_string(),
        category: "pm2_crash".to_string(),
        severity: "critical".to_string(),
        target: "srv-e2e-node-01:payment-worker".to_string(),
        description: "Process 'payment-worker' status changed to 'errored' with 15 restarts".to_string(),
        timestamp: Utc::now(),
        resolved: false,
    };
    let dispatcher = TelegramAlertDispatcher::new("mock_bot_token".to_string(), "123456789".to_string());
    let formatted = dispatcher.format_alert_markdown(&alert);
    assert!(formatted.contains("CRITICAL ALERT"));
    assert!(formatted.contains("pm2_crash"));
    assert!(formatted.contains("payment-worker"));

    server_task.abort();
}
