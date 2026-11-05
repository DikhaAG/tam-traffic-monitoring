use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionResult {
    pub target_name: String,
    pub action: String,
    pub status: String,
    pub output: String,
}

pub struct Pm2Executor {
    is_dry_run: bool,
}

impl Pm2Executor {
    pub fn new_live() -> Self {
        Self { is_dry_run: false }
    }

    pub fn new_dry_run() -> Self {
        Self { is_dry_run: true }
    }

    pub async fn execute(&self, target_name: &str, action: &str) -> Result<CommandExecutionResult, String> {
        let valid_actions = ["restart", "reload", "stop", "clear_logs"];
        if !valid_actions.contains(&action) {
            return Err(format!(
                "Invalid PM2 action '{}'. Allowed actions: {:?}",
                action, valid_actions
            ));
        }

        if self.is_dry_run {
            return Ok(CommandExecutionResult {
                target_name: target_name.to_string(),
                action: action.to_string(),
                status: "success".to_string(),
                output: format!("[DRY RUN] Executed 'pm2 {} {}'", action, target_name),
            });
        }

        let cmd_arg = match action {
            "clear_logs" => "flush",
            other => other,
        };

        let output = tokio::process::Command::new("pm2")
            .arg(cmd_arg)
            .arg(target_name)
            .output()
            .await
            .map_err(|e| format!("Failed to execute PM2 command: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(CommandExecutionResult {
                target_name: target_name.to_string(),
                action: action.to_string(),
                status: "success".to_string(),
                output: stdout.to_string(),
            })
        } else {
            Ok(CommandExecutionResult {
                target_name: target_name.to_string(),
                action: action.to_string(),
                status: "failed".to_string(),
                output: format!("STDOUT: {}\nSTDERR: {}", stdout, stderr),
            })
        }
    }
}

