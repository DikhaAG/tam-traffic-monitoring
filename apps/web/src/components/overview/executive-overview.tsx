"use client";

import React from "react";
import { MetricCard } from "@/components/ui/metric-card";
import { MetricsSnapshot } from "@/hooks/use-traffic-stream";
import { ArrowUpRight, ArrowDownLeft, Zap, Clock, AlertOctagon, Users } from "lucide-react";

interface ExecutiveOverviewProps {
  metrics: MetricsSnapshot;
}

export function ExecutiveOverview({ metrics }: ExecutiveOverviewProps) {
  const formatBandwidth = (bps: number) => {
    if (bps >= 1_000_000_000) return (bps / 1_000_000_000).toFixed(2) + " Gbps";
    if (bps >= 1_000_000) return (bps / 1_000_000).toFixed(1) + " Mbps";
    return (bps / 1_000).toFixed(0) + " Kbps";
  };

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <MetricCard
          title="Inbound Traffic"
          value={formatBandwidth(metrics.bandwidth_in_bps)}
          subtitle="Packets ingress"
          trend="+12.4% vs last hour"
          isPositiveTrend={true}
          icon={<ArrowDownLeft className="h-5 w-5 text-sky-400" />}
        />
        <MetricCard
          title="Outbound Traffic"
          value={formatBandwidth(metrics.bandwidth_out_bps)}
          subtitle="Egress throughput"
          trend="+8.1% normal"
          isPositiveTrend={true}
          icon={<ArrowUpRight className="h-5 w-5 text-indigo-400" />}
        />
        <MetricCard
          title="Total Throughput"
          value={`${metrics.total_rps.toFixed(0)} RPS`}
          subtitle="Requests per second"
          trend="Stable"
          isPositiveTrend={true}
          icon={<Zap className="h-5 w-5 text-amber-400" />}
        />
        <MetricCard
          title="Active Connections"
          value={`${metrics.active_connections}`}
          subtitle="Unique client IPs"
          trend="Distributed"
          isPositiveTrend={true}
          icon={<Users className="h-5 w-5 text-emerald-400" />}
        />
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <MetricCard
          title="Latency (p50 Median)"
          value={`${metrics.latency_p50_ms.toFixed(1)} ms`}
          subtitle="Fast responses"
          isPositiveTrend={true}
          icon={<Clock className="h-5 w-5 text-emerald-400" />}
        />
        <MetricCard
          title="Latency (p95 Percentile)"
          value={`${metrics.latency_p95_ms.toFixed(1)} ms`}
          subtitle="Tail latency threshold: 250ms"
          isPositiveTrend={metrics.latency_p95_ms < 200}
          icon={<Clock className="h-5 w-5 text-sky-400" />}
        />
        <MetricCard
          title="HTTP Error Rate (4xx / 5xx)"
          value={`${metrics.error_rate_percent.toFixed(2)}%`}
          subtitle="SLO Target < 1.0%"
          trend={metrics.error_rate_percent < 1.0 ? "SLO Compliant" : "Degraded"}
          isPositiveTrend={metrics.error_rate_percent < 1.0}
          icon={<AlertOctagon className="h-5 w-5 text-rose-400" />}
        />
      </div>
    </div>
  );
}

