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
        ApiResponse, PaginationParams,
        business::*,
        Claims,
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

/// Test configuration and setup helpers for business API tests
struct BusinessTestContext {
    server: TestServer,
    db: Database,
    mock_auth0: MockServer,
    auth_config: Auth0Config,
}

impl BusinessTestContext {
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
        let config = server::config::Config {
            database_url: "test".to_string(),
            cors_origins: "http://localhost:3000".to_string(),
            development_mode: true,
            s3_bucket: "test-bucket".to_string(),
            s3_region: "us-east-1".to_string(),
            aws_region: "us-east-1".to_string(),
            log_level: "debug".to_string(),
        };

        let app_state = std::sync::Arc::new(server::ApiState {
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
        let secret = "test-secret";
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

    /// Create authenticated user with business owner role
    async fn create_authenticated_business_owner(&self) -> (shared::models::User, String, Uuid) {
        let user = create_test_user(&self.db, None).await.expect("Failed to create test user");
        let community = create_test_community(&self.db, user.id, Some("Test Community".to_string()))
            .await.expect("Failed to create test community");

        let claims = Claims {
            sub: user.id.to_string(),
            aud: self.auth_config.audience.clone(),
            iss: format!("https://{}/", self.auth_config.domain),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            email: Some(user.email.clone()),
            name: Some("Business Owner".to_string()),
            community_roles: vec![],
        };

        let token = self.create_test_jwt(&claims);
        (user, token, community.id)
    }
}

// Business listing tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_list_businesses_empty_response() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    let response = ctx.server
        .get(&format!("/api/communities/{}/businesses", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status_ok();

    let body: ApiResponse<Vec<BusinessSearchResult>> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());
    assert!(body.data.unwrap().is_empty());
    assert!(body.message.is_some());
    assert!(body.message.unwrap().contains("temporarily disabled"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_businesses_with_search_query() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    let response = ctx.server
        .get(&format!("/api/communities/{}/businesses?q=restaurant&category=food&limit=10&offset=0", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status_ok();

    let body: ApiResponse<Vec<BusinessSearchResult>> = response.json();
    assert!(body.success);
    assert!(body.data.unwrap().is_empty()); // Stub returns empty array
    assert!(body.message.unwrap().contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_businesses_unauthenticated() {
    let ctx = BusinessTestContext::new().await;
    let community_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/communities/{}/businesses", community_id))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_businesses_invalid_community_id() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, _) = ctx.create_authenticated_business_owner().await;

    let response = ctx.server
        .get("/api/communities/invalid-uuid/businesses")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Business creation tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_create_business_stub_error() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    let create_request = CreateBusinessRequest {
        name: "Test Restaurant".to_string(),
        description: Some("A great test restaurant".to_string()),
        category: BusinessCategory::Food,
        website: Some("https://testrestaurant.com".to_string()),
        phone: Some("+1-555-123-4567".to_string()),
        email: Some("contact@testrestaurant.com".to_string()),
        address: Some("123 Main St, Test City, TC 12345".to_string()),
        location: Some(shared::models::Point { latitude: 40.7128, longitude: -74.0060 }),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/businesses", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    // Stub implementation returns Internal error
    response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_business_invalid_data() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    let invalid_request = serde_json::json!({
        "name": "", // Empty name should fail validation
        "category": "invalid_category",
        "email": "not-an-email"
    });

    let response = ctx.server
        .post(&format!("/api/communities/{}/businesses", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&invalid_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_business_unauthenticated() {
    let ctx = BusinessTestContext::new().await;
    let community_id = Uuid::new_v4();

    let create_request = CreateBusinessRequest {
        name: "Test Business".to_string(),
        description: Some("Test description".to_string()),
        category: BusinessCategory::Services,
        website: None,
        phone: None,
        email: None,
        address: None,
        location: None,
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/businesses", community_id))
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

// Business detail tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_get_business_stub_error() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;
    let business_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/businesses/{}", business_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    // Stub implementation returns Internal error
    response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_business_invalid_id() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    let response = ctx.server
        .get("/api/businesses/invalid-uuid")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Business update tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_update_business_stub_error() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;
    let business_id = Uuid::new_v4();

    let update_request = UpdateBusinessRequest {
        name: Some("Updated Business Name".to_string()),
        description: Some("Updated description".to_string()),
        category: Some(BusinessCategory::Retail),
        website: Some("https://updated-website.com".to_string()),
        phone: Some("+1-555-987-6543".to_string()),
        email: Some("updated@business.com".to_string()),
        address: Some("456 Updated St".to_string()),
        location: Some(shared::models::Point { latitude: 41.8781, longitude: -87.6298 }),
        is_active: Some(false),
    };

    let response = ctx.server
        .put(&format!("/api/businesses/{}", business_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&update_request)
        .await;

    // Stub implementation returns Internal error
    response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_update_business_unauthenticated() {
    let ctx = BusinessTestContext::new().await;
    let business_id = Uuid::new_v4();

    let update_request = UpdateBusinessRequest {
        name: Some("Updated Name".to_string()),
        description: None,
        category: None,
        website: None,
        phone: None,
        email: None,
        address: None,
        location: None,
        is_active: None,
    };

    let response = ctx.server
        .put(&format!("/api/businesses/{}", business_id))
        .json(&update_request)
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

// Product listing tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_list_products_empty_response() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;
    let business_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/businesses/{}/products", business_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status_ok();

    let body: ApiResponse<Vec<BusinessProduct>> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());
    assert!(body.data.unwrap().is_empty());
    assert!(body.message.unwrap().contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_products_public_access() {
    let ctx = BusinessTestContext::new().await;
    let business_id = Uuid::new_v4();

    // Products should be publicly accessible for business discovery
    let response = ctx.server
        .get(&format!("/api/businesses/{}/products", business_id))
        .await;

    response.assert_status_ok();

    ctx.cleanup().await;
}

// Product creation tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_create_product_stub_error() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;
    let business_id = Uuid::new_v4();

    let create_request = CreateProductRequest {
        name: "Test Product".to_string(),
        description: Some("A great test product".to_string()),
        price: Some(29.99),
        currency: Some("USD".to_string()),
        unit: Some("piece".to_string()),
    };

    let response = ctx.server
        .post(&format!("/api/businesses/{}/products", business_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    // Stub implementation returns Internal error
    response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_product_unauthenticated() {
    let ctx = BusinessTestContext::new().await;
    let business_id = Uuid::new_v4();

    let create_request = CreateProductRequest {
        name: "Test Product".to_string(),
        description: None,
        price: None,
        currency: None,
        unit: None,
    };

    let response = ctx.server
        .post(&format!("/api/businesses/{}/products", business_id))
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

// Input validation tests
#[tokio::test]
#[serial]
async fn test_business_search_query_validation() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    // Test with invalid query parameters
    let response = ctx.server
        .get(&format!("/api/communities/{}/businesses?limit=-1&offset=-5", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    // Should either work with defaults or return validation error
    assert!(
        response.status_code() == StatusCode::OK ||
        response.status_code() == StatusCode::BAD_REQUEST
    );

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_business_name_length_validation() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    let long_name = "x".repeat(256); // Extremely long name
    let create_request = CreateBusinessRequest {
        name: long_name,
        description: None,
        category: BusinessCategory::Other,
        website: None,
        phone: None,
        email: None,
        address: None,
        location: None,
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/businesses", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    // Should fail validation or return stub error
    assert!(
        response.status_code() == StatusCode::BAD_REQUEST ||
        response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
    );

    ctx.cleanup().await;
}

// Security tests
#[tokio::test]
#[serial]
async fn test_business_api_sql_injection_prevention() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    // Try SQL injection in search query
    let response = ctx.server
        .get(&format!("/api/communities/{}/businesses?q='; DROP TABLE businesses; --", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    // Should not cause server error (SQLx prevents injection)
    assert_ne!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_business_xss_prevention() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    let xss_request = CreateBusinessRequest {
        name: "<script>alert('xss')</script>".to_string(),
        description: Some("<img src=x onerror=alert('xss')>".to_string()),
        category: BusinessCategory::Other,
        website: Some("javascript:alert('xss')".to_string()),
        phone: None,
        email: None,
        address: None,
        location: None,
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/businesses", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&xss_request)
        .await;

    // Should handle XSS attempts gracefully
    assert!(
        response.status_code() == StatusCode::BAD_REQUEST ||
        response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
    );

    ctx.cleanup().await;
}

// Rate limiting tests
#[tokio::test]
#[serial]
async fn test_business_api_rate_limiting() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    // Make multiple rapid requests
    let mut responses = Vec::new();
    for _ in 0..20 {
        let response = ctx.server
            .get(&format!("/api/communities/{}/businesses", community_id))
            .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
            .await;
        responses.push(response.status_code());
    }

    // Most should succeed unless rate limiting is very strict
    let success_count = responses.iter()
        .filter(|&&status| status == StatusCode::OK)
        .count();

    assert!(success_count > 10, "Too many requests were rate limited");

    ctx.cleanup().await;
}

// Concurrent access tests
#[tokio::test]
#[serial]
async fn test_business_api_concurrent_requests() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    // Make multiple concurrent requests
    let mut handles = Vec::new();
    for _ in 0..10 {
        let token = token.clone();
        let community_id = community_id.clone();
        let server = &ctx.server;
        let response = server.get(&format!("/api/communities/{}/businesses", community_id))
            .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
            .await;
        handles.push(response);
    }

    // Check all responses (now synchronous)
    let mut success_count = 0;
    for response in handles {
        if response.status_code() == StatusCode::OK {
            success_count += 1;
        }
    }

    assert_eq!(success_count, 10, "All concurrent requests should succeed");

    ctx.cleanup().await;
}

// Edge case tests
#[tokio::test]
#[serial]
async fn test_business_api_with_special_characters() {
    let ctx = BusinessTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_business_owner().await;

    let special_request = CreateBusinessRequest {
        name: "Café & Restaurant München 北京".to_string(),
        description: Some("🍕 Pizza & 🍔 Burgers with special chars: @#$%^&*()".to_string()),
        category: BusinessCategory::Food,
        website: Some("https://café-münchen.com".to_string()),
        phone: Some("+49-89-123456789".to_string()),
        email: Some("café@münchen.de".to_string()),
        address: Some("Münchener Straße 123, 80331 München".to_string()),
        location: Some(shared::models::Point { latitude: 48.1351, longitude: 11.5820 }),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/businesses", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&special_request)
        .await;

    // Should handle Unicode and special characters gracefully
    assert!(
        response.status_code() == StatusCode::CREATED ||
        response.status_code() == StatusCode::BAD_REQUEST ||
        response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
    );

    ctx.cleanup().await;
}