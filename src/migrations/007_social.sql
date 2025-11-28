-- ============================================================================
-- Migration 007: Social Features
-- ============================================================================
-- Tables: user_follows, notifications
-- 
-- ID Strategy:
-- - user_follows: Composite PK (no separate id)
-- - notifications.id: UUID (app generates via Uuid::now_v7())
-- ============================================================================

-- ============================================================================
-- USER FOLLOWS (Composite PK - junction table)
-- ============================================================================

CREATE TABLE user_follows (
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (follower_id, following_id),
    CONSTRAINT no_self_follow CHECK (follower_id != following_id)
);

CREATE INDEX idx_user_follows_follower ON user_follows(follower_id);
CREATE INDEX idx_user_follows_following ON user_follows(following_id);
CREATE INDEX idx_user_follows_created_at ON user_follows(created_at DESC);

-- ============================================================================
-- NOTIFICATIONS (UUID - app generates)
-- ============================================================================

CREATE TABLE notifications (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,  -- 'follow', 'comment', 'reaction', 'mention', 'join_request', 'post'
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    target_type VARCHAR(50),  -- 'post', 'comment', 'community', 'user', 'proposal'
    target_id VARCHAR(100),   -- UUID or BIGINT as string (flexible)
    message TEXT,
    is_read BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON notifications(user_id);
CREATE INDEX idx_notifications_user_unread ON notifications(user_id, is_read) WHERE is_read = false;
CREATE INDEX idx_notifications_actor ON notifications(actor_id);
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);
CREATE INDEX idx_notifications_type ON notifications(type);
