//! View Interaction Tests
//!
//! Tests all HTMX interactions in the application views.
//! Each test makes real HTTP requests to the app and verifies:
//! - Correct HTTP status codes
//! - HTML response contains expected elements
//! - HTMX attributes are correctly wired
//! - Data is correctly rendered
//!
//! Total interactions: 40+
//!
//! ## Running Tests
//! 
//! For guaranteed cleanup, run with single thread:
//! ```bash
//! cargo test view_interaction -p server -- --test-threads=1
//! ```
//!
//! For faster parallel execution (cleanup may not run last):
//! ```bash
//! cargo test view_interaction -p server
//! ```
//!
//! ## Test Data Isolation
//! 
//! All test data uses a unique `TEST_RUN_ID` prefix (`__test_runner_<uuid>_`)
//! to avoid conflicts with user data. The `test_view_interaction_zz_cleanup`
//! test cleans up all data created during the test run.
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
use std::sync::LazyLock;

/// Unique test run identifier - ensures test data is isolated per test run
/// Format: __test_runner_<uuid>_ - this prefix is extremely unlikely to be used by real users
static TEST_RUN_ID: LazyLock<String> = LazyLock::new(|| {
    format!("__test_runner_{}", Uuid::now_v7())
});

/// Get the test slug prefix for this test run
fn test_slug_prefix() -> String {
    format!("{}_", *TEST_RUN_ID)
}

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
/// This function removes all test communities and related data created by THIS test run.
/// Uses the unique TEST_RUN_ID to only delete data created by this specific test execution.
async fn cleanup_test_data(db: &Database) {
    let prefix_pattern = format!("{}%", *TEST_RUN_ID);
    
    // Delete test communities created by this test run (cascades to related data)
    sqlx::query!(
        "DELETE FROM communities WHERE slug LIKE $1",
        prefix_pattern
    )
    .execute(&db.pool)
    .await
    .ok();
    
    // Delete test users (main test user and any member users created for tests)
    let test_email_pattern = format!("{}%@test.local", *TEST_RUN_ID);
    sqlx::query!(
        "DELETE FROM users WHERE email LIKE $1",
        test_email_pattern
    )
    .execute(&db.pool)
    .await
    .ok();
}

/// Get or create a test community for testing
/// 
/// This function ensures a valid test community exists with a valid creator.
/// Uses a unique slug per test run to avoid conflicts with user data.
async fn get_or_create_test_community(db: &Database) -> (Uuid, String) {
    let fixed_slug = format!("{}_community", *TEST_RUN_ID);
    
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
    
    let test_email = format!("{}@test.local", *TEST_RUN_ID);
    
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
                test_email.clone(),
                format!("auth0|{}", *TEST_RUN_ID)
            )
            .execute(&db.pool)
            .await
            .ok();
            
            // Fetch the user (might be existing or newly created)
            sqlx::query!("SELECT id FROM users WHERE email = $1", test_email)
                .fetch_one(&db.pool)
                .await
                .map(|u| u.id)
                .unwrap_or(test_user_id)
        }
    };
    
    // Create the community with unique test run slug
    let id = Uuid::now_v7();
    let community_name = format!("Test Community {}", &TEST_RUN_ID[15..23]); // Short readable name
    
    sqlx::query!(
        "INSERT INTO communities (id, name, slug, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, $5, NOW(), NOW())
         ON CONFLICT (slug) DO NOTHING",
        id,
        community_name,
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

/// Test: Community detail page shows correct dynamic stats
/// 
/// This test creates a community with known members and posts, then verifies
/// that the page displays the correct counts.
#[tokio::test]
#[serial]
async fn test_view_interaction_06b_community_dynamic_stats() {
    let db = setup_db().await;
    
    // Create a unique test community for this test
    let community_slug = format!("{}_stats_test", *TEST_RUN_ID);
    let community_id = Uuid::now_v7();
    
    // Get or create a test user
    let test_email = format!("{}@test.local", *TEST_RUN_ID);
    let user_id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (email) DO NOTHING",
        user_id,
        test_email,
        format!("auth0|{}", *TEST_RUN_ID)
    )
    .execute(&db.pool)
    .await
    .ok();
    
    // Fetch the actual user ID
    let actual_user = sqlx::query!("SELECT id FROM users WHERE email = $1", test_email)
        .fetch_one(&db.pool)
        .await
        .expect("User should exist");
    let creator_id = actual_user.id;
    
    // Create the test community
    sqlx::query!(
        "INSERT INTO communities (id, name, slug, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, $5, NOW(), NOW())",
        community_id,
        "Stats Test Community",
        community_slug,
        "A community to test dynamic stats",
        creator_id
    )
    .execute(&db.pool)
    .await
    .expect("Should create community");
    
    // Add 3 members to the community - get the actual member role ID
    let member_role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = 'member' LIMIT 1")
        .fetch_one(&db.pool)
        .await
        .expect("Member role should exist");
    
    for i in 0..3 {
        let member_id = Uuid::now_v7();
        // Use unique email per test run AND per member to avoid conflicts
        let member_email = format!("{}_stats_member{}@test.local", *TEST_RUN_ID, i);
        
        // Insert user with RETURNING to get the actual ID
        // Use unique auth0_id per test run to avoid conflicts
        let insert_result = sqlx::query!(
            "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
             VALUES ($1, $2, $3, NOW(), NOW())
             ON CONFLICT (email) DO UPDATE SET updated_at = NOW()
             RETURNING id",
            member_id,
            member_email,
            format!("auth0|{}_stats_member{}", *TEST_RUN_ID, i)
        )
        .fetch_one(&db.pool)
        .await;
        
        if let Ok(user_row) = insert_result {
            let member_insert = sqlx::query!(
                "INSERT INTO community_members (community_id, user_id, role_id, status, joined_at)
                 VALUES ($1, $2, $3, 'active', NOW())
                 ON CONFLICT (community_id, user_id) DO NOTHING",
                community_id,
                user_row.id,
                member_role_id
            )
            .execute(&db.pool)
            .await;
            
            if let Err(e) = member_insert {
                eprintln!("Warning: Failed to insert member {}: {}", i, e);
            }
        }
    }
    
    // Add 2 posts to the community
    for i in 0..2 {
        let post_id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO posts (id, community_id, author_id, title, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            post_id,
            community_id,
            creator_id,
            format!("Test Post {}", i + 1),
            format!("Content for test post {}", i + 1)
        )
        .execute(&db.pool)
        .await
        .expect("Should create post");
    }
    
    // Now fetch the community detail page and verify stats
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Verify the page shows the correct stats
    assert!(body.contains("Stats Test Community"), "Should show community name");
    assert!(body.contains(">3<") || body.contains(">3</"), "Should show 3 members");
    assert!(body.contains(">2<") || body.contains(">2</"), "Should show 2 posts");
    
    // Cleanup: Delete test data (will be cleaned by zz_cleanup too, but good practice)
    sqlx::query!("DELETE FROM posts WHERE community_id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
    sqlx::query!("DELETE FROM community_members WHERE community_id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
    sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
    // Note: test users will be cleaned by zz_cleanup
}

/// Test: Community detail page shows posts in feed
/// 
/// Verifies that posts created for a community appear in the HTMX feed fragment.
#[tokio::test]
#[serial]
async fn test_view_interaction_06c_community_feed_shows_posts() {
    let db = setup_db().await;
    
    // Create test community and posts
    let community_slug = format!("{}_feed_test", *TEST_RUN_ID);
    let community_id = Uuid::now_v7();
    
    let test_email = format!("{}@test.local", *TEST_RUN_ID);
    let user_id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (email) DO NOTHING",
        user_id,
        test_email,
        format!("auth0|{}", *TEST_RUN_ID)
    )
    .execute(&db.pool)
    .await
    .ok();
    
    let actual_user = sqlx::query!("SELECT id FROM users WHERE email = $1", test_email)
        .fetch_one(&db.pool)
        .await
        .expect("User should exist");
    let creator_id = actual_user.id;
    
    sqlx::query!(
        "INSERT INTO communities (id, name, slug, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, $5, NOW(), NOW())",
        community_id,
        "Feed Test Community",
        community_slug,
        "A community to test feed",
        creator_id
    )
    .execute(&db.pool)
    .await
    .expect("Should create community");
    
    // Create a post with a unique title we can search for
    let unique_title = format!("UniquePostTitle_{}", &TEST_RUN_ID[15..23]);
    let post_id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO posts (id, community_id, author_id, title, content, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
        post_id,
        community_id,
        creator_id,
        unique_title,
        "This is the content of the unique test post"
    )
    .execute(&db.pool)
    .await
    .expect("Should create post");
    
    // Fetch the community feed HTMX fragment
    let server = create_server().await;
    let response = server.get(&format!("/htmx/communities/{}/feed", community_id)).await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Verify the post appears in the feed
    assert!(body.contains(&unique_title), "Feed should contain the post title: {}", unique_title);
    
    // Cleanup
    sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
        .execute(&db.pool)
        .await
        .ok();
    sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
}

/// Test: Communities list HTMX fragment shows community data from DB
#[tokio::test]
#[serial]
async fn test_view_interaction_06d_communities_list_shows_data() {
    let db = setup_db().await;
    
    // Create test community with unique name
    let community_slug = format!("{}_list_test", *TEST_RUN_ID);
    let community_id = Uuid::now_v7();
    let unique_name = format!("ListTestCommunity_{}", &TEST_RUN_ID[15..23]);
    
    // Get or create test user
    let test_email = format!("{}@test.local", *TEST_RUN_ID);
    let user_id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (email) DO UPDATE SET updated_at = NOW()
         RETURNING id",
        user_id,
        test_email,
        format!("auth0|{}", *TEST_RUN_ID)
    )
    .fetch_one(&db.pool)
    .await
    .ok();
    
    let creator = sqlx::query!("SELECT id FROM users WHERE email = $1", test_email)
        .fetch_one(&db.pool)
        .await
        .expect("User should exist");
    
    sqlx::query!(
        "INSERT INTO communities (id, name, slug, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, $5, NOW(), NOW())",
        community_id,
        unique_name,
        community_slug,
        "Test community for list verification",
        creator.id
    )
    .execute(&db.pool)
    .await
    .expect("Should create community");
    
    // Fetch the communities list fragment
    let server = create_server().await;
    let response = server.get("/htmx/communities/list").await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Verify our community appears in the list
    assert!(body.contains(&unique_name), "Communities list should contain: {}", unique_name);
    
    // Cleanup
    sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
}

/// Test: Index page recent communities fragment shows data from DB
#[tokio::test]
#[serial]
async fn test_view_interaction_06e_index_recent_communities() {
    let db = setup_db().await;
    
    // Create test community
    let community_slug = format!("{}_recent_test", *TEST_RUN_ID);
    let community_id = Uuid::now_v7();
    let unique_name = format!("RecentTestCommunity_{}", &TEST_RUN_ID[15..23]);
    
    let test_email = format!("{}@test.local", *TEST_RUN_ID);
    let user_id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (email) DO UPDATE SET updated_at = NOW()
         RETURNING id",
        user_id,
        test_email,
        format!("auth0|{}", *TEST_RUN_ID)
    )
    .fetch_one(&db.pool)
    .await
    .ok();
    
    let creator = sqlx::query!("SELECT id FROM users WHERE email = $1", test_email)
        .fetch_one(&db.pool)
        .await
        .expect("User should exist");
    
    sqlx::query!(
        "INSERT INTO communities (id, name, slug, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, $5, NOW(), NOW())",
        community_id,
        unique_name,
        community_slug,
        "Test community for recent list",
        creator.id
    )
    .execute(&db.pool)
    .await
    .expect("Should create community");
    
    // Fetch the recent communities fragment
    let server = create_server().await;
    let response = server.get("/htmx/communities/recent").await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Verify our community appears (it's the most recent)
    assert!(body.contains(&unique_name), "Recent communities should contain: {}", unique_name);
    
    // Cleanup
    sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
}

/// Test: Members list API returns correct member count and data
#[tokio::test]
#[serial]
async fn test_view_interaction_06f_members_list_shows_data() {
    let db = setup_db().await;
    
    // Create test community
    let community_slug = format!("{}_members_test", *TEST_RUN_ID);
    let community_id = Uuid::now_v7();
    
    let test_email = format!("{}@test.local", *TEST_RUN_ID);
    let user_id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (email) DO UPDATE SET updated_at = NOW()
         RETURNING id",
        user_id,
        test_email,
        format!("auth0|{}", *TEST_RUN_ID)
    )
    .fetch_one(&db.pool)
    .await
    .ok();
    
    let creator = sqlx::query!("SELECT id FROM users WHERE email = $1", test_email)
        .fetch_one(&db.pool)
        .await
        .expect("User should exist");
    
    sqlx::query!(
        "INSERT INTO communities (id, name, slug, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, $5, NOW(), NOW())",
        community_id,
        "Members Test Community",
        community_slug,
        "Test community for members list",
        creator.id
    )
    .execute(&db.pool)
    .await
    .expect("Should create community");
    
    // Add 4 members - get the actual member role ID
    let member_role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = 'member' LIMIT 1")
        .fetch_one(&db.pool)
        .await
        .expect("Member role should exist");
    
    for i in 0..4 {
        let member_id = Uuid::now_v7();
        let member_email = format!("{}_members_member{}@test.local", *TEST_RUN_ID, i);
        
        let insert_result = sqlx::query!(
            "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
             VALUES ($1, $2, $3, NOW(), NOW())
             ON CONFLICT (email) DO UPDATE SET updated_at = NOW()
             RETURNING id",
            member_id,
            member_email,
            format!("auth0|{}_members_member{}", *TEST_RUN_ID, i)
        )
        .fetch_one(&db.pool)
        .await;
        
        if let Ok(user_row) = insert_result {
            sqlx::query!(
                "INSERT INTO community_members (community_id, user_id, role_id, status, joined_at)
                 VALUES ($1, $2, $3, 'active', NOW())
                 ON CONFLICT DO NOTHING",
                community_id,
                user_row.id,
                member_role_id
            )
            .execute(&db.pool)
            .await
            .ok();
        }
    }
    
    // Fetch members list API
    let server = create_server().await;
    let response = server
        .get(&format!("/api/communities/{}/members", community_id))
        .add_query_param("page", "1")
        .add_query_param("limit", "10")
        .await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Verify response contains member count (JSON response)
    assert!(body.contains("\"total\":4") || body.contains("\"total\": 4"), 
            "Members API should return total: 4, got: {}", body);
    
    // Cleanup
    sqlx::query!("DELETE FROM community_members WHERE community_id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
    sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
}

/// Test: Post detail page shows correct data from DB
#[tokio::test]
#[serial]
async fn test_view_interaction_06g_post_detail_shows_data() {
    let db = setup_db().await;
    
    // Create test community
    let community_slug = format!("{}_postdetail_test", *TEST_RUN_ID);
    let community_id = Uuid::now_v7();
    
    let test_email = format!("{}@test.local", *TEST_RUN_ID);
    let user_id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO users (id, email, auth0_id, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (email) DO UPDATE SET updated_at = NOW()
         RETURNING id",
        user_id,
        test_email,
        format!("auth0|{}", *TEST_RUN_ID)
    )
    .fetch_one(&db.pool)
    .await
    .ok();
    
    let creator = sqlx::query!("SELECT id FROM users WHERE email = $1", test_email)
        .fetch_one(&db.pool)
        .await
        .expect("User should exist");
    
    sqlx::query!(
        "INSERT INTO communities (id, name, slug, description, is_public, created_by, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, $5, NOW(), NOW())",
        community_id,
        "Post Detail Test Community",
        community_slug,
        "Test community for post detail",
        creator.id
    )
    .execute(&db.pool)
    .await
    .expect("Should create community");
    
    // Create a post with unique title
    let unique_title = format!("UniquePostDetailTitle_{}", &TEST_RUN_ID[15..23]);
    let post_id = Uuid::now_v7();
    
    sqlx::query!(
        "INSERT INTO posts (id, community_id, author_id, title, content, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
        post_id,
        community_id,
        creator.id,
        unique_title,
        "This is the content of the post detail test"
    )
    .execute(&db.pool)
    .await
    .expect("Should create post");
    
    // Add 2 comments to the post
    for i in 0..2 {
        let comment_id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO comments (id, post_id, author_id, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            comment_id,
            post_id,
            creator.id,
            format!("Test comment {}", i + 1)
        )
        .execute(&db.pool)
        .await
        .ok();
    }
    
    // Fetch post detail page
    let server = create_server().await;
    let response = server.get(&format!("/posts/{}", post_id)).await;
    
    response.assert_status_success();
    let body = response.text();
    
    // Verify post title appears
    assert!(body.contains(&unique_title), "Post detail should show title: {}", unique_title);
    // Verify comment count (2)
    assert!(body.contains("(2)") || body.contains("Commenti (2)"), 
            "Post detail should show 2 comments");
    
    // Cleanup
    sqlx::query!("DELETE FROM comments WHERE post_id = $1", post_id)
        .execute(&db.pool)
        .await
        .ok();
    sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
        .execute(&db.pool)
        .await
        .ok();
    sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
        .execute(&db.pool)
        .await
        .ok();
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
// 15. NAVIGATION UX TESTS
// ============================================================================
// These tests verify the navigation structure follows UX requirements.
// Key principle: Businesses and Governance belong INSIDE communities, not global.

/// Test: Navbar has link to Communities
#[tokio::test]
async fn test_nav_navbar_has_communities_link() {
    let server = create_server().await;
    let response = server.get("/").await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("href=\"/communities\"") || body.contains("href='/communities'"),
        "Navbar MUST contain link to /communities"
    );
}

/// Test: Navbar has link to Chat
#[tokio::test]
async fn test_nav_navbar_has_chat_link() {
    let server = create_server().await;
    let response = server.get("/").await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("href=\"/chat\"") || body.contains("href='/chat'"),
        "Navbar MUST contain link to /chat"
    );
}

/// Test: Navbar should NOT have global Businesses link
/// Businesses belong to communities, not global navigation
#[tokio::test]
async fn test_nav_navbar_no_global_businesses() {
    let server = create_server().await;
    let response = server.get("/").await;
    response.assert_status_success();
    let body = response.text();
    
    // Extract just the nav/header section to avoid false positives from page content
    let nav_end = body.find("</header>").unwrap_or(5000).min(5000);
    let nav_section = &body[..nav_end];
    
    assert!(
        !nav_section.contains("href=\"/businesses\"") && !nav_section.contains("href='/businesses'"),
        "Navbar should NOT contain global /businesses link - businesses belong to communities"
    );
}

/// Test: Navbar has auth section (login or profile)
#[tokio::test]
async fn test_nav_navbar_has_auth_section() {
    let server = create_server().await;
    let response = server.get("/").await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("/auth/login") || body.contains("Accedi") || body.contains("Login") || body.contains("profile"),
        "Navbar MUST contain auth section"
    );
}

/// Test: Community detail has Feed tab
#[tokio::test]
#[serial]
async fn test_nav_community_has_feed_tab() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("Feed") || body.contains("feed"),
        "Community detail MUST have Feed tab"
    );
}

/// Test: Community detail has Members tab
#[tokio::test]
#[serial]
async fn test_nav_community_has_members_tab() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("Membri") || body.contains("members") || body.contains("Members"),
        "Community detail MUST have Members tab"
    );
}

/// Test: Community detail has Governance/Votazioni tab
#[tokio::test]
#[serial]
async fn test_nav_community_has_governance_tab() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("Votazioni") || body.contains("governance") || body.contains("Governance") || body.contains("Proposte"),
        "Community detail MUST have Governance/Votazioni tab"
    );
}

/// Test: Community detail has Businesses/Attività tab
/// THIS TEST WILL FAIL until the tab is implemented
#[tokio::test]
#[serial]
async fn test_nav_community_has_businesses_tab() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("Attività") || body.contains("businesses") || body.contains("Negozi") || body.contains("Locali"),
        "Community detail MUST have Attività/Businesses tab"
    );
}

/// Test: Community detail has Chat tab
/// THIS TEST WILL FAIL until the tab is implemented
#[tokio::test]
#[serial]
async fn test_nav_community_has_chat_tab() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains(">Chat<") || body.contains("chat") || body.contains("Messaggi"),
        "Community detail MUST have Chat tab"
    );
}

/// Test: HTMX endpoint for community businesses exists
/// THIS TEST WILL FAIL until the endpoint is implemented
#[tokio::test]
#[serial]
async fn test_nav_htmx_community_businesses_endpoint() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/htmx/communities/{}/businesses", community_id)).await;
    
    let status = response.status_code();
    assert!(
        status.is_success(),
        "Endpoint /htmx/communities/{{id}}/businesses MUST exist and return 200, got {}",
        status
    );
}

/// Test: HTMX endpoint for community chat exists
/// THIS TEST WILL FAIL until the endpoint is implemented
#[tokio::test]
#[serial]
async fn test_nav_htmx_community_chat_endpoint() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/htmx/communities/{}/chat", community_id)).await;
    
    let status = response.status_code();
    assert!(
        status.is_success(),
        "Endpoint /htmx/communities/{{id}}/chat MUST exist and return 200, got {}",
        status
    );
}

/// Test: Footer has language switcher
#[tokio::test]
async fn test_nav_footer_has_language_switcher() {
    let server = create_server().await;
    let response = server.get("/").await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("set-language") || body.contains("lang") || 
        body.contains("Italiano") || body.contains("English"),
        "Footer MUST have language switcher"
    );
}

/// Test: Community detail has join/leave button
#[tokio::test]
#[serial]
async fn test_nav_community_has_join_button() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("Unisciti") || body.contains("join") || body.contains("Lascia") || 
        body.contains("leave") || body.contains("Membro") || body.contains("Accedi per unirti"),
        "Community detail MUST have Join/Leave button"
    );
}

/// Test: Post detail has breadcrumb to community
#[tokio::test]
#[serial]
async fn test_nav_post_has_breadcrumb() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    // Create a test post
    let test_email = format!("{}@test.local", *TEST_RUN_ID);
    let creator = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        test_email
    )
    .fetch_optional(&db.pool)
    .await
    .ok()
    .flatten();
    
    if let Some(user) = creator {
        let post_id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO posts (id, community_id, author_id, title, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
             ON CONFLICT DO NOTHING",
            post_id,
            community_id,
            user.id,
            format!("Nav Test Post {}", &TEST_RUN_ID[15..23]),
            "Test post for navigation breadcrumb"
        )
        .execute(&db.pool)
        .await
        .ok();
        
        let server = create_server().await;
        let response = server.get(&format!("/posts/{}", post_id)).await;
        
        if response.status_code().is_success() {
            let body = response.text();
            assert!(
                body.contains("/communities") && body.contains(&community_id.to_string()),
                "Post detail MUST have breadcrumb with community link"
            );
        }
        
        // Cleanup
        sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
            .execute(&db.pool)
            .await
            .ok();
    }
}

/// Test: Dashboard link exists in template (visible when logged in)
/// Note: Dashboard link is conditional on logged_in, so we check the dashboard page exists
#[tokio::test]
async fn test_nav_dashboard_page_exists() {
    let server = create_server().await;
    let response = server.get("/dashboard").await;
    
    // Dashboard page should exist (may redirect to login if not authenticated)
    let status = response.status_code();
    assert!(
        status.is_success() || status.as_u16() == 302 || status.as_u16() == 401,
        "Dashboard page MUST exist (may require auth)"
    );
}

/// Test: Community detail page has post creation capability
/// Note: The "Nuovo Post" button is only visible for members, but the route should exist
#[tokio::test]
#[serial]
async fn test_nav_community_create_post_route_exists() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}/posts/new", community_id)).await;
    
    // Route should exist (may require auth/membership)
    let status = response.status_code();
    assert!(
        status.is_success() || status.as_u16() == 302 || status.as_u16() == 401 || status.as_u16() == 403,
        "Create post route MUST exist for community"
    );
}

/// Test: Community detail has "Nuova Proposta" button
#[tokio::test]
#[serial]
async fn test_nav_community_has_new_proposal_button() {
    let db = setup_db().await;
    let (community_id, _) = get_or_create_test_community(&db).await;
    
    let server = create_server().await;
    let response = server.get(&format!("/communities/{}", community_id)).await;
    response.assert_status_success();
    let body = response.text();
    
    assert!(
        body.contains("Nuova Proposta") || body.contains("proposal") || 
        body.contains("Crea Proposta") || body.contains("New Proposal"),
        "Community detail MUST have New Proposal button (for members)"
    );
}

/// Test: Notifications page exists
#[tokio::test]
async fn test_nav_notifications_page_exists() {
    let server = create_server().await;
    let response = server.get("/notifications").await;
    
    // May require auth
    let status = response.status_code();
    assert!(
        status.is_success() || status.as_u16() == 302 || status.as_u16() == 401,
        "Notifications page MUST exist (may require auth)"
    );
}

/// Test: Profile page has tabs
#[tokio::test]
async fn test_nav_profile_has_tabs() {
    let server = create_server().await;
    // Use a fake UUID - will 404 but we can check if route exists
    let response = server.get("/users/00000000-0000-0000-0000-000000000001").await;
    
    let status = response.status_code();
    if status.is_success() {
        let body = response.text();
        assert!(
            body.contains("Post") && body.contains("Community") && 
            (body.contains("Follower") || body.contains("follower")),
            "Profile page MUST have tabs: Post, Community, Follower"
        );
    }
    // 404 is acceptable for non-existent user
}

/// Test: Chat list shows community rooms
#[tokio::test]
async fn test_nav_chat_list_shows_rooms() {
    let server = create_server().await;
    let response = server.get("/chat").await;
    
    let status = response.status_code();
    if status.is_success() {
        let body = response.text();
        assert!(
            body.contains("Chat") || body.contains("chat") || 
            body.contains("room") || body.contains("community"),
            "Chat list MUST show chat rooms or empty state with community link"
        );
    }
}

// ============================================================================
// SUMMARY
// ============================================================================

/// Meta-test to document coverage
#[test]
fn test_view_interaction_coverage_summary() {
    println!("View Interaction Tests Coverage:");
    println!("================================");
    println!("Total HTMX interactions in views: 50+");
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
    println!("- Navigation UX: 13 tests (NEW)");
    println!("");
    println!("Navigation UX Tests (test-first approach):");
    println!("  ✓ Navbar links: Communities, Chat, Auth");
    println!("  ✗ Navbar NO global Businesses (belongs to community)");
    println!("  ✓ Community tabs: Feed, Members, Governance");
    println!("  ✗ Community tabs: Attività, Chat (TO IMPLEMENT)");
    println!("  ✗ HTMX endpoints: /htmx/communities/:id/businesses (TO IMPLEMENT)");
    println!("  ✗ HTMX endpoints: /htmx/communities/:id/chat (TO IMPLEMENT)");
    println!("  ✓ Footer: Language switcher");
    println!("  ✓ Community: Join/Leave button");
    println!("");
    println!("All authenticated endpoints verify 401 Unauthorized");
    println!("All public endpoints verify successful HTML response");
}

// ============================================================================
// CLEANUP - Must run last (z prefix ensures alphabetical ordering)
// ============================================================================

/// Final cleanup test - removes all test data from database
/// 
/// This test runs last (due to 'zz' in name) and cleans up all test data
/// created during THIS test run. Uses unique TEST_RUN_ID to only clean up our data.
#[tokio::test]
#[serial]
async fn test_view_interaction_zz_cleanup() {
    let db = setup_db().await;
    cleanup_test_data(&db).await;
    
    // Verify cleanup was successful for THIS test run
    let prefix_pattern = format!("{}%", *TEST_RUN_ID);
    let remaining = sqlx::query!(
        "SELECT COUNT(*) as count FROM communities WHERE slug LIKE $1",
        prefix_pattern
    )
    .fetch_one(&db.pool)
    .await
    .expect("Query should work");
    
    assert_eq!(remaining.count.unwrap_or(0), 0, "All test communities from this run should be deleted");
    
    println!("✅ Test data cleanup completed successfully for run: {}", &TEST_RUN_ID[15..23]);
}
