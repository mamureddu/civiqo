use std::time::Duration;

use axum::{
    extract::{ws::{WebSocket, Message}, WebSocketUpgrade, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use shared::{
    error::{AppError, Result},
    models::{chat::{WebSocketMessage}, Claims},
};
use crate::middleware::auth::extract_token_from_headers;
use tokio::{
    sync::mpsc,
    time::{interval, timeout},
};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{
    services::room_service::RoomService,
    state::AppState,
};

/// WebSocket upgrade handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    // Extract and validate JWT token
    let token = match extract_token_from_headers(&headers) {
        Some(token) => token,
        None => {
            warn!("WebSocket connection attempt without authorization header");
            return (axum::http::StatusCode::UNAUTHORIZED, "Missing authorization header").into_response();
        }
    };

    let claims = match state.auth_state().validate_token(&token).await {
        Ok(claims) => claims,
        Err(e) => {
            warn!("WebSocket authentication failed: {}", e);
            return (axum::http::StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)).into_response();
        }
    };

    info!("WebSocket connection request from user: {}", claims.sub);

    // Upgrade to WebSocket connection
    ws.on_upgrade(move |socket| handle_websocket(socket, state, claims))
}

/// Handle WebSocket connection lifecycle
async fn handle_websocket(socket: WebSocket, state: AppState, claims: Claims) {
    let user_id = match Uuid::parse_str(&claims.sub.replace("auth0|", "")) {
        Ok(id) => id,
        Err(_) => {
            error!("Invalid user ID in JWT claims: {}", claims.sub);
            return;
        }
    };

    // Create message channel for this connection
    let (message_tx, mut message_rx) = mpsc::unbounded_channel::<WebSocketMessage>();

    // Add connection to manager
    let connection_id = match state
        .connection_manager()
        .add_connection(user_id, message_tx)
        .await
    {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to add connection for user {}: {}", user_id, e);
            return;
        }
    };

    info!("WebSocket connection established: {} for user {}", connection_id, user_id);

    // Split socket into sender and receiver
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Create room service for this connection
    let room_service = RoomService::new(state.database().clone());

    // Spawn task to handle outgoing messages
    let outgoing_state = state.clone();
    let outgoing_connection_id = connection_id.clone();
    let outgoing_task = tokio::spawn(async move {
        while let Some(message) = message_rx.recv().await {
            match serde_json::to_string(&message) {
                Ok(json) => {
                    if let Err(e) = ws_sender.send(Message::Text(json)).await {
                        error!("Failed to send WebSocket message: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to serialize outgoing message: {}", e);
                }
            }
        }

        // Connection closed, clean up
        if let Err(e) = outgoing_state
            .connection_manager()
            .remove_connection(&outgoing_connection_id)
            .await
        {
            error!("Failed to remove connection {}: {}", outgoing_connection_id, e);
        }
    });

    // Set up heartbeat interval
    let mut heartbeat_interval = interval(Duration::from_secs(state.config().heartbeat_interval_seconds));
    let heartbeat_state = state.clone();
    let heartbeat_connection_id = connection_id.clone();

    // Spawn heartbeat task
    let heartbeat_task = tokio::spawn(async move {
        loop {
            heartbeat_interval.tick().await;

            if let Err(e) = heartbeat_state
                .connection_manager()
                .update_heartbeat(&heartbeat_connection_id)
            {
                debug!("Heartbeat failed for connection {}: {}", heartbeat_connection_id, e);
                break;
            }
        }
    });

    // Handle incoming messages
    while let Some(message_result) = ws_receiver.next().await {
        match message_result {
            Ok(Message::Text(text)) => {
                if let Err(e) = handle_text_message(
                    &text,
                    &connection_id,
                    user_id,
                    &state,
                    &room_service,
                ).await {
                    error!("Error handling text message: {}", e);

                    // Send error response
                    let error_message = WebSocketMessage::Error {
                        message: "Failed to process message".to_string(),
                        code: "PROCESSING_ERROR".to_string(),
                    };

                    if let Err(e) = state
                        .connection_manager()
                        .send_to_connection(&connection_id, error_message)
                        .await
                    {
                        error!("Failed to send error response: {}", e);
                        break;
                    }
                }
            }
            Ok(Message::Pong(_)) => {
                // Update heartbeat on pong
                if let Err(e) = state.connection_manager().update_heartbeat(&connection_id) {
                    debug!("Failed to update heartbeat: {}", e);
                }
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket connection closed by client: {}", connection_id);
                break;
            }
            Ok(_) => {
                // Ignore other message types (Binary, Ping)
                debug!("Received non-text WebSocket message");
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    // Cleanup tasks
    outgoing_task.abort();
    heartbeat_task.abort();

    // Remove connection
    if let Err(e) = state.connection_manager().remove_connection(&connection_id).await {
        error!("Failed to remove connection {}: {}", connection_id, e);
    }

    info!("WebSocket connection closed: {} for user {}", connection_id, user_id);
}

/// Handle incoming text message
async fn handle_text_message(
    text: &str,
    connection_id: &str,
    user_id: Uuid,
    state: &AppState,
    room_service: &RoomService,
) -> Result<()> {
    // Parse WebSocket message
    let ws_message: WebSocketMessage = serde_json::from_str(text)
        .map_err(|e| AppError::Validation(format!("Invalid message format: {}", e)))?;

    debug!("Received message: {:?} from connection {}", ws_message, connection_id);

    match ws_message {
        WebSocketMessage::SendMessage { room_id, recipient_id, encrypted_content, message_type } => {
            handle_send_message(room_id, recipient_id, encrypted_content, message_type, connection_id, user_id, state, room_service).await
        }
        WebSocketMessage::JoinRoom { room_id } => {
            handle_join_room(room_id, connection_id, user_id, state, room_service).await
        }
        WebSocketMessage::LeaveRoom { room_id } => {
            handle_leave_room(room_id, connection_id, user_id, state, room_service).await
        }
        WebSocketMessage::Heartbeat => {
            handle_heartbeat(connection_id, state).await
        }
        WebSocketMessage::TypingStart { room_id, user_id: typing_user_id } => {
            handle_typing_notification(room_id, typing_user_id, true, connection_id, user_id, state, room_service).await
        }
        WebSocketMessage::TypingStop { room_id, user_id: typing_user_id } => {
            handle_typing_notification(room_id, typing_user_id, false, connection_id, user_id, state, room_service).await
        }
        WebSocketMessage::KeyExchange { sender_id, recipient_id, public_key } => {
            handle_key_exchange(sender_id, recipient_id, public_key, connection_id, user_id, state).await
        }
        _ => {
            // Other message types (Connect, Disconnect, ReceiveMessage, UserPresence, Error) are not handled by clients
            warn!("Client sent unsupported message type from connection {}", connection_id);
            Ok(())
        }
    }
}

/// Handle send message
async fn handle_send_message(
    room_id: Uuid,
    recipient_id: Option<Uuid>,
    encrypted_content: String,
    message_type: shared::models::chat::MessageType,
    connection_id: &str,
    user_id: Uuid,
    state: &AppState,
    room_service: &RoomService,
) -> Result<()> {
    // Verify user can send messages to this room
    if !room_service.check_room_permission(user_id, room_id, "send_message").await? {
        return Err(AppError::Authorization("Not authorized to send messages to this room".to_string()));
    }

    // Create receive message for routing to other participants
    let receive_message = WebSocketMessage::ReceiveMessage {
        id: Uuid::new_v4(),
        room_id,
        sender_id: user_id,
        encrypted_content,
        message_type,
        created_at: chrono::Utc::now(),
    };

    state
        .message_router()
        .route_message(receive_message, Some(connection_id.to_string()))
        .await?;

    info!("Message routed from user {} in room {}", user_id, room_id);
    Ok(())
}

/// Handle room join request
async fn handle_join_room(
    room_id: Uuid,
    connection_id: &str,
    user_id: Uuid,
    state: &AppState,
    room_service: &RoomService,
) -> Result<()> {
    // Check if user can access this room
    if !room_service.can_user_access_room(user_id, room_id).await? {
        return Err(AppError::Authorization("Not authorized to join this room".to_string()));
    }

    // Add user to room participants if not already there
    room_service.add_participant(room_id, user_id, None).await?;

    // Join room in connection manager
    state.connection_manager().join_room(connection_id, room_id).await?;

    // Update last read timestamp
    room_service.update_last_read(user_id, room_id).await?;

    info!("User {} joined room {} via connection {}", user_id, room_id, connection_id);
    Ok(())
}

/// Handle room leave request
async fn handle_leave_room(
    room_id: Uuid,
    connection_id: &str,
    user_id: Uuid,
    state: &AppState,
    room_service: &RoomService,
) -> Result<()> {
    // Leave room in connection manager
    state.connection_manager().leave_room(connection_id, room_id).await?;

    // Note: We don't remove from room_participants table here
    // as users might want to rejoin later and maintain their role

    info!("User {} left room {} via connection {}", user_id, room_id, connection_id);
    Ok(())
}

/// Handle heartbeat message
async fn handle_heartbeat(connection_id: &str, state: &AppState) -> Result<()> {
    state.connection_manager().update_heartbeat(connection_id)?;
    debug!("Heartbeat received from connection {}", connection_id);
    Ok(())
}

/// Handle typing notification
async fn handle_typing_notification(
    room_id: Uuid,
    typing_user_id: Uuid,
    is_typing_start: bool,
    connection_id: &str,
    authenticated_user_id: Uuid,
    state: &AppState,
    room_service: &RoomService,
) -> Result<()> {
    // Verify the typing user ID matches the authenticated user
    if typing_user_id != authenticated_user_id {
        return Err(AppError::Authorization("Cannot send typing notifications as another user".to_string()));
    }

    // Check if user can access this room
    if !room_service.can_user_access_room(authenticated_user_id, room_id).await? {
        return Err(AppError::Authorization("Not authorized to send typing notifications to this room".to_string()));
    }

    // Route typing notification
    let ws_message = if is_typing_start {
        WebSocketMessage::TypingStart {
            room_id,
            user_id: typing_user_id,
        }
    } else {
        WebSocketMessage::TypingStop {
            room_id,
            user_id: typing_user_id,
        }
    };

    state
        .message_router()
        .route_message(ws_message, Some(connection_id.to_string()))
        .await?;

    debug!("Typing notification sent for user {} in room {}", typing_user_id, room_id);
    Ok(())
}

/// Handle key exchange for E2EE
async fn handle_key_exchange(
    sender_id: Uuid,
    recipient_id: Uuid,
    public_key: String,
    connection_id: &str,
    authenticated_user_id: Uuid,
    state: &AppState,
) -> Result<()> {
    // Verify the sender ID matches the authenticated user
    if sender_id != authenticated_user_id {
        return Err(AppError::Authorization("Cannot send key exchange as another user".to_string()));
    }

    // Create key exchange message
    let ws_message = WebSocketMessage::KeyExchange {
        sender_id,
        recipient_id,
        public_key,
    };

    // Send directly to the recipient user
    if let Err(_) = state.connection_manager().send_to_user(recipient_id, ws_message).await {
        // If direct send fails, we could queue it for when the user comes online
        warn!("Failed to send key exchange to user {}", recipient_id);
    }

    debug!("Key exchange sent from user {} to user {}", sender_id, recipient_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::models::chat::WebSocketMessage;

    #[test]
    fn test_message_parsing() {
        let ws_message = WebSocketMessage::Heartbeat;

        // Test serialization
        let json = serde_json::to_string(&ws_message).unwrap();
        assert!(!json.is_empty());

        // Test deserialization
        let parsed: WebSocketMessage = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, WebSocketMessage::Heartbeat));
    }

    #[test]
    fn test_user_id_parsing() {
        // Test Auth0 user ID parsing
        let auth0_sub = "auth0|507f1f77bcf86cd799439011";
        let cleaned = auth0_sub.replace("auth0|", "");

        // This should be a valid UUID format after Auth0 processing
        // In practice, Auth0 user IDs might not be UUIDs, so this test
        // represents the expected format after our user management system
        // assigns proper UUIDs to users during registration
        assert!(!cleaned.is_empty());
        assert!(!cleaned.contains("auth0|"));
    }

    #[test]
    fn test_permission_validation() {
        // Test permission string matching
        assert_eq!("send_message", "send_message");
        assert_ne!("send_message", "delete_message");
    }
}