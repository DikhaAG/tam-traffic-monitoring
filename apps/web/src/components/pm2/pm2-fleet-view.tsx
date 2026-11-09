"use client";

import React, { useState, useEffect } from "react";
import { ServerNodeData, ServerNodeCard } from "./server-node-card";
import { Pm2Process, ProcessTable } from "./process-table";
import { LiveLogTerminal } from "./live-log-terminal";
import { EnvironmentTabs, EnvironmentFilter } from "@/components/environment-tabs";
import { Search } from "lucide-react";
import { getApiBaseUrl, getWsBaseUrl } from "@/lib/api-config";

interface Pm2FleetViewProps {
  initialServers?: ServerNodeData[];
  initialProcesses?: Record<string, Pm2Process[]>;
  onServerCountChange?: (count: number) => void;
}

export function Pm2FleetView({
  initialServers = [],
  initialProcesses = {},
  onServerCountChange,
}: Pm2FleetViewProps) {
  const [servers, setServers] = useState<ServerNodeData[]>(initialServers);
  const [processesMap, setProcessesMap] = useState<Record<string, Pm2Process[]>>(initialProcesses);
  const [filterEnv, setFilterEnv] = useState<EnvironmentFilter>("all");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedServerId, setSelectedServerId] = useState<string>(
    initialServers[0]?.id || ""
  );
  const [selectedLogProcess, setSelectedLogProcess] = useState<string | null>(null);

  // 1. Fetch live servers from backend API
  useEffect(() => {
    let isMounted = true;

    async function fetchServers() {
      try {
        const apiUrl = getApiBaseUrl();
        const res = await fetch(`${apiUrl}/api/v1/servers`, { cache: "no-store" });
        if (res.ok) {
          const data: ServerNodeData[] = await res.json();
          if (isMounted && data && data.length > 0) {
            setServers(data);
            if (onServerCountChange) {
              onServerCountChange(data.length);
            }
            setSelectedServerId((prev) => {
              if (!prev || !data.some((s) => s.id === prev)) {
                return data[0].id;
              }
              return prev;
            });
          }
        }
      } catch (err) {
        console.warn("Failed to fetch live servers from API:", err);
      }
    }

    fetchServers();
    const interval = setInterval(fetchServers, 5000);

    return () => {
      isMounted = false;
      clearInterval(interval);
    };
  }, [onServerCountChange]);

  // 2. Fetch PM2 processes for selected server
  useEffect(() => {
    if (!selectedServerId) return;

    let isMounted = true;

    async function fetchProcesses() {
      try {
        const apiUrl = getApiBaseUrl();
        const res = await fetch(`${apiUrl}/api/v1/servers/${selectedServerId}/pm2`, { cache: "no-store" });
        if (res.ok) {
          const procs: Pm2Process[] = await res.json();
          if (isMounted) {
            setProcessesMap((prev) => ({
              ...prev,
              [selectedServerId]: procs,
            }));
          }
        }
      } catch (err) {
        console.warn(`Failed to fetch PM2 processes for ${selectedServerId}:`, err);
      }
    }

    fetchProcesses();
    const interval = setInterval(fetchProcesses, 3000);

    return () => {
      isMounted = false;
      clearInterval(interval);
    };
  }, [selectedServerId]);

  // 3. Fallback mock data only if API returns empty and no servers connected yet
  const fallbackServers: ServerNodeData[] = [
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
  ];

  const activeServers = servers.length > 0 ? servers : fallbackServers;
  const currentSelectedId = selectedServerId || activeServers[0]?.id || "srv-prod-01";

  const counts: Record<EnvironmentFilter, number> = {
    all: activeServers.length,
    production: activeServers.filter((s) => s.environment === "production").length,
    staging: activeServers.filter((s) => s.environment === "staging").length,
    development: activeServers.filter((s) => s.environment === "development").length,
  };

  const filteredServers = activeServers.filter((s) => {
    const matchesEnv = filterEnv === "all" || s.environment === filterEnv;
    const matchesSearch =
      s.hostname.toLowerCase().includes(searchQuery.toLowerCase()) ||
      s.ip_address.includes(searchQuery);
    return matchesEnv && matchesSearch;
  });

  const selectedServer = activeServers.find((s) => s.id === currentSelectedId) || activeServers[0];
  const activeProcesses = processesMap[selectedServer?.id] || [];

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
        {filteredServers.map((server) => {
          const count = processesMap[server.id]?.length || 0;
          return (
            <ServerNodeCard
              key={server.id}
              server={server}
              isSelected={server.id === currentSelectedId}
              onSelect={(id) => {
                setSelectedServerId(id);
                setSelectedLogProcess(null);
              }}
              processCount={count}
            />
          );
        })}
      </div>

      {/* Selected Server PM2 Process Fleet */}
      {selectedServer && (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-base font-bold text-white flex items-center gap-2">
                <span>PM2 Applications on</span>
                <span className="text-sky-400 font-mono">{selectedServer.hostname}</span>
                <span className="text-xs bg-slate-800 text-slate-400 border border-slate-700 font-mono px-2 py-0.5 rounded-full">
                  {activeProcesses.length} Apps Active
                </span>
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
            serverId={selectedServer.id}
            serverHostname={selectedServer.hostname}
            processes={activeProcesses}
            onSelectLogs={(name) => setSelectedLogProcess(name)}
          />
        </div>
      )}
    </div>
  );
}
