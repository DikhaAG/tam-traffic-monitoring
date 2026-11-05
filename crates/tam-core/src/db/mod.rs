pub mod servers;
pub mod audit;

use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::models::server::{Environment, ServerNode};
use crate::models::pm2::{Pm2Command, Pm2ProcessMetrics};
use crate::models::alert::AlertEvent;

#[derive(Clone, Debug)]
pub struct DatabasePool {
    pool: PgPool,
}

impl DatabasePool {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn inner(&self) -> &PgPool {
        &self.pool
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        let migration_sql = include_str!("../../migrations/20261105000001_initial_schema.sql");
        sqlx::raw_sql(migration_sql).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn upsert_server_node(&self, node: &ServerNode) -> Result<(), sqlx::Error> {
        servers::upsert_server_node(&self.pool, node).await
    }

    pub async fn get_server_node(&self, id: &str) -> Result<Option<ServerNode>, sqlx::Error> {
        servers::get_server_node(&self.pool, id).await
    }

    pub async fn list_server_nodes(&self, env: Option<Environment>) -> Result<Vec<ServerNode>, sqlx::Error> {
        servers::list_server_nodes(&self.pool, env).await
    }

    pub async fn save_pm2_snapshot(&self, node_id: &str, processes: &[Pm2ProcessMetrics]) -> Result<(), sqlx::Error> {
        servers::save_pm2_snapshot(&self.pool, node_id, processes).await
    }

    pub async fn get_latest_pm2_processes(&self, node_id: &str) -> Result<Vec<Pm2ProcessMetrics>, sqlx::Error> {
        servers::get_latest_pm2_processes(&self.pool, node_id).await
    }

    pub async fn record_audit_log(&self, cmd: &Pm2Command) -> Result<(), sqlx::Error> {
        audit::record_audit_log(&self.pool, cmd).await
    }

    pub async fn get_audit_logs(&self, node_id: &str, limit: i64) -> Result<Vec<Pm2Command>, sqlx::Error> {
        audit::get_audit_logs(&self.pool, node_id, limit).await
    }

    pub async fn insert_alert_incident(&self, alert: &AlertEvent) -> Result<(), sqlx::Error> {
        audit::insert_alert_incident(&self.pool, alert).await
    }

    pub async fn list_alert_incidents(&self, resolved: Option<bool>, limit: i64) -> Result<Vec<AlertEvent>, sqlx::Error> {
        audit::list_alert_incidents(&self.pool, resolved, limit).await
    }
}
