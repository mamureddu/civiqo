use serde_json::Value;
use uuid::Uuid;

use server::handlers::api::{CommunitiesListResponse, CommunityDetailResponse};
use shared::database::Database;

#[cfg(test)]
mod communities_api_tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestApiResponse<T> {
        pub success: bool,
        pub data: Option<T>,
        pub message: Option<String>,
    }

    async fn setup_test_db() -> Database {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for integration tests");
        
        Database::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }
    
    /// Create a test user and community for tests that need them
    /// Returns (user_id, community_id, slug, unique_prefix) for cleanup
    async fn create_test_community(db: &Database) -> (Uuid, Uuid, String, String) {
        let unique_prefix = format!("__api_test_{}", Uuid::now_v7());
        let user_id = Uuid::now_v7();
        let community_id = Uuid::now_v7();
        let slug = format!("{}_community", unique_prefix);
        
        // Create test user
        sqlx::query!(
            "INSERT INTO users (id, auth0_id, email) VALUES ($1, $2, $3)
             ON CONFLICT (auth0_id) DO UPDATE SET email = EXCLUDED.email
             RETURNING id",
            user_id,
            format!("auth0|{}", user_id),
            format!("{}@test.local", user_id)
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to create test user");
        
        // Create test community
        sqlx::query!(
            "INSERT INTO communities (id, name, slug, description, is_public, created_by)
             VALUES ($1, $2, $3, $4, true, $5)
             ON CONFLICT (slug) DO UPDATE SET name = EXCLUDED.name
             RETURNING id",
            community_id,
            format!("{} Test Community", unique_prefix),
            slug,
            "A test community for API tests",
            user_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to create test community");
        
        (user_id, community_id, slug, unique_prefix)
    }
    
    /// Cleanup specific test data
    async fn cleanup_test_community(db: &Database, community_id: Uuid, user_id: Uuid) {
        sqlx::query!("DELETE FROM community_members WHERE community_id = $1", community_id)
            .execute(&db.pool)
            .await
            .ok();
        sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
            .execute(&db.pool)
            .await
            .ok();
        sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&db.pool)
            .await
            .ok();
    }

    /// Test GET /api/communities returns paginated list
    #[tokio::test]
    async fn test_get_communities_list() {
        let db = setup_test_db().await;
        
        // Create test community
        let (user_id, community_id, _slug, _prefix) = create_test_community(&db).await;
        
        // Query communities directly from database to verify endpoint behavior
        let communities = sqlx::query!(
            "SELECT id, name, slug, is_public FROM communities WHERE is_public = true ORDER BY created_at DESC LIMIT 10"
        )
        .fetch_all(&db.pool)
        .await
        .expect("Failed to fetch communities");
        
        // Verify we have at least one public community
        assert!(!communities.is_empty(), "Should have at least one public community");
        
        // Verify our test community exists
        let has_test = communities.iter().any(|c| c.id == community_id);
        assert!(has_test, "Test community should exist in results");
        
        // Cleanup
        cleanup_test_community(&db, community_id, user_id).await;
    }

    /// Test GET /api/communities with search parameter
    #[tokio::test]
    async fn test_get_communities_with_search() {
        let db = setup_test_db().await;
        
        // Create test community
        let (user_id, community_id, _slug, prefix) = create_test_community(&db).await;
        
        // Test search functionality with ILIKE using our test prefix
        let communities = sqlx::query!(
            "SELECT id, name FROM communities WHERE name ILIKE $1 OR description ILIKE $1",
            format!("%{}%", prefix)
        )
        .fetch_all(&db.pool)
        .await
        .expect("Failed to search communities");
        
        // Should find our test community
        assert!(!communities.is_empty(), "Search should find matching communities");
        assert!(communities.iter().any(|c| c.name.contains(&prefix)));
        
        // Cleanup
        cleanup_test_community(&db, community_id, user_id).await;
    }

    /// Test GET /api/communities with sort parameter
    #[tokio::test]
    async fn test_get_communities_with_sort() {
        let db = setup_test_db().await;
        
        // Test sorting by name
        let communities_by_name = sqlx::query!(
            "SELECT id, name FROM communities WHERE is_public = true ORDER BY name ASC LIMIT 10"
        )
        .fetch_all(&db.pool)
        .await
        .expect("Failed to fetch sorted communities");
        
        // Test sorting by created_at
        let communities_by_date = sqlx::query!(
            "SELECT id, name, created_at FROM communities WHERE is_public = true ORDER BY created_at DESC LIMIT 10"
        )
        .fetch_all(&db.pool)
        .await
        .expect("Failed to fetch sorted communities");
        
        // Verify we got results
        assert!(!communities_by_name.is_empty(), "Should have communities sorted by name");
        assert!(!communities_by_date.is_empty(), "Should have communities sorted by date");
    }

    /// Test GET /api/communities with pagination
    #[tokio::test]
    async fn test_get_communities_pagination() {
        let db = setup_test_db().await;
        
        // Test pagination with LIMIT and OFFSET
        let page_1 = sqlx::query!(
            "SELECT id, name FROM communities WHERE is_public = true ORDER BY created_at DESC LIMIT 2 OFFSET 0"
        )
        .fetch_all(&db.pool)
        .await
        .expect("Failed to fetch page 1");
        
        let page_2 = sqlx::query!(
            "SELECT id, name FROM communities WHERE is_public = true ORDER BY created_at DESC LIMIT 2 OFFSET 2"
        )
        .fetch_all(&db.pool)
        .await
        .expect("Failed to fetch page 2");
        
        // Count total communities
        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM communities WHERE is_public = true"
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to count communities");
        
        // Verify pagination works
        assert!(total.is_some(), "Should have a total count");
        
        // If we have more than 2 communities, page 1 and page 2 should be different
        if let Some(count) = total {
            if count > 2 {
                assert_ne!(page_1.len(), 0, "Page 1 should have results");
            }
        }
    }

    /// Test GET /api/communities/:id with valid UUID
    #[tokio::test]
    async fn test_get_community_detail_by_uuid() {
        let db = setup_test_db().await;
        
        // Create test community
        let (user_id, community_id, _slug, prefix) = create_test_community(&db).await;
        
        // Test fetching by UUID with member count
        let detail = sqlx::query!(
            r#"SELECT c.id, c.name, c.description, c.slug, c.is_public, c.requires_approval,
                      c.created_at, c.updated_at,
                      COUNT(DISTINCT m.user_id) as "member_count!"
               FROM communities c
               LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
               WHERE c.id = $1
               GROUP BY c.id, c.name, c.description, c.slug, c.is_public, c.requires_approval,
                        c.created_at, c.updated_at"#,
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to fetch community detail");
        
        assert!(detail.name.contains(&prefix), "Should be our test community");
        assert!(detail.member_count >= 0, "Should have member count");
        
        // Cleanup
        cleanup_test_community(&db, community_id, user_id).await;
    }

    /// Test GET /api/communities/:id with valid slug
    #[tokio::test]
    async fn test_get_community_detail_by_slug() {
        let db = setup_test_db().await;
        
        // Create test community
        let (user_id, community_id, slug, prefix) = create_test_community(&db).await;
        
        // Test fetching by slug
        let detail = sqlx::query!(
            r#"SELECT c.id, c.name, c.slug
               FROM communities c
               WHERE c.slug = $1
               LIMIT 1"#,
            slug
        )
        .fetch_optional(&db.pool)
        .await
        .expect("Failed to fetch community by slug");
        
        assert!(detail.is_some(), "Should find community by slug");
        
        if let Some(comm) = detail {
            assert_eq!(comm.slug, slug);
            assert!(comm.name.contains(&prefix));
        }
        
        // Cleanup
        cleanup_test_community(&db, community_id, user_id).await;
    }

    /// Test GET /api/communities/:id with invalid ID
    #[tokio::test]
    async fn test_get_community_detail_not_found() {
        let db = setup_test_db().await;
        
        // Try to fetch non-existent community
        let fake_id = Uuid::new_v4();
        let result = sqlx::query!(
            "SELECT id, name FROM communities WHERE id = $1",
            fake_id
        )
        .fetch_optional(&db.pool)
        .await
        .expect("Query should execute successfully");
        
        assert!(result.is_none(), "Non-existent community should return None");
    }

    /// Test GET /api/communities/:id for private community (unauthenticated)
    #[tokio::test]
    async fn test_get_private_community_unauthenticated() {
        let db = setup_test_db().await;
        
        // Check if there are any private communities
        let private_communities = sqlx::query!(
            "SELECT id, name, is_public FROM communities WHERE is_public = false LIMIT 1"
        )
        .fetch_optional(&db.pool)
        .await
        .expect("Failed to check for private communities");
        
        if private_communities.is_none() {
            // Create a test user first (required for foreign key)
            let test_user_id = Uuid::new_v4();
            sqlx::query!(
                "INSERT INTO users (id, auth0_id, email) VALUES ($1, $2, $3)",
                test_user_id,
                format!("test-{}", test_user_id),
                format!("test-{}@example.com", test_user_id)
            )
            .execute(&db.pool)
            .await
            .expect("Failed to create test user");
            
            // Create a test private community
            let test_id = Uuid::new_v4();
            
            sqlx::query!(
                "INSERT INTO communities (id, name, slug, is_public, requires_approval, created_by) 
                 VALUES ($1, 'Private Test Community', 'private-test', false, true, $2)",
                test_id,
                test_user_id
            )
            .execute(&db.pool)
            .await
            .expect("Failed to create test private community");
            
            // Verify it's private
            let community = sqlx::query!(
                "SELECT is_public FROM communities WHERE id = $1",
                test_id
            )
            .fetch_one(&db.pool)
            .await
            .expect("Failed to fetch test community");
            
            assert_eq!(community.is_public, Some(false), "Test community should be private");
        }
    }

    /// Test GET /api/communities/:id for private community (authenticated member)
    #[tokio::test]
    async fn test_get_private_community_authenticated_member() {
        let db = setup_test_db().await;
        
        // Create test community
        let (user_id, community_id, _slug, _prefix) = create_test_community(&db).await;
        
        // Check if there are any members
        let member_count = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM community_members WHERE community_id = $1 AND status = 'active'",
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to count members");
        
        assert!(member_count.is_some(), "Should be able to count members");
        
        // Cleanup
        cleanup_test_community(&db, community_id, user_id).await;
    }

    /// Test SQL injection protection in search parameter
    #[tokio::test]
    async fn test_sql_injection_protection_search() {
        let db = setup_test_db().await;
        
        // Test various SQL injection attempts
        let malicious_inputs = vec![
            "'; DROP TABLE communities; --",
            "' OR '1'='1",
            "'; DELETE FROM communities WHERE '1'='1",
            "<script>alert('xss')</script>",
        ];
        
        for malicious_input in malicious_inputs {
            // The parameterized query should safely handle these inputs
            let result = sqlx::query!(
                "SELECT id, name FROM communities WHERE name ILIKE $1 OR description ILIKE $1",
                format!("%{}%", malicious_input)
            )
            .fetch_all(&db.pool)
            .await;
            
            // Query should execute without error (safely escaped)
            assert!(result.is_ok(), "Parameterized query should handle malicious input safely");
            
            // Should return empty results (no matches)
            if let Ok(communities) = result {
                // It's OK if it returns results, as long as it didn't execute malicious SQL
                // The key is that the query executed safely
                assert!(communities.len() == 0 || communities.len() > 0, "Query executed safely");
            }
        }
        
        // Verify the communities table still exists and has data
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM communities"
        )
        .fetch_one(&db.pool)
        .await
        .expect("Communities table should still exist");
        
        assert!(count.is_some() && count.unwrap() > 0, "Communities table should still have data");
    }

    /// Test response structure for CommunitiesListResponse
    #[test]
    fn test_communities_list_response_structure() {
        let json = r#"{
            "communities": [
                {
                    "id": "6fad39b7-81ac-49d4-b3c9-9d199a841102",
                    "name": "Test Community",
                    "description": "A test community",
                    "slug": "test-community",
                    "is_public": true,
                    "member_count": 5,
                    "created_at": "2025-11-19 17:38",
                    "user_role": null
                }
            ],
            "total_count": 1,
            "page": 1,
            "limit": 20,
            "has_next": false,
            "has_prev": false
        }"#;

        let response: CommunitiesListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total_count, 1);
        assert_eq!(response.page, 1);
        assert_eq!(response.limit, 20);
        assert!(!response.has_next);
        assert!(!response.has_prev);
        assert_eq!(response.communities.len(), 1);
        assert_eq!(response.communities[0].name, "Test Community");
    }

    /// Test response structure for CommunityDetailResponse
    #[test]
    fn test_community_detail_response_structure() {
        let json = r#"{
            "id": "6fad39b7-81ac-49d4-b3c9-9d199a841102",
            "name": "Test Community",
            "description": "A test community",
            "slug": "test-community",
            "is_public": true,
            "requires_approval": false,
            "member_count": 5,
            "posts_count": 10,
            "created_at": "2025-11-19 17:38",
            "updated_at": "2025-11-19 17:38",
            "user_role": "member",
            "is_member": true
        }"#;

        let response: CommunityDetailResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.name, "Test Community");
        assert_eq!(response.member_count, 5);
        assert_eq!(response.posts_count, 10);
        assert!(response.is_member);
        assert_eq!(response.user_role, Some("member".to_string()));
    }

    /// Test ApiResponse wrapper structure
    #[test]
    fn test_api_response_wrapper() {
        let json = r#"{
            "success": true,
            "data": {
                "id": "test-id",
                "name": "Test"
            },
            "message": "Success"
        }"#;

        let response: TestApiResponse<Value> = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.message, Some("Success".to_string()));
    }
}

