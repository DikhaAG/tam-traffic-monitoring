export function getApiBaseUrl(): string {
  if (typeof window !== "undefined") {
    // If running in browser, connect to backend API on port 4000 using current hostname
    const host = window.location.hostname;
    const protocol = window.location.protocol;
    return `${protocol}//${host}:4000`;
  }
  return process.env.NEXT_PUBLIC_API_URL || "http://localhost:4000";
}

export function getWsBaseUrl(): string {
  if (typeof window !== "undefined") {
    const host = window.location.hostname;
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    return `${protocol}//${host}:4000/ws/live`;
  }
  return process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:4000/ws/live";
}
