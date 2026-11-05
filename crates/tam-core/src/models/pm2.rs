use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pm2ProcessMetrics {
    pub process_id: u32,
    pub name: String,
    pub status: String, // "online", "stopping", "stopped", "errored"
    pub cpu_percent: f32,
    pub memory_bytes: u64,
    pub uptime_seconds: u64,
    pub restart_count: u32,
    pub git_branch: Option<String>,
    pub git_commit: Option<String>,
    pub git_commit_msg: Option<String>,
    pub node_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pm2Command {
    pub id: String,
    pub node_id: String,
    pub target_name: String,
    pub action: String, // "restart", "reload", "stop", "clear_logs"
    pub issued_by: String,
    pub timestamp: DateTime<Utc>,
    pub status: String, // "pending", "success", "failed"
    pub output: Option<String>,
}

