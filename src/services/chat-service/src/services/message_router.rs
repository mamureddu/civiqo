use std::collections::HashMap;
use std::sync::Arc;

use shared::{
    error::{AppError, Result},
    models::chat::WebSocketMessage,
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Routes messages between local connections.
/// In single-instance mode (VPS), all routing is in-process.
#[allow(dead_code)]
pub struct MessageRouter {
    /// SQS queue URL (optional — disabled when None)
    sqs_queue_url: Option<String>,

    /// SNS topic ARN (optional — disabled when None)
    sns_topic_arn: Option<String>,

    /// Message TTL in seconds
    message_ttl_seconds: u64,

    /// Room membership tracking (room_id -> set of user_ids)
    room_membership: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>,
}

impl MessageRouter {
    pub fn new(
        sqs_queue_url: Option<String>,
        sns_topic_arn: Option<String>,
        message_ttl_seconds: u64,
    ) -> Self {
        if sqs_queue_url.is_none() {
            info!("SQS disabled — running in local-only mode");
        }
        if sns_topic_arn.is_none() {
            info!("SNS disabled — running in local-only mode");
        }

        Self {
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
        match &message {
            WebSocketMessage::ReceiveMessage { .. } => {
                self.route_receive_message(&message, sender_connection_id)
                    .await
            }
            WebSocketMessage::JoinRoom { room_id } => {
                debug!("Join room message for room {}", room_id);
                Ok(())
            }
            WebSocketMessage::LeaveRoom { room_id } => {
                debug!("Leave room message for room {}", room_id);
                Ok(())
            }
            WebSocketMessage::Heartbeat => {
                debug!("Heartbeat message received");
                Ok(())
            }
            WebSocketMessage::TypingStart { room_id, user_id }
            | WebSocketMessage::TypingStop { room_id, user_id } => {
                self.route_typing_notification(*room_id, *user_id, &message)
                    .await
            }
            WebSocketMessage::Error { .. } => {
                debug!("Error message received");
                Ok(())
            }
            _ => {
                debug!("Unsupported message type for routing");
                Ok(())
            }
        }
    }

    async fn route_receive_message(
        &self,
        message: &WebSocketMessage,
        sender_connection_id: Option<String>,
    ) -> Result<()> {
        let room_id = match message {
            WebSocketMessage::ReceiveMessage { room_id, .. } => *room_id,
            _ => return Err(AppError::Internal(anyhow::anyhow!("Invalid message type"))),
        };

        let participants = self.get_room_participants(room_id).await?;

        if participants.is_empty() {
            warn!("No participants found for room {}", room_id);
            return Ok(());
        }

        info!(
            "Message routed to {} participants from connection {:?}",
            participants.len(),
            sender_connection_id
        );

        Ok(())
    }

    async fn route_typing_notification(
        &self,
        room_id: Uuid,
        _typing_user_id: Uuid,
        _message: &WebSocketMessage,
    ) -> Result<()> {
        let _participants = self.get_room_participants(room_id).await?;
        Ok(())
    }

    pub async fn get_room_participants(&self, room_id: Uuid) -> Result<Vec<Uuid>> {
        let membership = self.room_membership.read().await;
        Ok(membership.get(&room_id).cloned().unwrap_or_default())
    }

    pub async fn join_room(&self, user_id: Uuid, room_id: Uuid) -> Result<()> {
        let mut membership = self.room_membership.write().await;
        let participants = membership.entry(room_id).or_insert_with(Vec::new);

        if !participants.contains(&user_id) {
            participants.push(user_id);
            info!("User {} joined room {}", user_id, room_id);
        }

        Ok(())
    }

    pub async fn leave_room(&self, user_id: Uuid, room_id: Uuid) -> Result<()> {
        let mut membership = self.room_membership.write().await;

        if let Some(participants) = membership.get_mut(&room_id) {
            participants.retain(|&id| id != user_id);

            if participants.is_empty() {
                membership.remove(&room_id);
            }

            info!("User {} left room {}", user_id, room_id);
        }

        Ok(())
    }
}
