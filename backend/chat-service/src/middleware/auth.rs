// Authentication middleware for WebSocket connections
// Currently implemented directly in the WebSocket handler
// This module can be used for future authentication middleware needs

use shared::{
    auth::{extract_bearer_token, AuthState},
    error::{AppError, Result},
    models::Claims,
};
use axum::http::HeaderMap;

/// Extract JWT token from WebSocket upgrade headers
pub fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers.get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_str| extract_bearer_token(auth_str).ok())
        .map(|token| token.to_string())
}

/// Extract and validate JWT token from WebSocket upgrade headers
pub async fn validate_websocket_auth(
    headers: &HeaderMap,
    auth_state: &AuthState,
) -> Result<Claims> {
    // Extract bearer token from headers
    let token = extract_token_from_headers(headers)
        .ok_or_else(|| AppError::Auth("Missing authorization header".to_string()))?;

    // Validate token using Auth0 state
    let claims = auth_state
        .validate_token(&token)
        .await
        .map_err(|e| AppError::Auth(format!("Invalid token: {}", e)))?;

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

        let token = extract_token_from_headers(&headers);
        assert_eq!(token, Some("test-token-123".to_string()));
    }

    #[test]
    fn test_missing_auth_header() {
        let headers = HeaderMap::new();
        let token = extract_token_from_headers(&headers);
        assert_eq!(token, None);
    }

    #[test]
    fn test_invalid_auth_header_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_static("InvalidFormat test-token-123"),
        );

        let token = extract_token_from_headers(&headers);
        assert_eq!(token, None);
    }
}