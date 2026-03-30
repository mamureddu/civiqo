/// Integration tests for Posts, Comments, and Reactions endpoints
/// Run with: cargo test -p server --test posts_integration

use uuid::Uuid;
use shared::database::Database;

#[cfg(test)]
mod posts_integration_tests {
    use super::*;

    async fn setup_test_db() -> Database {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for integration tests");
        Database::connect(&database_url).await.expect("Failed to connect to test database")
    }

    async fn create_test_user(db: &Database) -> Uuid {
        let user_id = Uuid::new_v4();
        let email = format!("test-{}@example.com", user_id);
        let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$dummy$dummyhash";

        sqlx::query!(
            "INSERT INTO users (id, email, password_hash, provider, created_at, updated_at)
             VALUES ($1, $2, $3, 'local', NOW(), NOW()) ON CONFLICT (id) DO NOTHING",
            user_id, email, password_hash
        ).execute(&db.pool).await.expect("Failed to create test user");

        user_id
    }

    async fn ensure_roles_exist(db: &Database) {
        // Create roles if they don't exist
        let _ = sqlx::query("INSERT INTO roles (name, description) VALUES ('owner', 'Community owner') ON CONFLICT (name) DO NOTHING")
            .execute(&db.pool).await;
        let _ = sqlx::query("INSERT INTO roles (name, description) VALUES ('admin', 'Community admin') ON CONFLICT (name) DO NOTHING")
            .execute(&db.pool).await;
        let _ = sqlx::query("INSERT INTO roles (name, description) VALUES ('member', 'Community member') ON CONFLICT (name) DO NOTHING")
            .execute(&db.pool).await;
    }

    async fn create_test_community(db: &Database, creator_id: Uuid) -> Uuid {
        ensure_roles_exist(db).await;
        
        let community_id = Uuid::now_v7();
        let slug = format!("test-community-{}", Uuid::new_v4());
        
        sqlx::query!(
            "INSERT INTO communities (id, name, description, slug, is_public, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            community_id, "Test Community", Some("Test description"), slug, true, creator_id
        ).execute(&db.pool).await.expect("Failed to create test community");

        // Add creator as member with owner role
        let owner_role_id: Option<i64> = sqlx::query_scalar("SELECT id FROM roles WHERE name = 'owner' LIMIT 1")
            .fetch_optional(&db.pool).await.expect("Failed to query roles").flatten();
        
        if let Some(role_id) = owner_role_id {
            let _ = sqlx::query!(
                "INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
                 VALUES ($1, $2, $3, 'active', NOW()) ON CONFLICT DO NOTHING",
                creator_id, community_id, role_id
            ).execute(&db.pool).await;
        }

        community_id
    }

    async fn create_test_post(db: &Database, community_id: Uuid, author_id: Uuid) -> Uuid {
        let post_id = Uuid::now_v7();
        
        sqlx::query!(
            "INSERT INTO posts (id, community_id, author_id, title, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            post_id, community_id, author_id, "Test Post", "Test content"
        ).execute(&db.pool).await.expect("Failed to create test post");

        post_id
    }

    async fn cleanup(db: &Database, user_id: Uuid, community_id: Uuid) {
        let _ = sqlx::query!("DELETE FROM communities WHERE id = $1", community_id).execute(&db.pool).await;
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id).execute(&db.pool).await;
    }

    // ========================================================================
    // Posts Tests
    // ========================================================================

    #[tokio::test]
    async fn test_create_post_in_database() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        
        let post_id = Uuid::now_v7();
        let result = sqlx::query!(
            "INSERT INTO posts (id, community_id, author_id, title, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING id",
            post_id, community_id, user_id, "My First Post", "This is the content"
        ).fetch_one(&db.pool).await;

        assert!(result.is_ok());
        
        // Verify post exists
        let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) as count FROM posts WHERE id = $1", post_id)
            .fetch_one(&db.pool).await.unwrap().unwrap_or(0);
        assert_eq!(count, 1);

        cleanup(&db, user_id, community_id).await;
    }

    #[tokio::test]
    async fn test_list_posts_in_community() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        
        // Create 3 posts
        for i in 0..3 {
            let post_id = Uuid::now_v7();
            sqlx::query!(
                "INSERT INTO posts (id, community_id, author_id, title, content, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
                post_id, community_id, user_id, format!("Post {}", i), format!("Content {}", i)
            ).execute(&db.pool).await.unwrap();
        }

        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM posts WHERE community_id = $1", community_id
        ).fetch_one(&db.pool).await.unwrap().unwrap_or(0);
        
        assert_eq!(count, 3);

        cleanup(&db, user_id, community_id).await;
    }

    #[tokio::test]
    async fn test_update_post() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        // Update post
        sqlx::query!(
            "UPDATE posts SET title = $1, content = $2, updated_at = NOW() WHERE id = $3",
            "Updated Title", "Updated Content", post_id
        ).execute(&db.pool).await.unwrap();

        // Verify update
        let title: String = sqlx::query_scalar!("SELECT title FROM posts WHERE id = $1", post_id)
            .fetch_one(&db.pool).await.unwrap();
        
        assert_eq!(title, "Updated Title");

        cleanup(&db, user_id, community_id).await;
    }

    #[tokio::test]
    async fn test_delete_post_cascades() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        // Add a comment
        let comment_id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO comments (id, post_id, author_id, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            comment_id, post_id, user_id, "Test comment"
        ).execute(&db.pool).await.unwrap();

        // Add a reaction
        sqlx::query!(
            "INSERT INTO reactions (post_id, user_id, reaction_type, created_at)
             VALUES ($1, $2, $3, NOW())",
            post_id, user_id, "like"
        ).execute(&db.pool).await.unwrap();

        // Delete post
        sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
            .execute(&db.pool).await.unwrap();

        // Verify cascade - comments deleted
        let comment_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM comments WHERE post_id = $1", post_id
        ).fetch_one(&db.pool).await.unwrap().unwrap_or(0);
        assert_eq!(comment_count, 0);

        // Verify cascade - reactions deleted
        let reaction_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM reactions WHERE post_id = $1", post_id
        ).fetch_one(&db.pool).await.unwrap().unwrap_or(0);
        assert_eq!(reaction_count, 0);

        cleanup(&db, user_id, community_id).await;
    }

    // ========================================================================
    // Comments Tests
    // ========================================================================

    #[tokio::test]
    async fn test_create_comment() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        let comment_id = Uuid::now_v7();
        let result = sqlx::query!(
            "INSERT INTO comments (id, post_id, author_id, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW()) RETURNING id",
            comment_id, post_id, user_id, "This is a comment"
        ).fetch_one(&db.pool).await;

        assert!(result.is_ok());

        cleanup(&db, user_id, community_id).await;
    }

    #[tokio::test]
    async fn test_comment_threading() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        // Create parent comment
        let parent_id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO comments (id, post_id, author_id, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            parent_id, post_id, user_id, "Parent comment"
        ).execute(&db.pool).await.unwrap();

        // Create reply
        let reply_id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO comments (id, post_id, author_id, parent_id, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            reply_id, post_id, user_id, parent_id, "Reply to parent"
        ).execute(&db.pool).await.unwrap();

        // Verify threading
        let reply_parent: Option<Uuid> = sqlx::query_scalar!(
            "SELECT parent_id FROM comments WHERE id = $1", reply_id
        ).fetch_one(&db.pool).await.unwrap();
        
        assert_eq!(reply_parent, Some(parent_id));

        cleanup(&db, user_id, community_id).await;
    }

    #[tokio::test]
    async fn test_update_comment_sets_edited_flag() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        let comment_id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO comments (id, post_id, author_id, content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            comment_id, post_id, user_id, "Original content"
        ).execute(&db.pool).await.unwrap();

        // Update comment
        sqlx::query!(
            "UPDATE comments SET content = $1, is_edited = true, updated_at = NOW() WHERE id = $2",
            "Edited content", comment_id
        ).execute(&db.pool).await.unwrap();

        // Verify is_edited flag
        let is_edited: Option<bool> = sqlx::query_scalar!(
            "SELECT is_edited FROM comments WHERE id = $1", comment_id
        ).fetch_one(&db.pool).await.unwrap();
        
        assert_eq!(is_edited, Some(true));

        cleanup(&db, user_id, community_id).await;
    }

    // ========================================================================
    // Reactions Tests
    // ========================================================================

    #[tokio::test]
    async fn test_add_reaction() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        let result = sqlx::query!(
            "INSERT INTO reactions (post_id, user_id, reaction_type, created_at)
             VALUES ($1, $2, $3, NOW()) RETURNING id",
            post_id, user_id, "like"
        ).fetch_one(&db.pool).await;

        assert!(result.is_ok());

        cleanup(&db, user_id, community_id).await;
    }

    #[tokio::test]
    async fn test_reaction_unique_constraint() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        // First reaction
        sqlx::query!(
            "INSERT INTO reactions (post_id, user_id, reaction_type, created_at)
             VALUES ($1, $2, $3, NOW())",
            post_id, user_id, "like"
        ).execute(&db.pool).await.unwrap();

        // Try to add duplicate - should fail or update
        let result = sqlx::query!(
            "INSERT INTO reactions (post_id, user_id, reaction_type, created_at)
             VALUES ($1, $2, $3, NOW())
             ON CONFLICT (post_id, user_id) DO UPDATE SET reaction_type = $3",
            post_id, user_id, "heart"
        ).execute(&db.pool).await;

        assert!(result.is_ok());

        // Verify only one reaction exists
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM reactions WHERE post_id = $1 AND user_id = $2",
            post_id, user_id
        ).fetch_one(&db.pool).await.unwrap().unwrap_or(0);
        
        assert_eq!(count, 1);

        // Verify reaction type was updated
        let reaction_type: String = sqlx::query_scalar!(
            "SELECT reaction_type FROM reactions WHERE post_id = $1 AND user_id = $2",
            post_id, user_id
        ).fetch_one(&db.pool).await.unwrap();
        
        assert_eq!(reaction_type, "heart");

        cleanup(&db, user_id, community_id).await;
    }

    #[tokio::test]
    async fn test_reaction_counts() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        // Create second user
        let user2_id = create_test_user(&db).await;

        // Add reactions from both users
        sqlx::query!(
            "INSERT INTO reactions (post_id, user_id, reaction_type, created_at)
             VALUES ($1, $2, $3, NOW())",
            post_id, user_id, "like"
        ).execute(&db.pool).await.unwrap();

        sqlx::query!(
            "INSERT INTO reactions (post_id, user_id, reaction_type, created_at)
             VALUES ($1, $2, $3, NOW())",
            post_id, user2_id, "like"
        ).execute(&db.pool).await.unwrap();

        // Count reactions
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM reactions WHERE post_id = $1",
            post_id
        ).fetch_one(&db.pool).await.unwrap().unwrap_or(0);
        
        assert_eq!(count, 2);

        // Cleanup
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user2_id).execute(&db.pool).await;
        cleanup(&db, user_id, community_id).await;
    }

    #[tokio::test]
    async fn test_remove_reaction() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        // Add reaction
        sqlx::query!(
            "INSERT INTO reactions (post_id, user_id, reaction_type, created_at)
             VALUES ($1, $2, $3, NOW())",
            post_id, user_id, "like"
        ).execute(&db.pool).await.unwrap();

        // Remove reaction
        sqlx::query!(
            "DELETE FROM reactions WHERE post_id = $1 AND user_id = $2",
            post_id, user_id
        ).execute(&db.pool).await.unwrap();

        // Verify removed
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM reactions WHERE post_id = $1 AND user_id = $2",
            post_id, user_id
        ).fetch_one(&db.pool).await.unwrap().unwrap_or(0);
        
        assert_eq!(count, 0);

        cleanup(&db, user_id, community_id).await;
    }

    // ========================================================================
    // View Count Tests
    // ========================================================================

    #[tokio::test]
    async fn test_increment_view_count() {
        let db = setup_test_db().await;
        let user_id = create_test_user(&db).await;
        let community_id = create_test_community(&db, user_id).await;
        let post_id = create_test_post(&db, community_id, user_id).await;

        // Initial view count should be 0
        let initial_count: Option<i64> = sqlx::query_scalar!(
            "SELECT view_count FROM posts WHERE id = $1", post_id
        ).fetch_one(&db.pool).await.unwrap();
        assert_eq!(initial_count.unwrap_or(0), 0);

        // Increment view count
        sqlx::query!("UPDATE posts SET view_count = view_count + 1 WHERE id = $1", post_id)
            .execute(&db.pool).await.unwrap();

        // Verify incremented
        let new_count: Option<i64> = sqlx::query_scalar!(
            "SELECT view_count FROM posts WHERE id = $1", post_id
        ).fetch_one(&db.pool).await.unwrap();
        assert_eq!(new_count.unwrap_or(0), 1);

        cleanup(&db, user_id, community_id).await;
    }
}
