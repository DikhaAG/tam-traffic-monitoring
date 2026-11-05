use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tam_core::models::pm2::Pm2Command;

pub struct AgentHub {
    channels: Arc<RwLock<HashMap<String, mpsc::Sender<Pm2Command>>>>,
}

impl AgentHub {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_agent(&self, node_id: &str, tx: mpsc::Sender<Pm2Command>) {
        let mut map = self.channels.write().await;
        map.insert(node_id.to_string(), tx);
    }

    pub async fn unregister_agent(&self, node_id: &str) {
        let mut map = self.channels.write().await;
        map.remove(node_id);
    }

    pub async fn dispatch_command(&self, node_id: &str, cmd: Pm2Command) -> Result<(), String> {
        let map = self.channels.read().await;
        if let Some(tx) = map.get(node_id) {
            tx.send(cmd).await.map_err(|e| format!("Failed to send command to agent: {}", e))
        } else {
            Err(format!("Agent for server '{}' is not currently connected", node_id))
        }
    }

    pub async fn is_agent_connected(&self, node_id: &str) -> bool {
        let map = self.channels.read().await;
        map.contains_key(node_id)
    }
}

