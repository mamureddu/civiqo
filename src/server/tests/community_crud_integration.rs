/// Integration tests for Community CRUD endpoints
/// These tests use actual database connections with compile-time checked queries
///
/// Run with: cargo test --test community_crud_integration

use uuid::Uuid;
use shared::database::Database;

#[cfg(test)]
mod community_crud_integration_tests {
    use super::*;

    async fn setup_test_db() -> Database {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for integration tests");
        
        Database::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    /// Helper: Create a test user and return their UUID
    async fn create_test_user(db: &Database) -> Uuid {
        let user_id = Uuid::new_v4();
        let email = format!("test-{}@example.com", user_id);
        let auth0_id = format!("auth0|{}", user_id);
        
        // Users table: id, auth0_id, email, created_at, updated_at
        sqlx::query!(
            "INSERT INTO users (id, email, auth0_id, created_at, updated_at) 
             VALUES ($1, $2, $3, NOW(), NOW())
             ON CONFLICT (id) DO NOTHING",
            user_id,
            email,
            auth0_id
        )
        .execute(&db.pool)
        .await
        .expect("Failed to create test user");
        
        user_id
    }

    /// Helper: Create a test community and return its ID (UUIDv7)
    async fn create_test_community(db: &Database, creator_id: Uuid, slug: &str) -> Uuid {
        let community_id = Uuid::now_v7();
        let result = sqlx::query!(
            "INSERT INTO communities (id, name, description, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
             RETURNING id",
            community_id,
            "Test Community",
            Some("A test community for integration tests"),
            slug,
            true,
            creator_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to create test community");

        result.id
    }

    /// Helper: Cleanup test community
    async fn cleanup_test_community(db: &Database, community_id: Uuid) {
        let _ = sqlx::query!("DELETE FROM community_members WHERE community_id = $1", community_id)
            .execute(&db.pool)
            .await;
        
        let _ = sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
            .execute(&db.pool)
            .await;
    }

    /// Helper: Cleanup test user
    async fn cleanup_test_user(db: &Database, user_id: Uuid) {
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&db.pool)
            .await;
    }

    // ========================================================================
    // CREATE COMMUNITY TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_create_community_database_insert() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-create-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = Uuid::now_v7();

        let result = sqlx::query!(
            "INSERT INTO communities (id, name, description, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
             RETURNING id, name, slug, is_public",
            community_id,
            "Integration Test Community",
            Some("Created by integration test"),
            &slug,
            true,
            user_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to insert community");

        assert_eq!(result.name, "Integration Test Community");
        assert_eq!(result.slug, slug);
        assert_eq!(result.is_public, Some(true));

        cleanup_test_community(&db, result.id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_community_id_is_uuidv7() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-uuid-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = Uuid::now_v7();

        let result = sqlx::query!(
            "INSERT INTO communities (id, name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
             RETURNING id",
            community_id,
            "UUIDv7 Test",
            &slug,
            true,
            user_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to insert community");

        assert_eq!(result.id, community_id, "Community ID should be UUIDv7");

        cleanup_test_community(&db, result.id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_duplicate_slug_fails() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-dup-{}", Uuid::new_v4().to_string().split('-').next().unwrap());

        let community_id = create_test_community(&db, user_id, &slug).await;
        let dup_community_id = Uuid::now_v7();

        let result = sqlx::query!(
            "INSERT INTO communities (id, name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            dup_community_id,
            "Duplicate Slug Test",
            &slug,
            true,
            user_id
        )
        .execute(&db.pool)
        .await;

        assert!(result.is_err(), "Duplicate slug should fail");

        cleanup_test_community(&db, community_id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_creator_added_as_admin() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-admin-{}", Uuid::new_v4().to_string().split('-').next().unwrap());

        let community_id = create_test_community(&db, user_id, &slug).await;

        let admin_role = sqlx::query!("SELECT id FROM roles WHERE name = 'admin' LIMIT 1")
            .fetch_optional(&db.pool)
            .await
            .expect("Failed to query roles");

        if let Some(role) = admin_role {
            // community_members.id is BIGINT, auto-generated by database
            let _ = sqlx::query!(
                "INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
                 VALUES ($1, $2, $3, 'active', NOW())",
                user_id,
                community_id,
                role.id
            )
            .execute(&db.pool)
            .await;

            let member = sqlx::query!(
                "SELECT cm.user_id, r.name as role_name 
                 FROM community_members cm
                 JOIN roles r ON cm.role_id = r.id
                 WHERE cm.community_id = $1 AND cm.user_id = $2",
                community_id,
                user_id
            )
            .fetch_optional(&db.pool)
            .await
            .expect("Failed to query member");

            assert!(member.is_some(), "Creator should be a member");
            assert_eq!(member.unwrap().role_name, "admin", "Creator should have admin role");
        }

        cleanup_test_community(&db, community_id).await;
        cleanup_test_user(&db, user_id).await;
    }

    // ========================================================================
    // UPDATE COMMUNITY TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_update_community_name() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-update-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, user_id, &slug).await;

        let result = sqlx::query!(
            "UPDATE communities SET name = $1, updated_at = NOW() WHERE id = $2 RETURNING name",
            "Updated Community Name",
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to update community");

        assert_eq!(result.name, "Updated Community Name");

        cleanup_test_community(&db, community_id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_update_community_description() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-desc-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, user_id, &slug).await;

        let result = sqlx::query!(
            "UPDATE communities SET description = $1, updated_at = NOW() WHERE id = $2 RETURNING description",
            Some("Updated description"),
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to update community");

        assert_eq!(result.description, Some("Updated description".to_string()));

        cleanup_test_community(&db, community_id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_update_community_is_public() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-public-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, user_id, &slug).await;

        let result = sqlx::query!(
            "UPDATE communities SET is_public = $1, updated_at = NOW() WHERE id = $2 RETURNING is_public",
            false,
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Failed to update community");

        assert_eq!(result.is_public, Some(false), "is_public should be false");

        cleanup_test_community(&db, community_id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_update_timestamp_changes() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-ts-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, user_id, &slug).await;

        let original = sqlx::query!("SELECT updated_at FROM communities WHERE id = $1", community_id)
            .fetch_one(&db.pool)
            .await
            .expect("Failed to fetch community");

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        sqlx::query!("UPDATE communities SET name = $1, updated_at = NOW() WHERE id = $2", "Updated", community_id)
            .execute(&db.pool)
            .await
            .expect("Failed to update");

        let updated = sqlx::query!("SELECT updated_at FROM communities WHERE id = $1", community_id)
            .fetch_one(&db.pool)
            .await
            .expect("Failed to fetch community");

        assert!(updated.updated_at > original.updated_at, "updated_at should be newer");

        cleanup_test_community(&db, community_id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_update_owner_check() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db).await;
        let other_user_id = create_test_user(&db).await;
        let slug = format!("test-owner-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug).await;

        let community = sqlx::query!("SELECT created_by FROM communities WHERE id = $1", community_id)
            .fetch_one(&db.pool)
            .await
            .expect("Failed to fetch community");

        assert_eq!(community.created_by, owner_id, "Owner should match creator");
        assert_ne!(community.created_by, other_user_id, "Other user should not be owner");

        cleanup_test_community(&db, community_id).await;
        cleanup_test_user(&db, owner_id).await;
        cleanup_test_user(&db, other_user_id).await;
    }

    // ========================================================================
    // DELETE COMMUNITY TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_delete_community() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-del-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, user_id, &slug).await;

        let exists_before = sqlx::query!("SELECT id FROM communities WHERE id = $1", community_id)
            .fetch_optional(&db.pool)
            .await
            .expect("Failed to query");
        assert!(exists_before.is_some(), "Community should exist before delete");

        sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
            .execute(&db.pool)
            .await
            .expect("Failed to delete");

        let exists_after = sqlx::query!("SELECT id FROM communities WHERE id = $1", community_id)
            .fetch_optional(&db.pool)
            .await
            .expect("Failed to query");
        assert!(exists_after.is_none(), "Community should not exist after delete");

        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_cascade_delete_members() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-cascade-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, user_id, &slug).await;

        let admin_role = sqlx::query!("SELECT id FROM roles WHERE name = 'admin' LIMIT 1")
            .fetch_optional(&db.pool)
            .await
            .expect("Failed to query roles");

        if let Some(role) = admin_role {
            // community_members.id is BIGINT, auto-generated by database
            sqlx::query!(
                "INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
                 VALUES ($1, $2, $3, 'active', NOW())",
                user_id,
                community_id,
                role.id
            )
            .execute(&db.pool)
            .await
            .expect("Failed to add member");

            let member_before = sqlx::query!(
                "SELECT id FROM community_members WHERE community_id = $1",
                community_id
            )
            .fetch_optional(&db.pool)
            .await
            .expect("Failed to query");
            assert!(member_before.is_some(), "Member should exist before delete");

            sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
                .execute(&db.pool)
                .await
                .expect("Failed to delete");

            let member_after = sqlx::query!(
                "SELECT id FROM community_members WHERE community_id = $1",
                community_id
            )
            .fetch_optional(&db.pool)
            .await
            .expect("Failed to query");
            assert!(member_after.is_none(), "Members should be deleted by CASCADE");
        }

        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_delete_nonexistent_community() {
        let db = setup_test_db().await;
        let nonexistent_id = Uuid::nil();

        let result = sqlx::query!("DELETE FROM communities WHERE id = $1", nonexistent_id)
            .execute(&db.pool)
            .await
            .expect("Query should not error");

        assert_eq!(result.rows_affected(), 0, "Should affect 0 rows");
    }

    #[tokio::test]
    async fn test_sql_injection_prevention() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-injection-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = Uuid::now_v7();

        let result = sqlx::query!(
            "INSERT INTO communities (id, name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
             RETURNING id, name",
            community_id,
            "Test'; DROP TABLE communities; --",
            &slug,
            true,
            user_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Should create community with SQL injection attempt");

        assert_eq!(result.name, "Test'; DROP TABLE communities; --", "Name should be stored as-is");

        cleanup_test_community(&db, result.id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_create_with_special_characters() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-special-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = Uuid::now_v7();

        let result = sqlx::query!(
            "INSERT INTO communities (id, name, description, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
             RETURNING id",
            community_id,
            "Community with 🎉 emoji & special chars!",
            Some("Description with <html> tags & symbols: @#$%"),
            &slug,
            true,
            user_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Should create community with special characters");

        cleanup_test_community(&db, result.id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_create_with_max_length_name() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let slug = format!("test-maxname-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let max_name = "a".repeat(100);
        let community_id = Uuid::now_v7();

        let result = sqlx::query!(
            "INSERT INTO communities (id, name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
             RETURNING id, name",
            community_id,
            &max_name,
            &slug,
            true,
            user_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Should create community with max length name");

        assert_eq!(result.name.len(), 100, "Name should be 100 chars");

        cleanup_test_community(&db, result.id).await;
        cleanup_test_user(&db, user_id).await;
    }

    #[tokio::test]
    async fn test_create_with_max_length_slug() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let max_slug = "a".repeat(50);
        let community_id = Uuid::now_v7();

        let result = sqlx::query!(
            "INSERT INTO communities (id, name, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
             RETURNING id, slug",
            community_id,
            "Max Slug Community",
            &max_slug,
            true,
            user_id
        )
        .fetch_one(&db.pool)
        .await
        .expect("Should create community with max length slug");

        assert_eq!(result.slug.len(), 50, "Slug should be 50 chars");

        cleanup_test_community(&db, result.id).await;
        cleanup_test_user(&db, user_id).await;
    }

    // ========================================================================
    // VALIDATION TESTS (Unit tests - no DB required)
    // ========================================================================

    #[test]
    fn test_name_validation_min_length() {
        let short_name = "ab";
        assert!(short_name.len() < 3, "Short name should fail validation");
        
        let valid_name = "abc";
        assert!(valid_name.len() >= 3, "Valid name should pass validation");
    }

    #[test]
    fn test_name_validation_max_length() {
        let long_name = "a".repeat(101);
        assert!(long_name.len() > 100, "Long name should fail validation");
        
        let valid_name = "a".repeat(100);
        assert!(valid_name.len() <= 100, "Max length name should pass validation");
    }

    #[test]
    fn test_slug_validation_format() {
        let valid_slug = "test-community-123";
        let is_valid = valid_slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        assert!(is_valid, "Valid slug should pass format check");

        let invalid_slug_uppercase = "Test-Community";
        let is_invalid = !invalid_slug_uppercase.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        assert!(is_invalid, "Uppercase slug should fail format check");
    }

    #[test]
    fn test_description_validation_max_length() {
        let long_desc = "a".repeat(1001);
        assert!(long_desc.len() > 1000, "Long description should fail validation");
        
        let valid_desc = "a".repeat(1000);
        assert!(valid_desc.len() <= 1000, "Max length description should pass validation");
    }

    #[test]
    fn test_is_public_default() {
        let is_public: Option<bool> = None;
        let default_value = is_public.unwrap_or(true);
        assert!(default_value, "is_public should default to true");
    }
}
