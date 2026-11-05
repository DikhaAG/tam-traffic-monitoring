use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::Serialize;

#[derive(Clone)]
pub struct RedisClient {
    manager: ConnectionManager,
}

impl RedisClient {
    pub async fn connect(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { manager })
    }

    pub async fn publish_event<T: Serialize>(&mut self, channel: &str, event: &T) -> Result<(), redis::RedisError> {
        if let Ok(payload) = serde_json::to_string(event) {
            let mut conn = self.manager.clone();
            let _: () = conn.publish(channel, payload).await?;
        }
        Ok(())
    }

    pub async fn increment_rate(&mut self, key: &str, ttl_seconds: u64) -> Result<i64, redis::RedisError> {
        let mut conn = self.manager.clone();
        let val: i64 = conn.incr(key, 1).await?;
        if val == 1 {
            let _: () = conn.expire(key, ttl_seconds as i64).await?;
        }
        Ok(val)
    }
}
