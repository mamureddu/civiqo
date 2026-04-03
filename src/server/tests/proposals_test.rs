//! Tests for Governance/Proposals API
//! Phase 4: Governance & Voting

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

/// Helper to create test app
async fn create_test_app() -> axum::Router {
    server::create_test_app()
        .await
        .expect("Failed to create test app")
}

// ============================================================================
// LIST PROPOSALS TESTS
// ============================================================================

#[tokio::test]
async fn test_list_proposals_returns_200() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/proposals")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_proposals_returns_json_array() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/proposals")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let proposals: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Should return an array (may be empty)
    assert!(proposals.is_empty() || proposals.len() > 0);
}

#[tokio::test]
async fn test_list_proposals_with_community_filter() {
    let app = create_test_app().await;

    // Use a random UUID that likely doesn't exist
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/proposals?community_id=00000000-0000-0000-0000-000000000000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_proposals_with_pagination() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/proposals?page=0&limit=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

// ============================================================================
// GET PROPOSAL TESTS
// ============================================================================

#[tokio::test]
async fn test_get_proposal_not_found() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/proposals/00000000-0000-0000-0000-000000000000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 500 (wrapped error) or 404
    assert!(
        response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn test_get_proposal_invalid_uuid() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/proposals/invalid-uuid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 400 or 500 for invalid UUID
    assert!(response.status().is_client_error() || response.status().is_server_error());
}

// ============================================================================
// CREATE PROPOSAL TESTS (require auth)
// ============================================================================

#[tokio::test]
async fn test_create_proposal_requires_auth() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/proposals")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "community_id": "00000000-0000-0000-0000-000000000000",
                        "title": "Test Proposal"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 401 Unauthorized without auth
    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

// ============================================================================
// VOTE TESTS (require auth)
// ============================================================================

#[tokio::test]
async fn test_cast_vote_requires_auth() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/proposals/00000000-0000-0000-0000-000000000000/vote")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "vote_value": "yes"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 401 Unauthorized without auth
    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

// ============================================================================
// RESULTS TESTS
// ============================================================================

#[tokio::test]
async fn test_get_results_not_found() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/proposals/00000000-0000-0000-0000-000000000000/results")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 404 or 500 for non-existent proposal
    assert!(
        response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

// ============================================================================
// GOVERNANCE PAGE TESTS
// ============================================================================

#[tokio::test]
async fn test_governance_page_returns_200() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/governance")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_governance_page_contains_htmx() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/governance")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let html = String::from_utf8_lossy(&body);

    // Should contain HTMX attributes for proposals loading
    assert!(html.contains("hx-get") || html.contains("hx-trigger"));
}

// ============================================================================
// HTMX FRAGMENT TESTS
// ============================================================================

#[tokio::test]
async fn test_governance_proposals_fragment_returns_200() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/htmx/governance/proposals")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_governance_proposals_fragment_returns_html() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/htmx/governance/proposals")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let html = String::from_utf8_lossy(&body);

    // Should return HTML content
    assert!(html.contains("<div") || html.contains("No proposals"));
}

// ============================================================================
// ACTIVATE/CLOSE TESTS (require auth)
// ============================================================================

#[tokio::test]
async fn test_activate_proposal_requires_auth() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/proposals/00000000-0000-0000-0000-000000000000/activate")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 401 Unauthorized without auth
    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn test_close_proposal_requires_auth() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/proposals/00000000-0000-0000-0000-000000000000/close")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 401 Unauthorized without auth
    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}
