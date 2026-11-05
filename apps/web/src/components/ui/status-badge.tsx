import React from "react";

interface StatusBadgeProps {
  status: string;
  size?: "sm" | "md";
}

export function StatusBadge({ status, size = "md" }: StatusBadgeProps) {
  const norm = status.toLowerCase();

  const isOnline = norm === "online" || norm === "active" || norm === "healthy";
  const isError = norm === "errored" || norm === "error" || norm === "offline" || norm === "failed";
  const isWarning = norm === "restarting" || norm === "stopping" || norm === "warning";

  let bgClass = "bg-slate-800 text-slate-400 border-slate-700";
  let dotClass = "bg-slate-400";

  if (isOnline) {
    bgClass = "bg-emerald-500/10 text-emerald-400 border-emerald-500/30";
    dotClass = "bg-emerald-400 animate-pulse";
  } else if (isError) {
    bgClass = "bg-rose-500/10 text-rose-400 border-rose-500/30";
    dotClass = "bg-rose-400 animate-pulse";
  } else if (isWarning) {
    bgClass = "bg-amber-500/10 text-amber-400 border-amber-500/30";
    dotClass = "bg-amber-400 animate-pulse";
  }

  const px = size === "sm" ? "px-2 py-0.5 text-xs" : "px-2.5 py-1 text-xs";

  return (
    <span className={`inline-flex items-center gap-1.5 font-medium rounded-full border ${bgClass} ${px}`}>
      <span className={`w-1.5 h-1.5 rounded-full ${dotClass}`} />
      <span className="capitalize">{status}</span>
    </span>
  );
}

