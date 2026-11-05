"use client";

import React from "react";
import { Globe, ArrowRight } from "lucide-react";

export function PacketInspector() {
  const topTalkers = [
    { ip: "192.168.1.100", location: "Internal Gateway", packets: "142,390", bytes: "184 MB", protocol: "TCP" },
    { ip: "10.0.1.15", location: "Worker Cluster", packets: "98,120", bytes: "112 MB", protocol: "HTTP" },
    { ip: "203.0.113.45", location: "Jakarta, ID", packets: "54,200", bytes: "68 MB", protocol: "HTTPS" },
    { ip: "198.51.100.82", location: "Singapore, SG", packets: "38,400", bytes: "45 MB", protocol: "TCP" },
    { ip: "1.1.1.1", location: "Cloudflare DNS", packets: "21,050", bytes: "14 MB", protocol: "DNS" },
  ];

  const protocols = [
    { name: "HTTP / HTTPS", percent: 64, color: "bg-sky-400" },
    { name: "TCP (Raw Socket)", percent: 24, color: "bg-indigo-400" },
    { name: "UDP (DNS / VoIP)", percent: 9, color: "bg-emerald-400" },
    { name: "ICMP / Other", percent: 3, color: "bg-amber-400" },
  ];

  return (
    <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
      {/* Protocol Breakdown */}
      <div className="bg-[#0f172a]/90 border border-slate-800 rounded-xl p-5 shadow-lg">
        <h3 className="font-semibold text-white text-sm mb-1">Protocol Distribution</h3>
        <p className="text-xs text-slate-400 mb-4">Traffic composition by layer 4/7 protocol</p>

        <div className="space-y-4">
          {protocols.map((p) => (
            <div key={p.name} className="space-y-1.5">
              <div className="flex justify-between text-xs">
                <span className="text-slate-300 font-medium">{p.name}</span>
                <span className="text-slate-400 font-mono">{p.percent}%</span>
              </div>
              <div className="w-full h-2 bg-slate-800 rounded-full overflow-hidden">
                <div className={`h-full ${p.color} rounded-full`} style={{ width: `${p.percent}%` }} />
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Top Talkers Table */}
      <div className="lg:col-span-2 bg-[#0f172a]/90 border border-slate-800 rounded-xl p-5 shadow-lg">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3 className="font-semibold text-white text-sm">Top Talkers (Bandwidth Consumers)</h3>
            <p className="text-xs text-slate-400">Ranked by total packet ingress/egress bytes</p>
          </div>
          <span className="text-[11px] text-sky-400 bg-sky-500/10 px-2 py-0.5 rounded border border-sky-500/30 font-mono">
            Live Top 5
          </span>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full text-left text-xs border-collapse">
            <thead>
              <tr className="border-b border-slate-800 text-slate-400 uppercase tracking-wider font-semibold">
                <th className="py-2.5 px-3">Client IP & Region</th>
                <th className="py-2.5 px-3">Protocol</th>
                <th className="py-2.5 px-3">Packets</th>
                <th className="py-2.5 px-3 text-right">Data Volume</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-800/60 font-mono">
              {topTalkers.map((talker, idx) => (
                <tr key={idx} className="hover:bg-slate-800/30">
                  <td className="py-2.5 px-3">
                    <div className="flex items-center gap-2">
                      <Globe className="h-3.5 w-3.5 text-slate-500" />
                      <span className="font-semibold text-white">{talker.ip}</span>
                      <span className="text-[10px] text-slate-400 font-sans">({talker.location})</span>
                    </div>
                  </td>
                  <td className="py-2.5 px-3">
                    <span className="px-1.5 py-0.5 rounded text-[10px] bg-slate-800 text-slate-300 border border-slate-700">
                      {talker.protocol}
                    </span>
                  </td>
                  <td className="py-2.5 px-3 text-slate-300">{talker.packets}</td>
                  <td className="py-2.5 px-3 text-right text-sky-400 font-bold">{talker.bytes}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

