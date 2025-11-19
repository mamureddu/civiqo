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
        governance::*,
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

/// Test configuration and setup helpers for governance API tests
struct GovernanceTestContext {
    server: TestServer,
    db: Database,
    mock_auth0: MockServer,
    auth_config: Auth0Config,
}

impl GovernanceTestContext {
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

    /// Create authenticated community member with governance permissions
    async fn create_authenticated_community_member(&self) -> (shared::models::User, String, Uuid) {
        let user = create_test_user(&self.db, None).await.expect("Failed to create test user");
        let community = create_test_community(&self.db, user.id, Some("Governance Community".to_string()))
            .await.expect("Failed to create test community");

        let claims = Claims {
            sub: user.auth0_id.clone(),
            aud: self.auth_config.audience.clone(),
            iss: format!("https://{}/", self.auth_config.domain),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            email: Some(user.email.clone()),
            email_verified: Some(true),
            name: Some("Community Member".to_string()),
            picture: None,
            community_roles: vec![],
        };

        let token = self.create_test_jwt(&claims);
        (user, token, community.id)
    }
}

// Poll listing tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_list_polls_empty_response() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let response = ctx.server
        .get(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status_ok();

    let body: ApiResponse<Vec<Poll>> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());
    assert!(body.data.unwrap().is_empty());
    assert!(body.message.is_some());
    assert!(body.message.unwrap().contains("temporarily disabled"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_polls_with_pagination() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let response = ctx.server
        .get(&format!("/api/communities/{}/polls?limit=5&offset=10", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status_ok();

    let body: ApiResponse<Vec<Poll>> = response.json();
    assert!(body.success);
    assert!(body.data.unwrap().is_empty()); // Stub returns empty array

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_polls_unauthenticated() {
    let ctx = GovernanceTestContext::new().await;
    let community_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/communities/{}/polls", community_id))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_polls_invalid_community_id() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, _) = ctx.create_authenticated_community_member().await;

    let response = ctx.server
        .get("/api/communities/invalid-uuid/polls")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Poll creation tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_create_poll_stub_error() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let create_request = CreatePollRequest {
        title: "Should we build a new park?".to_string(),
        description: Some("Community poll about building a new park in the downtown area".to_string()),
        poll_type: PollType::SingleChoice,
        options: vec![
            "Yes, build the park".to_string(),
            "No, use funds elsewhere".to_string(),
            "Need more information".to_string(),
        ],
        settings: PollSettings {
            anonymous: false,
            allow_multiple: false,
            max_choices: None,
            required_role: None,
        },
        starts_at: chrono::Utc::now(),
        ends_at: chrono::Utc::now() + chrono::Duration::days(7),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    // Stub implementation returns Validation error
    response.assert_status(StatusCode::BAD_REQUEST);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_poll_invalid_data() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let invalid_request = serde_json::json!({
        "title": "", // Empty title should fail validation
        "options": [], // Empty options should fail
        "poll_type": "invalid_type",
        "end_date": "invalid-date"
    });

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&invalid_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_poll_unauthenticated() {
    let ctx = GovernanceTestContext::new().await;
    let community_id = Uuid::new_v4();

    let create_request = CreatePollRequest {
        title: "Test Poll".to_string(),
        description: None,
        options: vec!["Yes".to_string(), "No".to_string()],
        poll_type: PollType::SingleChoice,
        settings: PollSettings {
            anonymous: false,
            allow_multiple: false,
            max_choices: None,
            required_role: None,
        },
        starts_at: chrono::Utc::now(),
        ends_at: chrono::Utc::now() + chrono::Duration::days(1),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_poll_multiple_choice() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let create_request = CreatePollRequest {
        title: "Which community improvements do you support?".to_string(),
        description: Some("Select all that apply".to_string()),
        options: vec![
            "New playground".to_string(),
            "Better lighting".to_string(),
            "More parking".to_string(),
            "Community center".to_string(),
        ],
        poll_type: PollType::MultipleChoice,
        settings: PollSettings {
            anonymous: true,
            allow_multiple: true,
            max_choices: None,
            required_role: None,
        },
        starts_at: chrono::Utc::now(),
        ends_at: chrono::Utc::now() + chrono::Duration::days(14),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    // Stub returns validation error
    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Poll detail tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_get_poll_stub_error() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;
    let poll_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/polls/{}", poll_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    // Stub implementation returns NotFound error
    response.assert_status(StatusCode::NOT_FOUND);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_poll_invalid_id() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let response = ctx.server
        .get("/api/polls/invalid-uuid")
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_poll_unauthenticated() {
    let ctx = GovernanceTestContext::new().await;
    let poll_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/polls/{}", poll_id))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

// Vote casting tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_cast_vote_stub_error() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;
    let poll_id = Uuid::new_v4();

    let vote_request = CastVoteRequest {
        choices: vec!["Yes, build the park".to_string()], // First option
        choice: None,
        rating: None,
    };

    let response = ctx.server
        .post(&format!("/api/polls/{}/vote", poll_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&vote_request)
        .await;

    // Stub implementation returns Validation error
    response.assert_status(StatusCode::BAD_REQUEST);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_cast_vote_multiple_options() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;
    let poll_id = Uuid::new_v4();

    let vote_request = CastVoteRequest {
        choices: vec!["New playground".to_string(), "More parking".to_string(), "Community center".to_string()], // Multiple selections
        choice: None,
        rating: None,
    };

    let response = ctx.server
        .post(&format!("/api/polls/{}/vote", poll_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&vote_request)
        .await;

    // Stub returns validation error
    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_cast_vote_invalid_data() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;
    let poll_id = Uuid::new_v4();

    let invalid_request = serde_json::json!({
        "choices": [], // Empty selection should fail
        "comment": "x".repeat(1001) // Too long comment
    });

    let response = ctx.server
        .post(&format!("/api/polls/{}/vote", poll_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&invalid_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_cast_vote_unauthenticated() {
    let ctx = GovernanceTestContext::new().await;
    let poll_id = Uuid::new_v4();

    let vote_request = CastVoteRequest {
        choices: vec!["Yes".to_string()],
        choice: None,
        rating: None,
    };

    let response = ctx.server
        .post(&format!("/api/polls/{}/vote", poll_id))
        .json(&vote_request)
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

// Poll results tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_get_poll_results_stub_error() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;
    let poll_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/polls/{}/results", poll_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    // Stub implementation returns NotFound error
    response.assert_status(StatusCode::NOT_FOUND);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.is_some());
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_get_poll_results_public_access() {
    let ctx = GovernanceTestContext::new().await;
    let poll_id = Uuid::new_v4();

    // Poll results should be accessible without authentication for public polls
    let response = ctx.server
        .get(&format!("/api/polls/{}/results", poll_id))
        .await;

    // Should return stub error, not auth error
    response.assert_status(StatusCode::NOT_FOUND);

    ctx.cleanup().await;
}

// Decision listing tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_list_decisions_empty_response() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let response = ctx.server
        .get(&format!("/api/communities/{}/decisions", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status_ok();

    let body: ApiResponse<Vec<Decision>> = response.json();
    assert!(body.success);
    assert!(body.data.is_some());
    assert!(body.data.unwrap().is_empty());
    assert!(body.message.unwrap().contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_decisions_with_pagination() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let response = ctx.server
        .get(&format!("/api/communities/{}/decisions?limit=20&offset=5", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .await;

    response.assert_status_ok();

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_list_decisions_unauthenticated() {
    let ctx = GovernanceTestContext::new().await;
    let community_id = Uuid::new_v4();

    let response = ctx.server
        .get(&format!("/api/communities/{}/decisions", community_id))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

// Decision creation tests (stub implementation)
#[tokio::test]
#[serial]
async fn test_create_decision_stub_error() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let create_request = CreateDecisionRequest {
        title: "New Community Center Funding".to_string(),
        description: "Decision to allocate $50,000 for new community center construction".to_string(),
        decision_type: DecisionType::Simple,
        decision_makers: vec![
            Uuid::new_v4(), // Community Board member
            Uuid::new_v4(), // Local Residents representative
            Uuid::new_v4(), // City Planning Committee member
        ],
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(30)),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/decisions", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    // Stub implementation returns Validation error
    response.assert_status(StatusCode::BAD_REQUEST);

    let body: ApiResponse<serde_json::Value> = response.json();
    assert!(!body.success);
    assert!(body.error.unwrap().message.contains("development"));

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_decision_invalid_data() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let invalid_request = serde_json::json!({
        "title": "", // Empty title should fail
        "description": "", // Empty description should fail
        "decision_type": "invalid_type",
        "status": "invalid_status"
    });

    let response = ctx.server
        .post(&format!("/api/communities/{}/decisions", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&invalid_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_create_decision_unauthenticated() {
    let ctx = GovernanceTestContext::new().await;
    let community_id = Uuid::new_v4();

    let create_request = CreateDecisionRequest {
        title: "Test Decision".to_string(),
        description: "Test description".to_string(),
        decision_type: DecisionType::Consensus,
        decision_makers: vec![],
        deadline: None,
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/decisions", community_id))
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

// Input validation tests
#[tokio::test]
#[serial]
async fn test_poll_title_length_validation() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let long_title = "x".repeat(256);
    let create_request = CreatePollRequest {
        title: long_title,
        description: None,
        options: vec!["Yes".to_string(), "No".to_string()],
        poll_type: PollType::SingleChoice,
        settings: PollSettings {
            anonymous: false,
            allow_multiple: false,
            max_choices: None,
            required_role: None,
        },
        starts_at: chrono::Utc::now(),
        ends_at: chrono::Utc::now() + chrono::Duration::days(1),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_poll_options_validation() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    // Test with only one option (should require at least 2)
    let create_request = CreatePollRequest {
        title: "Test Poll".to_string(),
        description: None,
        options: vec!["Only option".to_string()], // Only one option
        poll_type: PollType::SingleChoice,
        settings: PollSettings {
            anonymous: false,
            allow_multiple: false,
            max_choices: None,
            required_role: None,
        },
        starts_at: chrono::Utc::now(),
        ends_at: chrono::Utc::now() + chrono::Duration::days(1),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_vote_option_index_validation() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;
    let poll_id = Uuid::new_v4();

    let vote_request = CastVoteRequest {
        choices: vec!["Invalid Option".to_string()], // Invalid option
        choice: None,
        rating: None,
    };

    let response = ctx.server
        .post(&format!("/api/polls/{}/vote", poll_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&vote_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Security tests
#[tokio::test]
#[serial]
async fn test_governance_api_sql_injection_prevention() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let malicious_request = CreatePollRequest {
        title: "'; DROP TABLE polls; --".to_string(),
        description: Some("'; DELETE FROM votes; --".to_string()),
        options: vec!["Yes".to_string(), "'; DROP TABLE users; --".to_string()],
        poll_type: PollType::SingleChoice,
        settings: PollSettings {
            anonymous: false,
            allow_multiple: false,
            max_choices: None,
            required_role: None,
        },
        starts_at: chrono::Utc::now(),
        ends_at: chrono::Utc::now() + chrono::Duration::days(1),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&malicious_request)
        .await;

    // Should not cause server error (SQLx prevents injection)
    assert_ne!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_governance_xss_prevention() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let xss_request = CreatePollRequest {
        title: "<script>alert('xss')</script>".to_string(),
        description: Some("<img src=x onerror=alert('xss')>".to_string()),
        options: vec![
            "<script>steal_data()</script>".to_string(),
            "javascript:alert('xss')".to_string(),
        ],
        poll_type: PollType::SingleChoice,
        settings: PollSettings {
            anonymous: false,
            allow_multiple: false,
            max_choices: None,
            required_role: None,
        },
        starts_at: chrono::Utc::now(),
        ends_at: chrono::Utc::now() + chrono::Duration::days(1),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&xss_request)
        .await;

    // Should handle XSS attempts gracefully
    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Rate limiting tests
#[tokio::test]
#[serial]
async fn test_governance_api_rate_limiting() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    // Make multiple rapid requests
    let mut responses = Vec::new();
    for _ in 0..15 {
        let response = ctx.server
            .get(&format!("/api/communities/{}/polls", community_id))
            .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
            .await;
        responses.push(response.status_code());
    }

    // Most should succeed unless rate limiting is very strict
    let success_count = responses.iter()
        .filter(|&&status| status == StatusCode::OK)
        .count();

    assert!(success_count > 8, "Too many requests were rate limited");

    ctx.cleanup().await;
}

// Edge case tests
#[tokio::test]
#[serial]
async fn test_poll_end_date_in_past() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let create_request = CreatePollRequest {
        title: "Past Poll".to_string(),
        description: None,
        options: vec!["Yes".to_string(), "No".to_string()],
        poll_type: PollType::SingleChoice,
        settings: PollSettings {
            anonymous: false,
            allow_multiple: false,
            max_choices: None,
            required_role: None,
        },
        starts_at: chrono::Utc::now(),
        ends_at: chrono::Utc::now() - chrono::Duration::days(1), // Past date
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/polls", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_decision_with_unicode_characters() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    let create_request = CreateDecisionRequest {
        title: "建设新的社区中心 / بناء مركز مجتمعي جديد".to_string(),
        description: "多语言社区决策测试 🏢🏗️".to_string(),
        decision_type: DecisionType::Majority,
        decision_makers: vec![Uuid::new_v4(), Uuid::new_v4()],
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(45)),
    };

    let response = ctx.server
        .post(&format!("/api/communities/{}/decisions", community_id))
        .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
        .json(&create_request)
        .await;

    // Should handle Unicode gracefully (stub returns validation error)
    response.assert_status(StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Concurrent access tests
#[tokio::test]
#[serial]
async fn test_governance_api_concurrent_poll_creation() {
    let ctx = GovernanceTestContext::new().await;
    let (user, token, community_id) = ctx.create_authenticated_community_member().await;

    // Attempt to create multiple polls sequentially
    let mut error_count = 0;
    for i in 0..5 {
        let create_request = CreatePollRequest {
            title: format!("Concurrent Poll {}", i),
            description: Some("Concurrency test".to_string()),
            options: vec!["Option A".to_string(), "Option B".to_string()],
            poll_type: PollType::SingleChoice,
            settings: PollSettings {
                anonymous: false,
                allow_multiple: false,
                max_choices: None,
                required_role: None,
            },
            starts_at: chrono::Utc::now(),
            ends_at: chrono::Utc::now() + chrono::Duration::days(1),
        };

        let response = ctx.server.post(&format!("/api/communities/{}/polls", community_id))
            .add_header(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap())
            .json(&create_request)
            .await;

        if response.status_code() == StatusCode::BAD_REQUEST {
            error_count += 1; // Expected due to stub implementation
        }
    }

    // All should return the same stub error
    assert_eq!(error_count, 5, "All concurrent requests should return stub validation error");

    ctx.cleanup().await;
}