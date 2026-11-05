"use client";

import React, { useState } from "react";
import { ServerNodeData, ServerNodeCard } from "./server-node-card";
import { Pm2Process, ProcessTable } from "./process-table";
import { LiveLogTerminal } from "./live-log-terminal";
import { EnvironmentTabs, EnvironmentFilter } from "@/components/environment-tabs";
import { PlusCircle, Search } from "lucide-react";

interface Pm2FleetViewProps {
  initialServers?: ServerNodeData[];
  initialProcesses?: Record<string, Pm2Process[]>;
}

export function Pm2FleetView({
  initialServers = [],
  initialProcesses = {},
}: Pm2FleetViewProps) {
  const [filterEnv, setFilterEnv] = useState<EnvironmentFilter>("all");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedServerId, setSelectedServerId] = useState<string>(
    initialServers[0]?.id || "srv-prod-01"
  );
  const [selectedLogProcess, setSelectedLogProcess] = useState<string | null>(null);

  // Fallback realistic servers if initialServers empty
  const servers: ServerNodeData[] = initialServers.length > 0 ? initialServers : [
    {
      id: "srv-prod-01",
      hostname: "ubuntu-prod-gateway",
      ip_address: "10.0.1.10",
      environment: "production",
      os_info: "Ubuntu 24.04 LTS (x86_64)",
      cpu_cores: 8,
      total_memory_mb: 16384,
      disk_usage_percent: 44.5,
      status: "online",
      last_heartbeat: new Date().toISOString(),
    },
    {
      id: "srv-prod-02",
      hostname: "ubuntu-prod-worker",
      ip_address: "10.0.1.11",
      environment: "production",
      os_info: "Ubuntu 24.04 LTS (x86_64)",
      cpu_cores: 8,
      total_memory_mb: 16384,
      disk_usage_percent: 52.0,
      status: "online",
      last_heartbeat: new Date().toISOString(),
    },
    {
      id: "srv-stag-01",
      hostname: "ubuntu-staging-app",
      ip_address: "10.0.2.20",
      environment: "staging",
      os_info: "Ubuntu 22.04 LTS",
      cpu_cores: 4,
      total_memory_mb: 8192,
      disk_usage_percent: 38.0,
      status: "online",
      last_heartbeat: new Date().toISOString(),
    },
    {
      id: "srv-dev-01",
      hostname: "ubuntu-dev-sandbox",
      ip_address: "10.0.3.30",
      environment: "development",
      os_info: "Ubuntu 24.04 LTS",
      cpu_cores: 4,
      total_memory_mb: 8192,
      disk_usage_percent: 29.0,
      status: "online",
      last_heartbeat: new Date().toISOString(),
    },
  ];

  const fallbackProcesses: Record<string, Pm2Process[]> = {
    "srv-prod-01": [
      {
        process_id: 0,
        name: "tam-web-api",
        status: "online",
        cpu_percent: 1.8,
        memory_bytes: 84_500_000,
        uptime_seconds: 43200,
        restart_count: 0,
        git_branch: "main",
        git_commit: "9bf41a2",
        node_version: "v20.18.0",
      },
      {
        process_id: 1,
        name: "order-service",
        status: "online",
        cpu_percent: 3.2,
        memory_bytes: 142_000_000,
        uptime_seconds: 18000,
        restart_count: 1,
        git_branch: "main",
        git_commit: "9bf41a2",
        node_version: "v20.18.0",
      },
      {
        process_id: 2,
        name: "auth-gateway",
        status: "online",
        cpu_percent: 0.9,
        memory_bytes: 65_000_000,
        uptime_seconds: 72000,
        restart_count: 0,
        git_branch: "main",
        git_commit: "8cc21a0",
        node_version: "v20.18.0",
      },
    ],
    "srv-prod-02": [
      {
        process_id: 0,
        name: "email-queue-worker",
        status: "online",
        cpu_percent: 5.4,
        memory_bytes: 180_000_000,
        uptime_seconds: 86400,
        restart_count: 2,
        git_branch: "main",
        git_commit: "9bf41a2",
        node_version: "v20.18.0",
      },
      {
        process_id: 1,
        name: "telemetry-cron",
        status: "online",
        cpu_percent: 0.3,
        memory_bytes: 42_000_000,
        uptime_seconds: 14400,
        restart_count: 0,
        git_branch: "main",
        git_commit: "9bf41a2",
        node_version: "v20.18.0",
      },
    ],
    "srv-stag-01": [
      {
        process_id: 0,
        name: "staging-api",
        status: "online",
        cpu_percent: 2.0,
        memory_bytes: 90_000_000,
        uptime_seconds: 3600,
        restart_count: 0,
        git_branch: "staging",
        git_commit: "2f4e6d1",
        node_version: "v22.2.0",
      },
      {
        process_id: 1,
        name: "test-notifier",
        status: "errored",
        cpu_percent: 0.0,
        memory_bytes: 0,
        uptime_seconds: 0,
        restart_count: 5,
        git_branch: "staging",
        git_commit: "2f4e6d1",
        node_version: "v22.2.0",
      },
    ],
    "srv-dev-01": [
      {
        process_id: 0,
        name: "dev-sandbox-server",
        status: "online",
        cpu_percent: 1.1,
        memory_bytes: 75_000_000,
        uptime_seconds: 1800,
        restart_count: 0,
        git_branch: "feature/pm2-stream",
        git_commit: "1a3b5c7",
        node_version: "v22.2.0",
      },
    ],
  };

  const currentProcessesMap = Object.keys(initialProcesses).length > 0 ? initialProcesses : fallbackProcesses;

  const counts: Record<EnvironmentFilter, number> = {
    all: servers.length,
    production: servers.filter((s) => s.environment === "production").length,
    staging: servers.filter((s) => s.environment === "staging").length,
    development: servers.filter((s) => s.environment === "development").length,
  };

  const filteredServers = servers.filter((s) => {
    const matchesEnv = filterEnv === "all" || s.environment === filterEnv;
    const matchesSearch =
      s.hostname.toLowerCase().includes(searchQuery.toLowerCase()) ||
      s.ip_address.includes(searchQuery);
    return matchesEnv && matchesSearch;
  });

  const selectedServer = servers.find((s) => s.id === selectedServerId) || servers[0];
  const activeProcesses = currentProcessesMap[selectedServer?.id] || [];

  return (
    <div className="space-y-6">
      {/* Header filter controls */}
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <EnvironmentTabs
          current={filterEnv}
          onChange={setFilterEnv}
          counts={counts}
        />

        <div className="relative w-full sm:w-64">
          <Search className="absolute left-3 top-2.5 h-4 w-4 text-slate-500" />
          <input
            type="text"
            placeholder="Search hostname or IP..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-9 pr-4 py-1.5 bg-slate-900/90 border border-slate-800 rounded-lg text-xs text-white placeholder-slate-500 focus:outline-none focus:border-sky-500 transition-colors"
          />
        </div>
      </div>

      {/* Server Node Cards Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {filteredServers.map((server) => (
          <ServerNodeCard
            key={server.id}
            server={server}
            isSelected={server.id === selectedServerId}
            onSelect={(id) => {
              setSelectedServerId(id);
              setSelectedLogProcess(null);
            }}
            processCount={currentProcessesMap[server.id]?.length || 0}
          />
        ))}
      </div>

      {/* Selected Server PM2 Process Fleet */}
      {selectedServer && (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-base font-bold text-white flex items-center gap-2">
                <span>PM2 Applications on</span>
                <span className="text-sky-400 font-mono">{selectedServer.hostname}</span>
              </h2>
              <p className="text-xs text-slate-400">
                Live process status, CPU/memory telemetry, git commits and remote actions
              </p>
            </div>

            {selectedLogProcess && (
              <button
                onClick={() => setSelectedLogProcess(null)}
                className="text-xs text-slate-400 hover:text-white bg-slate-800 px-3 py-1.5 rounded-lg border border-slate-700 transition-colors"
              >
                Hide Log Terminal
              </button>
            )}
          </div>

          {selectedLogProcess && (
            <LiveLogTerminal
              serverHostname={selectedServer.hostname}
              processName={selectedLogProcess}
            />
          )}

          <ProcessTable
            serverHostname={selectedServer.hostname}
            processes={activeProcesses}
            onSelectLogs={(name) => setSelectedLogProcess(name)}
          />
        </div>
      )}
    </div>
  );
}

