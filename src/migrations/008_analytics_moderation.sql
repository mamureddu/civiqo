-- ============================================================================
-- Migration 008: Analytics & Moderation (Phase 7)
-- ============================================================================
-- Tables: analytics_events, moderation_queue, audit_logs
-- 
-- ID Strategy:
-- - analytics_events.id: BIGINT GENERATED ALWAYS AS IDENTITY
-- - moderation_queue.id: UUID (app generates via Uuid::now_v7())
-- - audit_logs.id: BIGINT GENERATED ALWAYS AS IDENTITY
-- ============================================================================

-- ============================================================================
-- ANALYTICS EVENTS (BIGINT - high volume, auto-increment)
-- ============================================================================

CREATE TABLE analytics_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,  -- 'page_view', 'click', 'search', 'login', 'signup', etc.
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    community_id UUID REFERENCES communities(id) ON DELETE SET NULL,
    session_id VARCHAR(100),
    metadata JSONB DEFAULT '{}',  -- Flexible event data
    ip_address VARCHAR(45),
    user_agent TEXT,
    referrer TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_analytics_event_type ON analytics_events(event_type);
CREATE INDEX idx_analytics_user ON analytics_events(user_id);
CREATE INDEX idx_analytics_community ON analytics_events(community_id);
CREATE INDEX idx_analytics_created_at ON analytics_events(created_at DESC);
CREATE INDEX idx_analytics_session ON analytics_events(session_id);
CREATE INDEX idx_analytics_metadata ON analytics_events USING GIN(metadata);

-- ============================================================================
-- MODERATION QUEUE (UUID - app generates)
-- ============================================================================

CREATE TABLE moderation_queue (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    content_type VARCHAR(50) NOT NULL,  -- 'post', 'comment', 'user', 'community', 'business', 'review'
    content_id VARCHAR(100) NOT NULL,   -- UUID or BIGINT as string
    reported_by UUID REFERENCES users(id) ON DELETE SET NULL,
    reason VARCHAR(255) NOT NULL,
    details TEXT,
    status VARCHAR(50) DEFAULT 'pending',  -- 'pending', 'reviewing', 'approved', 'rejected', 'escalated'
    priority VARCHAR(20) DEFAULT 'normal',  -- 'low', 'normal', 'high', 'urgent'
    moderator_id UUID REFERENCES users(id) ON DELETE SET NULL,
    resolution TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);

CREATE INDEX idx_moderation_status ON moderation_queue(status);
CREATE INDEX idx_moderation_content_type ON moderation_queue(content_type);
CREATE INDEX idx_moderation_reported_by ON moderation_queue(reported_by);
CREATE INDEX idx_moderation_moderator ON moderation_queue(moderator_id);
CREATE INDEX idx_moderation_priority ON moderation_queue(priority);
CREATE INDEX idx_moderation_created_at ON moderation_queue(created_at DESC);
CREATE INDEX idx_moderation_pending ON moderation_queue(status, priority, created_at) 
    WHERE status = 'pending';

-- ============================================================================
-- AUDIT LOGS (BIGINT - high volume, auto-increment)
-- ============================================================================

CREATE TABLE audit_logs (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,  -- 'create', 'update', 'delete', 'login', 'logout', 'permission_change'
    target_type VARCHAR(50),  -- 'user', 'community', 'post', 'business', etc.
    target_id VARCHAR(100),   -- UUID or BIGINT as string
    old_value JSONB,
    new_value JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_audit_user ON audit_logs(user_id);
CREATE INDEX idx_audit_action ON audit_logs(action);
CREATE INDEX idx_audit_target ON audit_logs(target_type, target_id);
CREATE INDEX idx_audit_created_at ON audit_logs(created_at DESC);

-- ============================================================================
-- ADMIN SETTINGS (Key-Value store for app configuration)
-- ============================================================================

CREATE TABLE admin_settings (
    key VARCHAR(100) PRIMARY KEY,
    value JSONB NOT NULL,
    description TEXT,
    updated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default settings
INSERT INTO admin_settings (key, value, description) VALUES
    ('maintenance_mode', 'false', 'Enable maintenance mode'),
    ('registration_enabled', 'true', 'Allow new user registrations'),
    ('max_communities_per_user', '10', 'Maximum communities a user can create'),
    ('max_posts_per_day', '50', 'Maximum posts per user per day'),
    ('moderation_auto_flag_threshold', '3', 'Auto-flag content after N reports');

-- ============================================================================
-- COMMUNITY STATS (Aggregated stats for dashboard)
-- ============================================================================

CREATE TABLE community_stats (
    community_id UUID PRIMARY KEY REFERENCES communities(id) ON DELETE CASCADE,
    total_members INT DEFAULT 0,
    total_posts INT DEFAULT 0,
    total_comments INT DEFAULT 0,
    total_proposals INT DEFAULT 0,
    active_proposals INT DEFAULT 0,
    posts_this_week INT DEFAULT 0,
    posts_this_month INT DEFAULT 0,
    engagement_score DECIMAL(5,2) DEFAULT 0,
    last_activity_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_community_stats_engagement ON community_stats(engagement_score DESC);
CREATE INDEX idx_community_stats_activity ON community_stats(last_activity_at DESC);
