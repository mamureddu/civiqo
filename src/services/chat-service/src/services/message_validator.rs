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
}
