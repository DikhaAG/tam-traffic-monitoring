use tam_agent::pm2::reader::Pm2Reader;

#[test]
fn test_parse_pm2_jlist_json() {
    let sample_jlist = r#"[
        {
            "pid": 48210,
            "name": "ecommerce-api",
            "pm_id": 0,
            "monit": {
                "memory": 104857600,
                "cpu": 2.5
            },
            "pm2_env": {
                "status": "online",
                "pm_uptime": 1730000000000,
                "restart_time": 1,
                "node_version": "v20.18.0",
                "pm_cwd": "/var/www/ecommerce-api"
            }
        },
        {
            "pid": 0,
            "name": "payment-worker",
            "pm_id": 1,
            "monit": {
                "memory": 0,
                "cpu": 0.0
            },
            "pm2_env": {
                "status": "errored",
                "pm_uptime": 1730000000000,
                "restart_time": 4,
                "node_version": "v20.18.0",
                "pm_cwd": "/var/www/payment-worker"
            }
        }
    ]"#;

    let processes = Pm2Reader::parse_jlist(sample_jlist).expect("Failed to parse sample jlist");
    assert_eq!(processes.len(), 2);

    let p0 = &processes[0];
    assert_eq!(p0.process_id, 0);
    assert_eq!(p0.name, "ecommerce-api");
    assert_eq!(p0.status, "online");
    assert!((p0.cpu_percent - 2.5).abs() < 0.1);
    assert_eq!(p0.memory_bytes, 104857600);
    assert_eq!(p0.restart_count, 1);
    assert_eq!(p0.node_version, Some("v20.18.0".to_string()));

    let p1 = &processes[1];
    assert_eq!(p1.process_id, 1);
    assert_eq!(p1.name, "payment-worker");
    assert_eq!(p1.status, "errored");
}

#[test]
fn test_simulated_pm2_processes() {
    let procs = Pm2Reader::generate_simulated_processes();
    assert!(!procs.is_empty());
    assert!(procs.iter().any(|p| p.status == "online"));
}

