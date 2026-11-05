import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "TAM — Traffic & PM2 Server Fleet Monitoring",
  description: "Enterprise-grade Network Traffic, Application Telemetry & Ubuntu PM2 Monitoring",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark">
      <body className="min-h-screen bg-[#080c14] text-slate-100 antialiased selection:bg-sky-500/30 selection:text-sky-200">
        {children}
      </body>
    </html>
  );
}

