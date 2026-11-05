"use client";

import React, { useState } from "react";
import { Navbar } from "@/components/navbar";
import { Pm2FleetView } from "@/components/pm2/pm2-fleet-view";
import { ExecutiveOverview } from "@/components/overview/executive-overview";
import { PacketInspector } from "@/components/network/packet-inspector";
import { AlertCenter } from "@/components/alerts/alert-center";
import { useTrafficStream } from "@/hooks/use-traffic-stream";
import { Server, Activity, Network, Bell } from "lucide-react";

type MainTab = "pm2" | "overview" | "network" | "alerts";

export default function DashboardPage() {
  const [activeTab, setActiveTab] = useState<MainTab>("pm2");
  const { isConnected, metrics } = useTrafficStream();

  const tabs: { key: MainTab; label: string; icon: React.ReactNode; badge?: string }[] = [
    {
      key: "pm2",
      label: "PM2 & Server Fleet",
      icon: <Server className="h-4 w-4" />,
      badge: "Ubuntu Live",
    },
    {
      key: "overview",
      label: "Executive Overview",
      icon: <Activity className="h-4 w-4" />,
    },
    {
      key: "network",
      label: "Network & Traffic",
      icon: <Network className="h-4 w-4" />,
    },
    {
      key: "alerts",
      label: "Alerts & Telegram",
      icon: <Bell className="h-4 w-4" />,
      badge: "1 Alert",
    },
  ];

  return (
    <div className="min-h-screen bg-[#080c14] flex flex-col">
      <Navbar serverCount={4} isWsConnected={isConnected} />

      <main className="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">
        {/* Main Tab Navigation */}
        <div className="flex items-center gap-2 border-b border-slate-800 pb-3">
          {tabs.map((tab) => {
            const isActive = activeTab === tab.key;
            return (
              <button
                key={tab.key}
                onClick={() => setActiveTab(tab.key)}
                className={`flex items-center gap-2 px-3.5 py-2 rounded-lg text-xs font-semibold transition-all ${
                  isActive
                    ? "bg-slate-800 text-white shadow-sm border border-slate-700"
                    : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/40"
                }`}
              >
                <span className={isActive ? "text-sky-400" : "text-slate-500"}>{tab.icon}</span>
                <span>{tab.label}</span>
                {tab.badge && (
                  <span
                    className={`text-[10px] font-mono px-1.5 py-0.2 rounded-full ${
                      tab.key === "alerts"
                        ? "bg-rose-500/20 text-rose-400 border border-rose-500/30"
                        : "bg-sky-500/20 text-sky-400 border border-sky-500/30"
                    }`}
                  >
                    {tab.badge}
                  </span>
                )}
              </button>
            );
          })}
        </div>

        {/* Tab Views */}
        {activeTab === "pm2" && <Pm2FleetView />}
        {activeTab === "overview" && <ExecutiveOverview metrics={metrics} />}
        {activeTab === "network" && <PacketInspector />}
        {activeTab === "alerts" && <AlertCenter />}
      </main>

      <footer className="border-t border-slate-900 py-4 text-center text-xs text-slate-400 font-mono">
        TAM Monitoring Platform v0.1.0 • High-Performance Rust & Next.js 15
      </footer>
    </div>
  );
}

