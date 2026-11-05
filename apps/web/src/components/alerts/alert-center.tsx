"use client";

import React, { useState } from "react";
import { AlertCircle, Send, CheckCircle2, ShieldAlert, Zap } from "lucide-react";
import { StatusBadge } from "@/components/ui/status-badge";

export function AlertCenter() {
  const [isSendingTest, setIsSendingTest] = useState(false);
  const [testSent, setTestSent] = useState(false);
  const [isInjectingSpike, setIsInjectingSpike] = useState(false);

  const initialAlerts = [
    {
      id: "al-1",
      category: "pm2_crash",
      severity: "critical",
      target: "srv-stag-01 / test-notifier",
      description: "Process crashed with exit code 1: Unhandled exception ReferenceError: config is not defined",
      timestamp: "5 mins ago",
      resolved: false,
    },
    {
      id: "al-2",
      category: "restart_loop",
      severity: "warning",
      target: "srv-prod-02 / email-queue-worker",
      description: "Process restarted 2 times within 5 minutes due to memory threshold limit (200MB exceeded)",
      timestamp: "24 mins ago",
      resolved: true,
    },
    {
      id: "al-3",
      category: "traffic_spike",
      severity: "warning",
      target: "Network Gateway (Port 443)",
      description: "Inbound throughput spiked to 450 RPS (3.2x baseline moving average)",
      timestamp: "1 hour ago",
      resolved: true,
    },
  ];

  const handleTestTelegram = async () => {
    setIsSendingTest(true);
    setTimeout(() => {
      setIsSendingTest(false);
      setTestSent(true);
      setTimeout(() => setTestSent(false), 3000);
    }, 1000);
  };

  const handleInjectSpike = () => {
    setIsInjectingSpike(true);
    setTimeout(() => setIsInjectingSpike(false), 2000);
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
      {/* Alert Feed */}
      <div className="lg:col-span-2 space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="font-semibold text-white text-sm">Incident & Anomaly Feed</h3>
            <p className="text-xs text-slate-400">Triggered by PM2 crash detector, node heartbeat monitor, and traffic threshold rules</p>
          </div>
          <span className="text-xs font-mono bg-rose-500/10 text-rose-400 border border-rose-500/30 px-2 py-0.5 rounded">
            1 Active Incident
          </span>
        </div>

        <div className="space-y-3 font-mono text-xs">
          {initialAlerts.map((alert) => (
            <div
              key={alert.id}
              className={`p-4 rounded-xl border ${
                alert.severity === "critical" && !alert.resolved
                  ? "bg-rose-950/20 border-rose-500/40 shadow-lg shadow-rose-950/30"
                  : "bg-[#0f172a]/90 border-slate-800"
              }`}
            >
              <div className="flex items-start justify-between gap-3 mb-2 font-sans">
                <div className="flex items-center gap-2">
                  <AlertCircle
                    className={`h-4 w-4 ${
                      alert.severity === "critical" ? "text-rose-400" : "text-amber-400"
                    }`}
                  />
                  <span className="font-semibold text-white text-sm">{alert.target}</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-slate-400 text-[11px] font-mono">{alert.timestamp}</span>
                  <StatusBadge status={alert.resolved ? "Resolved" : "Active"} size="sm" />
                </div>
              </div>

              <p className="text-slate-300 leading-relaxed font-mono text-[11px] mb-2">{alert.description}</p>

              <div className="flex items-center gap-3 text-[10px] text-slate-500 font-mono">
                <span>CATEGORY: {alert.category.toUpperCase()}</span>
                <span>SEVERITY: {alert.severity.toUpperCase()}</span>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Telegram & Simulator Settings Card */}
      <div className="space-y-6">
        <div className="bg-[#0f172a]/90 border border-slate-800 rounded-xl p-5 shadow-lg space-y-4">
          <div className="flex items-center gap-2.5">
            <div className="p-2 rounded-lg bg-sky-500/10 border border-sky-500/30 text-sky-400">
              <Send className="h-4 w-4" />
            </div>
            <div>
              <h4 className="font-semibold text-white text-sm">Telegram Bot Alerts</h4>
              <p className="text-[11px] text-slate-400">Instant notification to group/channel</p>
            </div>
          </div>

          <div className="space-y-2.5 text-xs font-mono">
            <div>
              <label className="text-slate-400 text-[11px]">Bot Token Status</label>
              <div className="mt-1 px-3 py-1.5 bg-slate-900 border border-slate-800 rounded text-slate-300 truncate">
                {process.env.TELEGRAM_BOT_TOKEN ? "••••••••••••••••" : "Configured via .env"}
              </div>
            </div>
            <div>
              <label className="text-slate-400 text-[11px]">Chat ID / Group</label>
              <div className="mt-1 px-3 py-1.5 bg-slate-900 border border-slate-800 rounded text-slate-300 truncate">
                {process.env.TELEGRAM_CHAT_ID ? process.env.TELEGRAM_CHAT_ID : "Configured via .env"}
              </div>
            </div>
          </div>

          <button
            onClick={handleTestTelegram}
            disabled={isSendingTest || testSent}
            className="w-full flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-semibold bg-sky-600 hover:bg-sky-500 text-white transition-all shadow-sm"
          >
            {isSendingTest ? (
              <span>Dispatching...</span>
            ) : testSent ? (
              <>
                <CheckCircle2 className="h-3.5 w-3.5" />
                <span>Test Alert Sent!</span>
              </>
            ) : (
              <>
                <Send className="h-3.5 w-3.5" />
                <span>Trigger Test Telegram Alert</span>
              </>
            )}
          </button>
        </div>

        {/* Traffic Spike Injection */}
        <div className="bg-[#0f172a]/90 border border-slate-800 rounded-xl p-5 shadow-lg space-y-3">
          <div className="flex items-center gap-2">
            <Zap className="h-4 w-4 text-amber-400" />
            <h4 className="font-semibold text-white text-sm">Chaos Simulator</h4>
          </div>
          <p className="text-xs text-slate-400 leading-relaxed">
            Inject a synthetic 5x traffic surge or simulated PM2 crash to test end-to-end alerting.
          </p>
          <button
            onClick={handleInjectSpike}
            disabled={isInjectingSpike}
            className="w-full flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-semibold bg-slate-800 hover:bg-slate-700 text-amber-400 border border-amber-500/30 transition-all"
          >
            {isInjectingSpike ? (
              <span>Injecting 5x Spike (5s)...</span>
            ) : (
              <>
                <Zap className="h-3.5 w-3.5" />
                <span>Inject Traffic Spike (5x)</span>
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}

