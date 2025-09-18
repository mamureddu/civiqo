// Authentication middleware for WebSocket connections
// Currently implemented directly in the WebSocket handler
// This module can be used for future authentication middleware needs

use shared::{
    auth::{extract_bearer_token, Claims},
    error::{AppError, Result},
};
use axum::http::HeaderMap;

/// Extract and validate JWT token from WebSocket upgrade headers
pub async fn validate_websocket_auth(
    headers: &HeaderMap,
    auth_state: &shared::auth::AuthState,
) -> Result<Claims> {
    // Extract bearer token from headers
    let token = extract_bearer_token(headers)
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    // Validate token using Auth0 state
    let claims = auth_state
        .validate_token(&token)
        .await
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderMap, HeaderValue};

    #[test]
    fn test_auth_header_extraction() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_static("Bearer test-token-123"),
        );

        let token = extract_bearer_token(&headers);
        assert_eq!(token, Some("test-token-123".to_string()));
    }

    #[test]
    fn test_missing_auth_header() {
        let headers = HeaderMap::new();
        let token = extract_bearer_token(&headers);
        assert_eq!(token, None);
    }

    #[test]
    fn test_invalid_auth_header_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_static("InvalidFormat test-token-123"),
        );

        let token = extract_bearer_token(&headers);
        assert_eq!(token, None);
    }
}