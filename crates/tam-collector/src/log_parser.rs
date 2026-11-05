use chrono::Utc;
use tam_core::models::traffic::AppRequestEvent;
use serde::Deserialize;

#[derive(Deserialize)]
struct JsonLogEntry {
    client_ip: String,
    method: String,
    path: String,
    status: u16,
    duration_ms: f32,
    user_agent: Option<String>,
}

pub struct LogParser;

impl LogParser {
    pub fn parse_nginx_line(line: &str) -> Option<AppRequestEvent> {
        // Example: 192.168.1.50 - - [05/Nov/2026:12:00:00 +0000] "GET /api/v1/servers HTTP/1.1" 200 1432 "https://..." "Mozilla/5.0" 0.045
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            return None;
        }

        let client_ip = parts[0].to_string();

        // Extract HTTP method and path
        let method_part_idx = line.find('"')?;
        let request_str = &line[method_part_idx + 1..];
        let end_quote_idx = request_str.find('"')?;
        let req_components: Vec<&str> = request_str[..end_quote_idx].split_whitespace().collect();
        if req_components.len() < 2 {
            return None;
        }
        let method = req_components[0].to_string();
        let path = req_components[1].to_string();

        // Find status code after the quote
        let after_quote = &request_str[end_quote_idx + 1..].trim();
        let after_parts: Vec<&str> = after_quote.split_whitespace().collect();
        if after_parts.is_empty() {
            return None;
        }
        let status_code: u16 = after_parts[0].parse().ok()?;

        // Duration is often the last component (in seconds, e.g. 0.045 -> 45.0 ms)
        let last_part = parts.last()?;
        let duration_ms = last_part.parse::<f32>().map(|s| s * 1000.0).unwrap_or(20.0);

        // Extract user agent if present (between 3rd and 4th quote or 5th and 6th quote)
        let user_agent = if let Some(last_quote_start) = line.rfind('"') {
            let before_last = &line[..last_quote_start];
            if let Some(prev_quote) = before_last.rfind('"') {
                Some(before_last[prev_quote + 1..].to_string())
            } else {
                None
            }
        } else {
            None
        };

        Some(AppRequestEvent {
            timestamp: Utc::now(),
            client_ip,
            method,
            path,
            status_code,
            duration_ms,
            user_agent,
        })
    }

    pub fn parse_json_line(line: &str) -> Option<AppRequestEvent> {
        let entry: JsonLogEntry = serde_json::from_str(line).ok()?;
        Some(AppRequestEvent {
            timestamp: Utc::now(),
            client_ip: entry.client_ip,
            method: entry.method,
            path: entry.path,
            status_code: entry.status,
            duration_ms: entry.duration_ms,
            user_agent: entry.user_agent,
        })
    }
}

