#!/usr/bin/env bash
set -euo pipefail

# TAM Monitoring - Ubuntu PM2 Agent Automated Installer
# Usage:
#   sudo TAM_SERVER_ID="srv-prod-01" \
#        TAM_ENVIRONMENT="production" \
#        TAM_WS_URL="ws://127.0.0.1:4000/ws/agent" \
#        TAM_AGENT_TOKEN="your_token" \
#        bash scripts/install-ubuntu-agent.sh

API_URL="${TAM_WS_URL:-${TAM_API_URL:-ws://127.0.0.1:4000/ws/agent}}"
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

# Detect actual user running PM2 (not root if run via sudo)
ACTUAL_USER="${SUDO_USER:-$USER}"
if [ "$ACTUAL_USER" = "root" ] && [ -n "${USER:-}" ] && [ "$USER" != "root" ]; then
  ACTUAL_USER="$USER"
fi
USER_HOME=$(getent passwd "$ACTUAL_USER" | cut -d: -f6 || echo "/home/$ACTUAL_USER")

echo "Target User:  $ACTUAL_USER (Home: $USER_HOME)"

# Detect PM2 binary path and user environment
PM2_BIN=$(su - "$ACTUAL_USER" -c 'which pm2' 2>/dev/null || which pm2 || echo "")
if [ -n "$PM2_BIN" ]; then
  PM2_DIR=$(dirname "$PM2_BIN")
  PATH_ENV="PATH=$PM2_DIR:/usr/local/bin:/usr/bin:/bin"
  echo "PM2 Binary:   $PM2_BIN"
else
  PATH_ENV="PATH=/usr/local/bin:/usr/bin:/bin"
  echo "Warning: pm2 not found in PATH for user $ACTUAL_USER."
fi

# Build or install tam-agent binary if not already present
if [ ! -f "$INSTALL_DIR/tam-agent" ]; then
  echo "Binary $INSTALL_DIR/tam-agent not found. Building with Docker..."
  if command -v docker >/dev/null 2>&1 && [ -f "Dockerfile.agent" ]; then
    docker build -t tam-agent-builder -f Dockerfile.agent .
    CONTAINER_ID=$(docker create tam-agent-builder)
    docker cp "$CONTAINER_ID:/app/tam-agent" "$INSTALL_DIR/tam-agent"
    docker rm "$CONTAINER_ID"
    chmod +x "$INSTALL_DIR/tam-agent"
    echo "✔ tam-agent binary successfully built and installed to $INSTALL_DIR/tam-agent"
  else
    echo "Notice: Ensure /usr/local/bin/tam-agent is copied to this server."
  fi
fi

# Generate systemd service unit
SERVICE_FILE="/etc/systemd/system/tam-agent.service"

cat <<EOF > "$SERVICE_FILE"
[Unit]
Description=TAM Telemetry & PM2 Monitoring Daemon
After=network.target

[Service]
Type=simple
User=$ACTUAL_USER
Group=$ACTUAL_USER
WorkingDirectory=$USER_HOME
Environment="$PATH_ENV"
Environment="PM2_HOME=$USER_HOME/.pm2"
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

# Reload systemd and enable service
systemctl daemon-reload
if [ -f "$INSTALL_DIR/tam-agent" ]; then
  systemctl enable --now tam-agent
  echo "✔ tam-agent service started and enabled."
fi

echo ""
echo "To check service status and logs:"
echo "  systemctl status tam-agent"
echo "  journalctl -u tam-agent -f"
echo ""
echo "=== Installation setup complete ==="
