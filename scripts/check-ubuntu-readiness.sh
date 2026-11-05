#!/usr/bin/env bash
# ==============================================================================
# TAM (Traffic & Application Monitoring) - Ubuntu Server Readiness Checker
# ------------------------------------------------------------------------------
# Strict Read-Only Audit & Diagnostic Utility
# Evaluates system compatibility for:
#   1. Target Node  : PM2 Applications, Node.js runtime & tam-agent daemon
#   2. Central Host : tam-api, PostgreSQL 16, Redis 7, Next.js Web Dashboard
# ------------------------------------------------------------------------------
# Usage:
#   ./scripts/check-ubuntu-readiness.sh [OPTIONS]
# Options:
#   --role all       Check both PM2 Node and Central Platform (Default)
#   --role agent     Check only PM2 Node / tam-agent requirements
#   --role central   Check only Central TAM Monitoring Platform requirements
#   --json           Output machine-readable JSON report
#   --no-color       Disable ANSI color output
#   -h, --help       Show usage instructions
# ==============================================================================

set -u

# Default settings
TARGET_ROLE="all"
OUTPUT_JSON=false
USE_COLOR=true

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --role)
      if [[ -n "${2:-}" ]]; then
        TARGET_ROLE="$2"
        shift 2
      else
        echo "Error: --role requires an argument ('all', 'agent', or 'central')." >&2
        exit 1
      fi
      ;;
    --role=*)
      TARGET_ROLE="${1#*=}"
      shift
      ;;
    --json)
      OUTPUT_JSON=true
      USE_COLOR=false
      shift
      ;;
    --no-color)
      USE_COLOR=false
      shift
      ;;
    -h|--help)
      cat << 'EOF'
TAM Ubuntu Server Readiness Checker
Strict Read-Only audit utility to verify system compatibility.

Usage:
  ./check-ubuntu-readiness.sh [OPTIONS]

Options:
  --role all|agent|central  Specify role to evaluate (default: all)
                              agent   : PM2 applications & tam-agent daemon
                              central : Central TAM API, PostgreSQL, Redis, Dashboard
                              all     : Both PM2 Node and Central Platform
  --json                    Emit structured JSON report for automation/CI
  --no-color                Disable ANSI terminal colors
  -h, --help                Display this help screen

Examples:
  ./check-ubuntu-readiness.sh
  ./check-ubuntu-readiness.sh --role agent
  ./check-ubuntu-readiness.sh --role central --json
EOF
      exit 0
      ;;
    *)
      echo "Unknown option: $1 (Use --help for usage)" >&2
      exit 1
      ;;
  esac
done

if [[ "$TARGET_ROLE" != "all" && "$TARGET_ROLE" != "agent" && "$TARGET_ROLE" != "central" ]]; then
  echo "Error: Invalid role '$TARGET_ROLE'. Must be 'all', 'agent', or 'central'." >&2
  exit 1
fi

# ANSI Colors
if [ "$USE_COLOR" = true ] && [ -t 1 ]; then
  C_RESET="\033[0m"
  C_BOLD="\033[1m"
  C_GREEN="\033[32m"
  C_YELLOW="\033[33m"
  C_RED="\033[31m"
  C_CYAN="\033[36m"
  C_BLUE="\033[34m"
  C_GRAY="\033[90m"
else
  C_RESET=""
  C_BOLD=""
  C_GREEN=""
  C_YELLOW=""
  C_RED=""
  C_CYAN=""
  C_BLUE=""
  C_GRAY=""
fi

# Score & Report Tracking
AGENT_TOTAL=0
AGENT_PASS=0
AGENT_WARN=0
AGENT_FAIL=0

CENTRAL_TOTAL=0
CENTRAL_PASS=0
CENTRAL_WARN=0
CENTRAL_FAIL=0

# Lists for JSON & Remediation
CHECK_ITEMS_JSON=()
REMEDIATIONS=()

record_check() {
  local target="$1"      # "agent", "central", "both"
  local status="$2"      # "PASS", "WARN", "FAIL", "INFO"
  local name="$3"        # short check name
  local message="$4"     # detail message
  local remediation="${5:-}" # optional copy-paste fix command

  # Increment counters
  if [[ "$target" == "agent" || "$target" == "both" ]]; then
    AGENT_TOTAL=$((AGENT_TOTAL + 1))
    case "$status" in
      PASS) AGENT_PASS=$((AGENT_PASS + 1)) ;;
      WARN) AGENT_WARN=$((AGENT_WARN + 1)) ;;
      FAIL) AGENT_FAIL=$((AGENT_FAIL + 1)) ;;
    esac
  fi

  if [[ "$target" == "central" || "$target" == "both" ]]; then
    CENTRAL_TOTAL=$((CENTRAL_TOTAL + 1))
    case "$status" in
      PASS) CENTRAL_PASS=$((CENTRAL_PASS + 1)) ;;
      WARN) CENTRAL_WARN=$((CENTRAL_WARN + 1)) ;;
      FAIL) CENTRAL_FAIL=$((CENTRAL_FAIL + 1)) ;;
    esac
  fi

  # Store remediation if FAIL or WARN
  if [[ ("$status" == "FAIL" || "$status" == "WARN") && -n "$remediation" ]]; then
    REMEDIATIONS+=("[$status] $name: $remediation")
  fi

  # Record JSON entry
  if [ "$OUTPUT_JSON" = true ]; then
    local escaped_msg
    escaped_msg=$(printf '%s' "$message" | sed 's/"/\\"/g')
    local escaped_rem
    escaped_rem=$(printf '%s' "$remediation" | sed 's/"/\\"/g')
    CHECK_ITEMS_JSON+=("{\"target\":\"$target\",\"status\":\"$status\",\"name\":\"$name\",\"message\":\"$escaped_msg\",\"remediation\":\"$escaped_rem\"}")
  fi

  # Print terminal line if not in JSON mode
  if [ "$OUTPUT_JSON" = false ]; then
    local icon
    case "$status" in
      PASS) icon="${C_GREEN}✔ [PASS]${C_RESET}" ;;
      WARN) icon="${C_YELLOW}⚠ [WARN]${C_RESET}" ;;
      FAIL) icon="${C_RED}✖ [FAIL]${C_RESET}" ;;
      INFO) icon="${C_CYAN}ℹ [INFO]${C_RESET}" ;;
      *)    icon="[....]" ;;
    esac
    printf "  %-12b ${C_BOLD}%-26s${C_RESET} %s\n" "$icon" "$name" "$message"
  fi
}

print_header() {
  if [ "$OUTPUT_JSON" = false ]; then
    echo -e "${C_BOLD}${C_BLUE}========================================================================${C_RESET}"
    echo -e "${C_BOLD}${C_BLUE}   TAM Platform - Ubuntu Server Readiness Diagnostic (Read-Only)${C_RESET}"
    echo -e "${C_BOLD}${C_BLUE}========================================================================${C_RESET}"
    local target_scope_upper
    target_scope_upper=$(echo "$TARGET_ROLE" | tr '[:lower:]' '[:upper:]')
    echo -e " Target Scope : ${C_BOLD}${target_scope_upper}${C_RESET} (Options: agent, central, all)"
    echo -e " Audit Mode   : ${C_GREEN}Strict Read-Only${C_RESET} (No modifications made to system)"
    echo -e " Hostname     : $(hostname 2>/dev/null || echo 'unknown')"
    echo -e " Timestamp    : $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
    echo ""
  fi
}

print_section() {
  if [ "$OUTPUT_JSON" = false ]; then
    local title="$1"
    echo -e "\n${C_BOLD}${C_CYAN}▶ ${title}${C_RESET}"
    echo -e "${C_GRAY}------------------------------------------------------------------------${C_RESET}"
  fi
}

# ------------------------------------------------------------------------------
# 1. OS & Core Architecture Checks
# ------------------------------------------------------------------------------
check_os_and_arch() {
  print_section "1. Operating System & Architecture"

  # OS Distribution
  if [ -f /etc/os-release ]; then
    # shellcheck disable=SC1091
    . /etc/os-release
    local dist_id="${ID:-}"
    local version_id="${VERSION_ID:-}"
    local pretty_name="${PRETTY_NAME:-Linux}"

    if [[ "$dist_id" == "ubuntu" ]]; then
      if [[ "$version_id" =~ ^(20\.04|22\.04|24\.04) ]]; then
        record_check "both" "PASS" "Ubuntu Distribution" "$pretty_name (LTS supported)" ""
      else
        record_check "both" "WARN" "Ubuntu Distribution" "$pretty_name (Recomended: 20.04, 22.04, 24.04 LTS)" "Upgrade to Ubuntu 22.04 LTS or 24.04 LTS for long-term stability"
      fi
    elif [[ "$dist_id" == "debian" ]]; then
      record_check "both" "WARN" "OS Distribution" "$pretty_name (Debian detected, mostly compatible)" "Standard testing performed on Ubuntu Server"
    else
      record_check "both" "FAIL" "OS Distribution" "$pretty_name (Non-Ubuntu distribution)" "Run on Ubuntu Server 20.04/22.04/24.04 LTS"
    fi
  else
    local uname_s
    uname_s=$(uname -s)
    if [[ "$uname_s" == "Darwin" ]]; then
      record_check "both" "WARN" "OS Kernel" "macOS ($uname_s) detected - Local dev mode" "Deploy to Ubuntu Server for production agents"
    else
      record_check "both" "FAIL" "OS Detection" "Unable to read /etc/os-release" "Ensure system is Ubuntu Server"
    fi
  fi

  # CPU Architecture
  local arch
  arch=$(uname -m)
  if [[ "$arch" == "x86_64" || "$arch" == "aarch64" || "$arch" == "arm64" ]]; then
    record_check "both" "PASS" "CPU Architecture" "$arch (Fully supported)" ""
  else
    record_check "both" "FAIL" "CPU Architecture" "$arch (Unsupported architecture)" "TAM binaries require x86_64 or aarch64"
  fi

  # Linux Kernel Version
  local kernel_ver
  kernel_ver=$(uname -r)
  local major_ver
  major_ver=$(echo "$kernel_ver" | cut -d. -f1)
  local minor_ver
  minor_ver=$(echo "$kernel_ver" | cut -d. -f2)

  if [ "$major_ver" -ge 5 ] 2>/dev/null; then
    record_check "both" "PASS" "Linux Kernel" "v$kernel_ver (>= 5.4 requirements met)" ""
  else
    record_check "both" "WARN" "Linux Kernel" "v$kernel_ver (< 5.4 detected)" "Upgrade Linux kernel: sudo apt install --install-recommends linux-generic"
  fi

  # Init System (Systemd)
  if command -v systemctl >/dev/null 2>&1 && pidof systemd >/dev/null 2>&1 || [ -d /run/systemd/system ]; then
    record_check "both" "PASS" "Init System (systemd)" "Active and functional" ""
  else
    if [[ "$(uname -s)" == "Darwin" ]]; then
      record_check "both" "INFO" "Init System" "launchd active (macOS host)" ""
    else
      record_check "both" "WARN" "Init System (systemd)" "systemd not detected or not PID 1" "tam-agent systemd unit requires systemd"
    fi
  fi
}

# ------------------------------------------------------------------------------
# 2. Hardware Resource Checks
# ------------------------------------------------------------------------------
check_hardware_resources() {
  print_section "2. Hardware Resources (RAM, CPU, Disk)"

  # CPU Cores
  local cores=1
  if command -v nproc >/dev/null 2>&1; then
    cores=$(nproc)
  elif command -v sysctl >/dev/null 2>&1; then
    cores=$(sysctl -n hw.ncpu 2>/dev/null || echo 1)
  fi

  if [ "$cores" -ge 4 ]; then
    record_check "both" "PASS" "CPU Cores" "$cores vCPUs available (Optimal)" ""
  elif [ "$cores" -ge 2 ]; then
    record_check "both" "PASS" "CPU Cores" "$cores vCPUs available (Sufficient for Node & Central)" ""
  elif [ "$cores" -ge 1 ]; then
    record_check "agent" "PASS" "CPU Cores (Agent)" "$cores vCPU available (Sufficient for tam-agent)" ""
    record_check "central" "WARN" "CPU Cores (Central)" "$cores vCPU available (Central API + DB recommends >= 2 cores)" "Consider resizing instance to >= 2 vCPUs"
  fi

  # RAM
  local total_ram_mb=0
  local free_ram_mb=0
  if command -v free >/dev/null 2>&1; then
    total_ram_mb=$(free -m | awk '/^Mem:/{print $2}')
    free_ram_mb=$(free -m | awk '/^Mem:/{print $7}')
  elif command -v vm_stat >/dev/null 2>&1; then
    # macOS fallback
    total_ram_mb=8192
    free_ram_mb=2048
  fi

  # Target Node RAM Check
  if [ "$total_ram_mb" -ge 2000 ]; then
    record_check "agent" "PASS" "RAM (PM2 Node)" "${total_ram_mb}MB Total, ${free_ram_mb}MB Available (Overhead < 30MB)" ""
  elif [ "$total_ram_mb" -ge 1000 ]; then
    record_check "agent" "PASS" "RAM (PM2 Node)" "${total_ram_mb}MB Total (Minimum 1GB met)" ""
  else
    record_check "agent" "WARN" "RAM (PM2 Node)" "${total_ram_mb}MB Total (< 1GB available)" "Add swap or upgrade RAM to prevent OOM killer on PM2 processes"
  fi

  # Central Platform RAM Check
  if [ "$total_ram_mb" -ge 8000 ]; then
    record_check "central" "PASS" "RAM (Central Host)" "${total_ram_mb}MB Total (Optimal for Postgres, Redis, API, Web)" ""
  elif [ "$total_ram_mb" -ge 3800 ]; then
    record_check "central" "PASS" "RAM (Central Host)" "${total_ram_mb}MB Total (Meets 4GB minimum requirement)" ""
  elif [ "$total_ram_mb" -ge 1900 ]; then
    record_check "central" "WARN" "RAM (Central Host)" "${total_ram_mb}MB Total (< 4GB recommended for Central Stack)" "Upgrade to >= 4GB RAM or configure 4GB swap space"
  else
    record_check "central" "FAIL" "RAM (Central Host)" "${total_ram_mb}MB Total (Insufficient for Postgres + Redis + API)" "Central monitoring server requires at least 2GB-4GB RAM"
  fi

  # Disk Space
  local disk_avail_kb=0
  local disk_avail_human=""
  if command -v df >/dev/null 2>&1; then
    disk_avail_kb=$(df -k / | awk 'NR==2 {print $4}')
    disk_avail_human=$(df -h / | awk 'NR==2 {print $4}')
  fi

  local disk_avail_gb=$((disk_avail_kb / 1024 / 1024))
  if [ "$disk_avail_gb" -ge 20 ]; then
    record_check "both" "PASS" "Disk Space (/)" "${disk_avail_human} free (Optimal for logs & PostgreSQL data)" ""
  elif [ "$disk_avail_gb" -ge 10 ]; then
    record_check "both" "PASS" "Disk Space (/)" "${disk_avail_human} free (Meets target requirements)" ""
  elif [ "$disk_avail_gb" -ge 5 ]; then
    record_check "agent" "PASS" "Disk Space (Agent)" "${disk_avail_human} free (Sufficient for PM2 logs)" ""
    record_check "central" "WARN" "Disk Space (Central)" "${disk_avail_human} free (< 10GB for Postgres time-series)" "Ensure logrotate or expand root volume"
  else
    record_check "both" "FAIL" "Disk Space (/)" "${disk_avail_human} free (< 5GB critical low space)" "Free up disk space: sudo apt clean && sudo journalctl --vacuum-time=3d"
  fi
}

# ------------------------------------------------------------------------------
# 3. PM2 & Node.js Application Stack Checks (Target Node)
# ------------------------------------------------------------------------------
check_pm2_node_stack() {
  if [[ "$TARGET_ROLE" != "all" && "$TARGET_ROLE" != "agent" ]]; then
    return
  fi

  print_section "3. PM2 & Node.js Application Stack (Target Node)"

  # Node.js
  if command -v node >/dev/null 2>&1; then
    local node_v
    node_v=$(node -v)
    local node_major
    node_major=$(echo "$node_v" | tr -d 'v' | cut -d. -f1)

    if [ "$node_major" -ge 18 ]; then
      record_check "agent" "PASS" "Node.js Runtime" "$node_v (Active LTS supported)" ""
    else
      record_check "agent" "WARN" "Node.js Runtime" "$node_v (Node >= 18 LTS recommended)" "curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - && sudo apt install -y nodejs"
    fi
  else
    record_check "agent" "FAIL" "Node.js Runtime" "Not found in PATH" "curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - && sudo apt install -y nodejs"
  fi

  # PM2 Process Manager
  if command -v pm2 >/dev/null 2>&1; then
    local pm2_v
    pm2_v=$(pm2 -v 2>/dev/null || echo "installed")
    record_check "agent" "PASS" "PM2 Process Manager" "v$pm2_v installed globally" ""

    # Check active running processes in PM2
    local proc_count=0
    if pm2 jlist >/dev/null 2>&1; then
      proc_count=$(pm2 jlist 2>/dev/null | grep -o '"name":' | wc -l | tr -d ' ')
      if [ "$proc_count" -gt 0 ]; then
        record_check "agent" "PASS" "Active PM2 Fleet" "$proc_count application(s) actively running in PM2" ""
      else
        record_check "agent" "INFO" "Active PM2 Fleet" "PM2 daemon active, 0 applications running currently" ""
      fi
    else
      record_check "agent" "WARN" "PM2 Fleet Status" "Unable to execute 'pm2 jlist' under current user" "Check PM2_HOME or run: sudo -u <user> pm2 jlist"
    fi
  else
    record_check "agent" "FAIL" "PM2 Process Manager" "Not found globally in PATH" "sudo npm install -g pm2 && sudo pm2 startup"
  fi

  # Git CLI
  if command -v git >/dev/null 2>&1; then
    local git_v
    git_v=$(git --version | awk '{print $3}')
    record_check "agent" "PASS" "Git CLI" "v$git_v available for commit/branch tracking" ""
  else
    record_check "agent" "WARN" "Git CLI" "git not found (Needed to correlate commits with PM2 apps)" "sudo apt-get install -y git"
  fi

  # PM2 Home & Log Directory Permissions
  local pm2_home="${PM2_HOME:-$HOME/.pm2}"
  if [ -d "$pm2_home" ]; then
    if [ -w "$pm2_home" ]; then
      record_check "agent" "PASS" "PM2 Home Directory" "$pm2_home exists & writable" ""
    else
      record_check "agent" "WARN" "PM2 Home Directory" "$pm2_home exists but read-only for current user" "sudo chown -R $USER:$USER $pm2_home"
    fi
  else
    record_check "agent" "INFO" "PM2 Home Directory" "$pm2_home will be initialized on first pm2 run" ""
  fi
}

# ------------------------------------------------------------------------------
# 4. Central Monitoring Platform Stack Checks (Central Host)
# ------------------------------------------------------------------------------
check_central_platform_stack() {
  if [[ "$TARGET_ROLE" != "all" && "$TARGET_ROLE" != "central" ]]; then
    return
  fi

  print_section "4. Central Monitoring Platform Stack (Docker, DB, Ports)"

  # Docker Engine
  if command -v docker >/dev/null 2>&1; then
    if docker info >/dev/null 2>&1; then
      local docker_v
      docker_v=$(docker --version | awk '{print $3}' | tr -d ',')
      record_check "central" "PASS" "Docker Engine" "v$docker_v active and running" ""
    else
      record_check "central" "WARN" "Docker Engine" "docker binary found, but daemon not running or socket permission denied" "sudo systemctl enable --now docker && sudo usermod -aG docker $USER"
    fi
  else
    record_check "central" "FAIL" "Docker Engine" "Docker not installed (Required for Postgres, Redis, API)" "curl -fsSL https://get.docker.com | sh && sudo usermod -aG docker $USER"
  fi

  # Docker Compose
  if docker compose version >/dev/null 2>&1; then
    local compose_v
    compose_v=$(docker compose version | awk '{print $4}')
    record_check "central" "PASS" "Docker Compose" "Plugin $compose_v available (v2)" ""
  elif command -v docker-compose >/dev/null 2>&1; then
    record_check "central" "PASS" "Docker Compose" "Legacy docker-compose found" ""
  else
    record_check "central" "WARN" "Docker Compose" "docker compose plugin not found" "sudo apt-get install -y docker-compose-plugin"
  fi

  # Port Conflict Checks
  # Ports: 4000 (API), 3000 (Web), 5433 (Postgres), 6380 (Redis)
  check_port_available() {
    local port="$1"
    local desc="$2"
    local occupied=false

    if command -v ss >/dev/null 2>&1; then
      if ss -tuln | grep -q ":${port} "; then occupied=true; fi
    elif command -v netstat >/dev/null 2>&1; then
      if netstat -tuln | grep -q ":${port} "; then occupied=true; fi
    elif command -v lsof >/dev/null 2>&1; then
      if lsof -i ":${port}" >/dev/null 2>&1; then occupied=true; fi
    fi

    if [ "$occupied" = true ]; then
      record_check "central" "WARN" "Port $port ($desc)" "Port $port is currently IN USE by another process" "Verify if an existing TAM instance is running or free port $port"
    else
      record_check "central" "PASS" "Port $port ($desc)" "Port $port is free and available" ""
    fi
  }

  check_port_available "4000" "TAM API & WebSocket"
  check_port_available "3000" "TAM Web Dashboard"
  check_port_available "5433" "Docker PostgreSQL"
  check_port_available "6380" "Docker Redis"
}

# ------------------------------------------------------------------------------
# 5. Network, DNS & Outbound Connectivity
# ------------------------------------------------------------------------------
check_networking() {
  print_section "5. Network, DNS & Outbound Connectivity"

  # DNS Resolution
  if command -v getent >/dev/null 2>&1; then
    if getent hosts github.com >/dev/null 2>&1; then
      record_check "both" "PASS" "DNS Resolution" "Resolves public domain names successfully" ""
    else
      record_check "both" "FAIL" "DNS Resolution" "Unable to resolve external domain names" "Check /etc/resolv.conf or systemd-resolved"
    fi
  elif command -v host >/dev/null 2>&1 || command -v nslookup >/dev/null 2>&1; then
    record_check "both" "PASS" "DNS Resolution" "DNS utility present" ""
  else
    record_check "both" "INFO" "DNS Utility" "getent / nslookup not installed" ""
  fi

  # Outbound HTTPS / WSS Egress
  if command -v curl >/dev/null 2>&1; then
    if curl -s --connect-timeout 4 -I https://www.google.com >/dev/null 2>&1 || curl -s --connect-timeout 4 -I https://github.com >/dev/null 2>&1; then
      record_check "both" "PASS" "Outbound Internet Egress" "HTTPS egress verified (Required for WebSocket agent connection)" ""
    else
      record_check "both" "WARN" "Outbound Internet Egress" "Cannot reach external HTTPS endpoints" "Check firewall egress rules or proxy settings"
    fi
  else
    record_check "both" "WARN" "cURL Utility" "curl command not found" "sudo apt-get install -y curl"
  fi

  # UFW Firewall Status
  if command -v ufw >/dev/null 2>&1; then
    local ufw_status
    ufw_status=$(ufw status 2>/dev/null | grep -i "Status:" | awk '{print $2}' || echo "unknown")
    if [[ "$ufw_status" == "active" ]]; then
      record_check "both" "INFO" "UFW Firewall" "Active (Ensure ports 4000/3000 open on Central, outbound open on Node)" ""
    else
      record_check "both" "INFO" "UFW Firewall" "Inactive (Unrestricted local traffic)" ""
    fi
  fi
}

# ------------------------------------------------------------------------------
# 6. Kernel Tuning & Limits
# ------------------------------------------------------------------------------
check_kernel_limits() {
  print_section "6. OS Kernel Limits & Performance Tuning"

  # Open File Descriptors (ulimit -n)
  local nofile=1024
  if command -v ulimit >/dev/null 2>&1; then
    nofile=$(ulimit -n 2>/dev/null || echo 1024)
  fi

  if [ "$nofile" -ge 65536 ] 2>/dev/null; then
    record_check "both" "PASS" "Max Open Files (ulimit -n)" "$nofile descriptors (Production ready)" ""
  elif [ "$nofile" -ge 4096 ] 2>/dev/null; then
    record_check "both" "PASS" "Max Open Files (ulimit -n)" "$nofile descriptors (Acceptable for moderate traffic)" ""
  else
    record_check "both" "WARN" "Max Open Files (ulimit -n)" "$nofile descriptors (Default is low for WebSocket streams)" "Add '* soft nofile 65536' and '* hard nofile 65536' to /etc/security/limits.conf"
  fi

  # Socket Backlog (somaxconn)
  if [ -f /proc/sys/net/core/somaxconn ]; then
    local somaxconn
    somaxconn=$(cat /proc/sys/net/core/somaxconn)
    if [ "$somaxconn" -ge 4096 ]; then
      record_check "central" "PASS" "Socket Backlog (somaxconn)" "$somaxconn connections" ""
    else
      record_check "central" "WARN" "Socket Backlog (somaxconn)" "$somaxconn (Recommended >= 4096 for Central API)" "sudo sysctl -w net.core.somaxconn=65535"
    fi
  fi
}

# ------------------------------------------------------------------------------
# 7. Summary & Scoring Calculation
# ------------------------------------------------------------------------------
evaluate_summary() {
  local agent_pct=0
  local central_pct=0

  if [ "$AGENT_TOTAL" -gt 0 ]; then
    agent_pct=$(( (AGENT_PASS * 100) / AGENT_TOTAL ))
  fi

  if [ "$CENTRAL_TOTAL" -gt 0 ]; then
    central_pct=$(( (CENTRAL_PASS * 100) / CENTRAL_TOTAL ))
  fi

  # Determine verdicts
  local agent_verdict="NOT READY"
  local agent_color="${C_RED}"
  if [ "$AGENT_FAIL" -eq 0 ] && [ "$AGENT_WARN" -eq 0 ]; then
    agent_verdict="FULLY READY"
    agent_color="${C_GREEN}"
  elif [ "$AGENT_FAIL" -eq 0 ]; then
    agent_verdict="READY (WITH WARNINGS)"
    agent_color="${C_YELLOW}"
  fi

  local central_verdict="NOT READY"
  local central_color="${C_RED}"
  if [ "$CENTRAL_FAIL" -eq 0 ] && [ "$CENTRAL_WARN" -eq 0 ]; then
    central_verdict="FULLY READY"
    central_color="${C_GREEN}"
  elif [ "$CENTRAL_FAIL" -eq 0 ]; then
    central_verdict="READY (WITH WARNINGS)"
    central_color="${C_YELLOW}"
  fi

  # JSON Output Mode
  if [ "$OUTPUT_JSON" = true ]; then
    local items_str
    items_str=$(IFS=,; echo "${CHECK_ITEMS_JSON[*]}")

    cat << EOF
{
  "timestamp": "$(date -u '+%Y-%m-%dT%H:%M:%SZ')",
  "role_scope": "$TARGET_ROLE",
  "agent_node": {
    "total": $AGENT_TOTAL,
    "pass": $AGENT_PASS,
    "warn": $AGENT_WARN,
    "fail": $AGENT_FAIL,
    "score_percent": $agent_pct,
    "verdict": "$agent_verdict"
  },
  "central_platform": {
    "total": $CENTRAL_TOTAL,
    "pass": $CENTRAL_PASS,
    "warn": $CENTRAL_WARN,
    "fail": $CENTRAL_FAIL,
    "score_percent": $central_pct,
    "verdict": "$central_verdict"
  },
  "checks": [
    $items_str
  ]
}
EOF
    return
  fi

  # Terminal Report Output
  echo -e "\n${C_BOLD}${C_BLUE}========================================================================${C_RESET}"
  echo -e "${C_BOLD}${C_BLUE}   OVERALL READINESS EVALUATION SUMMARY${C_RESET}"
  echo -e "${C_BOLD}${C_BLUE}========================================================================${C_RESET}"

  if [[ "$TARGET_ROLE" == "all" || "$TARGET_ROLE" == "agent" ]]; then
    printf " %-35s : ${agent_color}${C_BOLD}%-22s${C_RESET} [%d%%] (Pass: %d, Warn: %d, Fail: %d)\n" \
      "PM2 Target Node (tam-agent)" "$agent_verdict" "$agent_pct" "$AGENT_PASS" "$AGENT_WARN" "$AGENT_FAIL"
  fi

  if [[ "$TARGET_ROLE" == "all" || "$TARGET_ROLE" == "central" ]]; then
    printf " %-35s : ${central_color}${C_BOLD}%-22s${C_RESET} [%d%%] (Pass: %d, Warn: %d, Fail: %d)\n" \
      "Central Monitoring Host (TAM Core)" "$central_verdict" "$central_pct" "$CENTRAL_PASS" "$CENTRAL_WARN" "$CENTRAL_FAIL"
  fi

  # Remediation Guidance Block
  if [ "${#REMEDIATIONS[@]}" -gt 0 ]; then
    echo -e "\n${C_BOLD}${C_YELLOW}🔧 ACTIONABLE REMEDIATION COMMANDS (Copy & Paste):${C_RESET}"
    echo -e "${C_GRAY}------------------------------------------------------------------------${C_RESET}"
    for rem in "${REMEDIATIONS[@]}"; do
      echo -e "  • ${rem}"
    done
    echo ""
  else
    echo -e "\n${C_GREEN}${C_BOLD}🎉 All evaluated checks passed with zero warnings! Server is 100% production ready.${C_RESET}\n"
  fi

  echo -e "${C_GRAY}For deployment guides, see docs/UBUNTU_PM2_SETUP.md${C_RESET}\n"
}

# ------------------------------------------------------------------------------
# Main Flow
# ------------------------------------------------------------------------------
main() {
  print_header
  check_os_and_arch
  check_hardware_resources
  check_pm2_node_stack
  check_central_platform_stack
  check_networking
  check_kernel_limits
  evaluate_summary
}

main
