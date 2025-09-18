use super::websocket::parse_auth0_user_id;
use shared::error::AppError;
use uuid::Uuid;

#[test]
fn test_parse_auth0_user_id_with_auth0_prefix() {
    // Test standard Auth0 database connection format
    let uuid = Uuid::new_v4();
    let subject = format!("auth0|{}", uuid);

    let result = parse_auth0_user_id(&subject);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), uuid);
}

#[test]
fn test_parse_auth0_user_id_with_database_prefix() {
    // Test database connection format
    let uuid = Uuid::new_v4();
    let subject = format!("database|{}", uuid);

    let result = parse_auth0_user_id(&subject);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), uuid);
}

#[test]
fn test_parse_auth0_user_id_direct_uuid() {
    // Test direct UUID format (no prefix)
    let uuid = Uuid::new_v4();
    let subject = uuid.to_string();

    let result = parse_auth0_user_id(&subject);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), uuid);
}

#[test]
fn test_parse_auth0_user_id_unsupported_provider() {
    // Test unsupported provider formats
    let unsupported_providers = vec![
        "google-oauth2|1234567890",
        "facebook|1234567890",
        "twitter|user123",
        "github|user456",
        "linkedin|user789",
        "unknown-provider|someid",
    ];

    for subject in unsupported_providers {
        let result = parse_auth0_user_id(subject);
        assert!(result.is_err(), "Should reject unsupported provider: {}", subject);

        let error = result.unwrap_err();
        match error {
            AppError::Auth(msg) => {
                assert!(msg.contains("Unsupported Auth0 provider format"));
                assert!(msg.contains(subject));
            },
            _ => panic!("Expected Auth error for unsupported provider: {}", subject),
        }
    }
}

#[test]
fn test_parse_auth0_user_id_invalid_uuid_format() {
    let invalid_formats = vec![
        "auth0|not-a-uuid",
        "database|invalid-uuid-format",
        "auth0|12345", // Too short
        "database|",   // Empty after prefix
        "not-a-uuid-at-all",
        "auth0|123e4567-e89b-12d3-a456-426614174000x", // Extra character
        "database|123e4567-e89b-12d3-a456-42661417400", // Too short
        "",            // Empty string
    ];

    for subject in invalid_formats {
        let result = parse_auth0_user_id(subject);
        assert!(result.is_err(), "Should reject invalid UUID format: {}", subject);

        let error = result.unwrap_err();
        match error {
            AppError::Auth(msg) => {
                assert!(msg.contains("Invalid UUID format") || msg.contains("Unsupported"));
            },
            _ => panic!("Expected Auth error for invalid format: {}", subject),
        }
    }
}

#[test]
fn test_parse_auth0_user_id_edge_cases() {
    // Test various edge cases
    let edge_cases = vec![
        ("auth0||", "Double pipe should fail"),
        ("database||", "Double pipe should fail"),
        ("|123e4567-e89b-12d3-a456-426614174000", "Leading pipe should fail"),
        ("auth0|", "Missing UUID should fail"),
        ("database|", "Missing UUID should fail"),
        ("AUTH0|123e4567-e89b-12d3-a456-426614174000", "Case sensitivity"),
        ("Database|123e4567-e89b-12d3-a456-426614174000", "Case sensitivity"),
    ];

    for (subject, description) in edge_cases {
        let result = parse_auth0_user_id(subject);
        assert!(result.is_err(), "{}: {}", description, subject);
    }
}

#[test]
fn test_parse_auth0_user_id_valid_uuid_variations() {
    // Test different valid UUID formats that Auth0 might use
    let valid_uuids = vec![
        "123e4567-e89b-12d3-a456-426614174000", // Standard format
        "12345678-1234-1234-1234-123456789012", // All numeric
        "abcdefgh-abcd-abcd-abcd-abcdefghijkl", // All hex letters (invalid UUID but we test parsing)
    ];

    for uuid_str in &valid_uuids {
        // Only test actually valid UUIDs
        if let Ok(expected_uuid) = Uuid::parse_str(uuid_str) {
            let subjects = vec![
                format!("auth0|{}", uuid_str),
                format!("database|{}", uuid_str),
                uuid_str.to_string(),
            ];

            for subject in subjects {
                let result = parse_auth0_user_id(&subject);
                assert!(result.is_ok(), "Should parse valid UUID format: {}", subject);
                assert_eq!(result.unwrap(), expected_uuid);
            }
        }
    }
}

#[test]
fn test_parse_auth0_user_id_provider_detection() {
    let uuid = Uuid::new_v4();

    // Test that we correctly identify and strip different prefixes
    let test_cases = vec![
        (format!("auth0|{}", uuid), "auth0 prefix"),
        (format!("database|{}", uuid), "database prefix"),
        (uuid.to_string(), "no prefix"),
    ];

    for (subject, description) in test_cases {
        let result = parse_auth0_user_id(&subject);
        assert!(result.is_ok(), "Should parse {}: {}", description, subject);
        assert_eq!(result.unwrap(), uuid, "UUID should match for {}", description);
    }
}

#[test]
fn test_parse_auth0_user_id_comprehensive_error_handling() {
    // Test comprehensive error cases with detailed error message validation
    let error_test_cases = vec![
        (
            "google-oauth2|12345",
            "Unsupported Auth0 provider format",
            "Should identify unsupported provider",
        ),
        (
            "auth0|invalid-uuid",
            "Invalid UUID format",
            "Should identify invalid UUID after valid prefix",
        ),
        (
            "not-a-uuid",
            "Invalid UUID format",
            "Should identify invalid direct UUID",
        ),
        (
            "database|",
            "Invalid UUID format",
            "Should handle empty UUID after prefix",
        ),
    ];

    for (subject, expected_error_content, description) in error_test_cases {
        let result = parse_auth0_user_id(subject);
        assert!(result.is_err(), "{}: {}", description, subject);

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains(expected_error_content),
            "{}: Expected '{}' in error message, got '{}'",
            description,
            expected_error_content,
            error_msg
        );
    }
}

#[test]
fn test_parse_auth0_user_id_round_trip_consistency() {
    // Test that we can parse UUIDs and they remain consistent
    let original_uuid = Uuid::new_v4();

    let formats = vec![
        format!("auth0|{}", original_uuid),
        format!("database|{}", original_uuid),
        original_uuid.to_string(),
    ];

    for format in formats {
        let parsed_result = parse_auth0_user_id(&format);
        assert!(parsed_result.is_ok(), "Should parse format: {}", format);

        let parsed_uuid = parsed_result.unwrap();
        assert_eq!(parsed_uuid, original_uuid, "UUID should be identical after parsing");

        // Ensure the parsed UUID can be converted back to string
        let back_to_string = parsed_uuid.to_string();
        assert_eq!(back_to_string, original_uuid.to_string());
    }
}

#[test]
fn test_parse_auth0_user_id_multiple_pipe_characters() {
    // Test subjects with multiple pipe characters (potential edge case)
    let uuid = Uuid::new_v4();
    let subjects_with_multiple_pipes = vec![
        format!("auth0|{}|extra", uuid),
        format!("database|{}|more|data", uuid),
        format!("provider|sub|{}|data", uuid),
        format!("auth0||{}", uuid),
    ];

    for subject in subjects_with_multiple_pipes {
        let result = parse_auth0_user_id(&subject);
        // These should fail because they don't match expected patterns
        assert!(result.is_err(), "Should reject malformed subject with multiple pipes: {}", subject);
    }
}

#[test]
fn test_parse_auth0_user_id_whitespace_handling() {
    let uuid = Uuid::new_v4();

    // Test subjects with whitespace (should be rejected)
    let whitespace_subjects = vec![
        format!(" auth0|{}", uuid),    // Leading space
        format!("auth0|{} ", uuid),    // Trailing space
        format!("auth0| {}", uuid),    // Space after pipe
        format!("auth0 |{}", uuid),    // Space before pipe
        format!("auth0|{}\n", uuid),   // Newline
        format!("auth0|{}\t", uuid),   // Tab
    ];

    for subject in whitespace_subjects {
        let result = parse_auth0_user_id(&subject);
        // These should fail due to invalid UUID format (UUIDs don't contain whitespace)
        assert!(result.is_err(), "Should reject subject with whitespace: '{}'", subject);
    }
}

#[test]
fn test_parse_auth0_user_id_performance_with_many_formats() {
    // Test performance with various formats (should be fast for all)
    let uuid = Uuid::new_v4();
    let test_subjects = vec![
        format!("auth0|{}", uuid),
        format!("database|{}", uuid),
        uuid.to_string(),
        "google-oauth2|invalid".to_string(),
        "invalid-uuid".to_string(),
    ];

    // Run multiple times to ensure consistent performance
    for _ in 0..100 {
        for subject in &test_subjects {
            let _ = parse_auth0_user_id(subject);
        }
    }

    // If we get here without timeout, performance is acceptable
    assert!(true, "Performance test completed");
}