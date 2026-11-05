// Stubs for Task 1 initialization
pub mod server {
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    pub enum Environment {
        Production,
        Staging,
        Development,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServerNode {
        pub id: String,
        pub hostname: String,
        pub ip_address: String,
        pub environment: Environment,
        pub os_info: String,
        pub cpu_cores: u32,
        pub total_memory_mb: u64,
        pub status: String,
        pub last_heartbeat: DateTime<Utc>,
    }
}

pub mod pm2 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Pm2ProcessMetrics {
        pub process_id: u32,
        pub name: String,
        pub status: String,
        pub cpu_percent: f32,
        pub memory_bytes: u64,
        pub uptime_seconds: u64,
        pub restart_count: u32,
        pub git_branch: Option<String>,
        pub git_commit: Option<String>,
        pub node_version: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Pm2Command {
        pub id: String,
        pub node_id: String,
        pub target_name: String,
        pub action: String, // restart, reload, stop, clear_logs
        pub issued_by: String,
    }
}

pub mod traffic {
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NetworkPacketEvent {
        pub timestamp: DateTime<Utc>,
        pub src_ip: String,
        pub src_port: u16,
        pub dst_ip: String,
        pub dst_port: u16,
        pub protocol: String,
        pub size_bytes: u32,
        pub latency_ms: Option<f32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AppRequestEvent {
        pub timestamp: DateTime<Utc>,
        pub client_ip: String,
        pub method: String,
        pub path: String,
        pub status_code: u16,
        pub duration_ms: f32,
        pub user_agent: Option<String>,
    }
}

pub mod alert {
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AlertEvent {
        pub id: String,
        pub category: String,
        pub severity: String,
        pub target: String,
        pub description: String,
        pub timestamp: DateTime<Utc>,
        pub resolved: bool,
    }
}

