use chrono::Utc;
use tam_core::models::pm2::{Pm2Command, Pm2ProcessMetrics};
use tam_core::models::server::{Environment, ServerNode};
use tam_core::db::DatabasePool;

#[test]
fn test_models_serialization() {
    let node = ServerNode {
        id: "srv-prod-01".to_string(),
        hostname: "ubuntu-srv-01".to_string(),
        ip_address: "10.0.0.15".to_string(),
        environment: Environment::Production,
        os_info: "Ubuntu 24.04 LTS".to_string(),
        cpu_cores: 8,
        total_memory_mb: 16384,
        disk_usage_percent: 42.5,
        status: "online".to_string(),
        last_heartbeat: Utc::now(),
    };

    let json = serde_json::to_string(&node).expect("serialize server node");
    assert!(json.contains("srv-prod-01"));
    assert!(json.contains("production"));

    let pm2_proc = Pm2ProcessMetrics {
        process_id: 1,
        name: "tam-web-api".to_string(),
        status: "online".to_string(),
        cpu_percent: 1.4,
        memory_bytes: 124_000_000,
        uptime_seconds: 3600,
        restart_count: 0,
        git_branch: Some("main".to_string()),
        git_commit: Some("a1b2c3d".to_string()),
        git_commit_msg: Some("feat: add traffic endpoint".to_string()),
        node_version: Some("v22.2.0".to_string()),
    };

    let json_pm2 = serde_json::to_string(&pm2_proc).expect("serialize pm2 process");
    assert!(json_pm2.contains("tam-web-api"));

    let cmd = Pm2Command {
        id: "cmd-123".to_string(),
        node_id: "srv-prod-01".to_string(),
        target_name: "tam-web-api".to_string(),
        action: "restart".to_string(),
        issued_by: "admin@tam.local".to_string(),
        timestamp: Utc::now(),
        status: "pending".to_string(),
        output: None,
    };
    let json_cmd = serde_json::to_string(&cmd).expect("serialize cmd");
    assert!(json_cmd.contains("restart"));
}

#[tokio::test]
async fn test_db_pool_and_migrations() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://tam_user:tam_secret_password@localhost:5433/tam_db".to_string());

    let pool = DatabasePool::connect(&db_url).await.expect("Failed to connect to test PostgreSQL");
    pool.run_migrations().await.expect("Failed to run migrations");

    let node = ServerNode {
        id: "test-node-01".to_string(),
        hostname: "ubuntu-test-host".to_string(),
        ip_address: "127.0.0.1".to_string(),
        environment: Environment::Staging,
        os_info: "Ubuntu 24.04".to_string(),
        cpu_cores: 4,
        total_memory_mb: 8192,
        disk_usage_percent: 33.0,
        status: "online".to_string(),
        last_heartbeat: Utc::now(),
    };

    pool.upsert_server_node(&node).await.expect("upsert server node");
    let fetched = pool.get_server_node("test-node-01").await.expect("get server node");
    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.hostname, "ubuntu-test-host");
    assert_eq!(fetched.environment, Environment::Staging);

    let pm2_proc = Pm2ProcessMetrics {
        process_id: 10,
        name: "test-worker".to_string(),
        status: "online".to_string(),
        cpu_percent: 2.5,
        memory_bytes: 65_000_000,
        uptime_seconds: 120,
        restart_count: 0,
        git_branch: Some("dev".to_string()),
        git_commit: Some("c4d5e6f".to_string()),
        git_commit_msg: Some("test: initial commit".to_string()),
        node_version: Some("v20.10.0".to_string()),
    };

    pool.save_pm2_snapshot("test-node-01", &[pm2_proc]).await.expect("save pm2 snapshot");
    let procs = pool.get_latest_pm2_processes("test-node-01").await.expect("get latest pm2 processes");
    assert_eq!(procs.len(), 1);
    assert_eq!(procs[0].name, "test-worker");

    let cmd = Pm2Command {
        id: format!("cmd-test-{}", uuid::Uuid::new_v4()),
        node_id: "test-node-01".to_string(),
        target_name: "test-worker".to_string(),
        action: "restart".to_string(),
        issued_by: "operator@tam.local".to_string(),
        timestamp: Utc::now(),
        status: "success".to_string(),
        output: Some("PM2 process restarted successfully".to_string()),
    };
    pool.record_audit_log(&cmd).await.expect("record audit log");

    let logs = pool.get_audit_logs("test-node-01", 10).await.expect("get audit logs");
    assert!(!logs.is_empty());
    assert_eq!(logs[0].action, "restart");
}

