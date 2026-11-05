use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use tam_core::models::traffic::{AppRequestEvent, NetworkPacketEvent};

#[derive(Debug, Clone)]
pub struct SimulatorConfig {
    pub base_events_per_sec: u32,
    pub error_rate_percent: f32,
    pub server_ip: String,
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self {
            base_events_per_sec: 50,
            error_rate_percent: 2.0,
            server_ip: "192.168.1.10".to_string(),
        }
    }
}

pub struct TrafficSimulator {
    config: SimulatorConfig,
    spike_end: Option<DateTime<Utc>>,
    spike_multiplier: f64,
}

impl TrafficSimulator {
    pub fn new(config: SimulatorConfig) -> Self {
        Self {
            config,
            spike_end: None,
            spike_multiplier: 1.0,
        }
    }

    pub fn trigger_spike(&mut self, duration_secs: f64, multiplier: f64) {
        self.spike_end = Some(Utc::now() + Duration::milliseconds((duration_secs * 1000.0) as i64));
        self.spike_multiplier = multiplier;
    }

    pub fn generate_tick(&mut self, delta_seconds: f64) -> (Vec<NetworkPacketEvent>, Vec<AppRequestEvent>) {
        let now = Utc::now();
        let multiplier = if let Some(end) = self.spike_end {
            if now < end {
                self.spike_multiplier
            } else {
                self.spike_end = None;
                1.0
            }
        } else {
            1.0
        };

        let count = ((self.config.base_events_per_sec as f64 * delta_seconds * multiplier).round() as usize).max(1);

        let mut rng = rand::thread_rng();
        let mut packets = Vec::with_capacity(count);
        let mut requests = Vec::with_capacity(count / 2 + 1);

        let paths = [
            "/api/v1/servers",
            "/api/v1/servers/srv-prod-01/pm2",
            "/api/v1/metrics/overview",
            "/api/v1/logs",
            "/api/v1/alerts",
            "/api/v1/login",
            "/health",
        ];

        let protocols = ["TCP", "UDP", "HTTP", "DNS"];

        for _ in 0..count {
            let client_octet_3 = rng.gen_range(1..10);
            let client_octet_4 = rng.gen_range(2..254);
            let client_ip = format!("10.{}.{}.{}", client_octet_3, client_octet_3, client_octet_4);

            let protocol = protocols[rng.gen_range(0..protocols.len())].to_string();
            let size_bytes = rng.gen_range(128..1500);
            let latency_ms = Some(rng.gen_range(5.0..120.0));

            packets.push(NetworkPacketEvent {
                timestamp: now,
                src_ip: client_ip.clone(),
                src_port: rng.gen_range(30000..65000),
                dst_ip: self.config.server_ip.clone(),
                dst_port: if protocol == "HTTP" { 80 } else if protocol == "DNS" { 53 } else { 443 },
                protocol,
                size_bytes,
                latency_ms,
            });

            // 50% probability of generating corresponding application request
            if rng.gen_bool(0.5) {
                let path = paths[rng.gen_range(0..paths.len())].to_string();
                let is_error = rng.gen_range(0.0..100.0) < self.config.error_rate_percent * (multiplier as f32);
                let status_code = if is_error {
                    if rng.gen_bool(0.7) { 500 } else { 404 }
                } else {
                    200
                };

                requests.push(AppRequestEvent {
                    timestamp: now,
                    client_ip,
                    method: if rng.gen_bool(0.8) { "GET".to_string() } else { "POST".to_string() },
                    path,
                    status_code,
                    duration_ms: rng.gen_range(8.0..150.0),
                    user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
                });
            }
        }

        (packets, requests)
    }
}

