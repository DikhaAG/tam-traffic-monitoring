"use client";

import React, { useState } from "react";
import { GitBranch, GitCommit, RefreshCw, RotateCcw, Square, Terminal, Cpu, HardDrive } from "lucide-react";
import { StatusBadge } from "@/components/ui/status-badge";
import { ProcessActionModal } from "./process-action-modal";
import { getApiBaseUrl } from "@/lib/api-config";

export interface Pm2Process {
  process_id: number;
  name: string;
  status: string;
  cpu_percent: number;
  memory_bytes: number;
  uptime_seconds: number;
  restart_count: number;
  git_branch?: string;
  git_commit?: string;
  git_commit_msg?: string;
  node_version?: string;
}

interface ProcessTableProps {
  serverId: string;
  serverHostname: string;
  processes: Pm2Process[];
  onSelectLogs: (processName: string) => void;
}

export function ProcessTable({
  serverId,
  serverHostname,
  processes,
  onSelectLogs,
}: ProcessTableProps) {
  const [modalState, setModalState] = useState<{
    isOpen: boolean;
    processName: string;
    action: "restart" | "reload" | "stop" | "clear_logs";
  }>({
    isOpen: false,
    processName: "",
    action: "restart",
  });

  const formatMemory = (bytes: number) => {
    return (bytes / 1024 / 1024).toFixed(1) + " MB";
  };

  const formatUptime = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    if (hours > 0) return `${hours}h ${mins}m`;
    return `${mins}m`;
  };

  const handleActionClick = (processName: string, action: "restart" | "reload" | "stop" | "clear_logs") => {
    setModalState({
      isOpen: true,
      processName,
      action,
    });
  };

  const executeAction = async () => {
    // Send action payload to backend
    const apiUrl = getApiBaseUrl();
    await fetch(`${apiUrl}/api/v1/servers/${serverId}/pm2/${modalState.processName}/action`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ action: modalState.action, issued_by: "web-dashboard" }),
    });
  };

  return (
    <div className="bg-[#0f172a]/90 border border-slate-800 rounded-xl overflow-hidden shadow-lg">
      <div className="overflow-x-auto">
        <table className="w-full text-left border-collapse text-xs">
          <thead>
            <tr className="border-b border-slate-800 bg-slate-900/60 text-slate-400 uppercase tracking-wider font-semibold">
              <th className="py-3 px-4">PM2 ID & App Name</th>
              <th className="py-3 px-4">Status</th>
              <th className="py-3 px-4">CPU Usage</th>
              <th className="py-3 px-4">Memory</th>
              <th className="py-3 px-4">Uptime / Restarts</th>
              <th className="py-3 px-4">Git Version</th>
              <th className="py-3 px-4 text-right">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800/60 font-mono">
            {processes.map((proc) => (
              <tr key={proc.process_id} className="hover:bg-slate-800/30 transition-colors">
                <td className="py-3 px-4">
                  <div className="flex items-center gap-2">
                    <span className="text-slate-500 font-bold">#{proc.process_id}</span>
                    <span className="font-semibold text-white text-sm">{proc.name}</span>
                    {proc.node_version && (
                      <span className="text-[10px] text-slate-400 bg-slate-800 px-1.5 py-0.5 rounded">
                        {proc.node_version}
                      </span>
                    )}
                  </div>
                </td>

                <td className="py-3 px-4">
                  <StatusBadge status={proc.status} size="sm" />
                </td>

                <td className="py-3 px-4">
                  <div className="flex items-center gap-1.5 text-slate-200">
                    <Cpu className="h-3.5 w-3.5 text-sky-400" />
                    <span>{proc.cpu_percent.toFixed(1)}%</span>
                  </div>
                </td>

                <td className="py-3 px-4">
                  <div className="flex items-center gap-1.5 text-slate-200">
                    <HardDrive className="h-3.5 w-3.5 text-indigo-400" />
                    <span>{formatMemory(proc.memory_bytes)}</span>
                  </div>
                </td>

                <td className="py-3 px-4">
                  <div className="text-slate-200">{formatUptime(proc.uptime_seconds)}</div>
                  <div className={`text-[10px] ${proc.restart_count > 3 ? "text-rose-400 font-bold" : "text-slate-400"}`}>
                    Restarts: {proc.restart_count}
                  </div>
                </td>

                <td className="py-3 px-4">
                  <div className="flex items-center gap-2">
                    {proc.git_branch && (
                      <span className="inline-flex items-center gap-1 text-[11px] text-slate-300">
                        <GitBranch className="h-3 w-3 text-sky-400" />
                        <span>{proc.git_branch}</span>
                      </span>
                    )}
                    {proc.git_commit && (
                      <span className="inline-flex items-center gap-1 text-[10px] bg-slate-800 text-slate-400 px-1.5 py-0.5 rounded border border-slate-700">
                        <GitCommit className="h-2.5 w-2.5 text-slate-400" />
                        <span>{proc.git_commit}</span>
                      </span>
                    )}
                  </div>
                </td>

                <td className="py-3 px-4 text-right">
                  <div className="flex items-center justify-end gap-1.5">
                    <button
                      title="View live stdout/stderr logs"
                      onClick={() => onSelectLogs(proc.name)}
                      className="p-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-sky-400 transition-colors"
                    >
                      <Terminal className="h-3.5 w-3.5" />
                    </button>
                    <button
                      title="Zero-downtime reload"
                      onClick={() => handleActionClick(proc.name, "reload")}
                      className="p-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-emerald-400 transition-colors"
                    >
                      <RotateCcw className="h-3.5 w-3.5" />
                    </button>
                    <button
                      title="Restart process"
                      onClick={() => handleActionClick(proc.name, "restart")}
                      className="p-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-amber-400 transition-colors"
                    >
                      <RefreshCw className="h-3.5 w-3.5" />
                    </button>
                    <button
                      title="Stop process"
                      onClick={() => handleActionClick(proc.name, "stop")}
                      className="p-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-rose-400 transition-colors"
                    >
                      <Square className="h-3.5 w-3.5" />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <ProcessActionModal
        isOpen={modalState.isOpen}
        onClose={() => setModalState((prev) => ({ ...prev, isOpen: false }))}
        serverHostname={serverHostname}
        processName={modalState.processName}
        action={modalState.action}
        onConfirm={executeAction}
      />
    </div>
  );
}

