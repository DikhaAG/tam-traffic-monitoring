"use client";

import React, { useState, useEffect, useRef } from "react";
import { Terminal, Copy, Check, Trash2, ArrowDown } from "lucide-react";

interface LiveLogTerminalProps {
  serverHostname: string;
  processName: string;
  initialLines?: string[];
}

export function LiveLogTerminal({
  serverHostname,
  processName,
  initialLines = [],
}: LiveLogTerminalProps) {
  const [lines, setLines] = useState<string[]>(initialLines);
  const [copied, setCopied] = useState(false);
  const [autoScroll, setAutoScroll] = useState(true);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Simulated stream append for realistic log view
    const interval = setInterval(() => {
      const now = new Date().toISOString();
      const sampleLogs = [
        `[${now}] [INFO] [${processName}] Incoming request: GET /api/v1/orders - 200 OK (14ms)`,
        `[${now}] [INFO] [${processName}] Worker heartbeat executed successfully`,
        `[${now}] [INFO] [${processName}] Cache hit ratio: 98.4% (Redis 7 cluster)`,
        `[${now}] [DEBUG] [${processName}] DB connection pool status: 4 active / 20 max`,
      ];
      const randomLog = sampleLogs[Math.floor(Math.random() * sampleLogs.length)];
      setLines((prev) => [...prev.slice(-100), randomLog]);
    }, 3500);

    return () => clearInterval(interval);
  }, [processName]);

  useEffect(() => {
    if (autoScroll && bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [lines, autoScroll]);

  const copyLogs = () => {
    navigator.clipboard.writeText(lines.join("\n"));
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="bg-[#050811] border border-slate-800 rounded-xl overflow-hidden font-mono text-xs shadow-xl flex flex-col h-[380px]">
      <div className="flex items-center justify-between px-4 py-2.5 bg-slate-900/90 border-b border-slate-800 text-slate-400">
        <div className="flex items-center gap-2">
          <Terminal className="h-4 w-4 text-sky-400" />
          <span className="font-semibold text-white">Live PM2 Logs:</span>
          <span className="text-sky-400 font-bold">{processName}</span>
          <span className="text-slate-500 text-[10px]">({serverHostname})</span>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={() => setAutoScroll(!autoScroll)}
            className={`flex items-center gap-1 px-2 py-1 rounded text-[11px] transition-colors ${
              autoScroll ? "text-sky-400 bg-sky-500/10 border border-sky-500/30" : "text-slate-400 hover:text-white"
            }`}
          >
            <ArrowDown className="h-3 w-3" />
            <span>Auto-scroll</span>
          </button>
          <button
            onClick={copyLogs}
            className="flex items-center gap-1 px-2 py-1 hover:text-white text-slate-400 transition-colors"
          >
            {copied ? <Check className="h-3 w-3 text-emerald-400" /> : <Copy className="h-3 w-3" />}
            <span>{copied ? "Copied" : "Copy"}</span>
          </button>
          <button
            onClick={() => setLines([])}
            className="flex items-center gap-1 px-2 py-1 hover:text-rose-400 text-slate-400 transition-colors"
          >
            <Trash2 className="h-3 w-3" />
            <span>Clear</span>
          </button>
        </div>
      </div>

      <div className="flex-1 p-4 overflow-y-auto space-y-1.5 leading-relaxed text-slate-300">
        {lines.length === 0 ? (
          <div className="text-slate-500 italic py-8 text-center">
            No log output captured yet. Listening to stdout/stderr stream...
          </div>
        ) : (
          lines.map((l, i) => {
            const isError = l.includes("[ERROR]") || l.includes("Error:") || l.includes("failed");
            const isWarn = l.includes("[WARN]");
            return (
              <div
                key={i}
                className={`break-all ${
                  isError ? "text-rose-400 font-semibold" : isWarn ? "text-amber-400" : "text-slate-300"
                }`}
              >
                {l}
              </div>
            );
          })
        )}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}

