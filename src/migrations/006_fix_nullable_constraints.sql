-- Fix missing NOT NULL constraints that are causing SQLx compilation issues

-- Chat rooms - add NOT NULL constraints
ALTER TABLE chat_rooms ALTER COLUMN community_id SET NOT NULL;
ALTER TABLE chat_rooms ALTER COLUMN created_by SET NOT NULL;
ALTER TABLE chat_rooms ALTER COLUMN created_at SET NOT NULL;
ALTER TABLE chat_rooms ALTER COLUMN updated_at SET NOT NULL;
ALTER TABLE chat_rooms ALTER COLUMN is_private SET NOT NULL;

-- Room participants - add NOT NULL constraints
ALTER TABLE room_participants ALTER COLUMN room_id SET NOT NULL;
ALTER TABLE room_participants ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE room_participants ALTER COLUMN role SET NOT NULL;
ALTER TABLE room_participants ALTER COLUMN joined_at SET NOT NULL;

-- User keys - add NOT NULL constraints
ALTER TABLE user_keys ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE user_keys ALTER COLUMN created_at SET NOT NULL;

-- Active connections - add NOT NULL constraints
ALTER TABLE active_connections ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE active_connections ALTER COLUMN connected_at SET NOT NULL;
ALTER TABLE active_connections ALTER COLUMN last_heartbeat SET NOT NULL;

-- Temp offline messages - add NOT NULL constraints
ALTER TABLE temp_offline_messages ALTER COLUMN recipient_id SET NOT NULL;
ALTER TABLE temp_offline_messages ALTER COLUMN sender_id SET NOT NULL;
ALTER TABLE temp_offline_messages ALTER COLUMN room_id SET NOT NULL;
ALTER TABLE temp_offline_messages ALTER COLUMN created_at SET NOT NULL;