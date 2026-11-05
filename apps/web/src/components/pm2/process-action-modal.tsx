"use client";

import React, { useState } from "react";
import { AlertTriangle, RefreshCw, X, CheckCircle, Loader2 } from "lucide-react";

interface ProcessActionModalProps {
  isOpen: boolean;
  onClose: () => void;
  serverHostname: string;
  processName: string;
  action: "restart" | "reload" | "stop" | "clear_logs";
  onConfirm: () => Promise<void>;
}

export function ProcessActionModal({
  isOpen,
  onClose,
  serverHostname,
  processName,
  action,
  onConfirm,
}: ProcessActionModalProps) {
  const [isLoading, setIsLoading] = useState(false);
  const [isDone, setIsDone] = useState(false);

  if (!isOpen) return null;

  const handleAction = async () => {
    setIsLoading(true);
    try {
      await onConfirm();
      setIsDone(true);
      setTimeout(() => {
        setIsDone(false);
        setIsLoading(false);
        onClose();
      }, 1200);
    } catch {
      setIsLoading(false);
    }
  };

  const actionLabels: Record<string, { title: string; color: string; desc: string }> = {
    restart: {
      title: "Restart PM2 Process",
      color: "text-amber-400 bg-amber-500/10 border-amber-500/30",
      desc: "This will terminate the active process and spin up a fresh instance immediately.",
    },
    reload: {
      title: "Zero-Downtime Reload",
      color: "text-sky-400 bg-sky-500/10 border-sky-500/30",
      desc: "This sends a graceful reload signal to PM2, reloading workers sequentially without downtime.",
    },
    stop: {
      title: "Stop PM2 Process",
      color: "text-rose-400 bg-rose-500/10 border-rose-500/30",
      desc: "Warning: This will shut down the process. Traffic routed to this application will receive 502 Bad Gateway until restarted.",
    },
    clear_logs: {
      title: "Flush / Clear Logs",
      color: "text-slate-300 bg-slate-800 border-slate-700",
      desc: "This will truncate stdout and stderr log files for this process on the Ubuntu server.",
    },
  };

  const cfg = actionLabels[action] || actionLabels.restart;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm">
      <div className="bg-[#0f172a] border border-slate-800 rounded-xl max-w-md w-full p-6 shadow-2xl relative">
        <button
          onClick={onClose}
          disabled={isLoading}
          className="absolute top-4 right-4 text-slate-400 hover:text-white"
        >
          <X className="h-5 w-5" />
        </button>

        <div className="flex items-center gap-3 mb-4">
          <div className={`p-2.5 rounded-lg border ${cfg.color}`}>
            <AlertTriangle className="h-5 w-5" />
          </div>
          <div>
            <h3 className="font-semibold text-white text-base">{cfg.title}</h3>
            <p className="text-xs text-slate-400">Confirmation Required</p>
          </div>
        </div>

        <div className="bg-slate-900/80 border border-slate-800 rounded-lg p-3 text-xs mb-4 font-mono space-y-1">
          <div className="text-slate-400">
            Server: <span className="text-slate-200">{serverHostname}</span>
          </div>
          <div className="text-slate-400">
            Target App: <span className="text-sky-400 font-bold">{processName}</span>
          </div>
          <div className="text-slate-400">
            Command: <span className="text-amber-400 uppercase font-bold">{action}</span>
          </div>
        </div>

        <p className="text-xs text-slate-300 mb-6 leading-relaxed">{cfg.desc}</p>

        <div className="flex items-center justify-end gap-3">
          <button
            onClick={onClose}
            disabled={isLoading}
            className="px-4 py-2 text-xs font-medium text-slate-300 hover:text-white bg-slate-800 hover:bg-slate-700 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleAction}
            disabled={isLoading || isDone}
            className={`flex items-center gap-2 px-4 py-2 text-xs font-semibold rounded-lg shadow-sm transition-all ${
              action === "stop"
                ? "bg-rose-600 hover:bg-rose-500 text-white"
                : "bg-sky-600 hover:bg-sky-500 text-white"
            }`}
          >
            {isLoading ? (
              <>
                <Loader2 className="h-4 w-4 animate-spin" />
                <span>Executing...</span>
              </>
            ) : isDone ? (
              <>
                <CheckCircle className="h-4 w-4 text-emerald-300" />
                <span>Dispatched!</span>
              </>
            ) : (
              <>
                <RefreshCw className="h-4 w-4" />
                <span>Confirm {action}</span>
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}

