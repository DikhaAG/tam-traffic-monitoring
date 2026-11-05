use sqlx::{Pool, Postgres, Row};
use crate::models::server::{Environment, ServerNode};
use crate::models::pm2::Pm2ProcessMetrics;

pub async fn upsert_server_node(pool: &Pool<Postgres>, node: &ServerNode) -> Result<(), sqlx::Error> {
    let env_str = node.environment.to_string();
    sqlx::query(
        r#"
        INSERT INTO server_nodes (
            id, hostname, ip_address, environment, os_info,
            cpu_cores, total_memory_mb, disk_usage_percent, status, last_heartbeat
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (id) DO UPDATE SET
            hostname = EXCLUDED.hostname,
            ip_address = EXCLUDED.ip_address,
            environment = EXCLUDED.environment,
            os_info = EXCLUDED.os_info,
            cpu_cores = EXCLUDED.cpu_cores,
            total_memory_mb = EXCLUDED.total_memory_mb,
            disk_usage_percent = EXCLUDED.disk_usage_percent,
            status = EXCLUDED.status,
            last_heartbeat = EXCLUDED.last_heartbeat
        "#,
    )
    .bind(&node.id)
    .bind(&node.hostname)
    .bind(&node.ip_address)
    .bind(env_str)
    .bind(&node.os_info)
    .bind(node.cpu_cores as i32)
    .bind(node.total_memory_mb as i64)
    .bind(node.disk_usage_percent)
    .bind(&node.status)
    .bind(node.last_heartbeat)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_server_node(pool: &Pool<Postgres>, id: &str) -> Result<Option<ServerNode>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, hostname, ip_address, environment, os_info,
               cpu_cores, total_memory_mb, disk_usage_percent, status, last_heartbeat
        FROM server_nodes
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    if let Some(r) = row {
        let env_str: String = r.get("environment");
        let cpu_cores: i32 = r.get("cpu_cores");
        let total_memory_mb: i64 = r.get("total_memory_mb");
        Ok(Some(ServerNode {
            id: r.get("id"),
            hostname: r.get("hostname"),
            ip_address: r.get("ip_address"),
            environment: Environment::from(env_str),
            os_info: r.get("os_info"),
            cpu_cores: cpu_cores as u32,
            total_memory_mb: total_memory_mb as u64,
            disk_usage_percent: r.get("disk_usage_percent"),
            status: r.get("status"),
            last_heartbeat: r.get("last_heartbeat"),
        }))
    } else {
        Ok(None)
    }
}

pub async fn list_server_nodes(
    pool: &Pool<Postgres>,
    env: Option<Environment>,
) -> Result<Vec<ServerNode>, sqlx::Error> {
    let rows = if let Some(e) = env {
        sqlx::query(
            r#"
            SELECT id, hostname, ip_address, environment, os_info,
                   cpu_cores, total_memory_mb, disk_usage_percent, status, last_heartbeat
            FROM server_nodes
            WHERE environment = $1
            ORDER BY hostname ASC
            "#,
        )
        .bind(e.to_string())
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, hostname, ip_address, environment, os_info,
                   cpu_cores, total_memory_mb, disk_usage_percent, status, last_heartbeat
            FROM server_nodes
            ORDER BY hostname ASC
            "#,
        )
        .fetch_all(pool)
        .await?
    };

    let nodes = rows
        .into_iter()
        .map(|r| {
            let env_str: String = r.get("environment");
            let cpu_cores: i32 = r.get("cpu_cores");
            let total_memory_mb: i64 = r.get("total_memory_mb");
            ServerNode {
                id: r.get("id"),
                hostname: r.get("hostname"),
                ip_address: r.get("ip_address"),
                environment: Environment::from(env_str),
                os_info: r.get("os_info"),
                cpu_cores: cpu_cores as u32,
                total_memory_mb: total_memory_mb as u64,
                disk_usage_percent: r.get("disk_usage_percent"),
                status: r.get("status"),
                last_heartbeat: r.get("last_heartbeat"),
            }
        })
        .collect();

    Ok(nodes)
}

pub async fn save_pm2_snapshot(
    pool: &Pool<Postgres>,
    node_id: &str,
    processes: &[Pm2ProcessMetrics],
) -> Result<(), sqlx::Error> {
    for p in processes {
        sqlx::query(
            r#"
            INSERT INTO pm2_process_snapshots (
                node_id, process_id, name, status, cpu_percent, memory_bytes,
                uptime_seconds, restart_count, git_branch, git_commit, git_commit_msg, node_version
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(node_id)
        .bind(p.process_id as i32)
        .bind(&p.name)
        .bind(&p.status)
        .bind(p.cpu_percent)
        .bind(p.memory_bytes as i64)
        .bind(p.uptime_seconds as i64)
        .bind(p.restart_count as i32)
        .bind(&p.git_branch)
        .bind(&p.git_commit)
        .bind(&p.git_commit_msg)
        .bind(&p.node_version)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get_latest_pm2_processes(
    pool: &Pool<Postgres>,
    node_id: &str,
) -> Result<Vec<Pm2ProcessMetrics>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT ON (process_id)
            process_id, name, status, cpu_percent, memory_bytes,
            uptime_seconds, restart_count, git_branch, git_commit, git_commit_msg, node_version
        FROM pm2_process_snapshots
        WHERE node_id = $1
        ORDER BY process_id, recorded_at DESC
        "#,
    )
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    let procs = rows
        .into_iter()
        .map(|r| {
            let pid: i32 = r.get("process_id");
            let mem: i64 = r.get("memory_bytes");
            let uptime: i64 = r.get("uptime_seconds");
            let restarts: i32 = r.get("restart_count");
            Pm2ProcessMetrics {
                process_id: pid as u32,
                name: r.get("name"),
                status: r.get("status"),
                cpu_percent: r.get("cpu_percent"),
                memory_bytes: mem as u64,
                uptime_seconds: uptime as u64,
                restart_count: restarts as u32,
                git_branch: r.get("git_branch"),
                git_commit: r.get("git_commit"),
                git_commit_msg: r.get("git_commit_msg"),
                node_version: r.get("node_version"),
            }
        })
        .collect();

    Ok(procs)
}
