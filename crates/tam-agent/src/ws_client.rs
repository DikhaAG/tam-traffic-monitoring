use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tam_core::models::server::ServerNode;
use tam_core::models::pm2::{Pm2Command, Pm2ProcessMetrics};
use crate::pm2::executor::Pm2Executor;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum AgentMessage {
    Heartbeat(ServerNode),
    Pm2Metrics(Vec<Pm2ProcessMetrics>),
    CommandResponse { id: String, status: String, output: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ServerCommandMessage {
    ExecuteCommand(Pm2Command),
}

pub struct AgentWebSocketClient {
    server_ws_url: String,
    agent_token: String,
    node: ServerNode,
    executor: Pm2Executor,
}

impl AgentWebSocketClient {
    pub fn new(server_ws_url: String, agent_token: String, node: ServerNode, is_dry_run: bool) -> Self {
        Self {
            server_ws_url,
            agent_token,
            node,
            executor: if is_dry_run { Pm2Executor::new_dry_run() } else { Pm2Executor::new_live() },
        }
    }

    pub async fn run_loop(&mut self) {
        loop {
            tracing::info!("Connecting to TAM API server at {}...", self.server_ws_url);

            let url = format!("{}?token={}&node_id={}", self.server_ws_url, self.agent_token, self.node.id);
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    tracing::info!("Connected to TAM API server successfully");
                    let (mut write, mut read) = ws_stream.split();

                    // Send initial heartbeat
                    let hb = AgentMessage::Heartbeat(self.node.clone());
                    if let Ok(json) = serde_json::to_string(&hb) {
                        let _ = write.send(Message::Text(json.into())).await;
                    }

                    // Process incoming commands while sending periodic heartbeat
                    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));

                    loop {
                        tokio::select! {
                            _ = interval.tick() => {
                                let metrics = crate::pm2::reader::Pm2Reader::generate_simulated_processes();
                                let msg = AgentMessage::Pm2Metrics(metrics);
                                if let Ok(json) = serde_json::to_string(&msg) {
                                    if write.send(Message::Text(json.into())).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            msg_opt = read.next() => {
                                match msg_opt {
                                    Some(Ok(Message::Text(text))) => {
                                        if let Ok(cmd_msg) = serde_json::from_str::<ServerCommandMessage>(&text) {
                                            match cmd_msg {
                                                ServerCommandMessage::ExecuteCommand(cmd) => {
                                                    let res = self.executor.execute(&cmd.target_name, &cmd.action).await;
                                                    let (status, output) = match res {
                                                        Ok(r) => (r.status, r.output),
                                                        Err(e) => ("failed".to_string(), e),
                                                    };
                                                    let resp = AgentMessage::CommandResponse {
                                                        id: cmd.id,
                                                        status,
                                                        output,
                                                    };
                                                    if let Ok(resp_json) = serde_json::to_string(&resp) {
                                                        let _ = write.send(Message::Text(resp_json.into())).await;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(Ok(Message::Close(_))) | None => {
                                        tracing::warn!("Server disconnected");
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("WebSocket connection failed: {}. Retrying in 5 seconds...", e);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}

