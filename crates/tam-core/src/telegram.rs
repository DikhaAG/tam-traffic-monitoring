use crate::models::alert::AlertEvent;
use reqwest::Client;
use serde_json::json;

#[derive(Clone, Debug)]
pub struct TelegramAlertDispatcher {
    bot_token: String,
    chat_id: String,
    http_client: Client,
}

impl TelegramAlertDispatcher {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self {
            bot_token,
            chat_id,
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
        }
    }

    pub fn format_alert_markdown(&self, alert: &AlertEvent) -> String {
        let icon = match alert.severity.as_str() {
            "critical" => "🚨 *CRITICAL ALERT*",
            "warning" => "⚠️ *WARNING ALERT*",
            _ => "ℹ️ *INFO ALERT*",
        };

        format!(
            "{}\n\n*Category:* `{}`\n*Target:* `{}`\n*Time:* {}\n\n*Details:*\n{}\n\n— _TAM Monitoring Service_",
            icon,
            alert.category,
            alert.target,
            alert.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            alert.description
        )
    }

    pub async fn dispatch(&self, alert: &AlertEvent) -> Result<(), reqwest::Error> {
        if self.bot_token.trim().is_empty() || self.chat_id.trim().is_empty() {
            tracing::info!("Telegram notification skipped: Bot token or Chat ID not configured");
            return Ok(());
        }

        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        let text = self.format_alert_markdown(alert);

        let body = json!({
            "chat_id": self.chat_id,
            "text": text,
            "parse_mode": "Markdown",
            "disable_web_page_preview": true,
        });

        let res = self.http_client.post(&url).json(&body).send().await?;
        if !res.status().is_success() {
            tracing::warn!("Telegram dispatch failed with HTTP status: {}", res.status());
        }

        Ok(())
    }
}
