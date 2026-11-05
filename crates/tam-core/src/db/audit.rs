use sqlx::{Pool, Postgres, Row};
use crate::models::pm2::Pm2Command;
use crate::models::alert::AlertEvent;

pub async fn record_audit_log(pool: &Pool<Postgres>, cmd: &Pm2Command) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO pm2_audit_logs (id, node_id, target_name, action, issued_by, status, output, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&cmd.id)
    .bind(&cmd.node_id)
    .bind(&cmd.target_name)
    .bind(&cmd.action)
    .bind(&cmd.issued_by)
    .bind(&cmd.status)
    .bind(&cmd.output)
    .bind(cmd.timestamp)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_audit_logs(
    pool: &Pool<Postgres>,
    node_id: &str,
    limit: i64,
) -> Result<Vec<Pm2Command>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, node_id, target_name, action, issued_by, status, output, created_at
        FROM pm2_audit_logs
        WHERE node_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(node_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let logs = rows
        .into_iter()
        .map(|r| Pm2Command {
            id: r.get("id"),
            node_id: r.get("node_id"),
            target_name: r.get("target_name"),
            action: r.get("action"),
            issued_by: r.get("issued_by"),
            status: r.get("status"),
            output: r.get("output"),
            timestamp: r.get("created_at"),
        })
        .collect();

    Ok(logs)
}

pub async fn insert_alert_incident(pool: &Pool<Postgres>, alert: &AlertEvent) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO alert_incidents (id, category, severity, target, description, resolved, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&alert.id)
    .bind(&alert.category)
    .bind(&alert.severity)
    .bind(&alert.target)
    .bind(&alert.description)
    .bind(alert.resolved)
    .bind(alert.timestamp)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_alert_incidents(
    pool: &Pool<Postgres>,
    resolved: Option<bool>,
    limit: i64,
) -> Result<Vec<AlertEvent>, sqlx::Error> {
    let rows = if let Some(res) = resolved {
        sqlx::query(
            r#"
            SELECT id, category, severity, target, description, resolved, created_at
            FROM alert_incidents
            WHERE resolved = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(res)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, category, severity, target, description, resolved, created_at
            FROM alert_incidents
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    };

    let alerts = rows
        .into_iter()
        .map(|r| AlertEvent {
            id: r.get("id"),
            category: r.get("category"),
            severity: r.get("severity"),
            target: r.get("target"),
            description: r.get("description"),
            resolved: r.get("resolved"),
            timestamp: r.get("created_at"),
        })
        .collect();

    Ok(alerts)
}
