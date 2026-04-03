// Authentication middleware for WebSocket connections
// Currently implemented directly in the WebSocket handler
// This module can be used for future authentication middleware needs

use shared::auth::extract_bearer_token;
use axum::http::HeaderMap;

/// Extract JWT token from WebSocket upgrade headers
pub fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers.get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_str| extract_bearer_token(auth_str).ok())
        .map(|token| token.to_string())
}
