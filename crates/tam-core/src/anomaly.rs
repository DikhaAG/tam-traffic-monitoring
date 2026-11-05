use chrono::{Duration, Utc};
use std::collections::VecDeque;
use uuid::Uuid;
use crate::models::alert::AlertEvent;
use crate::models::pm2::Pm2ProcessMetrics;
use crate::models::server::ServerNode;
use crate::models::traffic::AggregatedMetricsSnapshot;

#[derive(Debug, Clone)]
pub struct AnomalyConfig {
    pub heartbeat_timeout_seconds: i64,
    pub restart_loop_threshold: u32,
    pub traffic_spike_multiplier: f64,
    pub error_rate_threshold_percent: f32,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        Self {
            heartbeat_timeout_seconds: 15,
            restart_loop_threshold: 3,
            traffic_spike_multiplier: 3.0,
            error_rate_threshold_percent: 10.0,
        }
    }
}

pub struct AnomalyDetector {
    config: AnomalyConfig,
    recent_snapshots: VecDeque<AggregatedMetricsSnapshot>,
}

impl AnomalyDetector {
    pub fn new(config: AnomalyConfig) -> Self {
        Self {
            config,
            recent_snapshots: VecDeque::new(),
        }
    }

    pub fn evaluate_pm2_process(&self, node_id: &str, proc: &Pm2ProcessMetrics) -> Vec<AlertEvent> {
        let mut alerts = Vec::new();

        if proc.status == "errored" {
            alerts.push(AlertEvent {
                id: Uuid::new_v4().to_string(),
                category: "pm2_crash".to_string(),
                severity: "critical".to_string(),
                target: format!("{} / {}", node_id, proc.name),
                description: format!(
                    "PM2 process '{}' (PID {}) crashed and is in errored status. Restarts: {}",
                    proc.name, proc.process_id, proc.restart_count
                ),
                timestamp: Utc::now(),
                resolved: false,
            });
        }

        if proc.restart_count >= self.config.restart_loop_threshold && proc.uptime_seconds < 120 {
            alerts.push(AlertEvent {
                id: Uuid::new_v4().to_string(),
                category: "restart_loop".to_string(),
                severity: "warning".to_string(),
                target: format!("{} / {}", node_id, proc.name),
                description: format!(
                    "PM2 process '{}' is in a rapid restart loop: {} restarts with uptime only {}s",
                    proc.name, proc.restart_count, proc.uptime_seconds
                ),
                timestamp: Utc::now(),
                resolved: false,
            });
        }

        alerts
    }

    pub fn evaluate_server_heartbeat(&self, node: &ServerNode) -> Vec<AlertEvent> {
        let now = Utc::now();
        let timeout = Duration::seconds(self.config.heartbeat_timeout_seconds);

        if now - node.last_heartbeat > timeout {
            vec![AlertEvent {
                id: Uuid::new_v4().to_string(),
                category: "node_offline".to_string(),
                severity: "critical".to_string(),
                target: format!("{} ({})", node.hostname, node.id),
                description: format!(
                    "Server node '{}' [{}] missed heartbeat. Last seen {} seconds ago.",
                    node.hostname,
                    node.environment,
                    (now - node.last_heartbeat).num_seconds()
                ),
                timestamp: now,
                resolved: false,
            }]
        } else {
            Vec::new()
        }
    }

    pub fn record_metric_snapshot(&mut self, snapshot: &AggregatedMetricsSnapshot) {
        self.recent_snapshots.push_back(snapshot.clone());
        if self.recent_snapshots.len() > 30 {
            self.recent_snapshots.pop_front();
        }
    }

    pub fn evaluate_traffic_snapshot(&mut self, current: &AggregatedMetricsSnapshot) -> Vec<AlertEvent> {
        let mut alerts = Vec::new();

        if !self.recent_snapshots.is_empty() {
            let avg_rps: f64 = self.recent_snapshots.iter().map(|s| s.total_rps).sum::<f64>()
                / self.recent_snapshots.len() as f64;

            if avg_rps > 1.0 && current.total_rps > avg_rps * self.config.traffic_spike_multiplier {
                alerts.push(AlertEvent {
                    id: Uuid::new_v4().to_string(),
                    category: "traffic_spike".to_string(),
                    severity: "warning".to_string(),
                    target: "Network Gateway".to_string(),
                    description: format!(
                        "Sudden traffic surge detected: {:.1} RPS (baseline: {:.1} RPS, {:.1}x surge)",
                        current.total_rps, avg_rps, current.total_rps / avg_rps
                    ),
                    timestamp: Utc::now(),
                    resolved: false,
                });
            }
        }

        if current.error_rate_percent >= self.config.error_rate_threshold_percent {
            alerts.push(AlertEvent {
                id: Uuid::new_v4().to_string(),
                category: "error_rate_surge".to_string(),
                severity: "critical".to_string(),
                target: "Application Routes".to_string(),
                description: format!(
                    "Critical HTTP error rate: {:.1}% (threshold: {:.1}%)",
                    current.error_rate_percent, self.config.error_rate_threshold_percent
                ),
                timestamp: Utc::now(),
                resolved: false,
            });
        }

        self.record_metric_snapshot(current);
        alerts
    }
}
