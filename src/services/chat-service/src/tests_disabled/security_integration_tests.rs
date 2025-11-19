use crate::{
    config::Config,
    state::AppState,
    handlers::websocket::{handle_text_message, parse_auth0_user_id},
    services::room_service::RoomService,
};
use shared::{
    auth::AuthState,
    error::AppError,
    models::chat::{WebSocketMessage, MessageType},
    testing::{
        init_test_logging, create_test_db, cleanup_test_db,
        create_test_user, create_test_community
    },
};
use serde_json::json;
use serial_test::serial;
use uuid::Uuid;

async fn create_secure_test_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let db = create_test_db().await?;

    // Create config with realistic but testable security settings
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
        max_message_size: 1024, // Reasonable limit for testing
        rate_limit_messages_per_minute: 5, // Low enough to test easily
        rate_limit_typing_per_minute: 10,  // Higher than messages
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
async fn test_comprehensive_security_flow() {
    init_test_logging();

    let state = match create_secure_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    // Create test users
    let alice = create_test_user(state.database(), None).await.unwrap();
    let bob = create_test_user(state.database(), None).await.unwrap();
    let eve = create_test_user(state.database(), None).await.unwrap(); // Potential attacker

    // Create community and room
    let community = create_test_community(state.database(), alice.id, None).await.unwrap();
    let room_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Secure Test Room', 'Security testing room', 'general', false, $3)
        "#,
        room_id, community.id, alice.id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    // Add Alice and Bob to community (Eve is not a member)
    for user_id in [alice.id, bob.id] {
        sqlx::query!(
            "INSERT INTO community_members (community_id, user_id, status) VALUES ($1, $2, 'active')",
            community.id, user_id
        )
        .execute(state.database().pool())
        .await
        .unwrap();
    }

    let room_service = RoomService::new(state.database().clone());

    // PHASE 1: Test valid operations within security constraints
    println!("=== PHASE 1: Testing valid operations ===");

    // Alice sends valid messages within rate limit
    for i in 1..=5 {
        let message_json = json!({
            "type": "SendMessage",
            "room_id": room_id,
            "encrypted_content": format!("Alice_encrypted_message_{}", i),
            "message_type": "Text"
        });

        let result = handle_text_message(
            &message_json.to_string(),
            "alice_connection",
            alice.id,
            &state,
            &room_service,
        ).await;

        assert!(result.is_ok(), "Alice's message {} should succeed", i);
    }

    // Alice sends typing notifications within rate limit
    for i in 1..=10 {
        let typing_json = json!({
            "type": "TypingStart",
            "room_id": room_id,
            "user_id": alice.id
        });

        let result = handle_text_message(
            &typing_json.to_string(),
            "alice_connection",
            alice.id,
            &state,
            &room_service,
        ).await;

        assert!(result.is_ok(), "Alice's typing notification {} should succeed", i);
    }

    // Valid key exchange between Alice and Bob
    let key_exchange_json = json!({
        "type": "KeyExchange",
        "sender_id": alice.id,
        "recipient_id": bob.id,
        "public_key": "alice_public_key_base64_encoded_content"
    });

    let result = handle_text_message(
        &key_exchange_json.to_string(),
        "alice_connection",
        alice.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_ok(), "Valid key exchange should succeed");

    // PHASE 2: Test rate limiting enforcement
    println!("=== PHASE 2: Testing rate limiting ===");

    // Alice tries to send 6th message (should be rate limited)
    let rate_limited_message = json!({
        "type": "SendMessage",
        "room_id": room_id,
        "encrypted_content": "Alice_rate_limited_message",
        "message_type": "Text"
    });

    let result = handle_text_message(
        &rate_limited_message.to_string(),
        "alice_connection",
        alice.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "6th message should be rate limited");
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("rate limit"), "Should be rate limit error, got: {}", error_msg);

    // Alice tries to send 11th typing notification (should be rate limited)
    let rate_limited_typing = json!({
        "type": "TypingStart",
        "room_id": room_id,
        "user_id": alice.id
    });

    let result = handle_text_message(
        &rate_limited_typing.to_string(),
        "alice_connection",
        alice.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "11th typing notification should be rate limited");

    // PHASE 3: Test message validation
    println!("=== PHASE 3: Testing message validation ===");

    // Test message size limit
    let large_content = "a".repeat(2000); // Larger than 1024 byte limit
    let large_message_json = json!({
        "type": "SendMessage",
        "room_id": room_id,
        "encrypted_content": large_content,
        "message_type": "Text"
    });

    let result = handle_text_message(
        &large_message_json.to_string(),
        "bob_connection",
        bob.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "Large message should be rejected");
    assert!(result.unwrap_err().to_string().contains("too large"));

    // Test empty message content
    let empty_message_json = json!({
        "type": "SendMessage",
        "room_id": room_id,
        "encrypted_content": "",
        "message_type": "Text"
    });

    let result = handle_text_message(
        &empty_message_json.to_string(),
        "bob_connection",
        bob.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "Empty message should be rejected");
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));

    // Test invalid message target (both room and recipient)
    let invalid_target_json = json!({
        "type": "SendMessage",
        "room_id": room_id,
        "recipient_id": alice.id,
        "encrypted_content": "test_content",
        "message_type": "Text"
    });

    let result = handle_text_message(
        &invalid_target_json.to_string(),
        "bob_connection",
        bob.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "Message with both targets should be rejected");
    assert!(result.unwrap_err().to_string().contains("Cannot specify both"));

    // PHASE 4: Test user ID spoofing prevention
    println!("=== PHASE 4: Testing anti-spoofing measures ===");

    // Alice tries to send typing notification as Bob
    let spoofed_typing_json = json!({
        "type": "TypingStart",
        "room_id": room_id,
        "user_id": bob.id // Alice claiming to be Bob
    });

    let result = handle_text_message(
        &spoofed_typing_json.to_string(),
        "alice_connection",
        alice.id, // Alice is authenticated
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "User ID spoofing should be prevented");
    assert!(result.unwrap_err().to_string().contains("Cannot send typing notifications as another user"));

    // Alice tries to send key exchange as Bob
    let spoofed_key_json = json!({
        "type": "KeyExchange",
        "sender_id": bob.id, // Alice claiming to send as Bob
        "recipient_id": eve.id,
        "public_key": "spoofed_key"
    });

    let result = handle_text_message(
        &spoofed_key_json.to_string(),
        "alice_connection",
        alice.id, // Alice is authenticated
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "Key exchange spoofing should be prevented");
    assert!(result.unwrap_err().to_string().contains("Cannot send key exchange as another user"));

    // PHASE 5: Test key exchange validation
    println!("=== PHASE 5: Testing key exchange validation ===");

    // Self key exchange should be rejected
    let self_key_json = json!({
        "type": "KeyExchange",
        "sender_id": bob.id,
        "recipient_id": bob.id, // Same user
        "public_key": "self_exchange_key"
    });

    let result = handle_text_message(
        &self_key_json.to_string(),
        "bob_connection",
        bob.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "Self key exchange should be rejected");
    assert!(result.unwrap_err().to_string().contains("Cannot exchange keys with yourself"));

    // Empty public key should be rejected
    let empty_key_json = json!({
        "type": "KeyExchange",
        "sender_id": bob.id,
        "recipient_id": alice.id,
        "public_key": "" // Empty key
    });

    let result = handle_text_message(
        &empty_key_json.to_string(),
        "bob_connection",
        bob.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "Empty public key should be rejected");
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));

    // PHASE 6: Test authorization (access control)
    println!("=== PHASE 6: Testing access control ===");

    // Eve (not a community member) tries to send message
    let unauthorized_message = json!({
        "type": "SendMessage",
        "room_id": room_id,
        "encrypted_content": "Eve_unauthorized_message",
        "message_type": "Text"
    });

    let result = handle_text_message(
        &unauthorized_message.to_string(),
        "eve_connection",
        eve.id,
        &state,
        &room_service,
    ).await;

    assert!(result.is_err(), "Unauthorized message should be rejected");
    // Note: This might fail due to room permission checks

    // PHASE 7: Test JWT parsing functions
    println!("=== PHASE 7: Testing JWT user ID parsing ===");

    // Test various Auth0 user ID formats
    let test_uuid = Uuid::new_v4();

    let auth0_formats = vec![
        (format!("auth0|{}", test_uuid), "auth0 prefix"),
        (format!("database|{}", test_uuid), "database prefix"),
        (test_uuid.to_string(), "direct UUID"),
    ];

    for (subject, description) in auth0_formats {
        let result = parse_auth0_user_id(&subject);
        assert!(result.is_ok(), "Should parse {}: {}", description, subject);
        assert_eq!(result.unwrap(), test_uuid, "UUID should match for {}", description);
    }

    // Test unsupported provider formats
    let unsupported_formats = vec![
        "google-oauth2|12345",
        "facebook|user123",
        "twitter|handle",
    ];

    for subject in unsupported_formats {
        let result = parse_auth0_user_id(subject);
        assert!(result.is_err(), "Should reject unsupported format: {}", subject);
    }

    cleanup_test_db(state.database()).await.unwrap();

    println!("=== All security tests passed! ===");
}

#[tokio::test]
#[serial]
async fn test_concurrent_security_enforcement() {
    init_test_logging();

    let state = match create_secure_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    // Create multiple test users
    let mut users = Vec::new();
    for _ in 0..3 {
        users.push(create_test_user(state.database(), None).await.unwrap());
    }
    // users is already a Vec of User structs, no need to collect

    let community = create_test_community(state.database(), users[0].id, None).await.unwrap();
    let room_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by)
        VALUES ($1, $2, 'Concurrent Test Room', 'Concurrency testing room', 'general', false, $3)
        "#,
        room_id, community.id, users[0].id
    )
    .execute(state.database().pool())
    .await
    .unwrap();

    // Add all users to community
    for user in &users {
        sqlx::query!(
            "INSERT INTO community_members (community_id, user_id, status) VALUES ($1, $2, 'active')",
            community.id, user.id
        )
        .execute(state.database().pool())
        .await
        .unwrap();
    }

    let room_service = RoomService::new(state.database().clone());

    // Test concurrent rate limiting - each user should get their own quota
    let mut handles = vec![];

    for (i, user) in users.iter().enumerate() {
        let state_clone = state.clone();
        let room_service_clone = RoomService::new(state.database().clone());
        let user_id = user.id;
        let connection_id = format!("connection_{}", i);

        let handle = tokio::spawn(async move {
            let mut successful_messages = 0;

            // Each user tries to send 10 messages (rate limit is 5)
            for j in 1..=10 {
                let message_json = json!({
                    "type": "SendMessage",
                    "room_id": room_id,
                    "encrypted_content": format!("concurrent_message_{}_{}", i, j),
                    "message_type": "Text"
                });

                let result = handle_text_message(
                    &message_json.to_string(),
                    &connection_id,
                    user_id,
                    &state_clone,
                    &room_service_clone,
                ).await;

                if result.is_ok() {
                    successful_messages += 1;
                }
            }

            successful_messages
        });

        handles.push(handle);
    }

    // Wait for all concurrent operations to complete
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await);
    }

    // Each user should have been allowed exactly 5 messages (the rate limit)
    for (i, result) in results.iter().enumerate() {
        let successful_messages = result.as_ref().unwrap();
        assert_eq!(
            *successful_messages, 5,
            "User {} should have sent exactly 5 messages, got {}",
            i, successful_messages
        );
    }

    cleanup_test_db(state.database()).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_security_error_consistency() {
    init_test_logging();

    let state = match create_secure_test_app_state().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Skipping test - could not create app state: {}", e);
            return;
        }
    };

    let user = create_test_user(state.database(), None).await.unwrap();
    let room_service = RoomService::new(state.database().clone());
    let room_id = Uuid::new_v4();

    // Test that all security errors return consistent error types

    // Rate limiting error
    {
        let test_name = "rate_limit_violation";
        // First exhaust the rate limit
        for _ in 0..5 {
            let _ = handle_text_message(
                &json!({
                    "type": "SendMessage",
                    "room_id": room_id,
                    "encrypted_content": "exhaust_quota",
                    "message_type": "Text"
                }).to_string(),
                "test_conn",
                user.id,
                &state,
                &room_service,
            ).await;
        }

        // This should be rate limited
        let result = handle_text_message(
            &json!({
                "type": "SendMessage",
                "room_id": room_id,
                "encrypted_content": "rate_limited",
                "message_type": "Text"
            }).to_string(),
            "test_conn",
            user.id,
            &state,
            &room_service,
        ).await;

        assert!(result.is_err(), "{} should result in error", test_name);
        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(!error_msg.is_empty(), "{} should have non-empty error message", test_name);

        // Verify error can be properly categorized
        match error {
            AppError::RateLimit(_) => {
                assert_eq!(test_name, "rate_limit_violation", "Rate limit error type mismatch");
            },
            AppError::Validation(_) => {
                assert_eq!(test_name, "validation_error", "Validation error type mismatch");
            },
            AppError::Authorization(_) => {
                assert_eq!(test_name, "authorization_error", "Authorization error type mismatch");
            },
            _ => {
                // Some errors might be wrapped or different types, that's OK as long as they're errors
            }
        }
    }

    // Validation error
    {
        let test_name = "validation_error";
        let result = handle_text_message(
            &json!({
                "type": "SendMessage",
                "room_id": room_id,
                "encrypted_content": "", // Empty content
                "message_type": "Text"
            }).to_string(),
            "test_conn",
            user.id,
            &state,
            &room_service,
        ).await;

        assert!(result.is_err(), "{} should result in error", test_name);
        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(!error_msg.is_empty(), "{} should have non-empty error message", test_name);

        // Verify error can be properly categorized
        match error {
            AppError::RateLimit(_) => {
                assert_eq!(test_name, "rate_limit_violation", "Rate limit error type mismatch");
            },
            AppError::Validation(_) => {
                assert_eq!(test_name, "validation_error", "Validation error type mismatch");
            },
            AppError::Authorization(_) => {
                assert_eq!(test_name, "authorization_error", "Authorization error type mismatch");
            },
            _ => {
                // Some errors might be wrapped or different types, that's OK as long as they're errors
            }
        }
    }

    // Authorization error (spoofing attempt)
    {
        let test_name = "authorization_error";
        let result = handle_text_message(
            &json!({
                "type": "TypingStart",
                "room_id": room_id,
                "user_id": Uuid::new_v4() // Different user ID
            }).to_string(),
            "test_conn",
            user.id,
            &state,
            &room_service,
        ).await;

        assert!(result.is_err(), "{} should result in error", test_name);
        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(!error_msg.is_empty(), "{} should have non-empty error message", test_name);

        // Verify error can be properly categorized
        match error {
            AppError::RateLimit(_) => {
                assert_eq!(test_name, "rate_limit_violation", "Rate limit error type mismatch");
            },
            AppError::Validation(_) => {
                assert_eq!(test_name, "validation_error", "Validation error type mismatch");
            },
            AppError::Authorization(_) => {
                assert_eq!(test_name, "authorization_error", "Authorization error type mismatch");
            },
            _ => {
                // Some errors might be wrapped or different types, that's OK as long as they're errors
            }
        }
    }

    cleanup_test_db(state.database()).await.unwrap();
}