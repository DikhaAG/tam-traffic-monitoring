import React from "react";

interface MetricCardProps {
  title: string;
  value: string;
  subtitle?: string;
  trend?: string;
  isPositiveTrend?: boolean;
  icon?: React.ReactNode;
}

export function MetricCard({
  title,
  value,
  subtitle,
  trend,
  isPositiveTrend,
  icon,
}: MetricCardProps) {
  return (
    <div className="bg-[#0f172a]/80 backdrop-blur border border-slate-800/80 rounded-xl p-5 hover:border-slate-700 transition-all shadow-sm">
      <div className="flex items-center justify-between text-slate-400 mb-2">
        <span className="text-xs font-semibold uppercase tracking-wider">{title}</span>
        {icon && <div className="text-slate-400">{icon}</div>}
      </div>
      <div className="text-2xl font-bold tracking-tight text-white mb-1 font-mono">
        {value}
      </div>
      {(subtitle || trend) && (
        <div className="flex items-center gap-2 text-xs">
          {trend && (
            <span
              className={`font-medium ${
                isPositiveTrend ? "text-emerald-400" : "text-rose-400"
              }`}
            >
              {trend}
            </span>
          )}
          {subtitle && <span className="text-slate-400">{subtitle}</span>}
        </div>
      )}
    </div>
  );
}

