use std::sync::Arc;
use std::time::{Duration, Instant};

use dashmap::DashMap;
use shared::{
    // database::Database,  // Uncomment when implementing persistent storage
    error::{AppError, Result},
    models::chat::{ConnectionInfo, WebSocketMessage},
};
use tokio::{
    sync::mpsc,
    time::interval,
};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::message_router::MessageRouter;

/// Information about an active WebSocket connection
#[derive(Debug)]
pub struct ActiveConnection {
    /// User ID
    pub user_id: Uuid,

    // ==========================================================
    // COMMENTED FIELD - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    /// Connection ID
    /// USAGE: When implementing connection tracking across multiple instances
    /// PURPOSE: Unique identifier for WebSocket connection debugging and management
    // pub connection_id: String,

    /// Channel for sending messages to this connection
    pub sender: mpsc::UnboundedSender<WebSocketMessage>,

    /// Timestamp of last heartbeat
    pub last_heartbeat: Instant,

    /// Currently joined rooms
    pub joined_rooms: Vec<Uuid>,
}

impl ActiveConnection {
    pub fn new(
        user_id: Uuid,
        // connection_id: String,  // Uncomment when implementing connection tracking
        sender: mpsc::UnboundedSender<WebSocketMessage>,
    ) -> Self {
        Self {
            user_id,
            // connection_id,      // Uncomment when implementing connection tracking
            sender,
            last_heartbeat: Instant::now(),
            joined_rooms: Vec::new(),
        }
    }

    /// Update the last heartbeat timestamp
    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = Instant::now();
    }

    /// Check if connection is expired based on heartbeat timeout
    pub fn is_expired(&self, timeout_duration: Duration) -> bool {
        self.last_heartbeat.elapsed() > timeout_duration
    }

    /// Add a room to the joined rooms list
    pub fn join_room(&mut self, room_id: Uuid) {
        if !self.joined_rooms.contains(&room_id) {
            self.joined_rooms.push(room_id);
        }
    }

    /// Remove a room from the joined rooms list
    pub fn leave_room(&mut self, room_id: Uuid) {
        self.joined_rooms.retain(|&id| id != room_id);
    }
}

/// Manages WebSocket connections and their lifecycle
pub struct ConnectionManager {
    /// Active connections mapped by connection ID
    connections: Arc<DashMap<String, ActiveConnection>>,

    // ==========================================================
    // COMMENTED FIELD - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    /// Database for persistent connection tracking
    /// USAGE: When implementing cross-instance connection synchronization
    /// PURPOSE: Store connection state for recovery and analytics
    // database: Database,

    /// Message router for handling message delivery
    message_router: Arc<MessageRouter>,

    /// Maximum number of concurrent connections
    max_connections: usize,

    /// Heartbeat timeout duration
    heartbeat_timeout: Duration,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new(
        // database: Database,  // Uncomment when implementing persistent storage
        message_router: Arc<MessageRouter>,
        max_connections: usize,
        heartbeat_interval_seconds: u64,
    ) -> Self {
        let heartbeat_timeout = Duration::from_secs(heartbeat_interval_seconds * 3); // 3x heartbeat interval

        let manager = Self {
            connections: Arc::new(DashMap::new()),
            // database,            // Uncomment when implementing persistent storage
            message_router,
            max_connections,
            heartbeat_timeout,
        };

        // Start background tasks
        manager.start_cleanup_task();

        manager
    }

    /// Add a new WebSocket connection
    pub async fn add_connection(
        &self,
        user_id: Uuid,
        sender: mpsc::UnboundedSender<WebSocketMessage>,
    ) -> Result<String> {
        // Check connection limit
        if self.connections.len() >= self.max_connections {
            return Err(AppError::Validation("Too many concurrent connections".to_string()));
        }

        // Generate unique connection ID
        let connection_id = format!("conn_{}", Uuid::new_v4());

        // Create connection info
        let connection = ActiveConnection::new(
            user_id, 
            // connection_id.clone(),  // Uncomment when implementing connection tracking
            sender
        );

        // Store in memory
        self.connections.insert(connection_id.clone(), connection);

        // Store in database for stateless tracking
        let connection_info = ConnectionInfo {
            user_id,
            connection_id: connection_id.clone(),
            rooms: Vec::new(),
            connected_at: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
        };

        if let Err(e) = self.store_connection_info(&connection_info).await {
            error!("Failed to store connection info in database: {}", e);
            // Continue anyway - in-memory tracking is sufficient for basic functionality
        }

        info!("New connection added: {} for user {}", connection_id, user_id);
        Ok(connection_id)
    }

    /// Remove a WebSocket connection
    pub async fn remove_connection(&self, connection_id: &str) -> Result<()> {
        if let Some((_, connection)) = self.connections.remove(connection_id) {
            info!("Connection removed: {} for user {}", connection_id, connection.user_id);

            // Remove from database
            if let Err(e) = self.remove_connection_info(connection_id).await {
                error!("Failed to remove connection info from database: {}", e);
            }

            // Leave all rooms
            for room_id in &connection.joined_rooms {
                if let Err(e) = self.message_router.leave_room(connection.user_id, *room_id).await {
                    error!("Failed to leave room {} for user {}: {}", room_id, connection.user_id, e);
                }
            }
        }

        Ok(())
    }

    /// Send a message to a specific connection
    pub async fn send_to_connection(
        &self,
        connection_id: &str,
        message: WebSocketMessage,
    ) -> Result<()> {
        if let Some(connection) = self.connections.get(connection_id) {
            if let Err(_) = connection.sender.send(message) {
                warn!("Failed to send message to connection {}, removing", connection_id);
                drop(connection); // Release the reference before removal
                self.remove_connection(connection_id).await?;
                return Err(AppError::ExternalService("Connection closed".to_string()));
            }
        } else {
            return Err(AppError::NotFound(format!("Connection {} not found", connection_id)));
        }

        Ok(())
    }

    /// Send a message to all connections for a specific user
    pub async fn send_to_user(&self, user_id: Uuid, message: WebSocketMessage) -> Result<usize> {
        let mut sent_count = 0;
        let mut failed_connections = Vec::new();

        // Find all connections for this user
        for entry in self.connections.iter() {
            if entry.value().user_id == user_id {
                if let Err(_) = entry.value().sender.send(message.clone()) {
                    failed_connections.push(entry.key().clone());
                } else {
                    sent_count += 1;
                }
            }
        }

        // Clean up failed connections
        for connection_id in failed_connections {
            self.remove_connection(&connection_id).await?;
        }

        Ok(sent_count)
    }

    /// Update heartbeat for a connection
    pub fn update_heartbeat(&self, connection_id: &str) -> Result<()> {
        if let Some(mut connection) = self.connections.get_mut(connection_id) {
            connection.update_heartbeat();
            debug!("Heartbeat updated for connection {}", connection_id);
        } else {
            return Err(AppError::NotFound(format!("Connection {} not found", connection_id)));
        }

        Ok(())
    }

    /// Join a room
    pub async fn join_room(&self, connection_id: &str, room_id: Uuid) -> Result<()> {
        if let Some(mut connection) = self.connections.get_mut(connection_id) {
            connection.join_room(room_id);

            // Notify message router
            self.message_router
                .join_room(connection.user_id, room_id)
                .await?;

            info!("Connection {} joined room {}", connection_id, room_id);
        } else {
            return Err(AppError::NotFound(format!("Connection {} not found", connection_id)));
        }

        Ok(())
    }

    /// Leave a room
    pub async fn leave_room(&self, connection_id: &str, room_id: Uuid) -> Result<()> {
        if let Some(mut connection) = self.connections.get_mut(connection_id) {
            connection.leave_room(room_id);

            // Notify message router
            self.message_router
                .leave_room(connection.user_id, room_id)
                .await?;

            info!("Connection {} left room {}", connection_id, room_id);
        } else {
            return Err(AppError::NotFound(format!("Connection {} not found", connection_id)));
        }

        Ok(())
    }

    // ==========================================================
    // COMMENTED METHODS - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // /// Get connection count
    // /// USAGE: When implementing monitoring and metrics
    // /// PURPOSE: Track active WebSocket connections for scaling decisions
    // pub fn connection_count(&self) -> usize {
    //     self.connections.len()
    // }

    // /// Get connections for a specific user
    // /// USAGE: When implementing multi-device support or connection management
    // /// PURPOSE: Allow users to manage multiple active connections
    // pub fn get_user_connections(&self, user_id: Uuid) -> Vec<String> {
    //     self.connections
    //         .iter()
    //         .filter(|entry| entry.value().user_id == user_id)
    //         .map(|entry| entry.key().clone())
    //         .collect()
    // }

    /// Start background cleanup task for expired connections
    fn start_cleanup_task(&self) {
        let connections = Arc::clone(&self.connections);
        let heartbeat_timeout = self.heartbeat_timeout;

        tokio::spawn(async move {
            let mut cleanup_interval = interval(Duration::from_secs(60)); // Cleanup every minute

            loop {
                cleanup_interval.tick().await;

                let mut expired_connections = Vec::new();

                // Find expired connections
                for entry in connections.iter() {
                    if entry.value().is_expired(heartbeat_timeout) {
                        expired_connections.push(entry.key().clone());
                    }
                }

                // Remove expired connections
                for connection_id in expired_connections {
                    connections.remove(&connection_id);
                    warn!("Removed expired connection: {}", connection_id);
                }
            }
        });
    }

    /// Store connection information in database (placeholder - would be implemented with proper schema)
    async fn store_connection_info(&self, _info: &ConnectionInfo) -> Result<()> {
        // For now, skip database storage as active_connections table doesn't exist yet
        // This would be implemented once we have proper schema for connection tracking
        debug!("Connection info storage skipped - not yet implemented in schema");
        Ok(())
    }

    /// Remove connection information from database (placeholder)
    async fn remove_connection_info(&self, _connection_id: &str) -> Result<()> {
        // For now, skip database removal as active_connections table doesn't exist yet
        debug!("Connection info removal skipped - not yet implemented in schema");
        Ok(())
    }

    // ==========================================================
    // COMMENTED METHOD - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // /// Get instance ID for this service instance
    // /// USAGE: When implementing distributed WebSocket coordination
    // /// PURPOSE: Identify specific service instances for load balancing and debugging
    // fn get_instance_id(&self) -> String {
    //     // In production, this could be AWS instance ID, container ID, etc.
    //     // For now, use a simple generated ID
    //     format!("chat-service-{}", Uuid::new_v4())
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use shared::models::chat::WebSocketMessage;
//     use tokio::sync::mpsc;
// 
//     #[test]
//     fn test_active_connection_creation() {
//         let user_id = Uuid::new_v4();
//         let connection_id = "test_conn".to_string();
//         let (sender, _) = mpsc::unbounded_channel();
// 
//         let connection = ActiveConnection::new(user_id, connection_id.clone(), sender);
// 
//         assert_eq!(connection.user_id, user_id);
//         assert_eq!(connection.connection_id, connection_id);
//         assert!(connection.joined_rooms.is_empty());
//     }
// 
//     #[test]
//     fn test_room_join_leave() {
//         let user_id = Uuid::new_v4();
//         let connection_id = "test_conn".to_string();
//         let (sender, _) = mpsc::unbounded_channel();
//         let room_id = Uuid::new_v4();
// 
//         let mut connection = ActiveConnection::new(user_id, connection_id, sender);
// 
//         // Test join
//         connection.join_room(room_id);
//         assert!(connection.joined_rooms.contains(&room_id));
// 
//         // Test leave
//         connection.leave_room(room_id);
//         assert!(!connection.joined_rooms.contains(&room_id));
//     }
// 
//     #[test]
//     fn test_heartbeat_expiration() {
//         let user_id = Uuid::new_v4();
//         let connection_id = "test_conn".to_string();
//         let (sender, _) = mpsc::unbounded_channel();
// 
//         let mut connection = ActiveConnection::new(user_id, connection_id, sender);
// 
//         // Should not be expired immediately
//         assert!(!connection.is_expired(Duration::from_secs(10)));
// 
//         // Simulate old heartbeat
//         connection.last_heartbeat = Instant::now() - Duration::from_secs(15);
//         assert!(connection.is_expired(Duration::from_secs(10)));
//     }
// }
