use super::room_service::RoomService;
use shared::{
    database::Database,
    models::chat::{ChatRoom, RoomParticipant, RoomType, ParticipantRole},
    testing::{init_test_logging, create_test_db, cleanup_test_db, create_test_user, create_test_community},
    error::AppError,
};
use serial_test::serial;
use uuid::Uuid;

#[tokio::test]
#[serial]
async fn test_room_service_get_room() {
    init_test_logging();

    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let room_service = RoomService::new(db.clone());

    // Test getting non-existent room
    let non_existent_id = Uuid::new_v4();
    let result = room_service.get_room(non_existent_id).await.unwrap();
    assert!(result.is_none());

    // Create test data
    let user = create_test_user(&db, None).await.unwrap();
    let community = create_test_community(&db, user.id, None).await.unwrap();

    // Create test room
    let room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, $3, $4, 'general', false, $5)
        "#,
        room_id,
        community.id,
        "Test Room",
        "Test room description",
        user.id
    )
    .execute(db.pool())
    .await
    .unwrap();

    // Test getting existing room
    let room = room_service.get_room(room_id).await.unwrap();
    assert!(room.is_some());
    let room = room.unwrap();
    assert_eq!(room.id, room_id);
    assert_eq!(room.name, "Test Room");
    assert_eq!(room.community_id, community.id);
    assert!(!room.is_private);

    cleanup_test_db(&db).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_room_service_get_community_rooms() {
    init_test_logging();

    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let room_service = RoomService::new(db.clone());
    let user = create_test_user(&db, None).await.unwrap();
    let community = create_test_community(&db, user.id, None).await.unwrap();

    // Test empty community
    let rooms = room_service.get_community_rooms(community.id).await.unwrap();
    assert_eq!(rooms.len(), 0);

    // Create multiple rooms
    let room1_id = Uuid::new_v4();
    let room2_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES
            ($1, $2, 'Room 1', 'First room', 'general', false, $3),
            ($4, $2, 'Room 2', 'Second room', 'topic', true, $3)
        "#,
        room1_id, community.id, user.id,
        room2_id
    )
    .execute(db.pool())
    .await
    .unwrap();

    // Test getting community rooms
    let rooms = room_service.get_community_rooms(community.id).await.unwrap();
    assert_eq!(rooms.len(), 2);

    // Verify rooms are ordered by created_at ASC
    assert_eq!(rooms[0].id, room1_id);
    assert_eq!(rooms[1].id, room2_id);

    cleanup_test_db(&db).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_room_service_user_access_permissions() {
    init_test_logging();

    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let room_service = RoomService::new(db.clone());
    let user = create_test_user(&db, None).await.unwrap();
    let other_user = create_test_user(&db, None).await.unwrap();
    let community = create_test_community(&db, user.id, None).await.unwrap();

    // Create public and private rooms
    let public_room_id = Uuid::new_v4();
    let private_room_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES
            ($1, $2, 'Public Room', 'Public room', 'general', false, $3),
            ($4, $2, 'Private Room', 'Private room', 'topic', true, $3)
        "#,
        public_room_id, community.id, user.id,
        private_room_id
    )
    .execute(db.pool())
    .await
    .unwrap();

    // Add user to community
    sqlx::query!(
        "INSERT INTO community_members (community_id, user_id, status) VALUES ($1, $2, 'active')",
        community.id, user.id
    )
    .execute(db.pool())
    .await
    .unwrap();

    // Test access to public room as community member
    let can_access = room_service.can_user_access_room(user.id, public_room_id).await.unwrap();
    assert!(can_access);

    // Test access to private room without being participant
    let can_access = room_service.can_user_access_room(user.id, private_room_id).await.unwrap();
    assert!(!can_access);

    // Add user as participant to private room
    room_service.add_participant(private_room_id, user.id, None).await.unwrap();

    // Test access to private room as participant
    let can_access = room_service.can_user_access_room(user.id, private_room_id).await.unwrap();
    assert!(can_access);

    // Test access by non-community member
    let can_access = room_service.can_user_access_room(other_user.id, public_room_id).await.unwrap();
    assert!(!can_access);

    cleanup_test_db(&db).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_room_service_participant_management() {
    init_test_logging();

    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let room_service = RoomService::new(db.clone());
    let user = create_test_user(&db, None).await.unwrap();
    let community = create_test_community(&db, user.id, None).await.unwrap();

    // Create test room
    let room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Test Room', 'Test room', 'general', false, $3)
        "#,
        room_id, community.id, user.id
    )
    .execute(db.pool())
    .await
    .unwrap();

    // Test adding participant
    room_service.add_participant(room_id, user.id, Some("admin".to_string())).await.unwrap();

    // Test getting user role
    let role = room_service.get_user_room_role(user.id, room_id).await.unwrap();
    assert_eq!(role, Some("admin".to_string()));

    // Test getting participants
    let participants = room_service.get_room_participants(room_id).await.unwrap();
    assert_eq!(participants.len(), 1);
    assert_eq!(participants[0].user_id, user.id);
    assert!(matches!(participants[0].role, ParticipantRole::Admin));

    // Test participant count
    let count = room_service.get_participant_count(room_id).await.unwrap();
    assert_eq!(count, 1);

    // Test removing participant
    room_service.remove_participant(room_id, user.id).await.unwrap();
    let participants = room_service.get_room_participants(room_id).await.unwrap();
    assert_eq!(participants.len(), 0);

    // Test removing non-existent participant returns error
    let result = room_service.remove_participant(room_id, user.id).await;
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), AppError::NotFound(_)));

    cleanup_test_db(&db).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_room_service_direct_message_room() {
    init_test_logging();

    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let room_service = RoomService::new(db.clone());
    let user1 = create_test_user(&db, None).await.unwrap();
    let user2 = create_test_user(&db, None).await.unwrap();
    let community = create_test_community(&db, user1.id, None).await.unwrap();

    // Create DM room
    let dm_room_id = room_service.create_direct_message_room(user1.id, user2.id, community.id).await.unwrap();

    // Verify room was created
    let room = room_service.get_room(dm_room_id).await.unwrap();
    assert!(room.is_some());
    let room = room.unwrap();
    assert!(room.is_private);
    assert!(matches!(room.room_type, RoomType::DirectMessage));
    assert!(room.name.contains(&user1.id.to_string()));
    assert!(room.name.contains(&user2.id.to_string()));

    // Verify both users are participants
    let participants = room_service.get_room_participants(dm_room_id).await.unwrap();
    assert_eq!(participants.len(), 2);
    let user_ids: Vec<Uuid> = participants.iter().map(|p| p.user_id).collect();
    assert!(user_ids.contains(&user1.id));
    assert!(user_ids.contains(&user2.id));

    // Test creating duplicate DM room returns existing room
    let duplicate_room_id = room_service.create_direct_message_room(user1.id, user2.id, community.id).await.unwrap();
    assert_eq!(dm_room_id, duplicate_room_id);

    // Test reverse order also returns same room
    let reverse_room_id = room_service.create_direct_message_room(user2.id, user1.id, community.id).await.unwrap();
    assert_eq!(dm_room_id, reverse_room_id);

    cleanup_test_db(&db).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_room_service_permissions() {
    init_test_logging();

    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let room_service = RoomService::new(db.clone());
    let admin_user = create_test_user(&db, None).await.unwrap();
    let moderator_user = create_test_user(&db, None).await.unwrap();
    let member_user = create_test_user(&db, None).await.unwrap();
    let community = create_test_community(&db, admin_user.id, None).await.unwrap();

    // Create test room
    let room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Test Room', 'Test room', 'general', false, $3)
        "#,
        room_id, community.id, admin_user.id
    )
    .execute(db.pool())
    .await
    .unwrap();

    // Add participants with different roles
    room_service.add_participant(room_id, admin_user.id, Some("admin".to_string())).await.unwrap();
    room_service.add_participant(room_id, moderator_user.id, Some("moderator".to_string())).await.unwrap();
    room_service.add_participant(room_id, member_user.id, Some("member".to_string())).await.unwrap();

    // Test admin permissions
    assert!(room_service.check_room_permission(admin_user.id, room_id, "send_message").await.unwrap());
    assert!(room_service.check_room_permission(admin_user.id, room_id, "delete_message").await.unwrap());
    assert!(room_service.check_room_permission(admin_user.id, room_id, "manage_participants").await.unwrap());

    // Test moderator permissions
    assert!(room_service.check_room_permission(moderator_user.id, room_id, "send_message").await.unwrap());
    assert!(room_service.check_room_permission(moderator_user.id, room_id, "delete_message").await.unwrap());
    assert!(!room_service.check_room_permission(moderator_user.id, room_id, "manage_participants").await.unwrap());

    // Test member permissions
    assert!(room_service.check_room_permission(member_user.id, room_id, "send_message").await.unwrap());
    assert!(!room_service.check_room_permission(member_user.id, room_id, "delete_message").await.unwrap());
    assert!(!room_service.check_room_permission(member_user.id, room_id, "manage_participants").await.unwrap());

    cleanup_test_db(&db).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_room_service_last_read_update() {
    init_test_logging();

    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let room_service = RoomService::new(db.clone());
    let user = create_test_user(&db, None).await.unwrap();
    let community = create_test_community(&db, user.id, None).await.unwrap();

    // Create test room
    let room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Test Room', 'Test room', 'general', false, $3)
        "#,
        room_id, community.id, user.id
    )
    .execute(db.pool())
    .await
    .unwrap();

    // Add participant
    room_service.add_participant(room_id, user.id, None).await.unwrap();

    // Get initial last_read_at (should be None)
    let participants = room_service.get_room_participants(room_id).await.unwrap();
    assert!(participants[0].last_read_at.is_none());

    // Update last read
    room_service.update_last_read(user.id, room_id).await.unwrap();

    // Verify last_read_at was updated
    let participants = room_service.get_room_participants(room_id).await.unwrap();
    assert!(participants[0].last_read_at.is_some());

    cleanup_test_db(&db).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_room_service_error_handling() {
    init_test_logging();

    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let room_service = RoomService::new(db.clone());
    let user = create_test_user(&db, None).await.unwrap();
    let non_existent_room_id = Uuid::new_v4();

    // Test adding participant to non-existent room
    let result = room_service.add_participant(non_existent_room_id, user.id, None).await;
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), AppError::NotFound(_)));

    // Test getting role for non-existent room
    let role = room_service.get_user_room_role(user.id, non_existent_room_id).await.unwrap();
    assert!(role.is_none());

    // Test checking permission for non-existent room
    let has_permission = room_service.check_room_permission(user.id, non_existent_room_id, "send_message").await.unwrap();
    assert!(!has_permission);

    cleanup_test_db(&db).await.unwrap();
}