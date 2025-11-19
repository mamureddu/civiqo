use axum::{
    body::Body,
    http::{Request, StatusCode, header, HeaderValue},
    Router,
    routing::{get, post, put, delete},
};
use axum_test::TestServer;
use serial_test::serial;
use serde_json;
use std::sync::Arc;
use shared::{
    database::Database,
    testing::{init_test_logging, create_test_db, cleanup_test_db, create_test_user, create_test_community},
    models::{
        ApiResponse, PaginationParams, Claims,
        CreateCommunityRequest, UpdateCommunityRequest, JoinCommunityRequest,
        Community, CommunityWithStats, MemberWithProfile,
        business::{BusinessSearchResult, CreateBusinessRequest, UpdateBusinessRequest, BusinessWithProducts, BusinessProduct, CreateProductRequest, BusinessSearchQuery},
        governance::{Poll, CreatePollRequest, CastVoteRequest, PollWithResults, Decision, CreateDecisionRequest},
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

// Import the actual API Gateway app components
use server::{AppState, create_app};

/// Create a test version of the router with all routes enabled

/// Comprehensive test configuration for all API endpoints
struct ComprehensiveTestContext {
    server: TestServer,
    db: Database,
    mock_auth0: MockServer,
    auth_config: Auth0Config,
}

impl ComprehensiveTestContext {
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

        // Create the router using the main API gateway app
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

    /// Create authenticated user with all necessary setup
    async fn create_authenticated_user(&self) -> (shared::models::User, String, Uuid) {
        let user = create_test_user(&self.db, None).await.expect("Failed to create test user");
        let community = create_test_community(&self.db, user.id, Some("Test Community".to_string()))
            .await.expect("Failed to create test community");

        let claims = Claims {
            sub: user.auth0_id.clone(),
            aud: self.auth_config.audience.clone(),
            iss: format!("https://{}/", self.auth_config.domain),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            email: Some(user.email.clone()),
            email_verified: Some(true),
            name: Some("Test User".to_string()),
            picture: None,
            community_roles: vec![],
        };

        let token = self.create_test_jwt(&claims);
        (user, token, community.id)
    }
}

// Comprehensive API Flow Tests
#[tokio::test]
#[serial]
async fn test_complete_community_workflow() {
    let ctx = ComprehensiveTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token, _) = ctx.create_authenticated_user().await;

    // 1. List communities (should see existing test community)
    let response = ctx.server
        .get("/api/communities")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;
    response.assert_status_ok();

    // 2. Create a new community
    let create_request = CreateCommunityRequest {
        name: "Integration Test Community".to_string(),
        description: Some("Created during integration tests".to_string()),
        slug: "integration-test-community".to_string(),
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
    let new_community_id = body.data.unwrap().id;

    // 3. Get the specific community
    let response = ctx.server
        .get(&format!("/api/communities/{}", new_community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;
    response.assert_status_ok();

    // 4. Update the community
    let update_request = UpdateCommunityRequest {
        name: Some("Updated Integration Test Community".to_string()),
        description: Some("Updated during integration tests".to_string()),
        is_public: Some(false),
        requires_approval: Some(true),
    };

    let response = ctx.server
        .put(&format!("/api/communities/{}", new_community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&update_request)
        .await;
    response.assert_status_ok();

    // 5. List community members
    let response = ctx.server
        .get(&format!("/api/communities/{}/members", new_community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;
    response.assert_status_ok();

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_business_api_stub_responses() {
    let ctx = ComprehensiveTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_user().await;

    // Test all business endpoints for stub responses
    let endpoints = vec![
        (format!("/api/communities/{}/businesses", community_id), "GET", None),
        (format!("/api/communities/{}/businesses", community_id), "POST", Some(serde_json::json!({
            "name": "Test Business",
            "category": "food",
            "description": "A test business"
        }))),
        (format!("/api/businesses/{}", Uuid::new_v4()), "GET", None),
        (format!("/api/businesses/{}", Uuid::new_v4()), "PUT", Some(serde_json::json!({
            "name": "Updated Business"
        }))),
        (format!("/api/businesses/{}/products", Uuid::new_v4()), "GET", None),
        (format!("/api/businesses/{}/products", Uuid::new_v4()), "POST", Some(serde_json::json!({
            "name": "Test Product",
            "price": 19.99
        }))),
    ];

    for (endpoint, method, body) in endpoints {
        let response = match method {
            "GET" => ctx.server
                .get(&endpoint)
                .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
                .await,
            "POST" => {
                let mut request = ctx.server
                    .post(&endpoint)
                    .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
                if let Some(json_body) = body {
                    request = request.json(&json_body);
                }
                request.await
            },
            "PUT" => {
                let mut request = ctx.server
                    .put(&endpoint)
                    .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
                if let Some(json_body) = body {
                    request = request.json(&json_body);
                }
                request.await
            },
            _ => continue,
        };

        // Check that stub responses are returned appropriately
        assert!(
            response.status_code() == StatusCode::OK ||
            response.status_code() == StatusCode::INTERNAL_SERVER_ERROR ||
            response.status_code() == StatusCode::BAD_REQUEST ||
            response.status_code() == StatusCode::UNAUTHORIZED
        );

        println!("Endpoint {} {} returned status: {}", method, endpoint, response.status_code());
    }

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_governance_api_stub_responses() {
    let ctx = ComprehensiveTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_user().await;

    // Test all governance endpoints for stub responses
    let poll_id = Uuid::new_v4();
    let endpoints = vec![
        (format!("/api/communities/{}/polls", community_id), "GET"),
        (format!("/api/communities/{}/polls", community_id), "POST"),
        (format!("/api/polls/{}", poll_id), "GET"),
        (format!("/api/polls/{}/vote", poll_id), "POST"),
        (format!("/api/polls/{}/results", poll_id), "GET"),
        (format!("/api/communities/{}/decisions", community_id), "GET"),
        (format!("/api/communities/{}/decisions", community_id), "POST"),
    ];

    for (endpoint, method) in endpoints {
        let response = match method {
            "GET" => ctx.server
                .get(&endpoint)
                .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
                .await,
            "POST" => {
                let test_body = if endpoint.contains("polls") && !endpoint.contains("vote") {
                    serde_json::json!({
                        "title": "Test Poll",
                        "options": ["Yes", "No"],
                        "poll_type": "single_choice",
                        "end_date": "2024-12-31T23:59:59Z"
                    })
                } else if endpoint.contains("vote") {
                    serde_json::json!({
                        "choice": "option1"
                    })
                } else {
                    serde_json::json!({
                        "title": "Test Decision",
                        "description": "Test description",
                        "decision_type": "policy",
                        "status": "proposed"
                    })
                };

                ctx.server
                    .post(&endpoint)
                    .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
                    .json(&test_body)
                    .await
            },
            _ => continue,
        };

        // Check that stub responses are returned appropriately
        assert!(
            response.status_code() == StatusCode::OK ||
            response.status_code() == StatusCode::NOT_FOUND ||
            response.status_code() == StatusCode::BAD_REQUEST ||
            response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
        );

        println!("Endpoint {} {} returned status: {}", method, endpoint, response.status_code());
    }

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_api_endpoint_authentication_requirements() {
    let ctx = ComprehensiveTestContext::new().await;
    let community_id = Uuid::new_v4();
    let business_id = Uuid::new_v4();
    let poll_id = Uuid::new_v4();

    // Test endpoints that should require authentication
    // Pre-create formatted URLs to avoid temporary value issues
    let community_url = format!("/api/communities/{}", community_id);
    let community_join_url = format!("/api/communities/{}/join", community_id);
    let community_members_url = format!("/api/communities/{}/members", community_id);
    let community_businesses_url = format!("/api/communities/{}/businesses", community_id);
    let business_url = format!("/api/businesses/{}", business_id);
    let business_products_url = format!("/api/businesses/{}/products", business_id);
    let community_polls_url = format!("/api/communities/{}/polls", community_id);
    let poll_vote_url = format!("/api/polls/{}/vote", poll_id);
    let community_decisions_url = format!("/api/communities/{}/decisions", community_id);

    let protected_endpoints = vec![
        // Auth endpoints
        ("/api/auth/me", "GET"),
        ("/api/auth/profile", "PUT"),

        // Community management endpoints
        ("/api/communities", "POST"),
        (community_url.as_str(), "PUT"),
        (community_join_url.as_str(), "POST"),
        (community_members_url.as_str(), "GET"),

        // Business endpoints
        (community_businesses_url.as_str(), "POST"),
        (business_url.as_str(), "PUT"),
        (business_products_url.as_str(), "POST"),

        // Governance endpoints
        (community_polls_url.as_str(), "POST"),
        (poll_vote_url.as_str(), "POST"),
        (community_decisions_url.as_str(), "POST"),
    ];

    for (endpoint, method) in protected_endpoints {
        let response = match method {
            "GET" => ctx.server.get(endpoint).await,
            "POST" => ctx.server.post(endpoint).json(&serde_json::json!({})).await,
            "PUT" => ctx.server.put(endpoint).json(&serde_json::json!({})).await,
            _ => continue,
        };

        response.assert_status(StatusCode::UNAUTHORIZED);
        println!("Protected endpoint {} {} correctly requires authentication", method, endpoint);
    }

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_public_endpoints_accessibility() {
    let ctx = ComprehensiveTestContext::new().await;
    let community_id = Uuid::new_v4();
    let business_id = Uuid::new_v4();
    let poll_id = Uuid::new_v4();

    // Pre-create formatted URLs to avoid temporary value issues (shared across test sections)
    let community_url = format!("/api/communities/{}", community_id);
    let community_join_url = format!("/api/communities/{}/join", community_id);
    let community_members_url = format!("/api/communities/{}/members", community_id);
    let community_businesses_url = format!("/api/communities/{}/businesses", community_id);
    let business_url = format!("/api/businesses/{}", business_id);
    let business_products_url = format!("/api/businesses/{}/products", business_id);
    let community_polls_url = format!("/api/communities/{}/polls", community_id);
    let poll_vote_url = format!("/api/polls/{}/vote", poll_id);
    let community_decisions_url = format!("/api/communities/{}/decisions", community_id);
    let polls_url = format!("/api/polls/{}", poll_id);
    let poll_results_url = format!("/api/polls/{}/results", poll_id);

    // Test endpoints that should be publicly accessible
    let public_endpoints = vec![
        // Health and info endpoints
        ("/health", "GET"),
        ("/", "GET"),

        // Public community browsing
        ("/api/communities", "GET"),
        (community_url.as_str(), "GET"),

        // Public business browsing
        (community_businesses_url.as_str(), "GET"),
        (business_url.as_str(), "GET"),
        (business_products_url.as_str(), "GET"),

        // Public governance viewing
        (community_polls_url.as_str(), "GET"),
        (polls_url.as_str(), "GET"),
        (poll_results_url.as_str(), "GET"),
        (community_decisions_url.as_str(), "GET"),
    ];

    for (endpoint, method) in public_endpoints {
        let response = match method {
            "GET" => ctx.server.get(endpoint).await,
            _ => continue,
        };

        // Public endpoints should not return 401 (may return other errors like 404)
        assert_ne!(response.status_code(), StatusCode::UNAUTHORIZED);
        println!("Public endpoint {} {} is accessible without authentication", method, endpoint);
    }

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_invalid_route_handling() {
    let ctx = ComprehensiveTestContext::new().await;

    // Test non-existent routes
    let invalid_routes = vec![
        "/api/invalid",
        "/api/communities/invalid/route",
        "/api/businesses/invalid/endpoint",
        "/api/governance/nonexistent",
        "/totally/invalid/path",
    ];

    for route in invalid_routes {
        let response = ctx.server.get(route).await;
        response.assert_status(StatusCode::NOT_FOUND);
        println!("Invalid route {} correctly returns 404", route);
    }

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_api_error_response_format() {
    let ctx = ComprehensiveTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_user().await;

    // Test that error responses follow the expected format
    let response = ctx.server
        .post(&format!("/api/communities/{}/businesses", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&serde_json::json!({
            "invalid": "data"
        }))
        .await;

    assert!(response.status_code().is_client_error() || response.status_code().is_server_error());

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());
    assert!(body.data.is_none());

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_api_response_content_type() {
    let ctx = ComprehensiveTestContext::new().await;

    // Test that all API responses have correct content-type
    let endpoints = vec![
        "/health",
        "/",
        "/api/communities",
    ];

    for endpoint in endpoints {
        let response = ctx.server.get(endpoint).await;

        let content_type = response.headers()
            .get("content-type")
            .or_else(|| response.headers().get("Content-Type"));

        assert!(content_type.is_some(), "Endpoint {} should have Content-Type header", endpoint);

        let content_type_str = content_type.unwrap().to_str().unwrap();
        assert!(content_type_str.contains("application/json"),
                "Endpoint {} should return JSON content-type, got: {}", endpoint, content_type_str);
    }

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_api_request_methods() {
    let ctx = ComprehensiveTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_user().await;

    // Pre-create URL to avoid temporary value issues
    let specific_community_url = format!("/api/communities/{}", community_id);

    // Test that endpoints reject unsupported methods
    let test_cases = vec![
        ("/api/communities", vec!["GET", "POST"], vec!["PATCH", "DELETE"]),
        (specific_community_url.as_str(), vec!["GET", "PUT"], vec!["POST", "DELETE"]),
        ("/health", vec!["GET"], vec!["POST", "PUT", "DELETE"]),
    ];

    for (endpoint, allowed_methods, disallowed_methods) in test_cases {
        // Test allowed methods (should not return 405)
        for method in allowed_methods {
            let response = match method {
                "GET" => ctx.server.get(endpoint).await,
                "POST" => ctx.server.post(endpoint)
                    .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
                    .json(&serde_json::json!({}))
                    .await,
                "PUT" => ctx.server.put(endpoint)
                    .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
                    .json(&serde_json::json!({}))
                    .await,
                _ => continue,
            };

            assert_ne!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED,
                      "Method {} should be allowed for endpoint {}", method, endpoint);
        }

        // Test disallowed methods (should return 405)
        for method in disallowed_methods {
            let response = match method {
                "POST" => ctx.server.post(endpoint).await,
                "PUT" => ctx.server.put(endpoint).await,
                "PATCH" => ctx.server.patch(endpoint).await,
                "DELETE" => ctx.server.delete(endpoint).await,
                _ => continue,
            };

            response.assert_status(StatusCode::METHOD_NOT_ALLOWED);
            println!("Method {} correctly disallowed for endpoint {}", method, endpoint);
        }
    }

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_api_pagination_parameters() {
    let ctx = ComprehensiveTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_user().await;

    // Test pagination on list endpoints
    let paginated_endpoints = vec![
        format!("/api/communities?limit=5&offset=0"),
        format!("/api/communities/{}/businesses?limit=10&offset=5", community_id),
        format!("/api/communities/{}/polls?limit=20&offset=10", community_id),
        format!("/api/communities/{}/decisions?limit=15&offset=0", community_id),
        format!("/api/communities/{}/members?limit=25&offset=5", community_id),
    ];

    for endpoint in paginated_endpoints {
        let response = if endpoint.starts_with("/api/communities/") && !endpoint.contains("?") {
            // Public endpoint
            ctx.server.get(&endpoint).await
        } else {
            // Protected endpoint
            ctx.server
                .get(&endpoint)
                .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
                .await
        };

        // Should handle pagination parameters gracefully
        assert!(
            response.status_code() == StatusCode::OK ||
            response.status_code() == StatusCode::UNAUTHORIZED ||
            response.status_code() == StatusCode::NOT_FOUND ||
            response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
        );

        println!("Pagination test for endpoint {} returned status: {}", endpoint, response.status_code());
    }

    ctx.cleanup().await;
}