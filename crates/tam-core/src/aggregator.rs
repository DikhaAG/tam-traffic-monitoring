use chrono::{DateTime, Duration, Utc};
use std::collections::{HashSet, VecDeque};
use crate::models::traffic::{AggregatedMetricsSnapshot, AppRequestEvent, NetworkPacketEvent};

#[derive(Debug, Clone)]
struct TimedPacket {
    timestamp: DateTime<Utc>,
    src_ip: String,
    size_bytes: u32,
    is_incoming: bool,
}

#[derive(Debug, Clone)]
struct TimedRequest {
    timestamp: DateTime<Utc>,
    status_code: u16,
    duration_ms: f32,
}

#[derive(Debug)]
pub struct SlidingWindowAggregator {
    window_duration: Duration,
    packets: VecDeque<TimedPacket>,
    requests: VecDeque<TimedRequest>,
    last_cleanup: DateTime<Utc>,
}

impl SlidingWindowAggregator {
    pub fn new(window_seconds: i64) -> Self {
        Self {
            window_duration: Duration::seconds(window_seconds),
            packets: VecDeque::new(),
            requests: VecDeque::new(),
            last_cleanup: Utc::now(),
        }
    }

    pub fn record_packet(&mut self, packet: &NetworkPacketEvent) {
        // Assume packets to local destination or typical server subnet are incoming
        let is_incoming = packet.dst_port == 80 || packet.dst_port == 443 || packet.dst_port == 4000;
        self.packets.push_back(TimedPacket {
            timestamp: packet.timestamp,
            src_ip: packet.src_ip.clone(),
            size_bytes: packet.size_bytes,
            is_incoming,
        });
        self.evict_expired(packet.timestamp);
    }

    pub fn record_request(&mut self, req: &AppRequestEvent) {
        self.requests.push_back(TimedRequest {
            timestamp: req.timestamp,
            status_code: req.status_code,
            duration_ms: req.duration_ms,
        });
        self.evict_expired(req.timestamp);
    }

    fn evict_expired(&mut self, now: DateTime<Utc>) {
        if now - self.last_cleanup < Duration::seconds(1) {
            return;
        }
        let cutoff = now - self.window_duration;
        while let Some(front) = self.packets.front() {
            if front.timestamp < cutoff {
                self.packets.pop_front();
            } else {
                break;
            }
        }
        while let Some(front) = self.requests.front() {
            if front.timestamp < cutoff {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        self.last_cleanup = now;
    }

    pub fn snapshot(&mut self) -> AggregatedMetricsSnapshot {
        let now = Utc::now();
        self.evict_expired(now);

        let mut bytes_in: u64 = 0;
        let mut bytes_out: u64 = 0;
        let mut unique_ips = HashSet::new();

        for p in &self.packets {
            unique_ips.insert(p.src_ip.clone());
            if p.is_incoming {
                bytes_in += p.size_bytes as u64;
            } else {
                bytes_out += p.size_bytes as u64;
            }
        }

        let mut total_requests = 0;
        let mut error_requests = 0;
        let mut durations = Vec::with_capacity(self.requests.len());

        for r in &self.requests {
            total_requests += 1;
            if r.status_code >= 400 {
                error_requests += 1;
            }
            durations.push(r.duration_ms);
        }

        durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let p50 = percentile(&durations, 0.50);
        let p95 = percentile(&durations, 0.95);
        let p99 = percentile(&durations, 0.99);

        let error_rate_percent = if total_requests > 0 {
            (error_requests as f32 / total_requests as f32) * 100.0
        } else {
            0.0
        };

        AggregatedMetricsSnapshot {
            timestamp: now,
            bandwidth_in_bps: bytes_in * 8,
            bandwidth_out_bps: bytes_out * 8,
            total_rps: total_requests as f64,
            latency_p50_ms: p50,
            latency_p95_ms: p95,
            latency_p99_ms: p99,
            error_rate_percent,
            active_connections: unique_ips.len() as u32,
        }
    }
}

fn percentile(sorted_data: &[f32], p: f32) -> f32 {
    if sorted_data.is_empty() {
        return 0.0;
    }
    let idx = ((sorted_data.len() as f32 * p).ceil() as usize).saturating_sub(1);
    sorted_data[idx.min(sorted_data.len() - 1)]
}
