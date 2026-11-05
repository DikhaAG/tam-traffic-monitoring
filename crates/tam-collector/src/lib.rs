pub mod log_parser;
pub mod simulator;
pub mod sniffer;

pub fn collector_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
