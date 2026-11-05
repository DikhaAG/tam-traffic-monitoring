#!/usr/bin/env bash
set -euo pipefail

# TAM Monitoring - Ubuntu PM2 Agent Automated Installer
# Usage:
#   curl -sSL https://raw.githubusercontent.com/.../install-ubuntu-agent.sh | bash -s -- \
#     --api-url "wss://your-tam-domain.com/ws/agent" \
#     --token "your-agent-secure-token" \
#     --environment "production" \
#     --server-id "srv-prod-01"

API_URL="${TAM_API_URL:-ws://127.0.0.1:4000/ws/agent}"
TOKEN="${TAM_AGENT_TOKEN:-tam_agent_secure_token}"
ENVIRONMENT="${TAM_ENVIRONMENT:-production}"
SERVER_ID="${TAM_SERVER_ID:-$(hostname)}"
INSTALL_DIR="/usr/local/bin"

echo "=== Installing TAM Telemetry & PM2 Agent on Ubuntu ==="
echo "Server ID:    $SERVER_ID"
echo "Environment:  $ENVIRONMENT"
echo "TAM API:      $API_URL"

# Check root permissions
if [ "$(id -u)" -ne 0 ]; then
  echo "Error: This installation script must be run as root (use sudo)." >&2
  exit 1
fi

# Ensure PM2 is installed
if ! command -v pm2 >/dev/null 2>&1; then
  echo "Warning: pm2 was not found in PATH. Ensure PM2 is installed globally (npm install -g pm2)."
fi

# Generate systemd service unit
SERVICE_FILE="/etc/systemd/system/tam-agent.service"

cat <<EOF > "$SERVICE_FILE"
[Unit]
Description=TAM Telemetry & PM2 Monitoring Daemon
After=network.target pm2-root.service

[Service]
Type=simple
User=root
WorkingDirectory=/var/log
Environment="NODE_ENV=$ENVIRONMENT"
Environment="TAM_SERVER_ID=$SERVER_ID"
Environment="TAM_ENVIRONMENT=$ENVIRONMENT"
Environment="TAM_WS_URL=$API_URL"
Environment="TAM_AGENT_TOKEN=$TOKEN"
ExecStart=$INSTALL_DIR/tam-agent run
Restart=always
RestartSec=5
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

echo "Systemd unit created at $SERVICE_FILE"

# Reload systemd
systemctl daemon-reload
echo "Systemd daemon reloaded."
echo ""
echo "To start the agent service, run:"
echo "  systemctl enable --now tam-agent"
echo ""
echo "To check service status and logs:"
echo "  systemctl status tam-agent"
echo "  journalctl -u tam-agent -f"
echo ""
echo "=== Installation setup complete ==="

