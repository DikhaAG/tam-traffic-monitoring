"use client";

import React from "react";
import { Server, Cpu, HardDrive, Clock, Shield } from "lucide-react";
import { StatusBadge } from "@/components/ui/status-badge";

export interface ServerNodeData {
  id: string;
  hostname: string;
  ip_address: string;
  environment: "production" | "staging" | "development";
  os_info: string;
  cpu_cores: number;
  total_memory_mb: number;
  disk_usage_percent: number;
  status: string;
  last_heartbeat: string;
}

interface ServerNodeCardProps {
  server: ServerNodeData;
  isSelected: boolean;
  onSelect: (id: string) => void;
  processCount: number;
}

export function ServerNodeCard({
  server,
  isSelected,
  onSelect,
  processCount,
}: ServerNodeCardProps) {
  const envBadgeColors = {
    production: "bg-rose-500/10 text-rose-400 border-rose-500/30",
    staging: "bg-amber-500/10 text-amber-400 border-amber-500/30",
    development: "bg-sky-500/10 text-sky-400 border-sky-500/30",
  };

  return (
    <div
      onClick={() => onSelect(server.id)}
      className={`p-4 rounded-xl border cursor-pointer transition-all ${
        isSelected
          ? "bg-slate-900 border-sky-500/80 shadow-lg shadow-sky-500/10 ring-1 ring-sky-500/40"
          : "bg-[#0f172a]/70 border-slate-800 hover:border-slate-700 hover:bg-slate-900/50"
      }`}
    >
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2.5">
          <div className="p-2 rounded-lg bg-slate-800 border border-slate-700/60 text-slate-300">
            <Server className="h-4 w-4" />
          </div>
          <div>
            <div className="font-semibold text-white text-sm tracking-tight">{server.hostname}</div>
            <div className="text-[11px] text-slate-400 font-mono">{server.ip_address}</div>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <span
            className={`text-[10px] font-mono uppercase px-2 py-0.5 rounded-full border ${
              envBadgeColors[server.environment]
            }`}
          >
            {server.environment}
          </span>
          <StatusBadge status={server.status} size="sm" />
        </div>
      </div>

      <div className="grid grid-cols-3 gap-2 py-2 border-y border-slate-800/80 text-xs font-mono text-slate-300 mb-3">
        <div className="flex items-center gap-1.5">
          <Cpu className="h-3.5 w-3.5 text-sky-400" />
          <span>{server.cpu_cores} Cores</span>
        </div>
        <div className="flex items-center gap-1.5">
          <HardDrive className="h-3.5 w-3.5 text-indigo-400" />
          <span>{(server.total_memory_mb / 1024).toFixed(0)} GB RAM</span>
        </div>
        <div className="flex items-center gap-1.5">
          <Shield className="h-3.5 w-3.5 text-emerald-400" />
          <span>{processCount} PM2 Apps</span>
        </div>
      </div>

      <div className="flex items-center justify-between text-[11px] text-slate-400">
        <span className="truncate max-w-[180px]">{server.os_info}</span>
        <div className="flex items-center gap-1">
          <Clock className="h-3 w-3" />
          <span>Heartbeat OK</span>
        </div>
      </div>
    </div>
  );
}

