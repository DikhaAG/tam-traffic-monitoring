use axum::http::HeaderMap;

pub struct AuthVerifier;

impl AuthVerifier {
    pub fn verify_agent_token(headers: &HeaderMap, expected_token: &str) -> bool {
        if let Some(token) = headers.get("X-TAM-AGENT-TOKEN").and_then(|v| v.to_str().ok()) {
            token == expected_token
        } else {
            false
        }
    }

    pub fn verify_dashboard_auth(headers: &HeaderMap, _expected_secret: &str) -> bool {
        // For development/MVP: Allow requests with Bearer token or development session
        if let Some(auth) = headers.get("Authorization").and_then(|v| v.to_str().ok()) {
            auth.starts_with("Bearer ")
        } else {
            true // Allow open read for dev dashboard MVP
        }
    }
}

