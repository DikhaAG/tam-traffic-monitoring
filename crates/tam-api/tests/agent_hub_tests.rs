use tam_api::ws::agent_hub::AgentHub;
use tam_core::models::pm2::Pm2Command;
use chrono::Utc;

#[tokio::test]
async fn test_agent_hub_channel_routing() {
    let hub = AgentHub::new();

    // Register a channel for test node
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    hub.register_agent("test-node-10", tx).await;

    let cmd = Pm2Command {
        id: "cmd-hub-1".to_string(),
        node_id: "test-node-10".to_string(),
        target_name: "test-app".to_string(),
        action: "restart".to_string(),
        issued_by: "test-admin".to_string(),
        timestamp: Utc::now(),
        status: "pending".to_string(),
        output: None,
    };

    let dispatched = hub.dispatch_command("test-node-10", cmd.clone()).await;
    assert!(dispatched.is_ok());

    let received = rx.recv().await;
    assert!(received.is_some());
    assert_eq!(received.unwrap().id, "cmd-hub-1");
}

