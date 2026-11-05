use std::sync::Arc;
use tam_api::app_state::AppState;
use tam_api::routes::create_router;
use tam_core::db::DatabasePool;
use tam_core::redis_client::RedisClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://tam_user:tam_secret_password@localhost:5433/tam_db".to_string());
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6380".to_string());
    let port: u16 = std::env::var("API_PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse()
        .unwrap_or(4000);
    let agent_token = std::env::var("AGENT_SHARED_TOKEN")
        .unwrap_or_else(|_| "tam_agent_secure_token".to_string());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "tam_jwt_secret_dev".to_string());

    tracing::info!("Connecting to PostgreSQL database...");
    let db = DatabasePool::connect(&db_url).await?;
    db.run_migrations().await?;
    tracing::info!("Database connected & migrations applied successfully");

    tracing::info!("Connecting to Redis...");
    let redis = RedisClient::connect(&redis_url).await?;
    tracing::info!("Redis connected successfully");

    let state = Arc::new(AppState::new(db, redis, agent_token, jwt_secret));
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("TAM API Server listening on http://0.0.0.0:{}", port);

    axum::serve(listener, app).await?;
    Ok(())
}
