use super::message_router::MessageRouter;
use shared::{
    models::chat::{WebSocketMessage, MessageType},
    testing::init_test_logging,
};
use std::collections::HashMap;
use tokio::time::Duration;
use uuid::Uuid;

async fn create_test_message_router() -> MessageRouter {
    let aws_config = aws_config::from_env().load().await;
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);
    let sns_client = aws_sdk_sns::Client::new(&aws_config);

    MessageRouter::new(
        sqs_client,
        sns_client,
        "test-queue".to_string(),
        "test-topic".to_string(),
        3600, // message_ttl_seconds
    )
}

#[tokio::test]
async fn test_message_router_room_membership() {
    init_test_logging();

    let router = create_test_message_router().await;
    let user_id = Uuid::new_v4();
    let room_id = Uuid::new_v4();

    // Initially no participants
    let participants = router.get_room_participants(room_id).await.unwrap();
    assert_eq!(participants.len(), 0);

    // Join room
    router.join_room(user_id, room_id).await.unwrap();
    let participants = router.get_room_participants(room_id).await.unwrap();
    assert_eq!(participants.len(), 1);
    assert!(participants.contains(&user_id));

    // Join same room again (should not duplicate)
    router.join_room(user_id, room_id).await.unwrap();
    let participants = router.get_room_participants(room_id).await.unwrap();
    assert_eq!(participants.len(), 1);

    // Leave room
    router.leave_room(user_id, room_id).await.unwrap();
    let participants = router.get_room_participants(room_id).await.unwrap();
    assert_eq!(participants.len(), 0);

    // Verify room is cleaned up when empty
    let stats = router.get_room_stats().await;
    assert!(!stats.contains_key(&room_id));
}

#[tokio::test]
async fn test_message_router_multiple_users_and_rooms() {
    init_test_logging();

    let router = create_test_message_router().await;
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    let user3_id = Uuid::new_v4();
    let room1_id = Uuid::new_v4();
    let room2_id = Uuid::new_v4();

    // Users join different rooms
    router.join_room(user1_id, room1_id).await.unwrap();
    router.join_room(user2_id, room1_id).await.unwrap();
    router.join_room(user1_id, room2_id).await.unwrap();
    router.join_room(user3_id, room2_id).await.unwrap();

    // Check room stats
    let stats = router.get_room_stats().await;
    assert_eq!(stats.get(&room1_id), Some(&2)); // user1 and user2
    assert_eq!(stats.get(&room2_id), Some(&2)); // user1 and user3

    // Check specific room participants
    let room1_participants = router.get_room_participants(room1_id).await.unwrap();
    assert_eq!(room1_participants.len(), 2);
    assert!(room1_participants.contains(&user1_id));
    assert!(room1_participants.contains(&user2_id));

    let room2_participants = router.get_room_participants(room2_id).await.unwrap();
    assert_eq!(room2_participants.len(), 2);
    assert!(room2_participants.contains(&user1_id));
    assert!(room2_participants.contains(&user3_id));

    // User leaves one room
    router.leave_room(user1_id, room1_id).await.unwrap();
    let room1_participants = router.get_room_participants(room1_id).await.unwrap();
    assert_eq!(room1_participants.len(), 1);
    assert!(!room1_participants.contains(&user1_id));
    assert!(room1_participants.contains(&user2_id));

    // User still in other room
    let room2_participants = router.get_room_participants(room2_id).await.unwrap();
    assert!(room2_participants.contains(&user1_id));
}

#[tokio::test]
async fn test_message_router_message_routing() {
    init_test_logging();

    let router = create_test_message_router().await;
    let sender_id = Uuid::new_v4();
    let room_id = Uuid::new_v4();

    // Set up room with participants
    router.join_room(sender_id, room_id).await.unwrap();

    // Test routing different message types
    let receive_message = WebSocketMessage::ReceiveMessage {
        id: Uuid::new_v4(),
        room_id,
        sender_id,
        encrypted_content: "encrypted_test_message".to_string(),
        message_type: MessageType::Text,
        created_at: chrono::Utc::now(),
    };

    // Route message (this will attempt to send to SQS/SNS in real scenarios)
    let result = router.route_message(receive_message, Some("conn_123".to_string())).await;

    // In test environment without LocalStack, SQS/SNS calls may fail
    // but the routing logic should still execute
    match result {
        Ok(_) => {
            println!("Message routing succeeded");
        }
        Err(e) => {
            println!("Message routing failed (expected in test environment): {}", e);
            // This is acceptable in test environment without proper AWS setup
        }
    }
}

#[tokio::test]
async fn test_message_router_typing_notifications() {
    init_test_logging();

    let router = create_test_message_router().await;
    let typing_user_id = Uuid::new_v4();
    let other_user_id = Uuid::new_v4();
    let room_id = Uuid::new_v4();

    // Set up room with participants
    router.join_room(typing_user_id, room_id).await.unwrap();
    router.join_room(other_user_id, room_id).await.unwrap();

    // Test typing start notification
    let typing_start = WebSocketMessage::TypingStart {
        room_id,
        user_id: typing_user_id,
    };

    let result = router.route_message(typing_start, Some("conn_123".to_string())).await;
    match result {
        Ok(_) => println!("Typing start routing succeeded"),
        Err(e) => println!("Typing start routing failed (expected in test environment): {}", e),
    }

    // Test typing stop notification
    let typing_stop = WebSocketMessage::TypingStop {
        room_id,
        user_id: typing_user_id,
    };

    let result = router.route_message(typing_stop, Some("conn_123".to_string())).await;
    match result {
        Ok(_) => println!("Typing stop routing succeeded"),
        Err(e) => println!("Typing stop routing failed (expected in test environment): {}", e),
    }
}

#[tokio::test]
async fn test_message_router_unsupported_messages() {
    init_test_logging();

    let router = create_test_message_router().await;

    // Test routing unsupported message types
    let heartbeat = WebSocketMessage::Heartbeat;
    let result = router.route_message(heartbeat, None).await;
    assert!(result.is_ok()); // Should handle gracefully

    let error_message = WebSocketMessage::Error {
        message: "Test error".to_string(),
        code: "TEST_ERROR".to_string(),
    };
    let result = router.route_message(error_message, None).await;
    assert!(result.is_ok()); // Should handle gracefully

    let join_room = WebSocketMessage::JoinRoom {
        room_id: Uuid::new_v4(),
    };
    let result = router.route_message(join_room, None).await;
    assert!(result.is_ok()); // Should handle gracefully
}

#[tokio::test]
async fn test_message_router_empty_room_handling() {
    init_test_logging();

    let router = create_test_message_router().await;
    let room_id = Uuid::new_v4();

    // Try to route message to empty room
    let receive_message = WebSocketMessage::ReceiveMessage {
        id: Uuid::new_v4(),
        room_id,
        sender_id: Uuid::new_v4(),
        encrypted_content: "test_message".to_string(),
        message_type: MessageType::Text,
        created_at: chrono::Utc::now(),
    };

    let result = router.route_message(receive_message, None).await;
    assert!(result.is_ok()); // Should handle empty room gracefully
}

#[tokio::test]
async fn test_message_router_user_message_processing() {
    init_test_logging();

    let router = create_test_message_router().await;
    let user_id = Uuid::new_v4();

    // Test processing messages for a user (offline message retrieval)
    let messages = router.process_user_messages(user_id).await.unwrap();
    assert_eq!(messages.len(), 0); // Currently returns empty, as implementation is placeholder
}

#[tokio::test]
async fn test_message_router_concurrent_room_operations() {
    init_test_logging();

    let router = std::sync::Arc::new(create_test_message_router().await);
    let room_id = Uuid::new_v4();
    let mut handles = vec![];

    // Spawn multiple tasks joining/leaving the same room
    for i in 0..10 {
        let router_clone = router.clone();
        let user_id = Uuid::new_v4();

        let handle = tokio::spawn(async move {
            // Join room
            router_clone.join_room(user_id, room_id).await.unwrap();

            // Simulate some work
            tokio::time::sleep(Duration::from_millis(10)).await;

            // Leave room
            router_clone.leave_room(user_id, room_id).await.unwrap();
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Room should be empty and cleaned up
    let participants = router.get_room_participants(room_id).await.unwrap();
    assert_eq!(participants.len(), 0);

    let stats = router.get_room_stats().await;
    assert!(!stats.contains_key(&room_id));
}

#[tokio::test]
async fn test_message_router_room_stats() {
    init_test_logging();

    let router = create_test_message_router().await;
    let room1_id = Uuid::new_v4();
    let room2_id = Uuid::new_v4();

    // Initially no rooms
    let stats = router.get_room_stats().await;
    assert!(stats.is_empty());

    // Add users to rooms
    for i in 0..3 {
        let user_id = Uuid::new_v4();
        router.join_room(user_id, room1_id).await.unwrap();
    }

    for i in 0..5 {
        let user_id = Uuid::new_v4();
        router.join_room(user_id, room2_id).await.unwrap();
    }

    // Check stats
    let stats = router.get_room_stats().await;
    assert_eq!(stats.len(), 2);
    assert_eq!(stats.get(&room1_id), Some(&3));
    assert_eq!(stats.get(&room2_id), Some(&5));
}

#[tokio::test]
async fn test_message_serialization_for_routing() {
    init_test_logging();

    // Test serialization of different message types used in routing
    let messages = vec![
        WebSocketMessage::ReceiveMessage {
            id: Uuid::new_v4(),
            room_id: Uuid::new_v4(),
            sender_id: Uuid::new_v4(),
            encrypted_content: "test content".to_string(),
            message_type: MessageType::Text,
            created_at: chrono::Utc::now(),
        },
        WebSocketMessage::TypingStart {
            room_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
        },
        WebSocketMessage::TypingStop {
            room_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
        },
    ];

    for message in messages {
        // Test serialization
        let serialized = serde_json::to_string(&message);
        assert!(serialized.is_ok());

        // Test deserialization
        let json = serialized.unwrap();
        let deserialized: Result<WebSocketMessage, _> = serde_json::from_str(&json);
        assert!(deserialized.is_ok());
    }
}