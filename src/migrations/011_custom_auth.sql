-- ============================================================================
-- Migration 011: Custom Authentication (replace Auth0)
-- ============================================================================
-- Changes:
-- - Add password_hash for local email/password auth
-- - Add email_verified flag
-- - Generalize auth0_id → provider + provider_id for SSO-ready design
-- - Provider values: 'local' (email/password), 'google', 'github', etc.
-- ============================================================================

-- 1. Add password_hash column (NULL for SSO-only users)
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255);

-- 2. Add email_verified flag
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_verified BOOLEAN DEFAULT false;

-- 3. Add provider column (defaults to 'local' for email/password)
ALTER TABLE users ADD COLUMN IF NOT EXISTS provider VARCHAR(50) DEFAULT 'local';

-- 4. Rename auth0_id → provider_id (generic for any OAuth provider)
ALTER TABLE users RENAME COLUMN auth0_id TO provider_id;

-- 5. Make provider_id nullable (local users don't have one)
ALTER TABLE users ALTER COLUMN provider_id DROP NOT NULL;

-- 6. Drop old Auth0-specific index
DROP INDEX IF EXISTS idx_users_auth0_id;

-- 7. Create new unique index on (provider, provider_id) for SSO lookups
CREATE UNIQUE INDEX idx_users_provider ON users(provider, provider_id) WHERE provider_id IS NOT NULL;
