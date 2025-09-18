use std::collections::HashMap;
use std::sync::Arc;

use shared::{
    error::{AppError, Result},
    models::chat::{WebSocketMessage, MessageType, ChatMessage},
};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Routes messages between local connections and external services (SQS/SNS)
pub struct MessageRouter {
    /// SQS client for offline message storage
    sqs_client: aws_sdk_sqs::Client,

    /// SNS client for cross-instance notifications
    sns_client: aws_sdk_sns::Client,

    /// SQS queue URL for offline messages
    sqs_queue_url: String,

    /// SNS topic ARN for cross-instance notifications
    sns_topic_arn: String,

    /// Message TTL in seconds
    message_ttl_seconds: u64,

    /// Room membership tracking (room_id -> set of user_ids)
    room_membership: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>,
}

impl MessageRouter {
    /// Create a new message router
    pub fn new(
        sqs_client: aws_sdk_sqs::Client,
        sns_client: aws_sdk_sns::Client,
        sqs_queue_url: String,
        sns_topic_arn: String,
        message_ttl_seconds: u64,
    ) -> Self {
        Self {
            sqs_client,
            sns_client,
            sqs_queue_url,
            sns_topic_arn,
            message_ttl_seconds,
            room_membership: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Route a message to its intended recipients
    pub async fn route_message(
        &self,
        message: WebSocketMessage,
        sender_connection_id: Option<String>,
    ) -> Result<()> {
        match &message.message_type {
            MessageType::ChatMessage(chat_msg) => {
                self.route_chat_message(chat_msg, sender_connection_id).await
            }
            MessageType::JoinRoom { room_id } => {
                // Room join is handled by connection manager
                debug!("Join room message for room {}", room_id);
                Ok(())
            }
            MessageType::LeaveRoom { room_id } => {
                // Room leave is handled by connection manager
                debug!("Leave room message for room {}", room_id);
                Ok(())
            }
            MessageType::Heartbeat => {
                // Heartbeat is handled by connection manager
                debug!("Heartbeat message received");
                Ok(())
            }
            MessageType::UserTyping { room_id, user_id } => {
                self.route_typing_notification(*room_id, *user_id).await
            }
            MessageType::Error { code: _, message: _ } => {
                // Error messages are typically responses, not routed
                debug!("Error message received");
                Ok(())
            }
        }
    }

    /// Route a chat message to all room participants
    async fn route_chat_message(
        &self,
        chat_msg: &ChatMessage,
        sender_connection_id: Option<String>,
    ) -> Result<()> {
        // Get room participants
        let participants = self.get_room_participants(chat_msg.room_id).await?;

        if participants.is_empty() {
            warn!("No participants found for room {}", chat_msg.room_id);
            return Ok(());
        }

        // Create WebSocket message
        let ws_message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::ChatMessage(chat_msg.clone()),
            timestamp: chrono::Utc::now(),
        };

        // Route to local and remote participants
        self.route_to_participants(&ws_message, &participants, sender_connection_id).await
    }

    /// Route a typing notification to room participants
    async fn route_typing_notification(&self, room_id: Uuid, typing_user_id: Uuid) -> Result<()> {
        // Get room participants
        let participants = self.get_room_participants(room_id).await?;

        // Create typing notification message
        let ws_message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::UserTyping {
                room_id,
                user_id: typing_user_id,
            },
            timestamp: chrono::Utc::now(),
        };

        // Route to all participants except the typing user
        let filtered_participants: Vec<Uuid> = participants
            .into_iter()
            .filter(|&user_id| user_id != typing_user_id)
            .collect();

        self.route_to_participants(&ws_message, &filtered_participants, None).await
    }

    /// Route a message to a list of participants
    async fn route_to_participants(
        &self,
        message: &WebSocketMessage,
        participants: &[Uuid],
        sender_connection_id: Option<String>,
    ) -> Result<()> {
        // For now, we'll store messages in SQS for all participants
        // In a multi-instance setup, we would:
        // 1. Check which users are connected locally
        // 2. Send to local connections directly
        // 3. Send to SQS for offline users
        // 4. Send to SNS for users on other instances

        for &user_id in participants {
            if let Err(e) = self.send_to_user_queue(user_id, message).await {
                error!("Failed to queue message for user {}: {}", user_id, e);
            }
        }

        // Also publish to SNS for cross-instance delivery
        if let Err(e) = self.publish_to_sns(message).await {
            error!("Failed to publish message to SNS: {}", e);
        }

        info!(
            "Message routed to {} participants from connection {:?}",
            participants.len(),
            sender_connection_id
        );

        Ok(())
    }

    /// Send a message to a user's SQS queue
    async fn send_to_user_queue(&self, user_id: Uuid, message: &WebSocketMessage) -> Result<()> {
        let message_body = serde_json::to_string(message)
            .map_err(|e| AppError::Serialization(format!("Failed to serialize message: {}", e)))?;

        let send_result = self
            .sqs_client
            .send_message()
            .queue_url(&self.sqs_queue_url)
            .message_body(message_body)
            .message_attributes(
                "user_id",
                aws_sdk_sqs::types::MessageAttributeValue::builder()
                    .string_value(user_id.to_string())
                    .data_type("String")
                    .build()
                    .map_err(|e| AppError::Config(format!("Failed to build message attribute: {}", e)))?,
            )
            .message_attributes(
                "ttl",
                aws_sdk_sqs::types::MessageAttributeValue::builder()
                    .string_value(self.message_ttl_seconds.to_string())
                    .data_type("Number")
                    .build()
                    .map_err(|e| AppError::Config(format!("Failed to build TTL attribute: {}", e)))?,
            )
            .send()
            .await
            .map_err(|e| AppError::External(format!("SQS send failed: {}", e)))?;

        debug!(
            "Message queued for user {} with message ID {:?}",
            user_id,
            send_result.message_id()
        );

        Ok(())
    }

    /// Publish a message to SNS for cross-instance delivery
    async fn publish_to_sns(&self, message: &WebSocketMessage) -> Result<()> {
        let message_body = serde_json::to_string(message)
            .map_err(|e| AppError::Serialization(format!("Failed to serialize message: {}", e)))?;

        let publish_result = self
            .sns_client
            .publish()
            .topic_arn(&self.sns_topic_arn)
            .message(message_body)
            .message_attributes(
                "message_type",
                aws_sdk_sns::types::MessageAttributeValue::builder()
                    .string_value("chat_message")
                    .data_type("String")
                    .build()
                    .map_err(|e| AppError::Config(format!("Failed to build message attribute: {}", e)))?,
            )
            .send()
            .await
            .map_err(|e| AppError::External(format!("SNS publish failed: {}", e)))?;

        debug!(
            "Message published to SNS with message ID {:?}",
            publish_result.message_id()
        );

        Ok(())
    }

    /// Get participants for a room from local cache
    async fn get_room_participants(&self, room_id: Uuid) -> Result<Vec<Uuid>> {
        let membership = self.room_membership.read().await;
        Ok(membership.get(&room_id).cloned().unwrap_or_default())
    }

    /// Add a user to a room
    pub async fn join_room(&self, user_id: Uuid, room_id: Uuid) -> Result<()> {
        let mut membership = self.room_membership.write().await;
        let participants = membership.entry(room_id).or_insert_with(Vec::new);

        if !participants.contains(&user_id) {
            participants.push(user_id);
            info!("User {} joined room {}", user_id, room_id);
        }

        Ok(())
    }

    /// Remove a user from a room
    pub async fn leave_room(&self, user_id: Uuid, room_id: Uuid) -> Result<()> {
        let mut membership = self.room_membership.write().await;

        if let Some(participants) = membership.get_mut(&room_id) {
            participants.retain(|&id| id != user_id);

            // Remove empty rooms
            if participants.is_empty() {
                membership.remove(&room_id);
            }

            info!("User {} left room {}", user_id, room_id);
        }

        Ok(())
    }

    /// Get room membership statistics
    pub async fn get_room_stats(&self) -> HashMap<Uuid, usize> {
        let membership = self.room_membership.read().await;
        membership
            .iter()
            .map(|(&room_id, participants)| (room_id, participants.len()))
            .collect()
    }

    /// Process incoming SQS messages for a user
    pub async fn process_user_messages(&self, user_id: Uuid) -> Result<Vec<WebSocketMessage>> {
        // This would typically be called when a user connects
        // to retrieve any offline messages from SQS

        // For now, return empty - implementation depends on SQS polling strategy
        debug!("Processing messages for user {}", user_id);
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::models::chat::{ChatMessage, MessageType, WebSocketMessage};

    #[tokio::test]
    async fn test_room_membership() {
        // Create router with mock AWS clients (in real tests, we'd use localstack)
        let config = aws_config::from_env().load().await;
        let sqs_client = aws_sdk_sqs::Client::new(&config);
        let sns_client = aws_sdk_sns::Client::new(&config);

        let router = MessageRouter::new(
            sqs_client,
            sns_client,
            "test-queue".to_string(),
            "test-topic".to_string(),
            3600,
        );

        let user_id = Uuid::new_v4();
        let room_id = Uuid::new_v4();

        // Test join room
        router.join_room(user_id, room_id).await.unwrap();
        let participants = router.get_room_participants(room_id).await.unwrap();
        assert_eq!(participants.len(), 1);
        assert!(participants.contains(&user_id));

        // Test leave room
        router.leave_room(user_id, room_id).await.unwrap();
        let participants = router.get_room_participants(room_id).await.unwrap();
        assert_eq!(participants.len(), 0);
    }

    #[test]
    fn test_message_serialization() {
        let chat_msg = ChatMessage {
            id: Uuid::new_v4(),
            room_id: Uuid::new_v4(),
            sender_id: Uuid::new_v4(),
            content: "test message".to_string(),
            encrypted_content: None,
            message_type: "text".to_string(),
            created_at: chrono::Utc::now(),
            edited_at: None,
            thread_id: None,
            reply_to_id: None,
        };

        let ws_message = WebSocketMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::ChatMessage(chat_msg),
            timestamp: chrono::Utc::now(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&ws_message).unwrap();
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized: WebSocketMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.id, ws_message.id);
    }
}