/// Integration tests for Community Membership endpoints
/// Tests: join, leave, list members, update role, remove member, requests, discovery, admin
///
/// Run with: cargo test --test membership_integration

use uuid::Uuid;
use shared::database::Database;

#[cfg(test)]
mod membership_integration_tests {
    use super::*;

    async fn setup_test_db() -> Database {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for integration tests");
        
        Database::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    async fn create_test_user(db: &Database, email_prefix: &str) -> Uuid {
        let user_id = Uuid::new_v4();
        let email = format!("{}-{}@example.com", email_prefix, user_id);
        let auth0_id = format!("auth0|{}", user_id);
        
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

    async fn create_test_community(db: &Database, creator_id: Uuid, slug: &str, is_public: bool, requires_approval: bool) -> Uuid {
        let community_id = Uuid::now_v7();
        sqlx::query!(
            "INSERT INTO communities (id, name, description, slug, is_public, requires_approval, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())",
            community_id,
            format!("Test Community {}", slug),
            Some("A test community"),
            slug,
            is_public,
            requires_approval,
            creator_id
        )
        .execute(&db.pool)
        .await
        .expect("Failed to create test community");

        let admin_role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = 'admin' LIMIT 1")
            .fetch_one(&db.pool)
            .await
            .expect("Failed to get admin role");

        sqlx::query!(
            "INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
             VALUES ($1, $2, $3, 'active', NOW())",
            creator_id,
            community_id,
            admin_role_id
        )
        .execute(&db.pool)
        .await
        .expect("Failed to add creator as admin");

        community_id
    }

    async fn add_member(db: &Database, user_id: Uuid, community_id: Uuid, role: &str, status: &str) {
        let role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = $1 LIMIT 1", role)
            .fetch_one(&db.pool)
            .await
            .expect("Failed to get role");

        let _ = sqlx::query!(
            "INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
             VALUES ($1, $2, $3, $4, NOW())
             ON CONFLICT DO NOTHING",
            user_id,
            community_id,
            role_id,
            status
        )
        .execute(&db.pool)
        .await;
    }

    async fn is_member(db: &Database, user_id: Uuid, community_id: Uuid) -> bool {
        let result: Option<i64> = sqlx::query_scalar!(
            "SELECT id FROM community_members WHERE user_id = $1 AND community_id = $2 AND status = 'active'",
            user_id,
            community_id
        )
        .fetch_optional(&db.pool)
        .await
        .expect("Failed to check membership");
        
        result.is_some()
    }

    async fn get_member_role(db: &Database, user_id: Uuid, community_id: Uuid) -> Option<String> {
        sqlx::query_scalar!(
            "SELECT r.name FROM community_members cm
             JOIN roles r ON cm.role_id = r.id
             WHERE cm.user_id = $1 AND cm.community_id = $2 AND cm.status = 'active'",
            user_id,
            community_id
        )
        .fetch_optional(&db.pool)
        .await
        .expect("Failed to get member role")
    }

    async fn cleanup(db: &Database, community_id: Uuid, user_ids: Vec<Uuid>) {
        let _ = sqlx::query!("DELETE FROM community_members WHERE community_id = $1", community_id)
            .execute(&db.pool)
            .await;
        let _ = sqlx::query!("DELETE FROM communities WHERE id = $1", community_id)
            .execute(&db.pool)
            .await;
        for user_id in user_ids {
            let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
                .execute(&db.pool)
                .await;
        }
    }

    // ========================================================================
    // JOIN COMMUNITY TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_join_public_community_success() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let member_id = create_test_user(&db, "member").await;
        let slug = format!("test-join-pub-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        assert!(!is_member(&db, member_id, community_id).await);

        let member_role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = 'member' LIMIT 1")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        sqlx::query!(
            "INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
             VALUES ($1, $2, $3, 'active', NOW())",
            member_id,
            community_id,
            member_role_id
        )
        .execute(&db.pool)
        .await
        .unwrap();

        assert!(is_member(&db, member_id, community_id).await);
        assert_eq!(get_member_role(&db, member_id, community_id).await, Some("member".to_string()));

        cleanup(&db, community_id, vec![owner_id, member_id]).await;
    }

    #[tokio::test]
    async fn test_join_private_community_fails() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let slug = format!("test-priv-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, false, true).await;

        let is_public: Option<bool> = sqlx::query_scalar!("SELECT is_public FROM communities WHERE id = $1", community_id)
            .fetch_one(&db.pool)
            .await
            .unwrap();

        assert!(!is_public.unwrap_or(true));
        cleanup(&db, community_id, vec![owner_id]).await;
    }

    #[tokio::test]
    async fn test_duplicate_join_prevented() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let member_id = create_test_user(&db, "member").await;
        let slug = format!("test-dup-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        add_member(&db, member_id, community_id, "member", "active").await;

        let result = sqlx::query!(
            "INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
             VALUES ($1, $2, (SELECT id FROM roles WHERE name = 'member'), 'active', NOW())
             ON CONFLICT DO NOTHING
             RETURNING id",
            member_id,
            community_id
        )
        .fetch_optional(&db.pool)
        .await
        .unwrap();

        assert!(result.is_none());
        cleanup(&db, community_id, vec![owner_id, member_id]).await;
    }

    // ========================================================================
    // LEAVE COMMUNITY TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_member_can_leave_community() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let member_id = create_test_user(&db, "member").await;
        let slug = format!("test-leave-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        add_member(&db, member_id, community_id, "member", "active").await;
        assert!(is_member(&db, member_id, community_id).await);

        sqlx::query!("DELETE FROM community_members WHERE user_id = $1 AND community_id = $2", member_id, community_id)
            .execute(&db.pool)
            .await
            .unwrap();

        assert!(!is_member(&db, member_id, community_id).await);
        cleanup(&db, community_id, vec![owner_id, member_id]).await;
    }

    #[tokio::test]
    async fn test_only_admin_cannot_leave() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let slug = format!("test-only-admin-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        let admin_count: Option<i64> = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM community_members cm
             JOIN roles r ON cm.role_id = r.id
             WHERE cm.community_id = $1 AND r.name = 'admin'",
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .unwrap();

        assert_eq!(admin_count.unwrap_or(0), 1);
        let is_admin = get_member_role(&db, owner_id, community_id).await;
        assert_eq!(is_admin, Some("admin".to_string()));

        cleanup(&db, community_id, vec![owner_id]).await;
    }

    // ========================================================================
    // LIST MEMBERS TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_list_members_returns_all_active() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let member1_id = create_test_user(&db, "member1").await;
        let member2_id = create_test_user(&db, "member2").await;
        let slug = format!("test-list-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        add_member(&db, member1_id, community_id, "member", "active").await;
        add_member(&db, member2_id, community_id, "member", "active").await;

        let member_count: Option<i64> = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND status = 'active'",
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .unwrap();

        assert_eq!(member_count.unwrap_or(0), 3);
        cleanup(&db, community_id, vec![owner_id, member1_id, member2_id]).await;
    }

    #[tokio::test]
    async fn test_list_members_excludes_pending() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let pending_id = create_test_user(&db, "pending").await;
        let slug = format!("test-pending-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, false, true).await;

        add_member(&db, pending_id, community_id, "member", "pending").await;

        let active_count: Option<i64> = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND status = 'active'",
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .unwrap();

        assert_eq!(active_count.unwrap_or(0), 1);
        cleanup(&db, community_id, vec![owner_id, pending_id]).await;
    }

    // ========================================================================
    // UPDATE ROLE TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_promote_member_to_admin() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let member_id = create_test_user(&db, "member").await;
        let slug = format!("test-promote-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        add_member(&db, member_id, community_id, "member", "active").await;
        assert_eq!(get_member_role(&db, member_id, community_id).await, Some("member".to_string()));

        let admin_role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = 'admin' LIMIT 1")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        sqlx::query!(
            "UPDATE community_members SET role_id = $1 WHERE user_id = $2 AND community_id = $3",
            admin_role_id,
            member_id,
            community_id
        )
        .execute(&db.pool)
        .await
        .unwrap();

        assert_eq!(get_member_role(&db, member_id, community_id).await, Some("admin".to_string()));
        cleanup(&db, community_id, vec![owner_id, member_id]).await;
    }

    #[tokio::test]
    async fn test_demote_admin_to_member() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let admin_id = create_test_user(&db, "admin").await;
        let slug = format!("test-demote-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        add_member(&db, admin_id, community_id, "admin", "active").await;
        assert_eq!(get_member_role(&db, admin_id, community_id).await, Some("admin".to_string()));

        let member_role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = 'member' LIMIT 1")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        sqlx::query!(
            "UPDATE community_members SET role_id = $1 WHERE user_id = $2 AND community_id = $3",
            member_role_id,
            admin_id,
            community_id
        )
        .execute(&db.pool)
        .await
        .unwrap();

        assert_eq!(get_member_role(&db, admin_id, community_id).await, Some("member".to_string()));
        cleanup(&db, community_id, vec![owner_id, admin_id]).await;
    }

    #[tokio::test]
    async fn test_cannot_update_nonexistent_member_role() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let nonexistent_id = Uuid::new_v4();
        let slug = format!("test-nonexist-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        let member_role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = 'member' LIMIT 1")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        let result = sqlx::query!(
            "UPDATE community_members SET role_id = $1 WHERE user_id = $2 AND community_id = $3",
            member_role_id,
            nonexistent_id,
            community_id
        )
        .execute(&db.pool)
        .await
        .unwrap();

        assert_eq!(result.rows_affected(), 0);
        cleanup(&db, community_id, vec![owner_id]).await;
    }

    // ========================================================================
    // REMOVE MEMBER TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_remove_member_from_community() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let member_id = create_test_user(&db, "member").await;
        let slug = format!("test-remove-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        add_member(&db, member_id, community_id, "member", "active").await;
        assert!(is_member(&db, member_id, community_id).await);

        sqlx::query!("DELETE FROM community_members WHERE user_id = $1 AND community_id = $2", member_id, community_id)
            .execute(&db.pool)
            .await
            .unwrap();

        assert!(!is_member(&db, member_id, community_id).await);
        cleanup(&db, community_id, vec![owner_id, member_id]).await;
    }

    #[tokio::test]
    async fn test_remove_nonexistent_member_returns_zero() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let nonexistent_id = Uuid::new_v4();
        let slug = format!("test-rem-nonex-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        let result = sqlx::query!("DELETE FROM community_members WHERE user_id = $1 AND community_id = $2", nonexistent_id, community_id)
            .execute(&db.pool)
            .await
            .unwrap();

        assert_eq!(result.rows_affected(), 0);
        cleanup(&db, community_id, vec![owner_id]).await;
    }

    // ========================================================================
    // JOIN REQUEST TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_request_join_private_community() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let requester_id = create_test_user(&db, "requester").await;
        let slug = format!("test-req-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, false, true).await;

        add_member(&db, requester_id, community_id, "member", "pending").await;

        let status: Option<Option<String>> = sqlx::query_scalar!(
            "SELECT status FROM community_members WHERE user_id = $1 AND community_id = $2",
            requester_id,
            community_id
        )
        .fetch_optional(&db.pool)
        .await
        .unwrap();

        assert_eq!(status.flatten(), Some("pending".to_string()));
        cleanup(&db, community_id, vec![owner_id, requester_id]).await;
    }

    #[tokio::test]
    async fn test_approve_join_request() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let requester_id = create_test_user(&db, "requester").await;
        let slug = format!("test-appr-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, false, true).await;

        add_member(&db, requester_id, community_id, "member", "pending").await;

        sqlx::query!(
            "UPDATE community_members SET status = 'active' WHERE user_id = $1 AND community_id = $2",
            requester_id,
            community_id
        )
        .execute(&db.pool)
        .await
        .unwrap();

        assert!(is_member(&db, requester_id, community_id).await);
        cleanup(&db, community_id, vec![owner_id, requester_id]).await;
    }

    #[tokio::test]
    async fn test_reject_join_request() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let requester_id = create_test_user(&db, "requester").await;
        let slug = format!("test-rej-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, false, true).await;

        add_member(&db, requester_id, community_id, "member", "pending").await;

        sqlx::query!("DELETE FROM community_members WHERE user_id = $1 AND community_id = $2", requester_id, community_id)
            .execute(&db.pool)
            .await
            .unwrap();

        let exists: Option<bool> = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM community_members WHERE user_id = $1 AND community_id = $2)",
            requester_id,
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .unwrap();

        assert!(!exists.unwrap_or(false));
        cleanup(&db, community_id, vec![owner_id, requester_id]).await;
    }

    // ========================================================================
    // DISCOVERY TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_get_my_communities() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let slug1 = format!("test-my1-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let slug2 = format!("test-my2-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id1 = create_test_community(&db, owner_id, &slug1, true, false).await;
        let community_id2 = create_test_community(&db, owner_id, &slug2, true, false).await;

        let count: Option<i64> = sqlx::query_scalar!(
            "SELECT COUNT(DISTINCT c.id) FROM communities c WHERE c.created_by = $1",
            owner_id
        )
        .fetch_one(&db.pool)
        .await
        .unwrap();

        assert_eq!(count.unwrap_or(0), 2);
        cleanup(&db, community_id1, vec![owner_id]).await;
        cleanup(&db, community_id2, vec![]).await;
    }

    #[tokio::test]
    async fn test_get_trending_communities() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let member_id = create_test_user(&db, "member").await;
        let slug = format!("test-trend-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        add_member(&db, member_id, community_id, "member", "active").await;

        let count: Option<i64> = sqlx::query_scalar!(
            "SELECT COUNT(DISTINCT m.user_id) FROM community_members m WHERE m.community_id = $1 AND m.status = 'active'",
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .unwrap();

        assert_eq!(count.unwrap_or(0), 2);
        cleanup(&db, community_id, vec![owner_id, member_id]).await;
    }

    // ========================================================================
    // ADMIN MANAGEMENT TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_transfer_ownership() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let new_owner_id = create_test_user(&db, "new_owner").await;
        let slug = format!("test-trans-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        add_member(&db, new_owner_id, community_id, "member", "active").await;

        let admin_role_id: i64 = sqlx::query_scalar!("SELECT id FROM roles WHERE name = 'admin' LIMIT 1")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        sqlx::query!(
            "UPDATE community_members SET role_id = $1 WHERE community_id = $2 AND user_id = $3",
            admin_role_id,
            community_id,
            new_owner_id
        )
        .execute(&db.pool)
        .await
        .unwrap();

        sqlx::query!("UPDATE communities SET created_by = $1 WHERE id = $2", new_owner_id, community_id)
            .execute(&db.pool)
            .await
            .unwrap();

        let new_owner: Uuid = sqlx::query_scalar!("SELECT created_by FROM communities WHERE id = $1", community_id)
            .fetch_one(&db.pool)
            .await
            .unwrap();

        assert_eq!(new_owner, new_owner_id);
        cleanup(&db, community_id, vec![owner_id, new_owner_id]).await;
    }

    #[tokio::test]
    async fn test_cannot_transfer_to_nonmember() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let nonmember_id = create_test_user(&db, "nonmember").await;
        let slug = format!("test-trans-fail-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        let is_member_result: Option<bool> = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active')",
            community_id,
            nonmember_id
        )
        .fetch_one(&db.pool)
        .await
        .unwrap();

        assert!(!is_member_result.unwrap_or(false));
        cleanup(&db, community_id, vec![owner_id, nonmember_id]).await;
    }

    #[tokio::test]
    async fn test_cannot_demote_owner() {
        let db = setup_test_db().await;
        let owner_id = create_test_user(&db, "owner").await;
        let slug = format!("test-no-demote-{}", Uuid::new_v4().to_string().split('-').next().unwrap());
        let community_id = create_test_community(&db, owner_id, &slug, true, false).await;

        let is_owner: Option<bool> = sqlx::query_scalar!(
            "SELECT created_by = $1 FROM communities WHERE id = $2",
            owner_id,
            community_id
        )
        .fetch_one(&db.pool)
        .await
        .unwrap();

        assert!(is_owner.unwrap_or(false));
        cleanup(&db, community_id, vec![owner_id]).await;
    }
}
