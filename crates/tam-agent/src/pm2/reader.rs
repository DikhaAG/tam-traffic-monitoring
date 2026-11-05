use serde::Deserialize;
use tam_core::models::pm2::Pm2ProcessMetrics;

#[derive(Deserialize)]
struct MonitData {
    memory: Option<u64>,
    cpu: Option<f32>,
}

#[derive(Deserialize)]
struct EnvData {
    status: Option<String>,
    pm_uptime: Option<u64>,
    restart_time: Option<u32>,
    node_version: Option<String>,
    pm_cwd: Option<String>,
}

#[derive(Deserialize)]
struct ProcessEntry {
    name: String,
    pm_id: u32,
    monit: Option<MonitData>,
    pm2_env: Option<EnvData>,
}

pub struct Pm2Reader;

impl Pm2Reader {
    pub fn parse_jlist(json_str: &str) -> Result<Vec<Pm2ProcessMetrics>, serde_json::Error> {
        let entries: Vec<ProcessEntry> = serde_json::from_str(json_str)?;
        let mut processes = Vec::with_capacity(entries.len());

        let now_ms = chrono::Utc::now().timestamp_millis() as u64;

        for e in entries {
            let cpu = e.monit.as_ref().and_then(|m| m.cpu).unwrap_or(0.0);
            let mem = e.monit.as_ref().and_then(|m| m.memory).unwrap_or(0);
            let status = e.pm2_env.as_ref().and_then(|env| env.status.clone()).unwrap_or_else(|| "unknown".to_string());
            let uptime_ms = e.pm2_env.as_ref().and_then(|env| env.pm_uptime).unwrap_or(now_ms);
            let uptime_sec = if now_ms > uptime_ms { (now_ms - uptime_ms) / 1000 } else { 0 };
            let restarts = e.pm2_env.as_ref().and_then(|env| env.restart_time).unwrap_or(0);
            let node_version = e.pm2_env.as_ref().and_then(|env| env.node_version.clone());

            let (git_branch, git_commit, git_commit_msg) = if let Some(cwd) = e.pm2_env.as_ref().and_then(|env| env.pm_cwd.as_deref()) {
                crate::git_meta::extract_git_metadata(cwd)
            } else {
                (None, None, None)
            };

            processes.push(Pm2ProcessMetrics {
                process_id: e.pm_id,
                name: e.name,
                status,
                cpu_percent: cpu,
                memory_bytes: mem,
                uptime_seconds: uptime_sec,
                restart_count: restarts,
                git_branch,
                git_commit,
                git_commit_msg,
                node_version,
            });
        }

        Ok(processes)
    }

    pub fn generate_simulated_processes() -> Vec<Pm2ProcessMetrics> {
        vec![
            Pm2ProcessMetrics {
                process_id: 0,
                name: "tam-web-api".to_string(),
                status: "online".to_string(),
                cpu_percent: 2.1,
                memory_bytes: 84_000_000,
                uptime_seconds: 7200,
                restart_count: 0,
                git_branch: Some("main".to_string()),
                git_commit: Some("c4f1a2b".to_string()),
                git_commit_msg: Some("feat: add health endpoint".to_string()),
                node_version: Some("v20.18.0".to_string()),
            },
            Pm2ProcessMetrics {
                process_id: 1,
                name: "billing-worker".to_string(),
                status: "online".to_string(),
                cpu_percent: 4.8,
                memory_bytes: 142_000_000,
                uptime_seconds: 14400,
                restart_count: 1,
                git_branch: Some("main".to_string()),
                git_commit: Some("9a8b7c6".to_string()),
                git_commit_msg: Some("fix: payment webhook timeout".to_string()),
                node_version: Some("v20.18.0".to_string()),
            },
            Pm2ProcessMetrics {
                process_id: 2,
                name: "notification-dispatcher".to_string(),
                status: "online".to_string(),
                cpu_percent: 0.8,
                memory_bytes: 62_000_000,
                uptime_seconds: 3600,
                restart_count: 0,
                git_branch: Some("staging".to_string()),
                git_commit: Some("3f2e1d0".to_string()),
                git_commit_msg: Some("feat: telegram alert dispatcher".to_string()),
                node_version: Some("v20.18.0".to_string()),
            },
        ]
    }

    pub async fn read_live() -> Result<Vec<Pm2ProcessMetrics>, String> {
        let output = tokio::process::Command::new("pm2")
            .arg("jlist")
            .output()
            .await
            .map_err(|e| format!("Failed to run 'pm2 jlist': {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("'pm2 jlist' exited with error: {}", stderr));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        Self::parse_jlist(&json_str).map_err(|e| format!("Failed to parse PM2 json: {}", e))
    }
}
