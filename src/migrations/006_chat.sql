-- ============================================================================
-- Migration 006: Chat System
-- ============================================================================
-- Tables: chat_rooms, room_participants, temp_offline_messages, active_connections
-- 
-- ID Strategy:
-- - chat_rooms.id: UUID (app generates via Uuid::now_v7())
-- - room_participants.id: UUID (app generates via Uuid::now_v7())
-- - temp_offline_messages.id: UUID (app generates via Uuid::now_v7())
-- - active_connections: Composite PK (no separate id)
-- ============================================================================

-- ============================================================================
-- CHAT ROOMS (UUID - app generates)
-- ============================================================================

CREATE TABLE chat_rooms (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    room_type VARCHAR(50) DEFAULT 'public',  -- 'public', 'private', 'dm'
    is_private BOOLEAN DEFAULT false,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_chat_rooms_community ON chat_rooms(community_id);
CREATE INDEX idx_chat_rooms_created_by ON chat_rooms(created_by);
CREATE INDEX idx_chat_rooms_type ON chat_rooms(room_type);
CREATE INDEX idx_chat_rooms_is_private ON chat_rooms(is_private);

-- ============================================================================
-- ROOM PARTICIPANTS (UUID - app generates)
-- ============================================================================

CREATE TABLE room_participants (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    room_id UUID NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(100) DEFAULT 'member',  -- 'owner', 'admin', 'member'
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    last_read_at TIMESTAMPTZ,
    UNIQUE(room_id, user_id)
);

CREATE INDEX idx_room_participants_room ON room_participants(room_id);
CREATE INDEX idx_room_participants_user ON room_participants(user_id);

-- ============================================================================
-- TEMP OFFLINE MESSAGES (UUID - app generates)
-- ============================================================================
-- Messages stored temporarily when recipient is offline
-- Deleted after delivery or expiration

CREATE TABLE temp_offline_messages (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    recipient_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,
    encrypted_content TEXT NOT NULL,
    message_type VARCHAR(100) DEFAULT 'text',  -- 'text', 'image', 'file', etc.
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ DEFAULT (NOW() + INTERVAL '24 hours')
);

CREATE INDEX idx_temp_offline_messages_recipient ON temp_offline_messages(recipient_id);
CREATE INDEX idx_temp_offline_messages_sender ON temp_offline_messages(sender_id);
CREATE INDEX idx_temp_offline_messages_room ON temp_offline_messages(room_id);
CREATE INDEX idx_temp_offline_messages_expires ON temp_offline_messages(expires_at);

-- ============================================================================
-- ACTIVE CONNECTIONS (Composite PK - ephemeral state)
-- ============================================================================
-- Tracks currently connected users per room
-- Cleaned up on disconnect

CREATE TABLE active_connections (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,
    connected_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, room_id)
);

CREATE INDEX idx_active_connections_user ON active_connections(user_id);
CREATE INDEX idx_active_connections_room ON active_connections(room_id);
