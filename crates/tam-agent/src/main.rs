use chrono::Utc;
use tam_core::models::server::{Environment, ServerNode};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("run");

    match command {
        "install-service" => {
            let bin_path = std::env::current_exe().unwrap_or_default();
            let unit = tam_agent::systemd::generate_systemd_unit(
                bin_path.to_str().unwrap_or("/usr/local/bin/tam-agent"),
                "srv-ubuntu-01",
                "production",
                "ws://127.0.0.1:4000/ws/agent",
                "tam_agent_secure_token",
            );
            println!("Generated systemd unit:\n\n{}", unit);
        }
        "run" | _ => {
            let server_url = std::env::var("TAM_WS_URL").unwrap_or_else(|_| "ws://127.0.0.1:4000/ws/agent".to_string());
            let token = std::env::var("TAM_AGENT_TOKEN").unwrap_or_else(|_| "tam_agent_secure_token".to_string());
            let node_id = std::env::var("TAM_SERVER_ID").unwrap_or_else(|_| "srv-ubuntu-01".to_string());

            let node = ServerNode {
                id: node_id,
                hostname: "ubuntu-server-prod".to_string(),
                ip_address: "127.0.0.1".to_string(),
                environment: Environment::Production,
                os_info: "Ubuntu 24.04 LTS".to_string(),
                cpu_cores: 4,
                total_memory_mb: 8192,
                disk_usage_percent: 45.0,
                status: "online".to_string(),
                last_heartbeat: Utc::now(),
            };

            let is_simulated = args.contains(&"--simulate-pm2".to_string());
            let mut client = tam_agent::ws_client::AgentWebSocketClient::new(server_url, token, node, is_simulated);
            client.run_loop().await;
        }
    }
}
