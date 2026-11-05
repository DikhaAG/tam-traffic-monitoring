"use client";

import React from "react";

export type EnvironmentFilter = "all" | "production" | "staging" | "development";

interface EnvironmentTabsProps {
  current: EnvironmentFilter;
  onChange: (env: EnvironmentFilter) => void;
  counts: Record<EnvironmentFilter, number>;
}

export function EnvironmentTabs({ current, onChange, counts }: EnvironmentTabsProps) {
  const tabs: { key: EnvironmentFilter; label: string }[] = [
    { key: "all", label: "All Servers" },
    { key: "production", label: "Production" },
    { key: "staging", label: "Staging" },
    { key: "development", label: "Development" },
  ];

  return (
    <div className="flex items-center gap-1.5 p-1 bg-slate-900/80 border border-slate-800 rounded-lg w-fit">
      {tabs.map((tab) => {
        const isActive = current === tab.key;
        const count = counts[tab.key] || 0;

        return (
          <button
            key={tab.key}
            onClick={() => onChange(tab.key)}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition-all ${
              isActive
                ? "bg-slate-800 text-white shadow-sm border border-slate-700/50"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/40"
            }`}
          >
            <span>{tab.label}</span>
            <span
              className={`px-1.5 py-0.2 rounded-full text-[10px] font-mono ${
                isActive
                  ? "bg-sky-500/20 text-sky-400 border border-sky-500/30"
                  : "bg-slate-800 text-slate-400"
              }`}
            >
              {count}
            </span>
          </button>
        );
      })}
    </div>
  );
}

