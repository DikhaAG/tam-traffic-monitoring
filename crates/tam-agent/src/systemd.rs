pub fn generate_systemd_unit(
    binary_path: &str,
    server_id: &str,
    environment: &str,
    tam_api_url: &str,
    agent_token: &str,
) -> String {
    format!(
        r#"[Unit]
Description=TAM Telemetry & PM2 Monitoring Daemon
After=network.target pm2-root.service

[Service]
Type=simple
User=root
WorkingDirectory=/var/log
Environment="NODE_ENV={env}"
Environment="TAM_SERVER_ID={id}"
Environment="TAM_ENVIRONMENT={env}"
Environment="TAM_API_URL={api_url}"
Environment="TAM_AGENT_TOKEN={token}"
ExecStart={bin} run
Restart=always
RestartSec=5
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
"#,
        bin = binary_path,
        id = server_id,
        env = environment,
        api_url = tam_api_url,
        token = agent_token,
    )
}
