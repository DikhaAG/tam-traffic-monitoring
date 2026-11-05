use std::sync::Arc;
use tokio::sync::Mutex;
use tam_core::aggregator::SlidingWindowAggregator;
use tam_core::db::DatabasePool;
use tam_core::redis_client::RedisClient;
use tam_core::telegram::TelegramAlertDispatcher;
use crate::ws::agent_hub::AgentHub;
use crate::ws::dashboard_hub::DashboardHub;

pub struct AppState {
    pub db: DatabasePool,
    pub redis: Mutex<RedisClient>,
    pub agent_hub: Arc<AgentHub>,
    pub dashboard_hub: Arc<DashboardHub>,
    pub aggregator: Arc<Mutex<SlidingWindowAggregator>>,
    pub telegram: TelegramAlertDispatcher,
    pub agent_token: String,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(
        db: DatabasePool,
        redis: RedisClient,
        agent_token: String,
        jwt_secret: String,
    ) -> Self {
        let telegram = TelegramAlertDispatcher::new(
            std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default(),
            std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default(),
        );

        Self {
            db,
            redis: Mutex::new(redis),
            agent_hub: Arc::new(AgentHub::new()),
            dashboard_hub: Arc::new(DashboardHub::new()),
            aggregator: Arc::new(Mutex::new(SlidingWindowAggregator::new(60))),
            telegram,
            agent_token,
            jwt_secret,
        }
    }
}

