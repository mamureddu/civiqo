-- ============================================================================
-- Seed Data: Default Roles
-- ============================================================================
-- This is NOT a migration - run manually or via application startup
-- 
-- Usage:
--   psql $DATABASE_URL -f seed/seed_roles.sql
-- 
-- Or in application code:
--   sqlx::query_file!("migrations/seed/seed_roles.sql")
-- ============================================================================

-- Default roles for community membership
INSERT INTO roles (name, description, is_default) 
VALUES ('member', 'Regular community member', true)
ON CONFLICT (name) DO NOTHING;

INSERT INTO roles (name, description, is_default) 
VALUES ('admin', 'Community administrator with full permissions', false)
ON CONFLICT (name) DO NOTHING;

INSERT INTO roles (name, description, is_default) 
VALUES ('moderator', 'Community moderator with limited admin permissions', false)
ON CONFLICT (name) DO NOTHING;

INSERT INTO roles (name, description, is_default) 
VALUES ('owner', 'Community owner with all permissions', false)
ON CONFLICT (name) DO NOTHING;

-- Verify roles were created
-- SELECT id, name, description, is_default FROM roles ORDER BY id;
