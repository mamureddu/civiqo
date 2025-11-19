use super::connection_manager::{ActiveConnection, ConnectionManager};
use super::message_router::MessageRouter;
use shared::{
    models::chat::{WebSocketMessage, MessageType},
    testing::{init_test_logging, create_test_db, cleanup_test_db},
    database::Database,
    error::AppError,
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use serial_test::serial;
use uuid::Uuid;

async fn create_test_connection_manager() -> Result<ConnectionManager, Box<dyn std::error::Error>> {
    let db = create_test_db().await?;
    let aws_config = aws_config::from_env().load().await;
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);
    let sns_client = aws_sdk_sns::Client::new(&aws_config);

    let message_router = Arc::new(MessageRouter::new(
        sqs_client,
        sns_client,
        "test-queue".to_string(),
        "test-topic".to_string(),
        3600,
    ));

    Ok(ConnectionManager::new(
        db,
        message_router,
        100, // max_connections
        30,  // heartbeat_interval_seconds
    ))
}

#[tokio::test]
#[serial]
async fn test_active_connection_lifecycle() {
    init_test_logging();

    let user_id = Uuid::new_v4();
    let connection_id = "test_conn_123".to_string();
    let (sender, _receiver) = mpsc::unbounded_channel();

    let mut connection = ActiveConnection::new(user_id, connection_id.clone(), sender);

    // Test initial state
    assert_eq!(connection.user_id, user_id);
    assert_eq!(connection.connection_id, connection_id);
    assert!(connection.joined_rooms.is_empty());
    assert!(!connection.is_expired(Duration::from_secs(60)));

    // Test heartbeat updates
    let initial_heartbeat = connection.last_heartbeat;
    tokio::time::sleep(Duration::from_millis(10)).await;
    connection.update_heartbeat();
    assert!(connection.last_heartbeat > initial_heartbeat);

    // Test room joining/leaving
    let room_id = Uuid::new_v4();
    connection.join_room(room_id);
    assert!(connection.joined_rooms.contains(&room_id));
    assert_eq!(connection.joined_rooms.len(), 1);

    // Test duplicate room join
    connection.join_room(room_id);
    assert_eq!(connection.joined_rooms.len(), 1);

    // Test room leaving
    connection.leave_room(room_id);
    assert!(!connection.joined_rooms.contains(&room_id));
    assert!(connection.joined_rooms.is_empty());

    // Test expiration
    connection.last_heartbeat = Instant::now() - Duration::from_secs(120);
    assert!(connection.is_expired(Duration::from_secs(60)));
}

#[tokio::test]
#[serial]
async fn test_connection_manager_add_remove_connections() {
    init_test_logging();

    let manager = match create_test_connection_manager().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Skipping test - could not create connection manager: {}", e);
            return;
        }
    };

    let user_id = Uuid::new_v4();
    let (sender, _receiver) = mpsc::unbounded_channel();

    // Test adding connection
    let connection_id = manager.add_connection(user_id, sender).await.unwrap();
    assert!(!connection_id.is_empty());
    assert!(connection_id.starts_with("conn_"));
    assert_eq!(manager.connection_count(), 1);

    // Test getting user connections
    let user_connections = manager.get_user_connections(user_id);
    assert_eq!(user_connections.len(), 1);
    assert_eq!(user_connections[0], connection_id);

    // Test removing connection
    manager.remove_connection(&connection_id).await.unwrap();
    assert_eq!(manager.connection_count(), 0);

    // Test removing non-existent connection (should not error)
    manager.remove_connection("non_existent").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_connection_manager_message_sending() {
    init_test_logging();

    let manager = match create_test_connection_manager().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Skipping test - could not create connection manager: {}", e);
            return;
        }
    };

    let user_id = Uuid::new_v4();
    let (sender, mut receiver) = mpsc::unbounded_channel();

    // Add connection
    let connection_id = manager.add_connection(user_id, sender).await.unwrap();

    // Test sending message to connection
    let test_message = WebSocketMessage::Heartbeat;
    manager.send_to_connection(&connection_id, test_message.clone()).await.unwrap();

    // Verify message was received
    let received_message = tokio::time::timeout(Duration::from_secs(1), receiver.recv()).await.unwrap();
    assert!(received_message.is_some());
    assert!(matches!(received_message.unwrap(), WebSocketMessage::Heartbeat));

    // Test sending to non-existent connection
    let result = manager.send_to_connection("non_existent", test_message).await;
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), AppError::NotFound(_)));

    manager.remove_connection(&connection_id).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_connection_manager_user_messaging() {
    init_test_logging();

    let manager = match create_test_connection_manager().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Skipping test - could not create connection manager: {}", e);
            return;
        }
    };

    let user_id = Uuid::new_v4();
    let (sender1, mut receiver1) = mpsc::unbounded_channel();
    let (sender2, mut receiver2) = mpsc::unbounded_channel();

    // Add multiple connections for same user
    let connection_id1 = manager.add_connection(user_id, sender1).await.unwrap();
    let connection_id2 = manager.add_connection(user_id, sender2).await.unwrap();

    assert_eq!(manager.get_user_connections(user_id).len(), 2);

    // Send message to user
    let test_message = WebSocketMessage::ReceiveMessage {
        id: Uuid::new_v4(),
        room_id: Uuid::new_v4(),
        sender_id: Uuid::new_v4(),
        encrypted_content: "test message".to_string(),
        message_type: MessageType::Text,
        created_at: chrono::Utc::now(),
    };

    let sent_count = manager.send_to_user(user_id, test_message.clone()).await.unwrap();
    assert_eq!(sent_count, 2);

    // Verify both connections received the message
    let received1 = tokio::time::timeout(Duration::from_secs(1), receiver1.recv()).await.unwrap();
    let received2 = tokio::time::timeout(Duration::from_secs(1), receiver2.recv()).await.unwrap();

    assert!(received1.is_some());
    assert!(received2.is_some());

    // Clean up
    manager.remove_connection(&connection_id1).await.unwrap();
    manager.remove_connection(&connection_id2).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_connection_manager_heartbeat() {
    init_test_logging();

    let manager = match create_test_connection_manager().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Skipping test - could not create connection manager: {}", e);
            return;
        }
    };

    let user_id = Uuid::new_v4();
    let (sender, _receiver) = mpsc::unbounded_channel();

    let connection_id = manager.add_connection(user_id, sender).await.unwrap();

    // Test updating heartbeat
    manager.update_heartbeat(&connection_id).unwrap();

    // Test updating heartbeat for non-existent connection
    let result = manager.update_heartbeat("non_existent");
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), AppError::NotFound(_)));

    manager.remove_connection(&connection_id).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_connection_manager_room_operations() {
    init_test_logging();

    let manager = match create_test_connection_manager().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Skipping test - could not create connection manager: {}", e);
            return;
        }
    };

    let user_id = Uuid::new_v4();
    let room_id = Uuid::new_v4();
    let (sender, _receiver) = mpsc::unbounded_channel();

    let connection_id = manager.add_connection(user_id, sender).await.unwrap();

    // Test joining room
    manager.join_room(&connection_id, room_id).await.unwrap();

    // Test leaving room
    manager.leave_room(&connection_id, room_id).await.unwrap();

    // Test operations with non-existent connection
    let result = manager.join_room("non_existent", room_id).await;
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), AppError::NotFound(_)));

    let result = manager.leave_room("non_existent", room_id).await;
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), AppError::NotFound(_)));

    manager.remove_connection(&connection_id).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_connection_manager_connection_limits() {
    init_test_logging();

    // Create manager with very low connection limit
    let db = match create_test_db().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Skipping test - could not create test database: {}", e);
            return;
        }
    };

    let aws_config = aws_config::from_env().load().await;
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);
    let sns_client = aws_sdk_sns::Client::new(&aws_config);

    let message_router = Arc::new(MessageRouter::new(
        sqs_client,
        sns_client,
        "test-queue".to_string(),
        "test-topic".to_string(),
        3600,
    ));

    let manager = ConnectionManager::new(
        db,
        message_router,
        2,  // max_connections = 2
        30,
    );

    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    let user3_id = Uuid::new_v4();

    let (sender1, _) = mpsc::unbounded_channel();
    let (sender2, _) = mpsc::unbounded_channel();
    let (sender3, _) = mpsc::unbounded_channel();

    // Add connections up to limit
    let conn1 = manager.add_connection(user1_id, sender1).await.unwrap();
    let conn2 = manager.add_connection(user2_id, sender2).await.unwrap();
    assert_eq!(manager.connection_count(), 2);

    // Try to add connection beyond limit
    let result = manager.add_connection(user3_id, sender3).await;
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), AppError::Validation(_)));

    // Clean up
    manager.remove_connection(&conn1).await.unwrap();
    manager.remove_connection(&conn2).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_connection_manager_failed_message_cleanup() {
    init_test_logging();

    let manager = match create_test_connection_manager().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Skipping test - could not create connection manager: {}", e);
            return;
        }
    };

    let user_id = Uuid::new_v4();
    let (sender, receiver) = mpsc::unbounded_channel();

    let connection_id = manager.add_connection(user_id, sender).await.unwrap();
    assert_eq!(manager.connection_count(), 1);

    // Close the receiver to simulate connection failure
    drop(receiver);

    // Try to send message - should fail and clean up connection
    let test_message = WebSocketMessage::Heartbeat;
    let result = manager.send_to_connection(&connection_id, test_message).await;
    assert!(result.is_err());

    // Connection should be removed due to send failure
    assert_eq!(manager.connection_count(), 0);
}

#[tokio::test]
#[serial]
async fn test_connection_manager_concurrent_operations() {
    init_test_logging();

    let manager = match create_test_connection_manager().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Skipping test - could not create connection manager: {}", e);
            return;
        }
    };

    let manager = Arc::new(manager);
    let mut handles = vec![];

    // Spawn multiple tasks adding/removing connections concurrently
    for i in 0..10 {
        let manager_clone = manager.clone();
        let handle = tokio::spawn(async move {
            let user_id = Uuid::new_v4();
            let (sender, _receiver) = mpsc::unbounded_channel();

            if let Ok(connection_id) = manager_clone.add_connection(user_id, sender).await {
                // Simulate some work
                tokio::time::sleep(Duration::from_millis(10)).await;

                let _ = manager_clone.remove_connection(&connection_id).await;
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // All connections should be cleaned up
    assert_eq!(manager.connection_count(), 0);
}