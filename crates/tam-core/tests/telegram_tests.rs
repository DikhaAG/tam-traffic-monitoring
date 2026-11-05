use chrono::Utc;
use tam_core::models::alert::AlertEvent;
use tam_core::telegram::TelegramAlertDispatcher;

#[test]
fn test_telegram_message_formatting() {
    let dispatcher = TelegramAlertDispatcher::new("mock_token".to_string(), "123456789".to_string());

    let alert = AlertEvent {
        id: "alert-99".to_string(),
        category: "pm2_crash".to_string(),
        severity: "critical".to_string(),
        target: "srv-prod-01 / auth-service".to_string(),
        description: "Process crashed with exit code 1: Unhandled exception ETIMEDOUT".to_string(),
        timestamp: Utc::now(),
        resolved: false,
    };

    let formatted = dispatcher.format_alert_markdown(&alert);
    assert!(formatted.contains("🚨 *CRITICAL ALERT*"));
    assert!(formatted.contains("srv-prod-01 / auth-service"));
    assert!(formatted.contains("Unhandled exception"));
    assert!(formatted.contains("TAM Monitoring"));
}
