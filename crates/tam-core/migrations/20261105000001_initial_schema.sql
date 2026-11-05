CREATE TABLE IF NOT EXISTS server_nodes (
    id VARCHAR(64) PRIMARY KEY,
    hostname VARCHAR(255) NOT NULL,
    ip_address VARCHAR(64) NOT NULL,
    environment VARCHAR(32) NOT NULL,
    os_info VARCHAR(255) NOT NULL,
    cpu_cores INT NOT NULL,
    total_memory_mb BIGINT NOT NULL,
    disk_usage_percent REAL NOT NULL DEFAULT 0.0,
    status VARCHAR(32) NOT NULL DEFAULT 'online',
    last_heartbeat TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS pm2_process_snapshots (
    id BIGSERIAL PRIMARY KEY,
    node_id VARCHAR(64) NOT NULL REFERENCES server_nodes(id) ON DELETE CASCADE,
    process_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(32) NOT NULL,
    cpu_percent REAL NOT NULL,
    memory_bytes BIGINT NOT NULL,
    uptime_seconds BIGINT NOT NULL,
    restart_count INT NOT NULL,
    git_branch VARCHAR(255),
    git_commit VARCHAR(64),
    git_commit_msg TEXT,
    node_version VARCHAR(64),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS pm2_audit_logs (
    id VARCHAR(64) PRIMARY KEY,
    node_id VARCHAR(64) NOT NULL REFERENCES server_nodes(id) ON DELETE CASCADE,
    target_name VARCHAR(255) NOT NULL,
    action VARCHAR(32) NOT NULL,
    issued_by VARCHAR(255) NOT NULL,
    status VARCHAR(32) NOT NULL,
    output TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS network_packets (
    id BIGSERIAL PRIMARY KEY,
    src_ip VARCHAR(64) NOT NULL,
    src_port INT NOT NULL,
    dst_ip VARCHAR(64) NOT NULL,
    dst_port INT NOT NULL,
    protocol VARCHAR(32) NOT NULL,
    size_bytes INT NOT NULL,
    latency_ms REAL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS app_requests (
    id BIGSERIAL PRIMARY KEY,
    client_ip VARCHAR(64) NOT NULL,
    method VARCHAR(16) NOT NULL,
    path TEXT NOT NULL,
    status_code INT NOT NULL,
    duration_ms REAL NOT NULL,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS alert_incidents (
    id VARCHAR(64) PRIMARY KEY,
    category VARCHAR(64) NOT NULL,
    severity VARCHAR(32) NOT NULL,
    target VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    resolved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_pm2_snapshots_node_time ON pm2_process_snapshots(node_id, recorded_at DESC);
CREATE INDEX IF NOT EXISTS idx_network_packets_time ON network_packets(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_app_requests_time ON app_requests(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_alert_incidents_time ON alert_incidents(created_at DESC);
