-- ======================================================================
-- Migration 013: Fix chat_rooms and business_products schema
-- Adds missing columns and fixes related tables for UUID consistency
-- ======================================================================

-- ======================================================================
-- Fix business_products table - needs id column and UUID business_id
-- ======================================================================

DROP TABLE IF EXISTS business_products CASCADE;

-- Note: id is UUID without default - application generates UUIDv7 via Uuid::now_v7()
CREATE TABLE business_products (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    product_name VARCHAR(255) NOT NULL,
    description TEXT,
    price NUMERIC(10, 2),
    currency VARCHAR(3) DEFAULT 'EUR',
    is_available BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_business_products_business ON business_products(business_id);
CREATE INDEX idx_business_products_available ON business_products(is_available);

-- ======================================================================
-- Fix business_hours table - needs UUID business_id
-- ======================================================================

DROP TABLE IF EXISTS business_hours CASCADE;

CREATE TABLE business_hours (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    day_of_week INT NOT NULL CHECK (day_of_week >= 0 AND day_of_week <= 6),
    open_time TIME,
    close_time TIME,
    is_closed BOOLEAN DEFAULT false,
    UNIQUE(business_id, day_of_week)
);

CREATE INDEX idx_business_hours_business ON business_hours(business_id);

-- ======================================================================
-- Fix business_images table - needs UUID business_id
-- ======================================================================

DROP TABLE IF EXISTS business_images CASCADE;

CREATE TABLE business_images (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    image_url TEXT NOT NULL,
    image_type VARCHAR(50) DEFAULT 'gallery',
    display_order INT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_business_images_business ON business_images(business_id);

-- Add missing columns to chat_rooms
ALTER TABLE chat_rooms ADD COLUMN IF NOT EXISTS description TEXT;
ALTER TABLE chat_rooms ADD COLUMN IF NOT EXISTS is_private BOOLEAN DEFAULT false;
ALTER TABLE chat_rooms ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;

-- Add index for is_private if needed for filtering
CREATE INDEX IF NOT EXISTS idx_chat_rooms_is_private ON chat_rooms(is_private);

-- ======================================================================
-- Fix room_participants table - room_id should be UUID to match chat_rooms
-- ======================================================================

-- Drop old table and recreate with correct types
DROP TABLE IF EXISTS room_participants CASCADE;

-- Note: id is UUID without default - application generates UUIDv7 via Uuid::now_v7()
CREATE TABLE room_participants (
    id UUID PRIMARY KEY,
    room_id UUID NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(100) DEFAULT 'member',
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_read_at TIMESTAMP WITH TIME ZONE,
    UNIQUE(room_id, user_id)
);

CREATE INDEX idx_room_participants_room_id ON room_participants(room_id);
CREATE INDEX idx_room_participants_user_id ON room_participants(user_id);

-- ======================================================================
-- Fix temp_offline_messages table - room_id should be UUID
-- ======================================================================

DROP TABLE IF EXISTS temp_offline_messages CASCADE;

-- Note: id is UUID without default - application generates UUIDv7 via Uuid::now_v7()
CREATE TABLE temp_offline_messages (
    id UUID PRIMARY KEY,
    recipient_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,
    encrypted_content TEXT NOT NULL,
    message_type VARCHAR(100) DEFAULT 'text',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT (CURRENT_TIMESTAMP + INTERVAL '24 hours')
);

CREATE INDEX idx_temp_offline_messages_recipient ON temp_offline_messages(recipient_id);
CREATE INDEX idx_temp_offline_messages_room ON temp_offline_messages(room_id);

-- ======================================================================
-- Fix active_connections table - room_id should be UUID
-- ======================================================================

DROP TABLE IF EXISTS active_connections CASCADE;

CREATE TABLE active_connections (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,
    connected_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, room_id)
);
