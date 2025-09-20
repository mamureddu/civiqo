use shared::error::{AppError, Result};
use uuid::Uuid;

/// Message validation service for checking message content and metadata
#[derive(Debug)]
pub struct MessageValidator {
    /// Maximum allowed message size in bytes
    max_message_size: usize,
}

impl MessageValidator {
    /// Create a new message validator with the specified size limit
    pub fn new(max_message_size: usize) -> Self {
        Self { max_message_size }
    }

    /// Validate a message before processing
    pub fn validate_message(
        &self,
        encrypted_content: &str,
        room_id: Option<Uuid>,
        recipient_id: Option<Uuid>,
        sender_user_id: Uuid,
    ) -> Result<()> {
        // Validate message size
        self.validate_message_size(encrypted_content)?;

        // Validate target specification (either room or direct message)
        self.validate_message_target(room_id, recipient_id)?;

        // Validate sender (prevent empty UUIDs)
        self.validate_sender(sender_user_id)?;

        // Validate content format (basic checks)
        self.validate_content_format(encrypted_content)?;

        Ok(())
    }

    /// Validate typing notification
    pub fn validate_typing_notification(
        &self,
        room_id: Option<Uuid>,
        recipient_id: Option<Uuid>,
        typing_user_id: Uuid,
    ) -> Result<()> {
        // Validate target specification
        self.validate_message_target(room_id, recipient_id)?;

        // Validate typing user ID
        self.validate_sender(typing_user_id)?;

        Ok(())
    }

    /// Validate key exchange request
    pub fn validate_key_exchange(
        &self,
        recipient_id: Uuid,
        sender_user_id: Uuid,
        public_key: &str,
    ) -> Result<()> {
        // Validate sender and recipient are different
        if sender_user_id == recipient_id {
            return Err(AppError::Validation(
                "Cannot exchange keys with yourself".to_string(),
            ));
        }

        // Validate public key format (basic length check)
        if public_key.is_empty() {
            return Err(AppError::Validation("Public key cannot be empty".to_string()));
        }

        if public_key.len() > 1024 {
            // Reasonable limit for public keys
            return Err(AppError::Validation("Public key too large".to_string()));
        }

        // Basic format check - should be base64 or similar encoding
        if !public_key.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=' || c == '_') {
            return Err(AppError::Validation(
                "Public key contains invalid characters".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate message size
    fn validate_message_size(&self, content: &str) -> Result<()> {
        let size = content.len();
        if size > self.max_message_size {
            return Err(AppError::Validation(format!(
                "Message too large: {} bytes (max: {} bytes)",
                size, self.max_message_size
            )));
        }
        Ok(())
    }

    /// Validate message target (room or recipient)
    fn validate_message_target(&self, room_id: Option<Uuid>, recipient_id: Option<Uuid>) -> Result<()> {
        match (room_id, recipient_id) {
            (Some(_), Some(_)) => Err(AppError::Validation(
                "Cannot specify both room_id and recipient_id".to_string(),
            )),
            (None, None) => Err(AppError::Validation(
                "Must specify either room_id or recipient_id".to_string(),
            )),
            (Some(room_id), None) => {
                // Validate room ID is not nil UUID
                if room_id == Uuid::nil() {
                    Err(AppError::Validation("Invalid room_id: cannot be nil UUID".to_string()))
                } else {
                    Ok(())
                }
            },
            (None, Some(recipient_id)) => {
                // Validate recipient ID is not nil UUID
                if recipient_id == Uuid::nil() {
                    Err(AppError::Validation("Invalid recipient_id: cannot be nil UUID".to_string()))
                } else {
                    Ok(())
                }
            },
        }
    }

    /// Validate sender user ID
    fn validate_sender(&self, user_id: Uuid) -> Result<()> {
        if user_id == Uuid::nil() {
            return Err(AppError::Validation("Invalid user_id: cannot be nil UUID".to_string()));
        }
        Ok(())
    }

    /// Validate encrypted content format
    fn validate_content_format(&self, content: &str) -> Result<()> {
        // Check for empty content
        if content.is_empty() {
            return Err(AppError::Validation("Message content cannot be empty".to_string()));
        }

        // Check for excessively long lines (potential attack vector)
        for line in content.lines() {
            if line.len() > 2048 {
                return Err(AppError::Validation(
                    "Message contains excessively long lines".to_string(),
                ));
            }
        }

        // Basic format check for encrypted content (should be base64-like or JSON-like)
        // Allow normal whitespace (space, \n, \t) but reject control characters like \0, \r
        let valid_chars = content
            .chars()
            .all(|c| {
                if c.is_whitespace() {
                    c == ' ' || c == '\n' || c == '\t'
                } else {
                    c.is_alphanumeric() ||
                    c == '+' || c == '/' || c == '=' ||
                    c == '{' || c == '}' || c == '"' || c == ':' || c == ',' ||
                    c == '_' || c == '-' || c == '.'
                }
            });

        if !valid_chars {
            return Err(AppError::Validation(
                "Message contains invalid characters for encrypted content".to_string(),
            ));
        }

        Ok(())
    }

    /// Get maximum message size (for testing)
    pub fn max_message_size(&self) -> usize {
        self.max_message_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_validator_creation() {
        let validator = MessageValidator::new(1024);
        assert_eq!(validator.max_message_size, 1024);
    }

    #[test]
    fn test_valid_room_message() {
        let validator = MessageValidator::new(1024);
        let room_id = Uuid::new_v4();
        let sender_id = Uuid::new_v4();
        let content = "encrypted_message_content";

        let result = validator.validate_message(content, Some(room_id), None, sender_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_direct_message() {
        let validator = MessageValidator::new(1024);
        let recipient_id = Uuid::new_v4();
        let sender_id = Uuid::new_v4();
        let content = "encrypted_message_content";

        let result = validator.validate_message(content, None, Some(recipient_id), sender_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_message_too_large() {
        let validator = MessageValidator::new(10); // Very small limit
        let room_id = Uuid::new_v4();
        let sender_id = Uuid::new_v4();
        let content = "this_message_is_too_long_for_the_limit";

        let result = validator.validate_message(content, Some(room_id), None, sender_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Message too large"));
    }

    #[test]
    fn test_both_room_and_recipient_specified() {
        let validator = MessageValidator::new(1024);
        let room_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let sender_id = Uuid::new_v4();
        let content = "test";

        let result = validator.validate_message(content, Some(room_id), Some(recipient_id), sender_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot specify both"));
    }

    #[test]
    fn test_neither_room_nor_recipient_specified() {
        let validator = MessageValidator::new(1024);
        let sender_id = Uuid::new_v4();
        let content = "test";

        let result = validator.validate_message(content, None, None, sender_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Must specify either"));
    }

    #[test]
    fn test_nil_uuid_validation() {
        let validator = MessageValidator::new(1024);
        let content = "test";

        // Test nil room_id
        let result = validator.validate_message(content, Some(Uuid::nil()), None, Uuid::new_v4());
        assert!(result.is_err());

        // Test nil recipient_id
        let result = validator.validate_message(content, None, Some(Uuid::nil()), Uuid::new_v4());
        assert!(result.is_err());

        // Test nil sender_id
        let result = validator.validate_message(content, Some(Uuid::new_v4()), None, Uuid::nil());
        assert!(result.is_err());
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
    fn test_typing_notification_validation() {
        let validator = MessageValidator::new(1024);
        let room_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Valid typing notification
        let result = validator.validate_typing_notification(Some(room_id), None, user_id);
        assert!(result.is_ok());

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
    }

    #[test]
    fn test_excessively_long_lines() {
        let validator = MessageValidator::new(10000); // Large overall limit
        let room_id = Uuid::new_v4();
        let sender_id = Uuid::new_v4();

        // Create content with a very long line
        let long_line = "a".repeat(3000); // Longer than 2048 char limit per line

        let result = validator.validate_message(&long_line, Some(room_id), None, sender_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("excessively long lines"));
    }
}