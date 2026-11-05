#[test]
fn test_workspace_crates_baseline() {
    assert_eq!(tam_agent::agent_version(), "0.1.0");
    assert_eq!(tam_collector::collector_version(), "0.1.0");
    assert_eq!(tam_api::api_version(), "0.1.0");
}

#[test]
fn test_core_models_serialization() {
    use tam_core::models::server::{Environment, ServerNode};
    use chrono::Utc;

    let node = ServerNode {
        id: "srv-prod-01".to_string(),
        hostname: "ubuntu-server-01".to_string(),
        ip_address: "192.168.1.100".to_string(),
        environment: Environment::Production,
        os_info: "Ubuntu 24.04 LTS".to_string(),
        cpu_cores: 8,
        total_memory_mb: 16384,
        status: "online".to_string(),
        last_heartbeat: Utc::now(),
    };

    let serialized = serde_json::to_string(&node).expect("Failed to serialize ServerNode");
    assert!(serialized.contains("ubuntu-server-01"));
    assert!(serialized.contains("production"));

    let deserialized: ServerNode = serde_json::from_str(&serialized).expect("Failed to deserialize ServerNode");
    assert_eq!(deserialized.environment, Environment::Production);
}
