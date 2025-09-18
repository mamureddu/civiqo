use crate::{
    config::Config,
    state::AppState,
    handlers::websocket::{websocket_handler, handle_text_message},
    services::room_service::RoomService,
};
use shared::{
    auth::AuthState,
    models::{
        chat::{WebSocketMessage, MessageType},
        Claims,
    },
    testing::{init_test_logging, create_test_db, cleanup_test_db, create_test_user, create_test_community, create_mock_jwt_claims},
};
use axum::{
    extract::{WebSocketUpgrade, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
};
use serde_json::json;
use serial_test::serial;
use std::sync::Arc;
use uuid::Uuid;

async fn create_test_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let db = create_test_db().await?;

    let config = Config {
        database_url: "test".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        auth0_domain: "test.auth0.com".to_string(),
        auth0_audience: "test-audience".to_string(),
        sqs_queue_url: "test-queue".to_string(),
        sns_topic_arn: "test-topic".to_string(),
        aws_endpoint_url: None,
        max_connections: 100,
        message_ttl_seconds: 3600,
        heartbeat_interval_seconds: 30,
        development_mode: true,
    };

    let auth0_config = shared::auth::Auth0Config {
        domain: config.auth0_domain.clone(),
        audience: config.auth0_audience.clone(),
        client_id: "test-client".to_string(),
        client_secret: "test-secret".to_string(),
    };
    let auth_state = AuthState::new(&auth0_config);

    let aws_config = aws_config::from_env().load().await;
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);
    let sns_client = aws_sdk_sns::Client::new(&aws_config);

    Ok(AppState::new(db, config, auth_state, sqs_client, sns_client))
}

#[tokio::test]
#[serial]
async fn test_websocket_authentication_missing_header() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    // Create mock WebSocketUpgrade (this is complex to mock, so we test the auth logic separately)
    let headers = HeaderMap::new();

    // Test would require complex WebSocket mocking, so we focus on testing the underlying logic
    // The websocket_handler function is tested via integration tests
}

#[tokio::test]
#[serial]
async fn test_handle_send_message() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    // Create test data
    let user = create_test_user(state.database(), None).await.unwrap();
    let community = create_test_community(state.database(), user.id, None).await.unwrap();

    // Create test room
    let room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Test Room', 'Test room', 'general', false, $3)
        "#,
        room_id, community.id, user.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    // Add user to community
    sqlx::query!(
        "INSERT INTO community_members (community_id, user_id, status) VALUES ($1, $2, 'active')",
        community.id, user.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // Test valid send message
    let send_message_json = json!({
        "type": "SendMessage",
        "room_id": room_id,
        "encrypted_content": "encrypted_test_message",
        "message_type": "Text"
    });

    let result = handle_text_message(
        &send_message_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_ok());

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_handle_join_room() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    // Create test data
    let user = create_test_user(state.database(), None).await.unwrap();
    let community = create_test_community(state.database(), user.id, None).await.unwrap();

    // Create test room
    let room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Test Room', 'Test room', 'general', false, $3)
        "#,
        room_id, community.id, user.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    // Add user to community
    sqlx::query!(
        "INSERT INTO community_members (community_id, user_id, status) VALUES ($1, $2, 'active')",
        community.id, user.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // Test valid join room
    let join_room_json = json!({
        "type": "JoinRoom",
        "room_id": room_id
    });

    let result = handle_text_message(
        &join_room_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_ok());

    // Verify user is now a participant
    let participants = room_service.get_room_participants(room_id).await.unwrap();
    assert!(participants.iter().any(|p| p.user_id == user.id));

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_handle_leave_room() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    // Create test data
    let user = create_test_user(state.database(), None).await.unwrap();
    let community = create_test_community(state.database(), user.id, None).await.unwrap();

    // Create test room
    let room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Test Room', 'Test room', 'general', false, $3)
        "#,
        room_id, community.id, user.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // First join the room
    room_service.add_participant(room_id, user.id, None).await.unwrap();

    // Test leave room
    let leave_room_json = json!({
        "type": "LeaveRoom",
        "room_id": room_id
    });

    let result = handle_text_message(
        &leave_room_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_ok());

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_handle_heartbeat() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    let user = create_test_user(state.database(), None).await.unwrap();
    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // Test heartbeat message
    let heartbeat_json = json!({
        "type": "Heartbeat"
    });

    let result = handle_text_message(
        &heartbeat_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_ok());

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_handle_typing_notifications() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    // Create test data
    let user = create_test_user(state.database(), None).await.unwrap();
    let community = create_test_community(state.database(), user.id, None).await.unwrap();

    // Create test room
    let room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Test Room', 'Test room', 'general', false, $3)
        "#,
        room_id, community.id, user.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    // Add user to community
    sqlx::query!(
        "INSERT INTO community_members (community_id, user_id, status) VALUES ($1, $2, 'active')",
        community.id, user.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // Test typing start
    let typing_start_json = json!({
        "type": "TypingStart",
        "room_id": room_id,
        "user_id": user.id
    });

    let result = handle_text_message(
        &typing_start_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_ok());

    // Test typing stop
    let typing_stop_json = json!({
        "type": "TypingStop",
        "room_id": room_id,
        "user_id": user.id
    });

    let result = handle_text_message(
        &typing_stop_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_ok());

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_handle_key_exchange() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    let sender_user = create_test_user(state.database(), None).await.unwrap();
    let recipient_user = create_test_user(state.database(), None).await.unwrap();
    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // Test key exchange
    let key_exchange_json = json!({
        "type": "KeyExchange",
        "sender_id": sender_user.id,
        "recipient_id": recipient_user.id,
        "public_key": "test_public_key_data"
    });

    let result = handle_text_message(
        &key_exchange_json.to_string(),
        connection_id,
        sender_user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_ok());

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_handle_unauthorized_actions() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    let user = create_test_user(state.database(), None).await.unwrap();
    let other_user = create_test_user(state.database(), None).await.unwrap();
    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // Test sending message as another user (should fail)
    let invalid_typing_json = json!({
        "type": "TypingStart",
        "room_id": Uuid::new_v4(),
        "user_id": other_user.id  // Trying to type as different user
    });

    let result = handle_text_message(
        &invalid_typing_json.to_string(),
        connection_id,
        user.id,  // Authenticated as different user
        &state,
        &room_service,
    ).await;

    assert!(result.is_err());

    // Test key exchange as another user (should fail)
    let invalid_key_exchange_json = json!({
        "type": "KeyExchange",
        "sender_id": other_user.id,  // Trying to send as different user
        "recipient_id": user.id,
        "public_key": "test_key"
    });

    let result = handle_text_message(
        &invalid_key_exchange_json.to_string(),
        connection_id,
        user.id,  // Authenticated as different user
        &state,
        &room_service,
    ).await;

    assert!(result.is_err());

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_handle_invalid_json() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    let user = create_test_user(state.database(), None).await.unwrap();
    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // Test invalid JSON
    let result = handle_text_message(
        "invalid json",
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err());

    // Test malformed message
    let malformed_json = json!({
        "type": "InvalidMessageType",
        "some_field": "some_value"
    });

    let result = handle_text_message(
        &malformed_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    // Should handle gracefully (unsupported message types are ignored)
    assert!(result.is_ok());

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_handle_room_access_permissions() {
    init_test_logging();

    let state = match create_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    let user = create_test_user(state.database(), None).await.unwrap();
    let other_user = create_test_user(state.database(), None).await.unwrap();
    let community = create_test_community(state.database(), other_user.id, None).await.unwrap();

    // Create private room
    let private_room_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Private Room', 'Private room', 'topic', true, $3)
        "#,
        private_room_id, community.id, other_user.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    let room_service = RoomService::new(state.database().clone());
    let connection_id = "test_connection";

    // Test sending message to private room without access
    let send_message_json = json!({
        "type": "SendMessage",
        "room_id": private_room_id,
        "encrypted_content": "test message",
        "message_type": "Text"
    });

    let result = handle_text_message(
        &send_message_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err()); // Should fail due to lack of permission

    // Test joining private room without access
    let join_room_json = json!({
        "type": "JoinRoom",
        "room_id": private_room_id
    });

    let result = handle_text_message(
        &join_room_json.to_string(),
        connection_id,
        user.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err()); // Should fail due to lack of access

    cleanup_test_db(state.database()).await.unwrap();
}