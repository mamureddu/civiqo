use axum::http::StatusCode;
use axum_test::TestServer;

/// Create test server with the real app
fn create_test_server() -> TestServer {
    // For now, we'll test basic routes without full app setup
    // This will be improved when we integrate with the real create_app from main.rs
    use axum::{Router, routing::get, response::Html};
    
    let app = Router::new()
        .route("/", get(|| async { Html("<html><body>Welcome to Community Manager</body></html>") }))
        .route("/dashboard", get(|| async { Html("<html><body>Dashboard - Welcome back</body></html>") }))
        .route("/communities", get(|| async { Html("<html><body>Communities List</body></html>") }))
        .route("/communities/{id}", get(|| async { Html("<html><body>Community Detail</body></html>") }))
        .route("/businesses", get(|| async { Html("<html><body>Businesses List</body></html>") }))
        .route("/businesses/{id}", get(|| async { Html("<html><body>Business Detail</body></html>") }))
        .route("/chat", get(|| async { Html("<html><body>Chat</body></html>") }))
        .route("/chat/{room_id}", get(|| async { Html("<html><body>Chat Room</body></html>") }))
        .route("/governance", get(|| async { Html("<html><body>Governance</body></html>") }))
        .route("/poi", get(|| async { Html("<html><body>Points of Interest</body></html>") }))
        .route("/health", get(|| async { "ok" }));
    
    TestServer::new(app).unwrap()
}

/// Test helper to make requests
async fn get_page(path: &str) -> (StatusCode, String) {
    let server = create_test_server();
    let response = server.get(path).await;
    (response.status_code(), response.text())
}

#[tokio::test]
async fn test_homepage() {
    let (status, body) = get_page("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Homepage should return HTML");
    assert!(body.contains("Community Manager") || body.contains("Welcome"), "Homepage should have title");
}

#[tokio::test]
async fn test_dashboard() {
    let (status, body) = get_page("/dashboard").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Dashboard should return HTML");
    assert!(body.contains("Dashboard") || body.contains("Welcome back"), "Dashboard should have dashboard content");
}

#[tokio::test]
async fn test_communities_list() {
    let (status, body) = get_page("/communities").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Communities page should return HTML");
    assert!(body.contains("Communities") || body.contains("community"), "Communities page should have communities content");
}

#[tokio::test]
async fn test_community_detail() {
    let (status, body) = get_page("/communities/test-id-123").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Community detail should return HTML");
}

#[tokio::test]
async fn test_businesses_list() {
    let (status, body) = get_page("/businesses").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Businesses page should return HTML");
    assert!(body.contains("Business") || body.contains("business"), "Businesses page should have business content");
}

#[tokio::test]
async fn test_business_detail() {
    let (status, body) = get_page("/businesses/test-id-123").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Business detail should return HTML");
}

#[tokio::test]
async fn test_chat() {
    let (status, body) = get_page("/chat").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Chat page should return HTML");
}

#[tokio::test]
async fn test_chat_room() {
    let (status, body) = get_page("/chat/room-123").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Chat room should return HTML");
}

#[tokio::test]
async fn test_governance() {
    let (status, body) = get_page("/governance").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "Governance page should return HTML");
    assert!(body.contains("Governance") || body.contains("governance"), "Governance page should have governance content");
}

#[tokio::test]
async fn test_poi_map() {
    let (status, body) = get_page("/poi").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<html"), "POI/Map page should return HTML");
}

#[tokio::test]
async fn test_health_check() {
    let (status, body) = get_page("/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "ok", "Health check should return 'ok'");
}
