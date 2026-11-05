# Ubuntu PM2 Monitoring & `tam-agent` Deployment Guide

This guide describes how to deploy the lightweight `tam-agent` daemon on Ubuntu servers to monitor PM2 applications, stream logs, report process health, and enable remote management from the central TAM dashboard.

---

## 1. Prerequisites on Ubuntu Server
- **OS**: Ubuntu 20.04 LTS / 22.04 LTS / 24.04 LTS.
- **Node.js & PM2**: Node.js 18+ and PM2 installed globally (`npm install -g pm2`).
- **Network**: Outbound internet/network access to your central TAM API server port (default `4000` or `443` over TLS/WSS).
- **Note**: No inbound firewall ports need to be opened on the Ubuntu server. All connections are **outbound persistent WebSocket (WSS)**.

---

## 2. Fast Installation (Automated)

Run the automated installer on your Ubuntu server as root:

```bash
sudo TAM_SERVER_ID="srv-prod-01" \
     TAM_ENVIRONMENT="production" \
     TAM_WS_URL="wss://tam.yourdomain.com/ws/agent" \
     TAM_AGENT_TOKEN="your_secure_shared_agent_token" \
     bash scripts/install-ubuntu-agent.sh
```

---

## 3. Manual Installation Step-by-Step

### Step 3.1: Copy the `tam-agent` Binary
Compile the binary for Linux x86_64:
```bash
cargo build --release -p tam-agent --target x86_64-unknown-linux-musl
```
Copy `tam-agent` to `/usr/local/bin/tam-agent` on your Ubuntu server and make it executable:
```bash
sudo chmod +x /usr/local/bin/tam-agent
```

### Step 3.2: Create the Systemd Unit File
Create `/etc/systemd/system/tam-agent.service`:

```ini
[Unit]
Description=TAM Telemetry & PM2 Monitoring Daemon
After=network.target pm2-root.service

[Service]
Type=simple
User=root
WorkingDirectory=/var/log
Environment="TAM_SERVER_ID=srv-prod-01"
Environment="TAM_ENVIRONMENT=production"
Environment="TAM_WS_URL=wss://tam.yourdomain.com/ws/agent"
Environment="TAM_AGENT_TOKEN=your_secure_shared_agent_token"
ExecStart=/usr/local/bin/tam-agent run
Restart=always
RestartSec=5
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
```

### Step 3.3: Start & Enable the Service
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now tam-agent
```

---

## 4. Verification & Diagnostics

1. Check systemd service status:
   ```bash
   systemctl status tam-agent
   ```
2. Inspect live agent logs:
   ```bash
   journalctl -u tam-agent -f
   ```
3. Test PM2 simulated mode locally (without real PM2):
   ```bash
   tam-agent run --simulate-pm2
   ```

---

## 5. Security & Isolation
- **Outbound Only**: `tam-agent` only dials outward to the configured `TAM_WS_URL`.
- **Token Verification**: Handshake requires `TAM_AGENT_TOKEN` matching the server's `AGENT_SHARED_TOKEN`.
- **Command Sanitization**: Remote commands are restricted to whitelist actions (`restart`, `reload`, `stop`, `clear_logs`) executed directly via PM2 without shell injection vulnerabilities.
- **Audit Logging**: Every executed remote action is recorded in the central PostgreSQL audit table with issuer user, server ID, target app, and execution output.
