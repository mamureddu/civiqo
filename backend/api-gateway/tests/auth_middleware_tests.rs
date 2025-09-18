use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    Router,
};
use axum_test::TestServer;
use serial_test::serial;
use serde_json;
use shared::{
    database::Database,
    testing::{init_test_logging, create_test_db, cleanup_test_db, create_test_user},
    models::{
        ApiResponse, Claims, UserWithProfile,
    },
    auth::{Auth0Config, JwtValidator},
    error::AppError,
};
use uuid::Uuid;
use wiremock::{
    matchers::{method, path, header as header_matcher},
    Mock, MockServer, ResponseTemplate,
};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Algorithm, Validation};
use chrono::{Utc, Duration};
use std::sync::Arc;

// Import the actual API Gateway app
use api_gateway::{AppState, create_app};

/// Test configuration for authentication and middleware tests
struct AuthTestContext {
    server: TestServer,
    db: Database,
    mock_auth0: MockServer,
    auth_config: Auth0Config,
}

impl AuthTestContext {
    async fn new() -> Self {
        init_test_logging();

        // Create test database
        let db = create_test_db().await.expect("Failed to create test database");

        // Setup mock Auth0 server
        let mock_auth0 = MockServer::start().await;
        let auth_config = Auth0Config {
            domain: mock_auth0.uri().trim_start_matches("http://").to_string(),
            audience: "test-audience".to_string(),
            client_id: "test-client-id".to_string(),
            client_secret: "test-client-secret".to_string(),
        };

        // Create app state
        let app_state = AppState {
            db: db.clone(),
            auth_config: auth_config.clone(),
        };

        // Create the router
        let app = create_app(app_state);
        let server = TestServer::new(app).unwrap();

        Self {
            server,
            db,
            mock_auth0,
            auth_config,
        }
    }

    async fn cleanup(&self) {
        cleanup_test_db(&self.db).await.expect("Failed to cleanup test database");
    }

    /// Create a valid JWT token for testing
    fn create_test_jwt(&self, claims: &Claims) -> String {
        let header = Header::new(Algorithm::HS256);
        let secret = "test-secret";
        encode(&header, claims, &EncodingKey::from_secret(secret.as_ref()))
            .expect("Failed to create test JWT")
    }

    /// Create an expired JWT token for testing
    fn create_expired_jwt(&self, claims: &Claims) -> String {
        let mut expired_claims = claims.clone();
        expired_claims.exp = (Utc::now() - Duration::hours(1)).timestamp(); // Expired 1 hour ago
        self.create_test_jwt(&expired_claims)
    }

    /// Create a JWT with invalid signature
    fn create_invalid_signature_jwt(&self) -> String {
        let claims = Claims {
            sub: "test|123456".to_string(),
            aud: self.auth_config.audience.clone(),
            iss: format!("https://{}/", self.auth_config.domain),
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            email: Some("test@example.com".to_string()),
            email_verified: Some(true),
            name: Some("Test User".to_string()),
            picture: None,
            community_roles: vec![],
        };

        let header = Header::new(Algorithm::HS256);
        let wrong_secret = "wrong-secret";
        encode(&header, &claims, &EncodingKey::from_secret(wrong_secret.as_ref()))
            .expect("Failed to create invalid JWT")
    }

    /// Setup JWKS mock for token validation
    async fn setup_jwks_mock(&self) {
        let jwks_response = serde_json::json!({
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "test-key-id",
                    "use": "sig",
                    "n": "test-n-value",
                    "e": "AQAB",
                    "x5c": ["LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t"]
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/.well-known/jwks.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks_response))
            .mount(&self.mock_auth0)
            .await;
    }

    /// Setup JWKS mock that returns error
    async fn setup_jwks_error_mock(&self) {
        Mock::given(method("GET"))
            .and(path("/.well-known/jwks.json"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&self.mock_auth0)
            .await;
    }

    /// Create test user and valid claims
    async fn create_test_user_with_claims(&self) -> (shared::models::User, Claims) {
        let user = create_test_user(&self.db, None).await.expect("Failed to create test user");

        let claims = Claims {
            sub: user.auth0_id.clone(),
            aud: self.auth_config.audience.clone(),
            iss: format!("https://{}/", self.auth_config.domain),
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            email: Some(user.email.clone()),
            email_verified: Some(true),
            name: Some("Test User".to_string()),
            picture: None,
            community_roles: vec![],
        };

        (user, claims)
    }
}

// JWT Token Validation Tests
#[tokio::test]
#[serial]
async fn test_valid_jwt_token() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    let body: ApiResponse<UserWithProfile> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());

    let returned_user = body.data.unwrap();
    assert_eq!(returned_user.id, user.id);
    assert_eq!(returned_user.auth0_id, user.auth0_id);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_missing_authorization_header() {
    let ctx = AuthTestContext::new().await;

    let response = ctx.server
        .get("/api/auth/me")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_malformed_authorization_header() {
    let ctx = AuthTestContext::new().await;

    // Test with malformed header (missing "Bearer " prefix)
    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, "InvalidToken")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_empty_bearer_token() {
    let ctx = AuthTestContext::new().await;

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, "Bearer ")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_invalid_jwt_format() {
    let ctx = AuthTestContext::new().await;

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, "Bearer not.a.jwt")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_expired_jwt_token() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let expired_token = ctx.create_expired_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", expired_token))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_invalid_jwt_signature() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let invalid_token = ctx.create_invalid_signature_jwt();

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", invalid_token))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_jwt_invalid_audience() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, mut claims) = ctx.create_test_user_with_claims().await;
    claims.aud = "wrong-audience".to_string();
    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_jwt_invalid_issuer() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, mut claims) = ctx.create_test_user_with_claims().await;
    claims.iss = "https://wrong-issuer.com/".to_string();
    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_jwt_missing_required_claims() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let mut claims = Claims {
        sub: "".to_string(), // Missing subject
        aud: ctx.auth_config.audience.clone(),
        iss: format!("https://{}/", ctx.auth_config.domain),
        exp: (Utc::now() + Duration::hours(24)).timestamp(),
        iat: Utc::now().timestamp(),
        email: None, // Missing email
        email_verified: Some(false),
        name: None,
        picture: None,
        community_roles: vec![],
    };

    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_jwks_endpoint_unavailable() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_error_mock().await; // JWKS returns 500 error

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

// Rate Limiting Tests
#[tokio::test]
#[serial]
async fn test_rate_limiting_same_user() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    // Make multiple rapid requests with same token
    let mut responses = Vec::new();
    for _ in 0..50 {
        let response = ctx.server
            .get("/api/auth/me")
            .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
            .await;
        responses.push(response.status_code());
    }

    let success_count = responses.iter()
        .filter(|&&status| status == StatusCode::OK)
        .count();

    let rate_limited_count = responses.iter()
        .filter(|&&status| status == StatusCode::TOO_MANY_REQUESTS)
        .count();

    // Either all should succeed (no rate limiting) or some should be rate limited
    assert!(success_count + rate_limited_count == 50);
    if rate_limited_count > 0 {
        println!("Rate limiting is working: {} requests were limited", rate_limited_count);
    }

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_rate_limiting_different_endpoints() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    // Test different endpoints for rate limiting
    let endpoints = vec!["/api/auth/me", "/health", "/"];
    let mut all_responses = Vec::new();

    for endpoint in &endpoints {
        for _ in 0..10 {
            let response = if endpoint.starts_with("/api/") {
                ctx.server
                    .get(endpoint)
                    .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
                    .await
            } else {
                ctx.server
                    .get(endpoint)
                    .await
            };
            all_responses.push((endpoint, response.status_code()));
        }
    }

    // Check that rate limiting might be applied per endpoint or globally
    for endpoint in &endpoints {
        let endpoint_responses: Vec<_> = all_responses.iter()
            .filter(|(ep, _)| ep == endpoint)
            .map(|(_, status)| *status)
            .collect();

        let success_count = endpoint_responses.iter()
            .filter(|&&status| status == StatusCode::OK)
            .count();

        println!("Endpoint {}: {} successful out of {}", endpoint, success_count, endpoint_responses.len());
        assert!(success_count > 0, "At least some requests should succeed for {}", endpoint);
    }

    ctx.cleanup().await;
}

// CORS Tests
#[tokio::test]
#[serial]
async fn test_cors_preflight_request() {
    let ctx = AuthTestContext::new().await;

    let response = ctx.server
        .method(axum::http::Method::OPTIONS)
        .uri("/api/auth/me")
        .add_header("Origin", "https://localhost:3000")
        .add_header("Access-Control-Request-Method", "GET")
        .add_header("Access-Control-Request-Headers", "authorization,content-type")
        .await;

    // Should handle CORS preflight
    assert!(
        response.status_code().is_success() ||
        response.status_code() == StatusCode::NO_CONTENT
    );

    // Check for CORS headers
    let headers = response.headers();
    assert!(headers.contains_key("access-control-allow-origin") ||
           headers.contains_key("Access-Control-Allow-Origin"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_cors_simple_request() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header("Origin", "https://localhost:3000")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    // Should include CORS headers
    let headers = response.headers();
    assert!(headers.contains_key("access-control-allow-origin") ||
           headers.contains_key("Access-Control-Allow-Origin"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_cors_unauthorized_origin() {
    let ctx = AuthTestContext::new().await;

    let response = ctx.server
        .get("/health")
        .add_header("Origin", "https://malicious-site.com")
        .await;

    // Should still respond (CORS is usually handled at preflight)
    response.assert_status_ok();

    ctx.cleanup().await;
}

// Input Validation and Security Tests
#[tokio::test]
#[serial]
async fn test_auth_header_length_limits() {
    let ctx = AuthTestContext::new().await;

    // Test with extremely long authorization header
    let long_token = "a".repeat(10000);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", long_token))
        .await;

    // Should handle gracefully (likely return 401 for invalid token)
    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_auth_special_characters() {
    let ctx = AuthTestContext::new().await;

    // Test with special characters in authorization header
    let special_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.%20%21%40%23%24%25%5E%26*().invalid";

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", special_token))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_multiple_auth_headers() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .add_header(header::AUTHORIZATION, "Bearer duplicate-token")
        .await;

    // Should handle duplicate headers gracefully
    assert!(
        response.status_code() == StatusCode::OK ||
        response.status_code() == StatusCode::UNAUTHORIZED ||
        response.status_code() == StatusCode::BAD_REQUEST
    );

    ctx.cleanup().await;
}

// User Context Extraction Tests
#[tokio::test]
#[serial]
async fn test_user_not_found_in_database() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    // Create claims for a user that doesn't exist in the database
    let claims = Claims {
        sub: "auth0|nonexistent-user".to_string(),
        aud: ctx.auth_config.audience.clone(),
        iss: format!("https://{}/", ctx.auth_config.domain),
        exp: (Utc::now() + Duration::hours(24)).timestamp(),
        iat: Utc::now().timestamp(),
        email: Some("nonexistent@example.com".to_string()),
        email_verified: Some(true),
        name: Some("Nonexistent User".to_string()),
        picture: None,
        community_roles: vec![],
    };

    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_user_email_verification_required() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, mut claims) = ctx.create_test_user_with_claims().await;
    claims.email_verified = Some(false); // Unverified email
    let token = ctx.create_test_jwt(&claims);

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    // Depending on implementation, might require email verification
    assert!(
        response.status_code() == StatusCode::OK ||
        response.status_code() == StatusCode::FORBIDDEN ||
        response.status_code() == StatusCode::UNAUTHORIZED
    );

    ctx.cleanup().await;
}

// Error Handling Tests
#[tokio::test]
#[serial]
async fn test_auth_middleware_database_error() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    // Close database connection to simulate DB error
    ctx.db.close().await;

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
        .await;

    // Should handle database errors gracefully
    response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);

    ctx.cleanup().await;
}

// Performance Tests
#[tokio::test]
#[serial]
async fn test_auth_middleware_concurrent_requests() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    // Make multiple concurrent authenticated requests
    let mut handles = Vec::new();
    for _ in 0..20 {
        let server = ctx.server.clone();
        let token = token.clone();

        handles.push(tokio::spawn(async move {
            server.get("/api/auth/me")
                .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
                .await
        }));
    }

    // Wait for all requests to complete
    let mut success_count = 0;
    let mut error_count = 0;

    for handle in handles {
        let response = handle.await.unwrap();
        match response.status_code() {
            StatusCode::OK => success_count += 1,
            _ => error_count += 1,
        }
    }

    assert!(success_count > 0, "At least some concurrent requests should succeed");
    println!("Concurrent auth test: {} successful, {} failed", success_count, error_count);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_auth_middleware_performance() {
    let ctx = AuthTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, claims) = ctx.create_test_user_with_claims().await;
    let token = ctx.create_test_jwt(&claims);

    // Measure performance of auth middleware
    let start_time = std::time::Instant::now();
    let mut successful_requests = 0;

    for _ in 0..100 {
        let response = ctx.server
            .get("/api/auth/me")
            .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
            .await;

        if response.status_code() == StatusCode::OK {
            successful_requests += 1;
        }
    }

    let elapsed = start_time.elapsed();
    println!("Auth performance test: {} requests in {:?} ({:.2} req/sec)",
             successful_requests, elapsed, successful_requests as f64 / elapsed.as_secs_f64());

    assert!(successful_requests > 90, "Most requests should succeed in performance test");
    assert!(elapsed.as_millis() < 30000, "Auth middleware should be reasonably fast");

    ctx.cleanup().await;
}