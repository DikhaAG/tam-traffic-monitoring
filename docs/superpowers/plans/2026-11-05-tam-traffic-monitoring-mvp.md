# TAM Traffic & PM2 Server Fleet Monitoring Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and deploy TAM (Traffic Activity & Application Monitoring), a dual-layer traffic and server fleet monitoring platform featuring a high-performance Rust backend, a dedicated lightweight Ubuntu PM2 agent daemon (`tam-agent`), PostgreSQL and Redis storage, outbound WebSocket telemetry and remote process control, Telegram Bot crash alerting, and a high-end Next.js 15 interactive dashboard with multi-environment server grouping (`production`, `staging`, `development`).

**Architecture:** Monorepo with a Cargo workspace containing `crates/tam-core` (models/DB/telemetry/Telegram), `crates/tam-agent` (Ubuntu PM2 daemon & process controller), `crates/tam-collector` (packet sniffer & traffic simulator), and `crates/tam-api` (Axum REST API & WebSocket broadcast hub) alongside `apps/web` (Next.js 15 App Router frontend). Docker Compose orchestrates local PostgreSQL and Redis for dev/MVP.

**Tech Stack:** Rust (Tokio, Axum, SQLx, Redis-rs, Pcap, Reqwest), PostgreSQL 16, Redis 7, Next.js 15, React 19, Tailwind CSS v4, Lucide React, Telegram Bot API, Docker Compose.

**Spec:** [tam-traffic-monitoring-spec.md](../../specs/tam-traffic-monitoring-spec.md)

---

## Global Constraints
- **Language & Runtime:** Rust stable (2021 edition) for all crates; Node.js 20+ for Next.js frontend.
- **TDD Requirement (`tdd`):** Red-Green-Refactor tests first for all domain logic, PM2 parsing, command execution, anomaly scoring, Telegram formatting, and API routes.
- **Code Hygiene (`no-ai-slop`, `caveman`):** Terse, production-grade code without redundant comments or generic AI filler. Deep modules with clean seams (`codebase-design`).
- **Security Standards (`security-review`, `vibe-security`):** Parameterized queries (SQLx), outbound-only agent connections (no open inbound ports on Ubuntu), constant-time API token comparisons, audit logging for remote PM2 actions.
- **System Administration Best Practices (`sysadmin`):** Clean systemd unit generator (`tam-agent.service`), graceful signal handling (SIGINT/SIGTERM), bounded log buffers, automatic exponential backoff.
- **UI/UX Standards (`ui-ux-pro-max`):** Executive dark-mode dashboard palette, environment grouping tabs, real-time pulse indicators, accessible terminal log viewer, zero layout shifts.

---

## Review Focus
The five failure modes and critical edge conditions that must be pinned by tests:
1. **PM2 Daemon Crash or Socket Disconnection on Ubuntu:** If the PM2 daemon restarts or socket disappears, `tam-agent` must not crash; it must backoff and reconnect cleanly.
2. **WebSocket Agent Disconnect & Command Loss:** If an Ubuntu agent temporarily drops connection, commands sent during the disconnect must either time out or queue with an explicit status rather than hang.
3. **Log Stream Overflow / Memory Exhaustion:** Tailing large PM2 log files (`stdout`/`stderr`) must use bounded circular ring buffers and rate-limited chunking to prevent memory leaks.
4. **Telegram API Rate Limits & Network Drops:** Telegram alert notifications must be dispatched asynchronously with timeout protection and retry limits so alert failures never block event processing.
5. **Unauthorized Remote PM2 Execution Attempt:** Commands to restart/stop processes must strictly require valid authentication and log the issuer in the PostgreSQL audit trail.

---

## Tasks

### Task 1: Monorepo Foundation, Crates Setup & Docker Compose

**Files:**
- Create: `Cargo.toml`
- Create: `docker-compose.yml`
- Create: `.env.example`
- Create: `crates/tam-core/Cargo.toml`
- Create: `crates/tam-agent/Cargo.toml`
- Create: `crates/tam-collector/Cargo.toml`
- Create: `crates/tam-api/Cargo.toml`
- Create: `Makefile`
- Test: `tests/smoke_test.rs`

- [ ] **Step 1: Write root workspace configuration and Cargo.toml**
  Define members `["crates/tam-core", "crates/tam-agent", "crates/tam-collector", "crates/tam-api"]` with shared workspace dependencies (`tokio`, `serde`, `sqlx`, `redis`, `thiserror`, `tracing`, `reqwest`).
- [ ] **Step 2: Create docker-compose.yml with PostgreSQL 16 & Redis 7**
  Configure PostgreSQL 16 (port 5432) and Redis 7 (port 6379) with healthchecks and persistent volumes for development.
- [ ] **Step 3: Write initial workspace smoke test**
  Add `tests/smoke_test.rs` asserting all 4 crates compile and load test configurations.
- [ ] **Step 4: Run test to verify build and test harness passes**
  Run `cargo test --workspace`.
- [ ] **Step 5: Commit changes**
  `git add . && git commit -m "feat(infra): setup cargo workspace with tam-agent, docker-compose and baseline crates"`

---

### Task 2: Core Domain Models, Migrations & Database Storage (`crates/tam-core`)

**Files:**
- Create: `crates/tam-core/src/models/mod.rs`
- Create: `crates/tam-core/src/models/server.rs`
- Create: `crates/tam-core/src/models/pm2.rs`
- Create: `crates/tam-core/src/models/traffic.rs`
- Create: `crates/tam-core/src/models/alert.rs`
- Create: `crates/tam-core/src/db/mod.rs`
- Create: `crates/tam-core/src/db/servers.rs`
- Create: `crates/tam-core/src/db/audit.rs`
- Create: `crates/tam-core/migrations/20261105000001_initial_schema.sql`
- Test: `crates/tam-core/tests/models_and_db_tests.rs`

- [ ] **Step 1: Write failing test for domain models and DB persistence**
  Write tests in `models_and_db_tests.rs` for:
  - Registering and querying `ServerNode` with environment grouping (`production`, `staging`, `development`).
  - Storing and retrieving `Pm2ProcessMetrics` snapshots.
  - Recording `Pm2Command` execution in the audit log table.
  - Inserting `AlertEvent` and `NetworkPacketEvent`.
- [ ] **Step 2: Run test to verify failure**
  Run `cargo test -p tam-core` and verify compilation fails on missing types.
- [ ] **Step 3: Implement domain models in `src/models/`**
  Implement Serde-serializable structs: `ServerNode`, `Pm2ProcessMetrics`, `Pm2Command`, `NetworkPacketEvent`, `AlertEvent`.
- [ ] **Step 4: Create SQL schema migrations**
  Create tables: `server_nodes`, `pm2_process_snapshots`, `pm2_audit_logs`, `network_packets`, `traffic_metrics`, `alert_incidents` with proper indices.
- [ ] **Step 5: Implement SQLx database repository in `src/db/`**
  Implement connection pooling, upsert for server nodes, batch process metric insertion, and audit trail writer.
- [ ] **Step 6: Run tests and ensure all pass**
  Run `cargo test -p tam-core` and verify all tests pass.
- [ ] **Step 7: Commit changes**
  `git add crates/tam-core && git commit -m "feat(core): implement server nodes, pm2 domain models, migrations and sqlx storage"`

---

### Task 3: In-Memory Sliding-Window Aggregator, Anomaly Detector & Telegram Dispatcher (`crates/tam-core`)

**Files:**
- Create: `crates/tam-core/src/aggregator.rs`
- Create: `crates/tam-core/src/anomaly.rs`
- Create: `crates/tam-core/src/telegram.rs`
- Create: `crates/tam-core/src/redis_client.rs`
- Test: `crates/tam-core/tests/aggregator_tests.rs`
- Test: `crates/tam-core/tests/anomaly_tests.rs`
- Test: `crates/tam-core/tests/telegram_tests.rs`

- [ ] **Step 1: Write failing test for sliding-window metrics aggregator**
  Verify per-second metric bucketing and rate calculation (bandwidth, RPS, p95 latency).
- [ ] **Step 2: Write failing test for anomaly detection & PM2 crash detection**
  Verify detector triggers an alert when a PM2 process status changes to `errored` or restart count increases > 3 in 2 minutes.
- [ ] **Step 3: Write failing test for Telegram alert formatting**
  Verify Telegram message payload formats markdown with server hostname, environment, process name, and crash reason.
- [ ] **Step 4: Implement SlidingWindowAggregator in `aggregator.rs`**
  Implement time-bucketed metric aggregator with lock-free counters.
- [ ] **Step 5: Implement AnomalyDetector in `anomaly.rs`**
  Implement dynamic threshold evaluator for traffic spikes and PM2 crash detection rules.
- [ ] **Step 6: Implement TelegramAlertDispatcher in `telegram.rs`**
  Implement async HTTP client pushing formatted alerts to Telegram Bot API with retry tolerance and disabled notification sound options for minor alerts.
- [ ] **Step 7: Implement Redis pub/sub client in `redis_client.rs`**
  Add broadcast publisher for live event channel `tam:live-events`.
- [ ] **Step 8: Run tests and ensure all pass**
  Run `cargo test -p tam-core`.
- [ ] **Step 9: Commit changes**
  `git add crates/tam-core && git commit -m "feat(core): implement aggregator, anomaly detector and telegram alert dispatcher"`

---

### Task 4: Ingestion Engine & Dual-Mode Traffic Simulator (`crates/tam-collector`)

**Files:**
- Create: `crates/tam-collector/src/lib.rs`
- Create: `crates/tam-collector/src/sniffer.rs`
- Create: `crates/tam-collector/src/simulator.rs`
- Create: `crates/tam-collector/src/log_parser.rs`
- Test: `crates/tam-collector/tests/simulator_tests.rs`
- Test: `crates/tam-collector/tests/parser_tests.rs`

- [ ] **Step 1: Write failing tests for log parsing and synthetic traffic generation**
  Test Nginx combined log parsing, JSON access log parsing, and synthetic traffic generation rates.
- [ ] **Step 2: Implement LogParser in `log_parser.rs`**
  Safe parser for client IP, HTTP method, URI path, status code, latency ms, and user-agent.
- [ ] **Step 3: Implement TrafficSimulator in `simulator.rs`**
  Configurable background simulator generating realistic HTTP/TCP flows and controllable traffic spikes.
- [ ] **Step 4: Implement Sniffer abstraction in `sniffer.rs`**
  EventStream trait abstracting live PCAP socket capture and synthetic mode.
- [ ] **Step 5: Run tests and verify all pass**
  Run `cargo test -p tam-collector`.
- [ ] **Step 6: Commit changes**
  `git add crates/tam-collector && git commit -m "feat(collector): implement dual-mode traffic sniffer, parser and simulator"`

---

### Task 5: Ubuntu Server PM2 Agent & Remote Control Engine (`crates/tam-agent`)

**Files:**
- Create: `crates/tam-agent/src/main.rs`
- Create: `crates/tam-agent/src/pm2/mod.rs`
- Create: `crates/tam-agent/src/pm2/reader.rs`
- Create: `crates/tam-agent/src/pm2/executor.rs`
- Create: `crates/tam-agent/src/logs/tailer.rs`
- Create: `crates/tam-agent/src/git_meta.rs`
- Create: `crates/tam-agent/src/ws_client.rs`
- Create: `crates/tam-agent/src/systemd.rs`
- Test: `crates/tam-agent/tests/pm2_reader_tests.rs`
- Test: `crates/tam-agent/tests/executor_tests.rs`
- Test: `crates/tam-agent/tests/log_tailer_tests.rs`

- [ ] **Step 1: Write failing tests for PM2 metrics extraction and command execution**
  Test parsing `pm2 jlist` JSON outputs into `Pm2ProcessMetrics`. Test command executor translating `restart`/`reload`/`stop` into PM2 actions.
- [ ] **Step 2: Write failing test for log tailer**
  Test tailing stdout/stderr log files with circular ring buffer.
- [ ] **Step 3: Implement PM2 Reader & Git Metadata Extractor**
  Extract process CPU %, RAM, status, uptime, restarts, plus git branch and commit hash from app directory. Support `--simulate-pm2` flag for dev/macOS testing.
- [ ] **Step 4: Implement Remote Command Executor in `executor.rs`**
  Execute PM2 commands (`pm2 restart <id>`, `pm2 reload <id>`, `pm2 stop <id>`) with output validation.
- [ ] **Step 5: Implement Outbound WebSocket Client in `ws_client.rs`**
  Maintain outbound connection to `wss://<tam-server>/ws/agent` with API Key, exponential backoff, heartbeat pings, and remote command listener.
- [ ] **Step 6: Implement Systemd Service Generator in `systemd.rs`**
  Command `tam-agent install-service` to generate and install `/etc/systemd/system/tam-agent.service`.
- [ ] **Step 7: Run tests and verify all pass**
  Run `cargo test -p tam-agent`.
- [ ] **Step 8: Commit changes**
  `git add crates/tam-agent && git commit -m "feat(agent): implement ubuntu pm2 telemetry agent, log tailer, git extractor and remote executor"`

---

### Task 6: Axum REST API, WebSocket Hubs & Agent Controller (`crates/tam-api`)

**Files:**
- Create: `crates/tam-api/src/main.rs`
- Create: `crates/tam-api/src/routes/mod.rs`
- Create: `crates/tam-api/src/routes/servers.rs`
- Create: `crates/tam-api/src/routes/pm2.rs`
- Create: `crates/tam-api/src/routes/metrics.rs`
- Create: `crates/tam-api/src/routes/logs.rs`
- Create: `crates/tam-api/src/routes/alerts.rs`
- Create: `crates/tam-api/src/ws/mod.rs`
- Create: `crates/tam-api/src/ws/agent_hub.rs`
- Create: `crates/tam-api/src/ws/dashboard_hub.rs`
- Create: `crates/tam-api/src/auth.rs`
- Test: `crates/tam-api/tests/server_routes_tests.rs`
- Test: `crates/tam-api/tests/agent_hub_tests.rs`

- [ ] **Step 1: Write failing test for Agent Hub and Server REST API**
  Verify agent connects to `/ws/agent`, server list endpoint returns nodes grouped by environment, and PM2 command dispatch sends action to target agent.
- [ ] **Step 2: Implement Auth middleware in `auth.rs`**
  Constant-time verification for `X-TAM-AGENT-TOKEN` and JWT verification for dashboard sessions.
- [ ] **Step 3: Implement Agent WebSocket Hub in `ws/agent_hub.rs`**
  Track active server connections by node ID, receive periodic telemetry, and route remote commands with timeout.
- [ ] **Step 4: Implement Dashboard WebSocket Hub in `ws/dashboard_hub.rs`**
  Stream live 1-second metric frames and PM2 updates to connected browser clients.
- [ ] **Step 5: Implement REST routes in `src/routes/`**
  Implement `/api/v1/servers`, `/api/v1/servers/:id/pm2`, `/api/v1/servers/:id/pm2/:pid/action`, `/api/v1/metrics/overview`, `/api/v1/logs`, `/api/v1/alerts`.
- [ ] **Step 6: Integrate Telegram alert dispatcher into event pipeline**
  Trigger Telegram messages on PM2 process crash and server node offline events.
- [ ] **Step 7: Run tests and ensure all pass**
  Run `cargo test -p tam-api`.
- [ ] **Step 8: Commit changes**
  `git add crates/tam-api && git commit -m "feat(api): implement axum api, agent hub, pm2 command dispatch and dashboard websocket stream"`

---

### Task 7: Next.js Frontend Scaffolding, Design System & Navigation (`apps/web`)

**Files:**
- Create: `apps/web/package.json`
- Create: `apps/web/tsconfig.json`
- Create: `apps/web/src/app/layout.tsx`
- Create: `apps/web/src/app/globals.css`
- Create: `apps/web/src/components/navbar.tsx`
- Create: `apps/web/src/components/ui/status-badge.tsx`
- Create: `apps/web/src/components/ui/metric-card.tsx`
- Create: `apps/web/src/components/environment-tabs.tsx`
- Test: `apps/web/src/components/__tests__/status-badge.test.tsx`

- [ ] **Step 1: Scaffold Next.js 15 App Router project with Tailwind CSS**
  Set up dark executive theme, high-contrast tokens, Lucide icons, and responsive shell following `ui-ux-pro-max`.
- [ ] **Step 2: Write failing test for StatusBadge and EnvironmentTabs**
  Assert badge styling for `online`, `errored`, `stopped`, and environment filter switching (`All`, `Production`, `Staging`, `Development`).
- [ ] **Step 3: Implement UI primitives and Navbar**
  Implement Navbar with active server count, overall health badge, and EnvironmentTabs.
- [ ] **Step 4: Run component tests**
  Execute `npm test` in `apps/web`.
- [ ] **Step 5: Commit changes**
  `git add apps/web && git commit -m "feat(web): initialize next.js frontend with dark-mode design system and environment tabs"`

---

### Task 8: PM2 Fleet Management, Process Control & Live Log Terminal Views (`apps/web`)

**Files:**
- Create: `apps/web/src/hooks/use-pm2-fleet.ts`
- Create: `apps/web/src/components/pm2/server-node-card.tsx`
- Create: `apps/web/src/components/pm2/process-table.tsx`
- Create: `apps/web/src/components/pm2/process-action-modal.tsx`
- Create: `apps/web/src/components/pm2/live-log-terminal.tsx`
- Create: `apps/web/src/components/pm2/pm2-fleet-view.tsx`
- Test: `apps/web/src/components/pm2/__tests__/process-table.test.tsx`
- Test: `apps/web/src/components/pm2/__tests__/process-action-modal.test.tsx`

- [ ] **Step 1: Write failing tests for PM2 process table and action modal**
  Test process status rendering, CPU/RAM values, git commit badges, and command confirmation modal dispatching restart/reload/stop API calls.
- [ ] **Step 2: Implement `usePm2Fleet` hook**
  Manage live WebSocket updates for server nodes and PM2 processes with fallback polling.
- [ ] **Step 3: Implement ServerNodeCard & ProcessTable**
  Display host health (CPU, RAM, disk) and responsive table of PM2 apps with status pills, uptime, and git metadata.
- [ ] **Step 4: Implement ProcessActionModal**
  Accessible confirmation dialog with action feedback, audit warning, and execution state spinner.
- [ ] **Step 5: Implement LiveLogTerminal**
  High-performance log viewer with stdout/stderr syntax coloring, search filter, auto-scroll toggle, and copy-to-clipboard button.
- [ ] **Step 6: Run tests and verify components render cleanly**
  Run `npm test` in `apps/web`.
- [ ] **Step 7: Commit changes**
  `git add apps/web && git commit -m "feat(web): implement pm2 fleet management, remote process control and live log terminal"`

---

### Task 9: Executive Overview, Network Inspector & Application Analytics (`apps/web`)

**Files:**
- Create: `apps/web/src/components/overview/executive-overview.tsx`
- Create: `apps/web/src/components/network/packet-inspector.tsx`
- Create: `apps/web/src/components/analytics/endpoint-analytics.tsx`
- Create: `apps/web/src/components/alerts/alert-center.tsx`
- Create: `apps/web/src/components/settings/telegram-settings.tsx`
- Create: `apps/web/src/app/page.tsx`
- Test: `apps/web/src/components/overview/__tests__/executive-overview.test.tsx`

- [ ] **Step 1: Write failing test for Executive Overview and Alert Center**
  Assert calculation of aggregate bandwidth, RPS, error rate, and alert list filtering.
- [ ] **Step 2: Implement Executive Overview & Packet Inspector**
  Interactive charts for live bandwidth in/out, protocol distributions (TCP, UDP, HTTP), and top talker IPs.
- [ ] **Step 3: Implement Endpoint Analytics & Telegram Settings**
  Display top API endpoints, latency percentiles, and test Telegram Bot integration button.
- [ ] **Step 4: Assemble comprehensive dashboard tabs in `src/app/page.tsx`**
  Unify PM2 Fleet, Executive Overview, Network, Application, and Alerts into clean tabbed views.
- [ ] **Step 5: Run frontend test suite**
  Execute `npm test` across all frontend components.
- [ ] **Step 6: Commit changes**
  `git add apps/web && git commit -m "feat(web): implement executive overview, network inspection and telegram alert center"`

---

### Task 10: End-to-End System Integration, Verification & Systemd Deployment Runbook

**Files:**
- Create: `Dockerfile.backend`
- Create: `Dockerfile.agent`
- Create: `Dockerfile.frontend`
- Modify: `docker-compose.yml`
- Create: `scripts/install-ubuntu-agent.sh`
- Create: `tests/e2e_pm2_flow_test.rs`
- Create: `README.md`
- Create: `docs/UBUNTU_PM2_SETUP.md`

- [ ] **Step 1: Write multi-stage Dockerfiles and Ubuntu install script**
  Provide single-line installer `curl -sSL https://.../install-ubuntu-agent.sh | bash` that installs `tam-agent` and sets up `systemd`.
- [ ] **Step 2: Implement comprehensive End-to-End integration test**
  Write `e2e_pm2_flow_test.rs`: Launch API server and simulated PM2 agent, trigger a process crash, verify Telegram notification is formatted and dispatched, and verify remote restart command flows through to agent.
- [ ] **Step 3: Verify single-command startup with Docker Compose**
  Test `docker compose up --build -d` ensuring PostgreSQL, Redis, API, and Next.js start cleanly.
- [ ] **Step 4: Write complete README.md and UBUNTU_PM2_SETUP.md**
  Document step-by-step installation on Ubuntu servers with PM2, environment configuration, Telegram Bot token setup, and troubleshooting.
- [ ] **Step 5: Run full test suite across workspace**
  Execute `cargo test --workspace` and `npm run test` in frontend.
- [ ] **Step 6: Final commit**
  `git add . && git commit -m "feat(release): finalize end-to-end integration, ubuntu pm2 deployment and documentation"`
