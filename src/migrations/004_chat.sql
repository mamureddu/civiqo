-- Chat and messaging tables (minimal storage for E2EE)

-- Chat rooms
CREATE TABLE chat_rooms (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    community_id UUID REFERENCES communities(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    room_type room_type NOT NULL,
    is_private BOOLEAN DEFAULT FALSE,
    created_by UUID REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Room participants
CREATE TABLE room_participants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_id UUID REFERENCES chat_rooms(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role participant_role DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    last_read_at TIMESTAMPTZ,
    UNIQUE(room_id, user_id)
);

-- User encryption keys (only public keys stored)
CREATE TABLE user_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    public_key TEXT NOT NULL,
    key_fingerprint VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    UNIQUE(user_id, key_fingerprint)
);

-- Connection tracking (in-memory alternative for stateless chat service)
-- This table is used only when instances need to find where users are connected
CREATE TABLE active_connections (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    connection_id VARCHAR(255) NOT NULL,
    instance_id VARCHAR(255) NOT NULL,
    rooms JSONB NOT NULL DEFAULT '[]', -- Array of room IDs user is in
    connected_at TIMESTAMPTZ DEFAULT NOW(),
    last_heartbeat TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, connection_id)
);

-- Message queue for offline users (temporary storage)
-- Note: This is for SQS simulation in local development only
-- In production, actual SQS will be used
CREATE TABLE temp_offline_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipient_id UUID REFERENCES users(id) ON DELETE CASCADE,
    sender_id UUID REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID REFERENCES chat_rooms(id) ON DELETE CASCADE,
    encrypted_content TEXT NOT NULL,
    message_type VARCHAR(50) NOT NULL DEFAULT 'text',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '24 hours')
);

-- Indexes for chat queries
CREATE INDEX idx_chat_rooms_community_id ON chat_rooms(community_id);
CREATE INDEX idx_chat_rooms_type ON chat_rooms(room_type);
CREATE INDEX idx_chat_rooms_private ON chat_rooms(is_private);
CREATE INDEX idx_room_participants_room_id ON room_participants(room_id);
CREATE INDEX idx_room_participants_user_id ON room_participants(user_id);
CREATE INDEX idx_user_keys_user_id ON user_keys(user_id);
CREATE INDEX idx_user_keys_fingerprint ON user_keys(key_fingerprint);
CREATE INDEX idx_active_connections_user_id ON active_connections(user_id);
CREATE INDEX idx_active_connections_instance ON active_connections(instance_id);
CREATE INDEX idx_active_connections_heartbeat ON active_connections(last_heartbeat);
CREATE INDEX idx_temp_offline_messages_recipient ON temp_offline_messages(recipient_id);
CREATE INDEX idx_temp_offline_messages_expires ON temp_offline_messages(expires_at);

-- Auto-cleanup expired connections and messages
CREATE OR REPLACE FUNCTION cleanup_expired_data()
RETURNS void AS $$
BEGIN
    -- Remove stale connections (no heartbeat for 5 minutes)
    DELETE FROM active_connections
    WHERE last_heartbeat < NOW() - INTERVAL '5 minutes';

    -- Remove expired offline messages
    DELETE FROM temp_offline_messages
    WHERE expires_at < NOW();
END;
$$ LANGUAGE plpgsql;

-- Update triggers
CREATE TRIGGER update_chat_rooms_updated_at BEFORE UPDATE ON chat_rooms FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();