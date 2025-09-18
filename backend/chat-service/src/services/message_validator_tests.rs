use super::message_validator::MessageValidator;
use uuid::Uuid;

#[test]
fn test_message_validator_creation() {
    let validator = MessageValidator::new(1024);
    assert_eq!(validator.max_message_size, 1024);

    let large_validator = MessageValidator::new(65536);
    assert_eq!(large_validator.max_message_size, 65536);
}

#[test]
fn test_valid_room_message() {
    let validator = MessageValidator::new(1024);
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();
    let content = "valid_encrypted_message_content";

    let result = validator.validate_message(content, Some(room_id), None, sender_id);
    assert!(result.is_ok());
}

#[test]
fn test_valid_direct_message() {
    let validator = MessageValidator::new(1024);
    let recipient_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();
    let content = "valid_encrypted_direct_message";

    let result = validator.validate_message(content, None, Some(recipient_id), sender_id);
    assert!(result.is_ok());
}

#[test]
fn test_message_too_large() {
    let validator = MessageValidator::new(20); // Very small limit
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();
    let content = "this_message_is_definitely_too_long_for_the_20_byte_limit";

    let result = validator.validate_message(content, Some(room_id), None, sender_id);
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Message too large"));
    assert!(error_msg.contains("20 bytes"));
}

#[test]
fn test_message_exactly_at_size_limit() {
    let validator = MessageValidator::new(10);
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();
    let content = "1234567890"; // Exactly 10 bytes

    let result = validator.validate_message(content, Some(room_id), None, sender_id);
    assert!(result.is_ok());

    // One more character should fail
    let oversized_content = "12345678901"; // 11 bytes
    let result = validator.validate_message(oversized_content, Some(room_id), None, sender_id);
    assert!(result.is_err());
}

#[test]
fn test_both_room_and_recipient_specified() {
    let validator = MessageValidator::new(1024);
    let room_id = Uuid::new_v4();
    let recipient_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();
    let content = "test_message";

    let result = validator.validate_message(content, Some(room_id), Some(recipient_id), sender_id);
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Cannot specify both"));
}

#[test]
fn test_neither_room_nor_recipient_specified() {
    let validator = MessageValidator::new(1024);
    let sender_id = Uuid::new_v4();
    let content = "test_message";

    let result = validator.validate_message(content, None, None, sender_id);
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Must specify either"));
}

#[test]
fn test_nil_uuid_validation() {
    let validator = MessageValidator::new(1024);
    let content = "test_message";

    // Test nil room_id
    let result = validator.validate_message(content, Some(Uuid::nil()), None, Uuid::new_v4());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid room_id: cannot be nil UUID"));

    // Test nil recipient_id
    let result = validator.validate_message(content, None, Some(Uuid::nil()), Uuid::new_v4());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid recipient_id: cannot be nil UUID"));

    // Test nil sender_id
    let result = validator.validate_message(content, Some(Uuid::new_v4()), None, Uuid::nil());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid user_id: cannot be nil UUID"));
}

#[test]
fn test_empty_message_content() {
    let validator = MessageValidator::new(1024);
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();

    let result = validator.validate_message("", Some(room_id), None, sender_id);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));
}

#[test]
fn test_valid_content_formats() {
    let validator = MessageValidator::new(1024);
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();

    // Test various valid encrypted content formats
    let valid_contents = vec![
        "base64encodedcontent",
        "ABC123xyz789",
        "encrypted+data/with=padding",
        r#"{"encrypted":"content","iv":"data"}"#,
        "simple_alphanumeric_123",
        "MixedCaseContent456",
    ];

    for content in valid_contents {
        let result = validator.validate_message(content, Some(room_id), None, sender_id);
        assert!(result.is_ok(), "Content '{}' should be valid", content);
    }
}

#[test]
fn test_invalid_content_characters() {
    let validator = MessageValidator::new(1024);
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();

    // Test content with invalid characters for encrypted data
    let invalid_contents = vec![
        "content<script>alert('xss')</script>",
        "content@with!special#chars$",
        "content with\x00null bytes",
        "content\nwith\tcontrol\rchars",
        "content®with™unicode©symbols",
    ];

    for content in invalid_contents {
        let result = validator.validate_message(content, Some(room_id), None, sender_id);
        assert!(result.is_err(), "Content '{}' should be invalid", content);
        assert!(result.unwrap_err().to_string().contains("invalid characters"));
    }
}

#[test]
fn test_excessively_long_lines() {
    let validator = MessageValidator::new(10000); // Large overall limit
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();

    // Create content with a very long line (over 2048 chars)
    let long_line = "a".repeat(3000);

    let result = validator.validate_message(&long_line, Some(room_id), None, sender_id);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("excessively long lines"));
}

#[test]
fn test_content_with_acceptable_line_lengths() {
    let validator = MessageValidator::new(10000);
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();

    // Create content with multiple lines, all under the limit
    let content_with_lines = vec![
        "line1".repeat(100), // 500 chars
        "line2".repeat(200), // 1000 chars
        "line3".repeat(400), // 2000 chars (just under limit)
    ].join("\n");

    let result = validator.validate_message(&content_with_lines, Some(room_id), None, sender_id);
    assert!(result.is_ok());
}

#[test]
fn test_typing_notification_validation() {
    let validator = MessageValidator::new(1024);
    let room_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let recipient_id = Uuid::new_v4();

    // Valid room typing notification
    let result = validator.validate_typing_notification(Some(room_id), None, user_id);
    assert!(result.is_ok());

    // Valid direct typing notification
    let result = validator.validate_typing_notification(None, Some(recipient_id), user_id);
    assert!(result.is_ok());

    // Invalid - both room and recipient specified
    let result = validator.validate_typing_notification(Some(room_id), Some(recipient_id), user_id);
    assert!(result.is_err());

    // Invalid - neither specified
    let result = validator.validate_typing_notification(None, None, user_id);
    assert!(result.is_err());

    // Invalid - nil user_id
    let result = validator.validate_typing_notification(Some(room_id), None, Uuid::nil());
    assert!(result.is_err());
}

#[test]
fn test_key_exchange_validation() {
    let validator = MessageValidator::new(1024);
    let sender_id = Uuid::new_v4();
    let recipient_id = Uuid::new_v4();
    let public_key = "valid_base64_key_content_123";

    // Valid key exchange
    let result = validator.validate_key_exchange(recipient_id, sender_id, public_key);
    assert!(result.is_ok());

    // Invalid - same sender and recipient
    let result = validator.validate_key_exchange(sender_id, sender_id, public_key);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Cannot exchange keys with yourself"));

    // Invalid - empty public key
    let result = validator.validate_key_exchange(recipient_id, sender_id, "");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));

    // Invalid - public key too large
    let huge_key = "a".repeat(2000); // Over 1024 limit
    let result = validator.validate_key_exchange(recipient_id, sender_id, &huge_key);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("too large"));
}

#[test]
fn test_public_key_format_validation() {
    let validator = MessageValidator::new(1024);
    let sender_id = Uuid::new_v4();
    let recipient_id = Uuid::new_v4();

    // Valid public key formats
    let valid_keys = vec![
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=",
        "base64encodedpublickey",
        "1234567890",
        "validKey123+/=",
        "A", // Single character is valid
    ];

    for key in valid_keys {
        let result = validator.validate_key_exchange(recipient_id, sender_id, key);
        assert!(result.is_ok(), "Public key '{}' should be valid", key);
    }

    // Invalid public key formats
    let invalid_keys = vec![
        "invalid@key",
        "key with spaces",
        "key-with-dashes",
        "key#with$symbols",
        "key\nwith\nnewlines",
        "key®with©unicode",
    ];

    for key in invalid_keys {
        let result = validator.validate_key_exchange(recipient_id, sender_id, key);
        assert!(result.is_err(), "Public key '{}' should be invalid", key);
        assert!(result.unwrap_err().to_string().contains("invalid characters"));
    }
}

#[test]
fn test_edge_case_message_sizes() {
    // Test various edge cases for message size validation
    let test_cases = vec![
        (0, "Should reject zero-size limit", false),
        (1, "Should allow single byte", true),
        (65536, "Should allow 64KB default", true),
        (1048576, "Should allow 1MB", true),
    ];

    for (size_limit, description, should_create) in test_cases {
        if should_create {
            let validator = MessageValidator::new(size_limit);
            assert_eq!(validator.max_message_size, size_limit, "{}", description);
        }
    }
}

#[test]
fn test_validator_with_different_size_limits() {
    let small_validator = MessageValidator::new(100);
    let large_validator = MessageValidator::new(1000000);

    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();
    let medium_content = "a".repeat(500); // 500 bytes

    // Should fail with small validator
    let result = small_validator.validate_message(&medium_content, Some(room_id), None, sender_id);
    assert!(result.is_err());

    // Should pass with large validator
    let result = large_validator.validate_message(&medium_content, Some(room_id), None, sender_id);
    assert!(result.is_ok());
}

#[test]
fn test_comprehensive_validation_scenarios() {
    let validator = MessageValidator::new(1024);

    // Scenario 1: Complete valid room message
    let room_message_result = validator.validate_message(
        "encrypted_room_message_content",
        Some(Uuid::new_v4()),
        None,
        Uuid::new_v4(),
    );
    assert!(room_message_result.is_ok());

    // Scenario 2: Complete valid direct message
    let direct_message_result = validator.validate_message(
        "encrypted_direct_message_content",
        None,
        Some(Uuid::new_v4()),
        Uuid::new_v4(),
    );
    assert!(direct_message_result.is_ok());

    // Scenario 3: Valid typing notification
    let typing_result = validator.validate_typing_notification(
        Some(Uuid::new_v4()),
        None,
        Uuid::new_v4(),
    );
    assert!(typing_result.is_ok());

    // Scenario 4: Valid key exchange
    let key_exchange_result = validator.validate_key_exchange(
        Uuid::new_v4(),
        Uuid::new_v4(),
        "validPublicKey123",
    );
    assert!(key_exchange_result.is_ok());
}

#[test]
fn test_validation_error_messages() {
    let validator = MessageValidator::new(50);
    let room_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();

    // Test specific error messages for debugging
    let long_message = "a".repeat(100);
    let result = validator.validate_message(&long_message, Some(room_id), None, sender_id);
    let error = result.unwrap_err().to_string();
    assert!(error.contains("100 bytes"));
    assert!(error.contains("50 bytes"));

    // Test empty content error
    let result = validator.validate_message("", Some(room_id), None, sender_id);
    let error = result.unwrap_err().to_string();
    assert!(error.contains("cannot be empty"));

    // Test nil UUID error
    let result = validator.validate_message("test", Some(Uuid::nil()), None, sender_id);
    let error = result.unwrap_err().to_string();
    assert!(error.contains("nil UUID"));

    // Test target specification error
    let result = validator.validate_message("test", None, None, sender_id);
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Must specify either"));
}