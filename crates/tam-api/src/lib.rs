pub mod app_state;
pub mod auth;
pub mod routes;
pub mod ws;

pub fn api_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
