use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Production,
    Staging,
    Development,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Production => write!(f, "production"),
            Environment::Staging => write!(f, "staging"),
            Environment::Development => write!(f, "development"),
        }
    }
}

impl From<String> for Environment {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "production" | "prod" => Environment::Production,
            "staging" | "stag" => Environment::Staging,
            _ => Environment::Development,
        }
    }
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
    pub disk_usage_percent: f32,
    pub status: String, // "online", "offline"
    pub last_heartbeat: DateTime<Utc>,
}
