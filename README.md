# TAM Traffic Monitoring & Ubuntu PM2 Fleet Management Platform

[![Rust Workspace](https://img.shields.io/badge/Rust-1.94+-orange.svg)](https://www.rust-lang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-15.1-black.svg)](https://nextjs.org/)
[![Docker Compose](https://img.shields.io/badge/Docker-Postgres%2016%20%2B%20Redis%207-blue.svg)](https://docker.com/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

TAM (Traffic & Application Monitoring) is a high-performance observability and remote process fleet management platform engineered in **Rust** and **Next.js 15**. It unifies network traffic inspection, HTTP request analytics, and Ubuntu Server **PM2 process monitoring** with live log streaming, zero-inbound-port outbound WebSocket agents, and instant Telegram Bot incident alerts.

---

## 🏛 Architecture Overview

```mermaid
flowchart TD
    subgraph Ubuntu_Server["Ubuntu Fleet (Production / Staging / Dev)"]
        PM2["PM2 Runtime (pm2 jlist)"]
        Logs["Process Logs (~/.pm2/logs/*.log)"]
        Git["Git Metadata (Branch, Commit Hash)"]
        Agent["tam-agent Daemon (Outbound WSS)"]
        
        PM2 --> Agent
        Logs --> Agent
        Git --> Agent
    end

    subgraph Central_Platform["Central TAM Platform"]
        API["tam-api (Axum 0.8, WS Hub, REST)"]
        Aggregator["tam-core (Sliding Window Aggregator)"]
        Anomaly["tam-core (Anomaly & Alert Engine)"]
        Collector["tam-collector (Nginx & Network Parser)"]
        DB[(PostgreSQL 16)]
        Cache[(Redis 7)]
        Telegram["Telegram Bot Dispatcher"]
        
        API --> Aggregator
        API --> DB
        API --> Cache
        Aggregator --> Anomaly
        Anomaly --> Telegram
    end

    subgraph Operations_Dashboard["Executive Web Dashboard (Next.js 15)"]
        FleetView["PM2 Fleet Management"]
        LogTerm["Live Log Terminal"]
        ActionModal["Remote PM2 Control (Restart/Reload/Stop)"]
        TrafficView["Network & Bandwidth Inspector"]
        AlertCenter["Telegram Incident Center"]
    end

    Agent -- "Outbound WSS (/ws/agent)" --> API
    API -- "Broadcast Telemetry (/ws/live)" --> Operations_Dashboard
    Operations_Dashboard -- "HTTP Actions (/api/v1/servers/.../action)" --> API
    API -- "Dispatched Commands (Tunnel)" --> Agent
```

---

## ✨ Key Capabilities

1. **Ubuntu Server & PM2 Process Fleet Management**
   - **Zero Open Inbound Ports**: Agents dial out via persistent TLS WebSocket (`wss://.../ws/agent`).
   - **Full Process Telemetry**: Live CPU %, RAM MB, status (`online`, `errored`, `stopping`, `stopped`), uptime, restart counts, and Node.js version.
   - **Git Code Association**: Automatically correlates running processes with active Git branch and short commit SHA.
   - **Remote Process Operations**: Safe, audited commands (`restart`, `reload`, `stop`, `clear_logs`) routed via existing WebSocket tunnels.
   - **Live Log Streaming**: Bounded ring-buffer tailer streams stdout/stderr directly into an interactive browser terminal.

2. **Automated Anomaly Detection & Telegram Bot Alerts**
   - Instant notifications on PM2 process crashes, rapid restart loops, host offline events, and network bandwidth spikes.
   - Formatted Markdown Telegram messages with direct server and app context.

3. **Multi-Environment Fleet Segmentation**
   - Filter and manage servers across `production`, `staging`, and `development` environments.

4. **Network & Application Telemetry**
   - Sliding-window aggregation of throughput (bytes in/out, packets/sec), HTTP request rate (RPS), p50/p95 latency percentiles, and HTTP 4xx/5xx error rates.
   - Nginx Combined and JSON access log parsers and synthetic packet simulators.

---

## 📦 Project Structure

```
.
├── Cargo.toml                      # Root Cargo workspace manifest
├── docker-compose.yml              # PostgreSQL 16 (port 5433) & Redis 7 (port 6380)
├── Dockerfile.backend              # Multi-stage Rust build for tam-api
├── Dockerfile.agent                # Containerized agent with PM2 runtime
├── Dockerfile.frontend             # Standalone Next.js 15 image
├── crates/
│   ├── tam-core/                   # Data models, SQLx Postgres migrations, Redis, Aggregator, Telegram
│   ├── tam-agent/                  # Ubuntu daemon, PM2 jlist reader, remote executor, log tailer
│   ├── tam-collector/              # Packet sniffer abstractions, Nginx log parsers, traffic simulator
│   └── tam-api/                    # Axum 0.8 REST API & dual WebSocket hubs (/ws/agent & /ws/live)
├── apps/
│   └── web/                        # Next.js 15 App Router, React 19, Tailwind CSS dashboard
├── docs/
│   ├── specs/                      # Architecture and specifications
│   ├── superpowers/plans/          # TDD execution plans
│   └── UBUNTU_PM2_SETUP.md         # Ubuntu server setup runbook
└── scripts/
    └── install-ubuntu-agent.sh     # One-step systemd agent installer
```

---

## 🚀 Quickstart Guide

### 1. Start Infrastructure (PostgreSQL & Redis)

```bash
# Start PostgreSQL (5433:5432) and Redis (6380:6379)
docker compose up -d

# Verify health
docker compose ps
```

### 2. Configure Environment

Copy `.env.example` to `.env`:

```bash
cp .env.example .env
```

Default local `.env` settings:
```env
DATABASE_URL=postgres://tam_user:tam_secret_password@localhost:5433/tam_db
REDIS_URL=redis://127.0.0.1:6380
API_PORT=4000
AGENT_SHARED_TOKEN=tam_agent_secret_token_2026
JWT_SECRET=tam_jwt_secret_dev_key
TELEGRAM_BOT_TOKEN=
TELEGRAM_CHAT_ID=
NEXT_PUBLIC_API_URL=http://localhost:4000
NEXT_PUBLIC_WS_URL=ws://localhost:4000/ws/live
```

### 3. Run Central Backend (`tam-api`)

```bash
cargo run -p tam-api
```
The API server starts listening on `http://0.0.0.0:4000` and automatically runs database schema migrations on startup.

### 4. Start Next.js Operations Dashboard

```bash
pnpm --filter web dev
```
Open [http://localhost:3000](http://localhost:3000) to view the executive dark-mode dashboard.

### 5. Launch Simulated Ubuntu PM2 Agent

To test without an Ubuntu server, run the built-in simulator:

```bash
cargo run -p tam-agent -- --simulate-pm2
```
The agent connects to `ws://127.0.0.1:4000/ws/agent`, registers as an Ubuntu node, periodically transmits process metrics, and responds to remote restart/reload commands.

---

## 🐧 Ubuntu Server Deployment

To install `tam-agent` on an actual Ubuntu server running PM2:

```bash
sudo TAM_SERVER_ID="srv-ubuntu-prod-01" \
     TAM_ENVIRONMENT="production" \
     TAM_WS_URL="wss://tam.yourdomain.com/ws/agent" \
     TAM_AGENT_TOKEN="tam_agent_secret_token_2026" \
     bash scripts/install-ubuntu-agent.sh
```

For detailed manual instructions, systemd unit configuration, and security options, see the [Ubuntu PM2 Setup Guide](docs/UBUNTU_PM2_SETUP.md).

---

## 🧪 Verification & Testing

### Rust Workspace Tests (100% Pass)
```bash
cargo test --workspace
```
Runs all 21 unit and integration tests including `e2e_pm2_flow_test`.

### Frontend Typecheck & Build
```bash
pnpm --filter web build
```

---

## 🔒 Security Posture

- **Zero Inbound Attack Surface**: Ubuntu nodes only establish outbound connections.
- **Mutual Pre-shared Key Authentication**: All WebSocket connections require pre-shared authentication tokens.
- **Whitelist-only PM2 Actions**: Commands are strictly validated against `["restart", "reload", "stop", "clear_logs"]`.
- **Full Central Audit Trail**: Every executed action records timestamp, initiator, target, and output in the `pm2_audit_logs` table.

