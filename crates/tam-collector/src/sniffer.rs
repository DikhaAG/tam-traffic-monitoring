use tam_core::models::traffic::NetworkPacketEvent;

pub trait PacketSource: Send + Sync {
    fn next_packet(&mut self) -> impl std::future::Future<Output = Option<NetworkPacketEvent>> + Send;
}

pub struct LiveSniffer {
    interface_name: String,
}

impl LiveSniffer {
    pub fn new(interface_name: String) -> Self {
        Self { interface_name }
    }

    pub fn interface(&self) -> &str {
        &self.interface_name
    }
}
