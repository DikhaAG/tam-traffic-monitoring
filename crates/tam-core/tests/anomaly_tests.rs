use chrono::Utc;
use tam_core::anomaly::{AnomalyDetector, AnomalyConfig};
use tam_core::models::pm2::Pm2ProcessMetrics;
use tam_core::models::server::{Environment, ServerNode};
use tam_core::models::traffic::AggregatedMetricsSnapshot;

#[test]
fn test_anomaly_detection_pm2_crash() {
    let detector = AnomalyDetector::new(AnomalyConfig::default());

    let crashed_proc = Pm2ProcessMetrics {
        process_id: 1,
        name: "payment-gateway".to_string(),
        status: "errored".to_string(),
        cpu_percent: 0.0,
        memory_bytes: 0,
        uptime_seconds: 0,
        restart_count: 5,
        git_branch: Some("main".to_string()),
        git_commit: Some("deadbeef".to_string()),
        git_commit_msg: None,
        node_version: Some("v22.0.0".to_string()),
    };

    let alerts = detector.evaluate_pm2_process("srv-prod-01", &crashed_proc);
    assert!(!alerts.is_empty());
    assert_eq!(alerts[0].category, "pm2_crash");
    assert_eq!(alerts[0].severity, "critical");
    assert!(alerts[0].description.contains("payment-gateway"));
}

#[test]
fn test_anomaly_detection_server_offline() {
    let detector = AnomalyDetector::new(AnomalyConfig::default());

    let offline_node = ServerNode {
        id: "srv-prod-02".to_string(),
        hostname: "worker-02".to_string(),
        ip_address: "10.0.0.12".to_string(),
        environment: Environment::Production,
        os_info: "Ubuntu 22.04".to_string(),
        cpu_cores: 4,
        total_memory_mb: 8192,
        disk_usage_percent: 55.0,
        status: "online".to_string(),
        last_heartbeat: Utc::now() - chrono::Duration::seconds(45), // 45s ago (> 15s threshold)
    };

    let alerts = detector.evaluate_server_heartbeat(&offline_node);
    assert!(!alerts.is_empty());
    assert_eq!(alerts[0].category, "node_offline");
    assert_eq!(alerts[0].severity, "critical");
}

#[test]
fn test_anomaly_detection_traffic_spike() {
    let mut detector = AnomalyDetector::new(AnomalyConfig::default());

    // Normal baseline traffic
    for _ in 0..10 {
        detector.record_metric_snapshot(&AggregatedMetricsSnapshot {
            timestamp: Utc::now(),
            bandwidth_in_bps: 1_000_000,
            bandwidth_out_bps: 1_000_000,
            total_rps: 100.0,
            latency_p50_ms: 15.0,
            latency_p95_ms: 30.0,
            latency_p99_ms: 50.0,
            error_rate_percent: 0.5,
            active_connections: 50,
        });
    }

    // Sudden spike: 600 RPS (6x normal) and 25% error rate
    let spike_snapshot = AggregatedMetricsSnapshot {
        timestamp: Utc::now(),
        bandwidth_in_bps: 10_000_000,
        bandwidth_out_bps: 10_000_000,
        total_rps: 600.0,
        latency_p50_ms: 120.0,
        latency_p95_ms: 400.0,
        latency_p99_ms: 900.0,
        error_rate_percent: 25.0,
        active_connections: 500,
    };

    let alerts = detector.evaluate_traffic_snapshot(&spike_snapshot);
    assert!(!alerts.is_empty());
    assert!(alerts.iter().any(|a| a.category == "traffic_spike" || a.category == "error_rate_surge"));
}

