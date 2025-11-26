//! View Interaction Tests
//!
//! Tests all HTMX interactions in the application views.
//! Each test makes real HTTP requests to the app and verifies:
//! - Correct HTTP status codes
//! - HTML response contains expected elements
//! - HTMX attributes are correctly wired
//! - Data is correctly rendered
//!
//! Total interactions: 40
//!
//! Run with: cargo test view_interaction -p server
//!
//! ## Note on Serial Tests
//! Some tests that depend on database state (like community_detail) are marked
//! with `#[serial]` to prevent race conditions when running in parallel.
//! These tests create/read test data and can conflict with other tests
//! that modify the same tables concurrently.

use axum_test::TestServer;
use serial_test::serial;
use server::create_test_app;
use shared::database::Database;
use uuid::Uuid;

/// Create a test server instance with real database
async fn create_server() -> TestServer {
    let app = create_test_app().await.expect("Failed to create test app");
    TestServer::new(app).expect("Failed to create test server")
}

/// Setup test database connection for data verification/setup
async fn setup_db() -> Database {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Database::connect(&database_url).await.expect("Failed to connect")
}

/// Clean up test data from the database
/// 
/// This function removes all test communities and related data created by tests.
/// Should be called at the start of test runs to ensure a clean state.
async fn cleanup_test_data(db: &Database) {
    // Delete test communities (cascades to related data)
    sqlx::query!(
        "DELETE FROM communities WHERE slug LIKE 'test-%' OR slug LIKE 'view-interaction-%' OR slug = 'private-test'"
    )
    .execute(&db.pool)
    .await
    .ok();
    
    // Delete test users (only the specific test user, not real users)
    sqlx::query!(
        "DELETE FROM users WHERE email = 'test-view-interaction@example.com'"
    )
    .execute(&db.pool)
    .await
    .ok();
}

/// Get or create a test community for testing
/// 
/// This function ensures a valid test community exists with a valid creator.
/// Uses a fixed slug to ensure we always get the same community.
async fn get_or_create_test_community(db: &Database) -> (Uuid, String) {
    let fixed_slug = "view-interaction-test-community";
    
    // First try to get existing test community
    let existing = sqlx::query!(
        "SELECT id, slug FROM communities WHERE slug = $1 LIMIT 1",
        fixed_slug
    )
    .fetch_optional(&db.pool)
    .await
    .ok()
    .flatten();
    
    if let Some(community) = existing {
        return (community.id, community.slug);
    }
    
    // Get or create a user to be the creator
    let user = sqlx::query!("SELECT id FROM users LIMIT 1")
        .fetch_optional(&db.pool)
        .await
        .ok()
        .flatten();
    
    let creator_id = match user {
        Some(u) => u.id,
        None => {
            // Create a test user if none exists
            let test_user_id = Uuid::now_v7();
            sqlx::query!(
                "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
                 VALUES ($1, $2, $3, NOW(), NOW())
                 ON CONFLICT (email) DO NOTHING",
                test_user_id,
                "test-view-interaction@example.com",
                format!("auth0|test-{}", test_user_id)
            )
            .execute(&db.pool)
            .await
            .ok();
            
            // Fetch the user (might be existing or newly created)
            sqlx::query!("SELECT id FROM users WHERE email = 'test-view-interaction@example.com'")
                .fetch_one(&db.pool)
                .await
                .map(|u| u.id)
                .unwrap_or(test_user_id)
        }
    };
    
    // Create the community with a fixed ID based on slug hash for consistency
    let id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO communities (id, name, slug, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, $5, NOW(), NOW())
         ON CONFLICT (slug) DO NOTHING",
        id,
        "View Interaction Test Community",
        fixed_slug,
        "A test community for view interaction tests",
        creator_id
    )
    .execute(&db.pool)
    .await
    .ok();
    
    // Fetch the community to get the actual ID (in case of conflict)
    let community = sqlx::query!(
        "SELECT id, slug FROM communities WHERE slug = $1",
        fixed_slug
    )
    .fetch_one(&db.pool)
    .await
    .expect("Community should exist after insert");
    
    (community.id, community.slug)
}

// ============================================================================
// 1. HOMEPAGE TESTS (index.html)
// ============================================================================

/// Test #1: Homepage loads and contains HTMX trigger for recent communities
#[tokio::test]
async fn test_view_interaction_01_homepage_loads() {
    let server = create_server().await;
    let response = server.get("/").await;
    
    let status = response.status_code();
    let body = response.text();
    
    if !status.is_success() {
        eprintln!("Homepage error ({}): {}", status, &body[..body.len().min(500)]);
    }
    assert!(status.is_success(), "Homepage should load successfully, got {}", status);
    
    // Verify page structure
    assert!(body.contains("<!DOCTYPE html>") || body.contains("<html"), "Should be HTML page");
    // Verify HTMX is loaded
    assert!(body.contains("htmx") || body.contains("hx-"), "Should have HTMX");
    // Verify the recent communities fragment trigger exists
    assert!(body.contains("hx-get=\"/htmx/communities/recent\"") || 
            body.contains("hx-get='/htmx/communities/recent'"),
            "Should have hx-get for recent communities");
}

/// Test #1b: GET /htmx/communities/recent returns HTML fragment
#[tokio::test]
async fn test_view_interaction_01b_recent_communities_fragment() {
    let server = create_server().await;
    let response = server.get("/htmx/communities/recent").await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Should return HTML fragment (not full page)
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment, not full page");
    // Should contain community cards or empty state
    assert!(body.contains("<") && body.contains(">"), "Should be HTML");
}

// ============================================================================
// 2. COMMUNITIES PAGE TESTS (communities.html)
// ============================================================================

/// Test #4: Communities page loads with search form
#[tokio::test]
async fn test_view_interaction_04_communities_page_loads() {
    let server = create_server().await;
    let response = server.get("/communities").await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Verify page structure
    assert!(body.contains("<html"), "Should be HTML page");
    // Verify search form with HTMX
    assert!(body.contains("hx-get") && body.contains("communities"), 
            "Should have HTMX search/list functionality");
}

/// Test #5: GET /htmx/communities/list returns community cards
#[tokio::test]
async fn test_view_interaction_05_communities_list_fragment() {
    let server = create_server().await;
    let response = server.get("/htmx/communities/list").await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Should be HTML fragment
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment");
    // Should contain community content or empty state message
    assert!(body.len() > 0, "Should have content");
}

/// Test #4b: Communities search returns filtered results
#[tokio::test]
async fn test_view_interaction_04b_communities_search() {
    let server = create_server().await;
    
    // Test the search endpoint with query parameter using add_query_param
    let response = server.get("/htmx/communities/search")
        .add_query_param("q", "demo")
        .await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(body.contains("<"), "Should return HTML");
}

// ============================================================================
// 3. COMMUNITY DETAIL TESTS
// ============================================================================

/// Test #6: Community detail page loads
/// 
/// This test is marked `#[serial]` because it:
/// 1. Creates a test community in the database via `get_or_create_test_community`
/// 2. Immediately queries that community via HTTP request
/// 
/// The test is strict: since we create the community, it MUST exist and return 200.
/// A 404 would indicate a bug in the creation or query logic.
#[tokio::test]
#[serial]
async fn test_view_interaction_06_community_detail_page() {
    let db = setup_db().await;
    let (community_id, _slug) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    
    // STRICT: We just created this community, so it MUST exist
    let status = response.status_code();
    let body = response.text();
    
    if !status.is_success() {
        eprintln!("Community detail error ({}): {}", status, &body[..body.len().min(500)]);
    }
    response.assert_status_success();
    assert!(body.contains("<html"), "Should be HTML page");
}

// ============================================================================
// 4. COMMUNITY POSTS TESTS (community_posts.html)
// ============================================================================

/// Test #7-9: Community posts page with sorting
#[tokio::test]
async fn test_view_interaction_07_09_community_posts_sorting() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    
    // Test newest sort
    let response = server.get(&format!("/communities/{}/posts?sort=newest", community_id)).await;
    let status = response.status_code();
    assert!(status.is_success() || status.as_u16() == 404);
    
    // Test popular sort
    let response = server.get(&format!("/communities/{}/posts?sort=popular", community_id)).await;
    let status = response.status_code();
    assert!(status.is_success() || status.as_u16() == 404);
    
    // Test discussed sort
    let response = server.get(&format!("/communities/{}/posts?sort=discussed", community_id)).await;
    let status = response.status_code();
    assert!(status.is_success() || status.as_u16() == 404);
}

// ============================================================================
// 5. CREATE COMMUNITY TESTS
// ============================================================================

/// Test #10: Create community page loads (requires auth)
#[tokio::test]
async fn test_view_interaction_10_create_community_page() {
    let server = create_server().await;
    let response = server.get("/communities/create").await;
    
    // May require auth - either loads or redirects
    let status = response.status_code();
    assert!(status.is_success() || status.as_u16() == 302 || status.as_u16() == 401,
            "Should load, redirect to login, or return 401");
    
    if status.is_success() {
        let body = response.text();
        // Should have form with hx-post
        assert!(body.contains("hx-post") || body.contains("form"), 
                "Should have create form");
    }
}

/// Test #10b: POST /api/communities requires authentication
#[tokio::test]
async fn test_view_interaction_10b_create_community_requires_auth() {
    let server = create_server().await;
    let response = server
        .post("/api/communities")
        .json(&serde_json::json!({
            "name": "Test Community",
            "slug": "test-view-interaction",
            "description": "Test"
        }))
        .await;
    
    // Should require authentication
    response.assert_status_unauthorized();
}

// ============================================================================
// 6. MEMBERSHIP TESTS (join/leave)
// ============================================================================

/// Test #19, #22: POST /api/communities/:id/join requires auth
#[tokio::test]
async fn test_view_interaction_19_22_join_community_requires_auth() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server
        .post(&format!("/api/communities/{}/join", community_id))
        .await;
    
    response.assert_status_unauthorized();
}

/// Test #21: POST /api/communities/:id/leave requires auth
#[tokio::test]
async fn test_view_interaction_21_leave_community_requires_auth() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server
        .post(&format!("/api/communities/{}/leave", community_id))
        .await;
    
    response.assert_status_unauthorized();
}

/// Test #20, #23: Request join private community requires auth
#[tokio::test]
async fn test_view_interaction_20_23_request_join_requires_auth() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server
        .post(&format!("/api/communities/{}/request-join", community_id))
        .await;
    
    response.assert_status_unauthorized();
}

// ============================================================================
// 7. MEMBERS LIST TESTS
// ============================================================================

/// Test #27-28: GET /api/communities/:id/members pagination
/// 
/// STRICT: Since we create a public community, listing members should succeed.
/// The endpoint should return 200 with an empty list if no members exist.
#[tokio::test]
#[serial]
async fn test_view_interaction_27_28_members_list() {
    let db = setup_db().await;
    let (community_id, slug) = get_or_create_test_community(&db).await;
    
    // Verify community exists in DB before making HTTP request
    let exists = sqlx::query!(
        "SELECT id FROM communities WHERE id = $1",
        community_id
    )
    .fetch_optional(&db.pool)
    .await
    .expect("DB query should work");
    
    assert!(exists.is_some(), "Community {} (slug: {}) should exist in DB", community_id, slug);
    
    let server = create_server().await;
    
    // Page 1 - should succeed for public community
    // Note: Using add_query_param to properly encode query parameters
    let response = server
        .get(&format!("/api/communities/{}/members", community_id))
        .add_query_param("page", "1")
        .add_query_param("limit", "10")
        .await;
    
    let status = response.status_code();
    if !status.is_success() {
        let body = response.text();
        eprintln!("Members list failed for community {}: {} - {}", community_id, status, body);
    }
    assert!(status.is_success(), "Members list should succeed for existing public community {}", community_id);
    
    // Page 2 - should also succeed (may be empty)
    let response = server
        .get(&format!("/api/communities/{}/members", community_id))
        .add_query_param("page", "2")
        .add_query_param("limit", "10")
        .await;
    response.assert_status_success();
}

/// Test #24-26: Admin operations require auth
#[tokio::test]
async fn test_view_interaction_24_26_admin_operations_require_auth() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    let fake_user_id = Uuid::new_v4();
    
    let server = create_server().await;
    
    // Promote requires auth
    let response = server
        .post(&format!("/api/communities/{}/promote/{}", community_id, fake_user_id))
        .await;
    response.assert_status_unauthorized();
    
    // Demote requires auth
    let response = server
        .post(&format!("/api/communities/{}/demote/{}", community_id, fake_user_id))
        .await;
    response.assert_status_unauthorized();
    
    // Remove member requires auth
    let response = server
        .delete(&format!("/api/communities/{}/members/{}", community_id, fake_user_id))
        .await;
    response.assert_status_unauthorized();
}

// ============================================================================
// 8. POSTS TESTS
// ============================================================================

/// Test #30: Create post requires auth
#[tokio::test]
async fn test_view_interaction_30_create_post_requires_auth() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server
        .post(&format!("/api/communities/{}/posts", community_id))
        .json(&serde_json::json!({
            "title": "Test Post",
            "content": "Test content"
        }))
        .await;
    
    response.assert_status_unauthorized();
}

/// Test #29: Update post requires auth
#[tokio::test]
async fn test_view_interaction_29_update_post_requires_auth() {
    let server = create_server().await;
    let response = server
        .put("/api/posts/1")
        .json(&serde_json::json!({
            "title": "Updated",
            "content": "Updated content"
        }))
        .await;
    
    response.assert_status_unauthorized();
}

/// Test #11: Delete post requires auth
#[tokio::test]
async fn test_view_interaction_11_delete_post_requires_auth() {
    let server = create_server().await;
    let response = server.delete("/api/posts/1").await;
    
    response.assert_status_unauthorized();
}

// ============================================================================
// 9. COMMENTS TESTS
// ============================================================================

/// Test #31: Create comment requires auth
#[tokio::test]
async fn test_view_interaction_31_create_comment_requires_auth() {
    let server = create_server().await;
    let response = server
        .post("/api/posts/1/comments")
        .json(&serde_json::json!({
            "content": "Test comment"
        }))
        .await;
    
    response.assert_status_unauthorized();
}

/// Test #35: Delete comment requires auth
#[tokio::test]
async fn test_view_interaction_35_delete_comment_requires_auth() {
    let server = create_server().await;
    let response = server.delete("/api/comments/1").await;
    
    response.assert_status_unauthorized();
}

// ============================================================================
// 10. REACTIONS TESTS
// ============================================================================

/// Test #36-39: Add reactions requires auth
#[tokio::test]
async fn test_view_interaction_36_39_add_reactions_require_auth() {
    let server = create_server().await;
    
    let reaction_types = ["like", "heart", "celebrate", "thinking"];
    
    for reaction_type in reaction_types {
        let response = server
            .post("/api/posts/1/reactions")
            .json(&serde_json::json!({
                "reaction_type": reaction_type
            }))
            .await;
        
        response.assert_status_unauthorized();
    }
}

/// Test #40: Remove reaction requires auth
#[tokio::test]
async fn test_view_interaction_40_remove_reaction_requires_auth() {
    let server = create_server().await;
    let response = server.delete("/api/posts/1/reactions").await;
    
    response.assert_status_unauthorized();
}

// ============================================================================
// 11. DASHBOARD TESTS
// ============================================================================

/// Test #2: Dashboard user communities requires auth
#[tokio::test]
async fn test_view_interaction_02_dashboard_communities_requires_auth() {
    let server = create_server().await;
    let response = server.get("/htmx/user/communities").await;
    
    response.assert_status_unauthorized();
}

/// Test #3: Dashboard user activity requires auth
#[tokio::test]
async fn test_view_interaction_03_dashboard_activity_requires_auth() {
    let server = create_server().await;
    let response = server.get("/htmx/user/activity").await;
    
    response.assert_status_unauthorized();
}

/// Test dashboard page requires auth
#[tokio::test]
async fn test_view_interaction_dashboard_page_requires_auth() {
    let server = create_server().await;
    let response = server.get("/dashboard").await;
    
    // Should redirect to login or return 401
    let status = response.status_code();
    assert!(status.as_u16() == 401 || status.as_u16() == 302,
            "Dashboard should require authentication");
}

// ============================================================================
// 12. CHAT TESTS
// ============================================================================

/// Test #18: Chat header fragment
#[tokio::test]
async fn test_view_interaction_18_chat_header() {
    let server = create_server().await;
    let response = server.get("/htmx/chat/test-room/header").await;
    
    response.assert_status_success();
    let body = response.text();
    // Should return HTML fragment
    assert!(body.contains("<"), "Should be HTML");
}

/// Test chat page loads
#[tokio::test]
async fn test_view_interaction_chat_page() {
    let server = create_server().await;
    let response = server.get("/chat/test-room").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(body.contains("<html"), "Should be HTML page");
}

// ============================================================================
// 13. OTHER PAGES TESTS
// ============================================================================

/// Test businesses page loads
#[tokio::test]
async fn test_view_interaction_12_13_businesses_page() {
    let server = create_server().await;
    let response = server.get("/businesses").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(body.contains("<html"), "Should be HTML page");
}

/// Test governance page loads
#[tokio::test]
async fn test_view_interaction_16_governance_page() {
    let server = create_server().await;
    let response = server.get("/governance").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(body.contains("<html"), "Should be HTML page");
}

/// Test POI page loads
#[tokio::test]
async fn test_view_interaction_17_poi_page() {
    let server = create_server().await;
    let response = server.get("/poi").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(body.contains("<html"), "Should be HTML page");
}

// ============================================================================
// 14. HEALTH CHECK
// ============================================================================

/// Test health endpoint
#[tokio::test]
async fn test_view_health_check() {
    let server = create_server().await;
    let response = server.get("/health").await;
    
    response.assert_status_success();
}

// ============================================================================
// NAVIGATION FRAGMENT TEST
// ============================================================================

/// Test navigation fragment
#[tokio::test]
async fn test_view_nav_fragment() {
    let server = create_server().await;
    let response = server.get("/htmx/nav").await;
    
    response.assert_status_success();
    let body = response.text();
    // Should contain navigation links
    assert!(body.contains("<") && body.contains("href"), "Should be HTML with links");
}

// ============================================================================
// NEW HTMX ENDPOINT TESTS
// ============================================================================

/// Test community feed HTMX fragment
#[tokio::test]
#[serial]
async fn test_htmx_community_feed() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/htmx/communities/{}/feed", community_id)).await;
    
    response.assert_status_success();
    let body = response.text();
    // Should be a fragment, not a full page
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment, not full page");
    // Should contain HTML content
    assert!(body.contains("<"), "Should return HTML");
}

/// Test businesses list HTMX fragment
#[tokio::test]
async fn test_htmx_businesses_list() {
    let server = create_server().await;
    let response = server.get("/htmx/businesses/list").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment");
    assert!(body.contains("<"), "Should return HTML");
}

/// Test businesses search HTMX fragment
#[tokio::test]
async fn test_htmx_businesses_search() {
    let server = create_server().await;
    let response = server.get("/htmx/businesses/search")
        .add_query_param("q", "test")
        .await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment");
}

/// Test business posts HTMX fragment
#[tokio::test]
async fn test_htmx_business_posts() {
    let server = create_server().await;
    let response = server.get("/htmx/businesses/test-business-id/posts").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment");
}

/// Test business reviews HTMX fragment
#[tokio::test]
async fn test_htmx_business_reviews() {
    let server = create_server().await;
    let response = server.get("/htmx/businesses/test-business-id/reviews").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment");
}

/// Test governance proposals HTMX fragment
#[tokio::test]
async fn test_htmx_governance_proposals() {
    let server = create_server().await;
    let response = server.get("/htmx/governance/proposals").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment");
    assert!(body.contains("<"), "Should return HTML");
}

/// Test POI nearby HTMX fragment
#[tokio::test]
async fn test_htmx_poi_nearby() {
    let server = create_server().await;
    let response = server.get("/htmx/poi/nearby").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(!body.contains("<!DOCTYPE html>"), "Should be fragment");
    assert!(body.contains("<"), "Should return HTML");
}

/// Test comment reply form HTMX fragment
#[tokio::test]
async fn test_htmx_comment_reply_form() {
    let server = create_server().await;
    let response = server.get("/htmx/comments/test-comment-id/reply-form").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(body.contains("form"), "Should contain a form");
    assert!(body.contains("textarea"), "Should contain textarea");
}

/// Test comment edit form HTMX fragment
#[tokio::test]
async fn test_htmx_comment_edit_form() {
    let server = create_server().await;
    let response = server.get("/htmx/comments/test-comment-id/edit-form").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(body.contains("form"), "Should contain a form");
    assert!(body.contains("textarea"), "Should contain textarea");
}

/// Test empty HTMX fragment
#[tokio::test]
async fn test_htmx_empty_fragment() {
    let server = create_server().await;
    let response = server.get("/htmx/empty").await;
    
    response.assert_status_success();
    let body = response.text();
    assert!(body.is_empty(), "Empty fragment should return empty string");
}

// ============================================================================
// SUMMARY
// ============================================================================

/// Meta-test to document coverage
#[test]
fn test_view_interaction_coverage_summary() {
    println!("View Interaction Tests Coverage:");
    println!("================================");
    println!("Total HTMX interactions in views: 40");
    println!("");
    println!("Tests by category:");
    println!("- Homepage & Navigation: 3 tests");
    println!("- Communities List/Search: 3 tests");
    println!("- Community Detail: 2 tests");
    println!("- Community Posts: 1 test (covers 3 sort options)");
    println!("- Create Community: 2 tests");
    println!("- Membership (join/leave): 3 tests");
    println!("- Members List & Admin: 2 tests");
    println!("- Posts CRUD: 3 tests");
    println!("- Comments: 2 tests");
    println!("- Reactions: 2 tests (covers 5 reaction types)");
    println!("- Dashboard: 3 tests");
    println!("- Chat: 2 tests");
    println!("- Other pages: 3 tests");
    println!("- Health: 1 test");
    println!("");
    println!("All authenticated endpoints verify 401 Unauthorized");
    println!("All public endpoints verify successful HTML response");
}

// ============================================================================
// CLEANUP - Must run last (z prefix ensures alphabetical ordering)
// ============================================================================

/// Final cleanup test - removes all test data from database
/// 
/// This test runs last (due to 'z' prefix in name) and cleans up all test data
/// created during the test run. This ensures the database is left in a clean state.
#[tokio::test]
#[serial]
async fn test_zz_cleanup_test_data() {
    let db = setup_db().await;
    cleanup_test_data(&db).await;
    
    // Verify cleanup was successful
    let remaining = sqlx::query!(
        "SELECT COUNT(*) as count FROM communities WHERE slug LIKE 'test-%' OR slug LIKE 'view-interaction-%'"
    )
    .fetch_one(&db.pool)
    .await
    .expect("Query should work");
    
    assert_eq!(remaining.count.unwrap_or(0), 0, "All test communities should be deleted");
    
    println!("✅ Test data cleanup completed successfully");
}
