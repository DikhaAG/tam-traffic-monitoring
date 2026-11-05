use tam_collector::simulator::{TrafficSimulator, SimulatorConfig};

#[test]
fn test_traffic_simulator_generation() {
    let mut config = SimulatorConfig::default();
    config.base_events_per_sec = 100;
    let mut simulator = TrafficSimulator::new(config);

    // Generate batch of 20 simulated events
    let (packets, requests) = simulator.generate_tick(0.2); // 0.2 second tick

    assert!(!packets.is_empty());
    assert!(!requests.is_empty());

    // Verify packet properties
    for p in &packets {
        assert!(p.size_bytes > 0);
        assert!(["TCP", "UDP", "HTTP", "DNS"].contains(&p.protocol.as_str()));
    }

    // Verify request properties
    for r in &requests {
        assert!(!r.client_ip.is_empty());
        assert!(!r.path.is_empty());
        assert!(r.status_code >= 200 && r.status_code < 600);
    }
}

#[test]
fn test_traffic_simulator_spike_injection() {
    let mut config = SimulatorConfig::default();
    config.base_events_per_sec = 50;
    let mut simulator = TrafficSimulator::new(config);

    let (normal_packets, _) = simulator.generate_tick(1.0);
    let normal_count = normal_packets.len();

    // Trigger 5x spike
    simulator.trigger_spike(5.0, 5.0); // 5 seconds duration, 5x multiplier

    let (spike_packets, _) = simulator.generate_tick(1.0);
    let spike_count = spike_packets.len();

    assert!(spike_count > normal_count * 2);
}
