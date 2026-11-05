use tam_collector::log_parser::LogParser;

#[test]
fn test_parse_nginx_combined_log() {
    let line = r#"192.168.1.50 - - [05/Nov/2026:12:00:00 +0000] "GET /api/v1/servers HTTP/1.1" 200 1432 "https://dashboard.tam.local" "Mozilla/5.0" 0.045"#;
    let req = LogParser::parse_nginx_line(line).expect("Failed to parse nginx line");

    assert_eq!(req.client_ip, "192.168.1.50");
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/v1/servers");
    assert_eq!(req.status_code, 200);
    assert!((req.duration_ms - 45.0).abs() < 0.1);
    assert_eq!(req.user_agent, Some("Mozilla/5.0".to_string()));
}

#[test]
fn test_parse_json_log_line() {
    let json_line = r#"{"client_ip": "10.0.0.9", "method": "POST", "path": "/api/v1/login", "status": 401, "duration_ms": 12.3, "user_agent": "curl/8.1"}"#;
    let req = LogParser::parse_json_line(json_line).expect("Failed to parse json line");

    assert_eq!(req.client_ip, "10.0.0.9");
    assert_eq!(req.method, "POST");
    assert_eq!(req.path, "/api/v1/login");
    assert_eq!(req.status_code, 401);
    assert!((req.duration_ms - 12.3).abs() < 0.1);
}

#[test]
fn test_parse_invalid_log_returns_none() {
    assert!(LogParser::parse_nginx_line("gibberish malformed log").is_none());
    assert!(LogParser::parse_json_line("{invalid_json}").is_none());
}

