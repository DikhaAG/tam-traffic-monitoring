use chrono::Utc;
use tam_core::aggregator::SlidingWindowAggregator;
use tam_core::models::traffic::{AppRequestEvent, NetworkPacketEvent};

#[test]
fn test_sliding_window_aggregator_metrics() {
    let mut aggregator = SlidingWindowAggregator::new(60); // 60 seconds window

    let now = Utc::now();

    // Record 100 packets: 50 incoming, 50 outgoing
    for i in 0..50 {
        let client_ip = format!("10.0.0.{}", i + 2);
        aggregator.record_packet(&NetworkPacketEvent {
            timestamp: now,
            src_ip: client_ip.clone(),
            src_port: 50000 + i,
            dst_ip: "192.168.1.1".to_string(),
            dst_port: 80,
            protocol: "TCP".to_string(),
            size_bytes: 1000,
            latency_ms: Some(10.0),
        });
        aggregator.record_packet(&NetworkPacketEvent {
            timestamp: now,
            src_ip: "192.168.1.1".to_string(),
            src_port: 80,
            dst_ip: client_ip,
            dst_port: 50000 + i,
            protocol: "TCP".to_string(),
            size_bytes: 2000,
            latency_ms: Some(15.0),
        });
    }

    // Record 20 app requests: 18 success (200), 2 error (500)
    for i in 0..20 {
        let status = if i < 18 { 200 } else { 500 };
        let duration = (i as f32) * 5.0 + 10.0; // 10ms to 105ms
        aggregator.record_request(&AppRequestEvent {
            timestamp: now,
            client_ip: "10.0.0.5".to_string(),
            method: "GET".to_string(),
            path: "/api/v1/orders".to_string(),
            status_code: status,
            duration_ms: duration,
            user_agent: Some("Mozilla/5.0".to_string()),
        });
    }

    let snapshot = aggregator.snapshot();
    assert!(snapshot.active_connections >= 50);
    // 50 * 1000 * 8 = 400,000 bits
    assert!(snapshot.bandwidth_in_bps > 0);
    // 50 * 2000 * 8 = 800,000 bits
    assert!(snapshot.bandwidth_out_bps > 0);
    assert_eq!(snapshot.total_rps, 20.0);
    // 2 errors out of 20 = 10%
    assert!((snapshot.error_rate_percent - 10.0).abs() < 0.1);
    // Latencies: p50 should be around median, p95 and p99 higher
    assert!(snapshot.latency_p50_ms > 0.0);
    assert!(snapshot.latency_p95_ms >= snapshot.latency_p50_ms);
    assert!(snapshot.latency_p99_ms >= snapshot.latency_p95_ms);
}
