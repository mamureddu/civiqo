-- ============================================================================
-- Migration 009: Single Community Instance
-- ============================================================================
-- Transforms Civiqo from multi-community to single-community instance
-- 
-- Changes:
-- - Add instance_settings table for global configuration
-- - Add federation_config table for future federation support
-- - Add instance_admins table for instance-level admins
-- - Add branding fields to communities table
-- ============================================================================

-- ============================================================================
-- INSTANCE SETTINGS (key-value store for instance configuration)
-- ============================================================================

CREATE TABLE IF NOT EXISTS instance_settings (
    key VARCHAR(100) PRIMARY KEY,
    value TEXT,
    value_type VARCHAR(50) DEFAULT 'string',  -- 'string', 'boolean', 'number', 'json'
    description TEXT,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Default settings
INSERT INTO instance_settings (key, value, value_type, description) VALUES
    ('instance_name', 'Civiqo', 'string', 'Nome dell''istanza'),
    ('instance_description', 'Piattaforma di partecipazione civica', 'string', 'Descrizione istanza'),
    ('setup_completed', 'false', 'boolean', 'Setup wizard completato'),
    ('allow_registration', 'true', 'boolean', 'Permetti nuove registrazioni'),
    ('default_language', 'it', 'string', 'Lingua predefinita')
ON CONFLICT (key) DO NOTHING;

-- ============================================================================
-- FEDERATION CONFIG (for future federation with Civiqo network)
-- ============================================================================

CREATE TABLE IF NOT EXISTS federation_config (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    hub_url VARCHAR(500),
    api_key VARCHAR(255),
    instance_id VARCHAR(100),  -- Unique ID for this instance in the network
    enabled BOOLEAN DEFAULT false,
    sync_members BOOLEAN DEFAULT false,
    sync_posts BOOLEAN DEFAULT false,
    sync_proposals BOOLEAN DEFAULT false,
    sync_events BOOLEAN DEFAULT false,
    last_sync_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- INSTANCE ADMINS (users with instance-level admin rights)
-- ============================================================================

CREATE TABLE IF NOT EXISTS instance_admins (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    permissions JSONB DEFAULT '{"all": true}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

-- ============================================================================
-- BRANDING FIELDS for communities
-- ============================================================================

-- Add branding columns to communities if they don't exist
ALTER TABLE communities ADD COLUMN IF NOT EXISTS logo_url TEXT;
ALTER TABLE communities ADD COLUMN IF NOT EXISTS cover_url TEXT;
ALTER TABLE communities ADD COLUMN IF NOT EXISTS primary_color VARCHAR(7) DEFAULT '#2563EB';
ALTER TABLE communities ADD COLUMN IF NOT EXISTS secondary_color VARCHAR(7) DEFAULT '#57C98A';
ALTER TABLE communities ADD COLUMN IF NOT EXISTS accent_color VARCHAR(7) DEFAULT '#FF6B6B';

-- ============================================================================
-- INDEXES
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_instance_settings_updated ON instance_settings(updated_at);
CREATE INDEX IF NOT EXISTS idx_federation_enabled ON federation_config(enabled);
CREATE INDEX IF NOT EXISTS idx_instance_admins_created ON instance_admins(created_at);

