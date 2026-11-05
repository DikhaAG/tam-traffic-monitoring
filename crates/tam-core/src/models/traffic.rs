use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPacketEvent {
    pub timestamp: DateTime<Utc>,
    pub src_ip: String,
    pub src_port: u16,
    pub dst_ip: String,
    pub dst_port: u16,
    pub protocol: String, // "TCP", "UDP", "ICMP", "HTTP", "DNS"
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetricsSnapshot {
    pub timestamp: DateTime<Utc>,
    pub bandwidth_in_bps: u64,
    pub bandwidth_out_bps: u64,
    pub total_rps: f64,
    pub latency_p50_ms: f32,
    pub latency_p95_ms: f32,
    pub latency_p99_ms: f32,
    pub error_rate_percent: f32,
    pub active_connections: u32,
}
