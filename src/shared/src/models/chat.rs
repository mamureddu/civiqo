use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatRoom {
    pub id: Uuid,
    pub community_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub room_type: RoomType,
    pub is_private: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "room_type", rename_all = "snake_case")]
pub enum RoomType {
    General,
    Announcement,
    Topic,
    DirectMessage,
    Group,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoomParticipant {
    pub id: Uuid,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub role: Option<String>, // VARCHAR in DB: 'admin', 'moderator', 'member'
    pub joined_at: DateTime<Utc>,
    pub last_read_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "participant_role", rename_all = "snake_case")]
pub enum ParticipantRole {
    Admin,
    Moderator,
    Member,
}

// Note: Messages are NOT stored in the database for E2EE
// This is only for temporary offline message queuing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryMessage {
    pub id: Uuid,
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Option<Uuid>, // For direct messages
    pub encrypted_content: String, // Encrypted message content
    pub message_type: MessageType,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>, // Auto-delete after 24 hours
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Image,
    File,
    Voice,
    System,
}

// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    // Connection management
    Connect {
        user_id: Uuid,
        rooms: Vec<Uuid>,
    },
    Disconnect {
        user_id: Uuid,
    },

    // Room management
    JoinRoom {
        room_id: Uuid,
    },
    LeaveRoom {
        room_id: Uuid,
    },

    // Messaging
    SendMessage {
        room_id: Uuid,
        recipient_id: Option<Uuid>, // For direct messages
        encrypted_content: String,
        message_type: MessageType,
    },
    ReceiveMessage {
        id: Uuid,
        room_id: Uuid,
        sender_id: Uuid,
        encrypted_content: String,
        message_type: MessageType,
        created_at: DateTime<Utc>,
    },

    // Presence
    UserPresence {
        user_id: Uuid,
        status: PresenceStatus,
        last_seen: DateTime<Utc>,
    },

    // Typing indicators
    TypingStart {
        room_id: Uuid,
        user_id: Uuid,
    },
    TypingStop {
        room_id: Uuid,
        user_id: Uuid,
    },

    // Key exchange for E2EE
    KeyExchange {
        sender_id: Uuid,
        recipient_id: Uuid,
        public_key: String,
    },

    // System messages
    Error {
        message: String,
        code: String,
    },
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PresenceStatus {
    Online,
    Away,
    Busy,
    Offline,
}

// User encryption keys (only public keys stored)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserKeys {
    pub id: Uuid,
    pub user_id: Uuid,
    pub public_key: String,
    pub key_fingerprint: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

// Request/Response types
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub description: Option<String>,
    pub room_type: RoomType,
    pub is_private: bool,
    pub participants: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoomRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomWithParticipants {
    #[serde(flatten)]
    pub room: ChatRoom,
    pub participants: Vec<RoomParticipant>,
    pub unread_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub user_id: Uuid,
    pub connection_id: String,
    pub rooms: Vec<Uuid>,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}