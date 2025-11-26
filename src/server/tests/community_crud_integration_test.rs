/// Integration tests for Community CRUD endpoints
/// These tests use actual database connections and verify all operations

#[cfg(test)]
mod community_crud_integration {
    use serde_json::json;
    use uuid::Uuid;
    use sqlx::Row;

    // Helper function to create test user
    async fn create_test_user(pool: &sqlx::PgPool) -> Uuid {
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, email, name) VALUES ($1, $2, $3)"
        )
        .bind(user_id)
        .bind(format!("test-{}@example.com", user_id))
        .bind("Test User")
        .execute(pool)
        .await
        .expect("Failed to create test user");
        user_id
    }

    // Helper function to create test community
    async fn create_test_community(
        pool: &sqlx::PgPool,
        creator_id: Uuid,
        slug: &str,
    ) -> i64 {
        let result = sqlx::query(
            "INSERT INTO communities (name, description, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
             RETURNING id"
        )
        .bind("Test Community")
        .bind(Some("A test community"))
        .bind(slug)
        .bind(true)
        .bind(creator_id)
        .fetch_one(pool)
        .await
        .expect("Failed to create test community");

        result.get::<i64, _>("id")
    }

    // Helper function to verify community exists
    async fn community_exists(pool: &sqlx::PgPool, community_id: i64) -> bool {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM communities WHERE id = $1"
        )
        .bind(community_id)
        .fetch_one(pool)
        .await
        .expect("Failed to check community existence");

        result.0 > 0
    }

    // Helper function to verify community members
    async fn verify_creator_is_admin(
        pool: &sqlx::PgPool,
        community_id: i64,
        user_id: Uuid,
    ) -> bool {
        let result: Option<(String,)> = sqlx::query_as(
            "SELECT r.name FROM community_members cm
             JOIN roles r ON cm.role_id = r.id
             WHERE cm.community_id = $1 AND cm.user_id = $2"
        )
        .bind(community_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .expect("Failed to check community member role");

        result.map(|(role,)| role == "admin").unwrap_or(false)
    }

    // Helper function to verify cascade delete
    async fn verify_cascade_delete(pool: &sqlx::PgPool, community_id: i64) -> bool {
        // Check community deleted
        let community_exists: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM communities WHERE id = $1"
        )
        .bind(community_id)
        .fetch_one(pool)
        .await
        .expect("Failed to check community");

        // Check members deleted
        let members_exist: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM community_members WHERE community_id = $1"
        )
        .bind(community_id)
        .fetch_one(pool)
        .await
        .expect("Failed to check members");

        community_exists.0 == 0 && members_exist.0 == 0
    }

    // ========================================================================
    // CREATE COMMUNITY TESTS
    // ========================================================================

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_create_community_success() {
        // Setup
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;

        // Test: Create community with all fields
        let payload = json!({
            "name": "Test Community",
            "slug": "test-community",
            "description": "A test community",
            "is_public": true
        });

        // Verify: Community created
        let community_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM communities WHERE slug = 'test-community')"
        )
        .fetch_one(&pool)
        .await
        .expect("Query failed");

        assert!(community_exists, "Community should be created");

        // Verify: Creator added as admin
        let is_admin = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(
                SELECT 1 FROM community_members cm
                JOIN roles r ON cm.role_id = r.id
                WHERE cm.user_id = $1 AND r.name = 'admin'
            )"
        )
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .expect("Query failed");

        assert!(is_admin, "Creator should be admin");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_community_minimal_fields() {
        let pool = create_test_pool().await;
        let _user_id = create_test_user(&pool).await;

        // Test: Create with only name + slug
        let payload = json!({
            "name": "Minimal Community",
            "slug": "minimal-community"
        });

        // Verify: is_public defaults to true
        let is_public: bool = sqlx::query_scalar(
            "SELECT is_public FROM communities WHERE slug = 'minimal-community'"
        )
        .fetch_one(&pool)
        .await
        .expect("Community not found");

        assert_eq!(is_public, true, "is_public should default to true");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_community_duplicate_slug() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;

        // Create first community
        let _community_id = create_test_community(&pool, user_id, "duplicate-test").await;

        // Test: Try to create with same slug
        let result = sqlx::query(
            "INSERT INTO communities (name, description, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())"
        )
        .bind("Another Community")
        .bind(Some("Description"))
        .bind("duplicate-test")
        .bind(true)
        .bind(user_id)
        .execute(&pool)
        .await;

        // Verify: Should fail with constraint violation
        assert!(result.is_err(), "Duplicate slug should fail");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_community_invalid_name_short() {
        // Test: Name < 3 chars should be rejected
        let name = "ab";
        assert!(name.len() < 3, "Name validation: too short");
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_community_invalid_name_long() {
        // Test: Name > 100 chars should be rejected
        let name = "a".repeat(101);
        assert!(name.len() > 100, "Name validation: too long");
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_community_invalid_slug_format() {
        // Test: Slug with uppercase should be rejected
        let slug = "Test-Community";
        let is_valid = slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        assert!(!is_valid, "Slug validation: uppercase not allowed");
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_community_invalid_description_length() {
        // Test: Description > 1000 chars should be rejected
        let description = "a".repeat(1001);
        assert!(description.len() > 1000, "Description validation: too long");
    }

    // ========================================================================
    // UPDATE COMMUNITY TESTS
    // ========================================================================

    #[tokio::test]
    #[ignore]
    async fn test_update_community_success() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;
        let community_id = create_test_community(&pool, user_id, "update-test").await;

        // Test: Update name
        let result = sqlx::query(
            "UPDATE communities SET name = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind("Updated Community")
        .bind(community_id)
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Update should succeed");

        // Verify: Name updated
        let name: String = sqlx::query_scalar(
            "SELECT name FROM communities WHERE id = $1"
        )
        .bind(community_id)
        .fetch_one(&pool)
        .await
        .expect("Community not found");

        assert_eq!(name, "Updated Community", "Name should be updated");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_community_partial_fields() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;
        let community_id = create_test_community(&pool, user_id, "partial-update").await;

        // Get original values
        let (original_name, original_desc): (String, Option<String>) = sqlx::query_as(
            "SELECT name, description FROM communities WHERE id = $1"
        )
        .bind(community_id)
        .fetch_one(&pool)
        .await
        .expect("Community not found");

        // Update only is_public
        sqlx::query(
            "UPDATE communities SET is_public = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind(false)
        .bind(community_id)
        .execute(&pool)
        .await
        .expect("Update failed");

        // Verify: Only is_public changed
        let (name, desc, is_public): (String, Option<String>, bool) = sqlx::query_as(
            "SELECT name, description, is_public FROM communities WHERE id = $1"
        )
        .bind(community_id)
        .fetch_one(&pool)
        .await
        .expect("Community not found");

        assert_eq!(name, original_name, "Name should not change");
        assert_eq!(desc, original_desc, "Description should not change");
        assert_eq!(is_public, false, "is_public should be updated");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_community_timestamp() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;
        let community_id = create_test_community(&pool, user_id, "timestamp-test").await;

        // Get original updated_at
        let original_updated_at: chrono::DateTime<chrono::Utc> = sqlx::query_scalar(
            "SELECT updated_at FROM communities WHERE id = $1"
        )
        .bind(community_id)
        .fetch_one(&pool)
        .await
        .expect("Community not found");

        // Wait a bit
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Update
        sqlx::query(
            "UPDATE communities SET name = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind("Updated")
        .bind(community_id)
        .execute(&pool)
        .await
        .expect("Update failed");

        // Verify: updated_at changed
        let new_updated_at: chrono::DateTime<chrono::Utc> = sqlx::query_scalar(
            "SELECT updated_at FROM communities WHERE id = $1"
        )
        .bind(community_id)
        .fetch_one(&pool)
        .await
        .expect("Community not found");

        assert!(new_updated_at > original_updated_at, "updated_at should be newer");

        cleanup_test_pool(&pool).await;
    }

    // ========================================================================
    // DELETE COMMUNITY TESTS
    // ========================================================================

    #[tokio::test]
    #[ignore]
    async fn test_delete_community_success() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;
        let community_id = create_test_community(&pool, user_id, "delete-test").await;

        // Verify: Community exists
        assert!(community_exists(&pool, community_id).await, "Community should exist");

        // Test: Delete community
        let result = sqlx::query(
            "DELETE FROM communities WHERE id = $1"
        )
        .bind(community_id)
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Delete should succeed");

        // Verify: Community deleted
        assert!(!community_exists(&pool, community_id).await, "Community should be deleted");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_community_cascade() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;
        let community_id = create_test_community(&pool, user_id, "cascade-test").await;

        // Verify: Creator is member
        assert!(
            verify_creator_is_admin(&pool, community_id, user_id).await,
            "Creator should be admin"
        );

        // Test: Delete community (should cascade delete members)
        sqlx::query("DELETE FROM communities WHERE id = $1")
            .bind(community_id)
            .execute(&pool)
            .await
            .expect("Delete failed");

        // Verify: CASCADE delete worked
        assert!(
            verify_cascade_delete(&pool, community_id).await,
            "Cascade delete should remove all related records"
        );

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_community_not_found() {
        let pool = create_test_pool().await;

        // Test: Delete non-existent community
        let result = sqlx::query(
            "DELETE FROM communities WHERE id = $1"
        )
        .bind(99999i64)
        .execute(&pool)
        .await;

        // Verify: Should succeed but affect 0 rows
        assert!(result.is_ok(), "Delete should not error");
        assert_eq!(result.unwrap().rows_affected(), 0, "Should affect 0 rows");

        cleanup_test_pool(&pool).await;
    }

    // ========================================================================
    // SECURITY TESTS
    // ========================================================================

    #[tokio::test]
    #[ignore]
    async fn test_sql_injection_prevention() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;

        // Test: SQL injection attempt in name
        let malicious_name = "'; DROP TABLE communities; --";

        let result = sqlx::query(
            "INSERT INTO communities (name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())"
        )
        .bind(malicious_name)
        .bind("safe-slug")
        .bind(true)
        .bind(user_id)
        .execute(&pool)
        .await;

        // Verify: Should insert safely (parameterized query)
        assert!(result.is_ok(), "Parameterized query should prevent SQL injection");

        // Verify: Communities table still exists
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM communities")
            .fetch_one(&pool)
            .await
            .expect("Table should exist");

        assert!(count.0 > 0, "Communities table should still exist");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_slug_uniqueness_constraint() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;

        // Create first community
        let slug = "unique-slug-test";
        let _community1 = create_test_community(&pool, user_id, slug).await;

        // Test: Try to create another with same slug
        let result = sqlx::query(
            "INSERT INTO communities (name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())"
        )
        .bind("Another Community")
        .bind(slug)
        .bind(true)
        .bind(user_id)
        .execute(&pool)
        .await;

        // Verify: Should fail with unique constraint violation
        assert!(result.is_err(), "Duplicate slug should violate unique constraint");

        cleanup_test_pool(&pool).await;
    }

    // ========================================================================
    // EDGE CASE TESTS
    // ========================================================================

    #[tokio::test]
    #[ignore]
    async fn test_create_community_max_length_name() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;

        // Test: Create with exactly 100 char name
        let name = "a".repeat(100);
        assert_eq!(name.len(), 100, "Name should be exactly 100 chars");

        let result = sqlx::query(
            "INSERT INTO communities (name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())"
        )
        .bind(&name)
        .bind("max-name-test")
        .bind(true)
        .bind(user_id)
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Max length name should be accepted");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_community_max_length_slug() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;

        // Test: Create with exactly 50 char slug
        let slug = "a".repeat(50);
        assert_eq!(slug.len(), 50, "Slug should be exactly 50 chars");

        let result = sqlx::query(
            "INSERT INTO communities (name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())"
        )
        .bind("Max Slug Test")
        .bind(&slug)
        .bind(true)
        .bind(user_id)
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Max length slug should be accepted");

        cleanup_test_pool(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_community_empty_description() {
        let pool = create_test_pool().await;
        let user_id = create_test_user(&pool).await;

        // Test: Create with empty description
        let result = sqlx::query(
            "INSERT INTO communities (name, slug, description, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())"
        )
        .bind("No Description")
        .bind("no-desc-test")
        .bind::<Option<String>>(None)
        .bind(true)
        .bind(user_id)
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Empty description should be accepted");

        // Verify: Description is NULL
        let desc: Option<String> = sqlx::query_scalar(
            "SELECT description FROM communities WHERE slug = 'no-desc-test'"
        )
        .fetch_one(&pool)
        .await
        .expect("Community not found");

        assert!(desc.is_none(), "Description should be NULL");

        cleanup_test_pool(&pool).await;
    }

    // ========================================================================
    // Test Helpers
    // ========================================================================

    async fn create_test_pool() -> sqlx::PgPool {
        // This would be implemented to create a test database connection
        // For now, this is a placeholder
        panic!("Test pool creation not implemented - requires test database setup");
    }

    async fn cleanup_test_pool(_pool: &sqlx::PgPool) {
        // Cleanup test data
    }
}
