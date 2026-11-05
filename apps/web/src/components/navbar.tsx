"use client";

import React, { useState, useEffect } from "react";
import { Activity, ShieldCheck, Server, Radio } from "lucide-react";

interface NavbarProps {
  serverCount: number;
  isWsConnected: boolean;
}

export function Navbar({ serverCount, isWsConnected }: NavbarProps) {
  const [time, setTime] = useState<string>("");

  useEffect(() => {
    const updateTime = () => {
      const now = new Date();
      setTime(now.toISOString().replace("T", " ").substring(0, 19) + " UTC");
    };
    updateTime();
    const interval = setInterval(updateTime, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <header className="sticky top-0 z-40 border-b border-slate-800/80 bg-[#080c14]/90 backdrop-blur">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="h-9 w-9 rounded-lg bg-gradient-to-br from-sky-500 to-indigo-600 flex items-center justify-center shadow-lg shadow-sky-500/20">
            <Activity className="h-5 w-5 text-white" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <span className="font-bold text-lg tracking-tight text-white">TAM</span>
              <span className="text-xs bg-sky-500/10 text-sky-400 border border-sky-500/30 font-mono px-1.5 py-0.5 rounded">
                v0.1.0-mvp
              </span>
            </div>
            <p className="text-[11px] text-slate-400 font-medium">Traffic & PM2 Server Fleet Monitoring</p>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <div className="hidden sm:flex items-center gap-2 bg-slate-900/80 border border-slate-800 rounded-lg px-3 py-1.5 text-xs">
            <Server className="h-3.5 w-3.5 text-slate-400" />
            <span className="text-slate-400">Connected Nodes:</span>
            <span className="font-semibold text-white font-mono">{serverCount}</span>
          </div>

          <div className="flex items-center gap-2 bg-slate-900/80 border border-slate-800 rounded-lg px-3 py-1.5 text-xs">
            <Radio className={`h-3.5 w-3.5 ${isWsConnected ? "text-emerald-400 animate-pulse" : "text-rose-400"}`} />
            <span className="text-slate-400">Live Feed:</span>
            <span className={`font-semibold ${isWsConnected ? "text-emerald-400" : "text-rose-400"}`}>
              {isWsConnected ? "Streaming" : "Disconnected"}
            </span>
          </div>

          <div className="hidden md:block text-xs font-mono text-slate-400">
            {time}
          </div>
        </div>
      </div>
    </header>
  );
}

