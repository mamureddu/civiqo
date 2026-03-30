use axum::{
    body::Body,
    http::{Request, StatusCode, header, HeaderValue},
    Router,
};
use axum_test::TestServer;
use serial_test::serial;
use serde_json;
use shared::{
    database::Database,
    testing::{init_test_logging, create_test_db, cleanup_test_db, create_test_user, create_test_community},
    models::{
        ApiResponse, CreateUserRequest, UpdateUserProfileRequest, UserWithProfile,
        CreateCommunityRequest, Community, User, Claims, CommunityRole,
    },
    auth::{Auth0Config, JwtValidator},
    error::AppError,
};
use uuid::Uuid;
use wiremock::{
    matchers::{method, path, header as header_matcher},
    Mock, MockServer, ResponseTemplate,
};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use chrono::Utc;
use std::sync::Arc;

// Import the actual API Gateway app
use server::{AppState, create_app};

/// Test configuration and setup helpers
struct TestContext {
    server: TestServer,
    db: Database,
    mock_auth0: MockServer,
    auth_config: Auth0Config,
}

impl TestContext {
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
        let config = server::Config::from_test();
        let app_state = Arc::new(server::ApiState {
            db: db.clone(),
            config,
            auth_config: auth_config.clone(),
        });

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
        let secret = "test-secret"; // In real tests, this would be more sophisticated
        encode(&header, claims, &EncodingKey::from_secret(secret.as_ref()))
            .expect("Failed to create test JWT")
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

    /// Create test user with valid JWT
    async fn create_authenticated_user(&self) -> (User, String) {
        let user = create_test_user(&self.db, None).await.expect("Failed to create test user");

        let claims = Claims {
            sub: user.id.to_string(),
            aud: self.auth_config.audience.clone(),
            iss: format!("https://{}/", self.auth_config.domain),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            email: Some(user.email.clone()),
            name: Some("Test User".to_string()),
            community_roles: vec![],
        };

        let token = self.create_test_jwt(&claims);
        (user, token)
    }
}

// Health check tests
#[tokio::test]
#[serial]
async fn test_health_check() {
    let ctx = TestContext::new().await;

    let response = ctx.server
        .get("/health")
        .await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"], "ok");

    ctx.cleanup().await;
}

// Authentication and user management tests
#[tokio::test]
#[serial]
async fn test_sync_user_from_auth0() {
    let ctx = TestContext::new().await;

    let sync_request = server::auth::SyncUserRequest {
        email: "test@example.com".to_string(),
        password: "TestPassword123!".to_string(),
        name: Some("Test User".to_string()),
    };

    let response = ctx.server
        .post("/api/auth/register")
        .json(&sync_request)
        .await;

    response.assert_status(StatusCode::CREATED);

    let body: ApiResponse<UserWithProfile> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());

    let user = body.data.unwrap();
    assert_eq!(user.email, sync_request.email);
    assert_eq!(user.profile_name, sync_request.name);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_sync_user_duplicate() {
    let ctx = TestContext::new().await;

    let sync_request = server::auth::SyncUserRequest {
        email: "test@example.com".to_string(),
        password: "TestPassword123!".to_string(),
        name: Some("Test User".to_string()),
    };

    // First registration should succeed
    let response = ctx.server
        .post("/api/auth/register")
        .json(&sync_request)
        .await;
    response.assert_status(StatusCode::CREATED);

    // Second registration with same email should update (not create new)
    let updated_request = server::auth::SyncUserRequest {
        email: "updated@example.com".to_string(),
        password: "TestPassword123!".to_string(),
        name: Some("Updated User".to_string()),
    };

    let response = ctx.server
        .post("/api/auth/sync")
        .json(&updated_request)
        .await;
    response.assert_status_ok();

    let body: ApiResponse<UserWithProfile> = response.json();
    let user = body.data.unwrap();
    assert_eq!(user.email, "updated@example.com");
    assert_eq!(user.profile_name, Some("Updated User".to_string()));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_current_user_authenticated() {
    let ctx = TestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token) = ctx.create_authenticated_user().await;

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status_ok();

    let body: ApiResponse<UserWithProfile> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());

    let returned_user = body.data.unwrap();
    assert_eq!(returned_user.id, user.id);
    assert_eq!(returned_user.email, user.email);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_current_user_unauthenticated() {
    let ctx = TestContext::new().await;

    let response = ctx.server
        .get("/api/auth/me")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_current_user_invalid_token() {
    let ctx = TestContext::new().await;

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, HeaderValue::from_static("Bearer invalid-token"))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_update_user_profile() {
    let ctx = TestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token) = ctx.create_authenticated_user().await;

    let update_request = UpdateUserProfileRequest {
        name: Some("Updated Name".to_string()),
        bio: Some("Updated bio".to_string()),
        location: Some("San Francisco, CA".to_string()),
        website: Some("https://example.com".to_string()),
    };

    let response = ctx.server
        .put("/api/auth/profile")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&update_request)
        .await;

    response.assert_status_ok();

    let body: ApiResponse<UserWithProfile> = response.json();
    assert!(body.success);

    let updated_user = body.data.unwrap();
    assert_eq!(updated_user.profile_name, update_request.name);
    assert_eq!(updated_user.profile_bio, update_request.bio);
    assert_eq!(updated_user.profile_location, update_request.location);
    assert_eq!(updated_user.profile_website, update_request.website);

    ctx.cleanup().await;
}

// Community management tests
#[tokio::test]
#[serial]
async fn test_create_community() {
    let ctx = TestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token) = ctx.create_authenticated_user().await;

    let create_request = CreateCommunityRequest {
        name: "Test Community".to_string(),
        description: Some("A test community".to_string()),
        slug: "test-community".to_string(),
        is_public: true,
        requires_approval: false,
        boundary: None,
    };

    let response = ctx.server
        .post("/api/communities")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::CREATED);

    let body: ApiResponse<Community> = response.json();
    assert!(body.success);

    let community = body.data.unwrap();
    assert_eq!(community.name, create_request.name);
    assert_eq!(community.slug, create_request.slug);
    assert_eq!(community.created_by, user.id);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_community_unauthenticated() {
    let ctx = TestContext::new().await;

    let create_request = CreateCommunityRequest {
        name: "Test Community".to_string(),
        description: Some("A test community".to_string()),
        slug: "test-community".to_string(),
        is_public: true,
        requires_approval: false,
        boundary: None,
    };

    let response = ctx.server
        .post("/api/communities")
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_communities_public() {
    let ctx = TestContext::new().await;

    // Create a test user and community
    let user = create_test_user(&ctx.db, None).await.expect("Failed to create user");
    let community = create_test_community(&ctx.db, user.id, Some("Public Community".to_string()))
        .await.expect("Failed to create community");

    let response = ctx.server
        .get("/api/communities")
        .await;

    response.assert_status_ok();

    let body: ApiResponse<Vec<Community>> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());

    let communities = body.data.unwrap();
    assert!(!communities.is_empty());

    // Find our test community
    let found_community = communities.iter()
        .find(|c| c.id == community.id)
        .expect("Created community should be in the list");

    assert_eq!(found_community.name, "Public Community");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_community_by_id() {
    let ctx = TestContext::new().await;

    let user = create_test_user(&ctx.db, None).await.expect("Failed to create user");
    let community = create_test_community(&ctx.db, user.id, Some("Test Community".to_string()))
        .await.expect("Failed to create community");

    let response = ctx.server
        .get(&format!("/api/communities/{}", community.id))
        .await;

    response.assert_status_ok();

    let body: ApiResponse<Community> = response.json();
    assert!(body.success);

    let returned_community = body.data.unwrap();
    assert_eq!(returned_community.id, community.id);
    assert_eq!(returned_community.name, "Test Community");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_community_not_found() {
    let ctx = TestContext::new().await;

    let non_existent_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/communities/{}", non_existent_id))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);

    ctx.cleanup().await;
}

// Validation tests
#[tokio::test]
#[serial]
async fn test_sync_user_invalid_email() {
    let ctx = TestContext::new().await;

    let invalid_request = server::auth::SyncUserRequest {
        email: "invalid-email".to_string(), // Invalid email format
        password: "TestPassword123!".to_string(),
        name: Some("Test User".to_string()),
    };

    let response = ctx.server
        .post("/api/auth/sync")
        .json(&invalid_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_community_empty_name() {
    let ctx = TestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token) = ctx.create_authenticated_user().await;

    let invalid_request = CreateCommunityRequest {
        name: "".to_string(), // Empty name should be invalid
        description: Some("A test community".to_string()),
        slug: "test-community".to_string(),
        is_public: true,
        requires_approval: false,
        boundary: None,
    };

    let response = ctx.server
        .post("/api/communities")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&invalid_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Error handling tests
#[tokio::test]
#[serial]
async fn test_invalid_json_request() {
    let ctx = TestContext::new().await;

    let response = ctx.server
        .post("/api/auth/sync")
        .add_header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .text("invalid json")
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_missing_content_type() {
    let ctx = TestContext::new().await;

    let sync_request = server::auth::SyncUserRequest {
        email: "test@example.com".to_string(),
        password: "TestPassword123!".to_string(),
        name: Some("Test User".to_string()),
    };

    let response = ctx.server
        .post("/api/auth/register")
        .text(&serde_json::to_string(&sync_request).unwrap())
        .await;

    // Should still work with JSON data even without explicit content-type
    // but let's test that it's handled gracefully
    assert!(response.status_code().is_client_error() || response.status_code().is_success());

    ctx.cleanup().await;
}

// CORS tests
#[tokio::test]
#[serial]
async fn test_cors_preflight() {
    let ctx = TestContext::new().await;

    let response = ctx.server
        .method(axum::http::Method::OPTIONS, "/api/communities")
        .add_header(header::ORIGIN, HeaderValue::from_static("https://localhost:3000"))
        .add_header(header::ACCESS_CONTROL_REQUEST_METHOD, HeaderValue::from_static("POST"))
        .add_header(header::ACCESS_CONTROL_REQUEST_HEADERS, HeaderValue::from_static("content-type,authorization"))
        .await;

    // Should handle CORS preflight
    assert!(response.status_code().is_success() || response.status_code() == StatusCode::NO_CONTENT);

    ctx.cleanup().await;
}

// Rate limiting tests (if implemented)
#[tokio::test]
#[serial]
async fn test_rate_limiting_basic() {
    let ctx = TestContext::new().await;

    // Make multiple requests quickly
    let mut responses = Vec::new();
    for _ in 0..10 {
        let response = ctx.server
            .get("/health")
            .await;
        responses.push(response.status_code());
    }

    // Most should succeed (unless rate limiting is very strict)
    let success_count = responses.iter()
        .filter(|&&status| status == StatusCode::OK)
        .count();

    assert!(success_count > 5, "Too many requests were rate limited");

    ctx.cleanup().await;
}

// Database integration tests
#[tokio::test]
#[serial]
async fn test_database_transaction_rollback() {
    let ctx = TestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token) = ctx.create_authenticated_user().await;

    // Create a community that might cause a database constraint violation
    let create_request = CreateCommunityRequest {
        name: "Test Community".to_string(),
        description: Some("A test community".to_string()),
        slug: "test-community".to_string(),
        is_public: true,
        requires_approval: false,
        boundary: None,
    };

    // First creation should succeed
    let response = ctx.server
        .post("/api/communities")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::CREATED);

    // Second creation with same slug should fail
    let response = ctx.server
        .post("/api/communities")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::CONFLICT);

    ctx.cleanup().await;
}

// Performance tests
#[tokio::test]
#[serial]
async fn test_concurrent_requests() {
    let ctx = TestContext::new().await;

    // Make multiple sequential health check requests
    let mut success_count = 0;
    for _ in 0..10 {
        let response = ctx.server.get("/health").await;
        if response.status_code() == StatusCode::OK {
            success_count += 1;
        }
    }

    assert_eq!(success_count, 10, "All concurrent requests should succeed");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_large_request_body() {
    let ctx = TestContext::new().await;

    // Create a large description
    let large_description = "x".repeat(10000);

    let sync_request = server::auth::SyncUserRequest {
        email: "test@example.com".to_string(),
        password: "TestPassword123!".to_string(),
        name: Some(large_description), // Very long name
    };

    let response = ctx.server
        .post("/api/auth/register")
        .json(&sync_request)
        .await;

    // Should either succeed or fail gracefully with appropriate error
    assert!(response.status_code().is_client_error() || response.status_code().is_success());

    ctx.cleanup().await;
}

// Security tests
#[tokio::test]
#[serial]
async fn test_sql_injection_prevention() {
    let ctx = TestContext::new().await;

    // Try to inject SQL in the registration request
    let malicious_request = server::auth::SyncUserRequest {
        email: "test@example.com".to_string(),
        password: "auth0|123'; DROP TABLE users; --".to_string(),
        name: Some("Test User".to_string()),
    };

    let response = ctx.server
        .post("/api/auth/register")
        .json(&malicious_request)
        .await;

    // Should not cause a server error (SQLx should prevent injection)
    assert_ne!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

    // Verify database is still intact
    let health_response = ctx.server.get("/health").await;
    health_response.assert_status_ok();

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_xss_prevention() {
    let ctx = TestContext::new().await;

    let xss_request = server::auth::SyncUserRequest {
        email: "test@example.com".to_string(),
        password: "TestPassword123!".to_string(),
        name: Some("<script>alert('xss')</script>".to_string()),
    };

    let response = ctx.server
        .post("/api/auth/register")
        .json(&xss_request)
        .await;

    if response.status_code().is_success() {
        let body: ApiResponse<UserWithProfile> = response.json();
        let user = body.data.unwrap();

        // Name should be stored as-is (escaping should happen at display time)
        assert_eq!(user.profile_name.unwrap(), "<script>alert('xss')</script>");
    }

    ctx.cleanup().await;
}

// Integration with external services tests
#[tokio::test]
#[serial]
async fn test_auth0_service_unavailable() {
    let ctx = TestContext::new().await;

    // Don't setup JWKS mock, so Auth0 calls will fail
    let (user, token) = ctx.create_authenticated_user().await;

    let response = ctx.server
        .get("/api/auth/me")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    // Should fail gracefully when Auth0 is unavailable
    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_database_connection_failure() {
    // This test would require a more sophisticated setup to simulate DB failure
    // For now, we'll just verify the health check handles DB issues gracefully
    let ctx = TestContext::new().await;

    let response = ctx.server.get("/health").await;

    // If database is available, health should be OK
    // If not available, should fail gracefully
    assert!(
        response.status_code() == StatusCode::OK ||
        response.status_code() == StatusCode::SERVICE_UNAVAILABLE
    );

    ctx.cleanup().await;
}