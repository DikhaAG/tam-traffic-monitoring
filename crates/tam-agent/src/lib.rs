pub mod pm2;
pub mod logs;
pub mod git_meta;
pub mod systemd;
pub mod ws_client;

pub fn agent_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
