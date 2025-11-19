use crate::middleware::auth::extract_token_from_headers;
use axum::http::{HeaderMap, HeaderValue};
use shared::testing::init_test_logging;

#[test]
fn test_extract_token_from_headers_with_bearer() {
    init_test_logging();

    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test.token"),
    );

    let token = extract_token_from_headers(&headers);
    assert!(token.is_some());
    assert_eq!(token.unwrap(), "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test.token");
}

#[test]
fn test_extract_token_from_headers_without_bearer() {
    init_test_logging();

    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test.token"),
    );

    let token = extract_token_from_headers(&headers);
    // Should fail because extract_bearer_token requires "Bearer " prefix
    assert!(token.is_none());
}

#[test]
fn test_extract_token_from_headers_missing_header() {
    init_test_logging();

    let headers = HeaderMap::new();

    let token = extract_token_from_headers(&headers);
    assert!(token.is_none());
}

#[test]
fn test_extract_token_from_headers_invalid_header_value() {
    init_test_logging();

    let mut headers = HeaderMap::new();
    // Insert invalid UTF-8 header value
    headers.insert(
        "authorization",
        HeaderValue::from_bytes(&[0xFF, 0xFE]).unwrap(),
    );

    let token = extract_token_from_headers(&headers);
    assert!(token.is_none());
}

#[test]
fn test_extract_token_from_headers_empty_header() {
    init_test_logging();

    let mut headers = HeaderMap::new();
    headers.insert("authorization", HeaderValue::from_static(""));

    let token = extract_token_from_headers(&headers);
    assert!(token.is_none());
}

#[test]
fn test_extract_token_from_headers_bearer_only() {
    init_test_logging();

    let mut headers = HeaderMap::new();
    headers.insert("authorization", HeaderValue::from_static("Bearer"));

    let token = extract_token_from_headers(&headers);
    assert!(token.is_none());
}

#[test]
fn test_extract_token_from_headers_bearer_with_space() {
    init_test_logging();

    let mut headers = HeaderMap::new();
    headers.insert("authorization", HeaderValue::from_static("Bearer "));

    let token = extract_token_from_headers(&headers);
    // Should return empty string because there's a space after Bearer but no token
    assert!(token.is_some());
    assert_eq!(token.unwrap(), "");
}

#[test]
fn test_extract_token_from_headers_multiple_spaces() {
    init_test_logging();

    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer   eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test.token"),
    );

    let token = extract_token_from_headers(&headers);
    assert!(token.is_some());
    // extract_bearer_token takes everything after "Bearer " including extra spaces
    assert_eq!(token.unwrap(), "  eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test.token");
}

#[test]
fn test_extract_token_from_headers_case_insensitive() {
    init_test_logging();

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization", // Capital A
        HeaderValue::from_static("Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test.token"),
    );

    let token = extract_token_from_headers(&headers);
    assert!(token.is_some());
    assert_eq!(token.unwrap(), "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test.token");
}

#[test]
fn test_extract_token_from_headers_bearer_case_variations() {
    init_test_logging();

    let test_cases = vec![
        ("bearer token123", false), // Wrong case
        ("BEARER token123", false), // Wrong case
        ("Bearer token123", true),  // Correct case
        ("bEaReR token123", false), // Wrong case
    ];

    for (test_case, should_succeed) in test_cases {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static(test_case));

        let token = extract_token_from_headers(&headers);
        if should_succeed {
            assert!(token.is_some());
            assert_eq!(token.unwrap(), "token123");
        } else {
            assert!(token.is_none());
        }
    }
}

#[test]
fn test_extract_token_from_headers_malformed_bearer() {
    init_test_logging();

    let malformed_cases = vec![
        "Bearertoken123", // No space
        "Bear token123",  // Incomplete "Bearer"
        "Basic token123", // Different auth type
        "token123 Bearer", // Reversed order
    ];

    for test_case in malformed_cases {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static(test_case));

        let token = extract_token_from_headers(&headers);
        // All these should fail because extract_bearer_token is strict about "Bearer " prefix
        assert!(token.is_none(), "Expected None for malformed case: {}", test_case);
    }
}

#[test]
fn test_extract_token_typical_jwt_tokens() {
    init_test_logging();

    let jwt_tokens = vec![
        "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IlJUVTVNRFEwTjBKRE56WkZNVUk1UXpBME5rSTFNRVEwUTBOQ04wSkdSVVkzTkRKRFFUYzJOUSJ9.eyJpc3MiOiJodHRwczovL2NvbW11bml0eS1tYW5hZ2VyLWRldi5ldS5hdXRoMC5jb20vIiwic3ViIjoiYXV0aDB8NjY5ZjU3YjEyMjg4ZjdkOTQ0MzU5YjJmIiwiYXVkIjoiY29tbXVuaXR5LW1hbmFnZXItZGV2IiwiaWF0IjoxNzM0NTQ1MDIyLCJleHAiOjE3MzQ2MzE0MjIsInNjb3BlIjoib3BlbmlkIHByb2ZpbGUgZW1haWwiLCJhenAiOiJMeVNnZ2FIRnFSbEZuUVI1aThFUFNoUEVNNDJjb0xabSJ9.test_signature",
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJ0ZXN0IiwiaWF0IjoxNjA5NDU5MjAwLCJleHAiOjE2NDA5OTUyMDAsImF1ZCI6InRlc3QiLCJzdWIiOiJ0ZXN0LXVzZXIifQ.test_signature"
    ];

    for jwt_token in jwt_tokens {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {}", jwt_token)).unwrap(),
        );

        let token = extract_token_from_headers(&headers);
        assert!(token.is_some());
        assert_eq!(token.unwrap(), jwt_token);
    }
}

#[test]
fn test_extract_token_websocket_auth_patterns() {
    init_test_logging();

    // Test common WebSocket authentication patterns
    let auth_patterns = vec![
        ("authorization", "Bearer ws_token_123"),
        ("Authorization", "Bearer ws_token_456"),
        ("sec-websocket-protocol", "bearer_token_789"), // Some clients use this
        ("x-auth-token", "direct_token_abc"),
    ];

    for (header_name, header_value) in auth_patterns {
        let mut headers = HeaderMap::new();

        if header_name == "authorization" || header_name == "Authorization" {
            headers.insert(header_name, HeaderValue::from_static(header_value));

            let token = extract_token_from_headers(&headers);
            assert!(token.is_some());

            if header_value.starts_with("Bearer ") {
                let expected = header_value.strip_prefix("Bearer ").unwrap();
                assert_eq!(token.unwrap(), expected);
            } else {
                assert_eq!(token.unwrap(), header_value);
            }
        }
        // Note: extract_token_from_headers only looks for "authorization" header
        // Other headers would require different extraction logic
    }
}