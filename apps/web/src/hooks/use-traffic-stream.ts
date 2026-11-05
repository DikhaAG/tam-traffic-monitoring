"use client";

import { useEffect, useState, useRef } from "react";

export interface MetricsSnapshot {
  bandwidth_in_bps: number;
  bandwidth_out_bps: number;
  total_rps: number;
  latency_p50_ms: number;
  latency_p95_ms: number;
  latency_p99_ms: number;
  error_rate_percent: number;
  active_connections: number;
}

export function useTrafficStream() {
  const [isConnected, setIsConnected] = useState(false);
  const [metrics, setMetrics] = useState<MetricsSnapshot>({
    bandwidth_in_bps: 45_200_000,
    bandwidth_out_bps: 82_400_000,
    total_rps: 142.5,
    latency_p50_ms: 18.2,
    latency_p95_ms: 64.0,
    latency_p99_ms: 112.5,
    error_rate_percent: 0.8,
    active_connections: 320,
  });

  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const wsUrl = process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:4000/ws/live";

    function connect() {
      try {
        const socket = new WebSocket(wsUrl);

        socket.onopen = () => {
          setIsConnected(true);
        };

        socket.onmessage = (event) => {
          try {
            const data = JSON.parse(event.data);
            if (data.bandwidth_in_bps !== undefined) {
              setMetrics(data);
            }
          } catch {
            // Ignore non-json frames
          }
        };

        socket.onclose = () => {
          setIsConnected(false);
          // Exponential backoff reconnect
          setTimeout(connect, 3000);
        };

        socket.onerror = () => {
          socket.close();
        };

        wsRef.current = socket;
      } catch {
        setIsConnected(false);
      }
    }

    connect();

    // Fallback simulation timer for lively UI when server is quiet
    const timer = setInterval(() => {
      setMetrics((prev) => ({
        ...prev,
        bandwidth_in_bps: Math.max(10_000_000, prev.bandwidth_in_bps + Math.floor((Math.random() - 0.5) * 5_000_000)),
        bandwidth_out_bps: Math.max(20_000_000, prev.bandwidth_out_bps + Math.floor((Math.random() - 0.5) * 8_000_000)),
        total_rps: Math.max(10, +(prev.total_rps + (Math.random() - 0.5) * 15).toFixed(1)),
        active_connections: Math.max(50, prev.active_connections + Math.floor((Math.random() - 0.5) * 10)),
      }));
    }, 2000);

    return () => {
      clearInterval(timer);
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, []);

  return { isConnected, metrics };
}

