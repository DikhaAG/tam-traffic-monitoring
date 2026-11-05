use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub id: String,
    pub category: String, // "pm2_crash", "restart_loop", "node_offline", "bandwidth_spike", "error_rate"
    pub severity: String, // "info", "warning", "critical"
    pub target: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
}
