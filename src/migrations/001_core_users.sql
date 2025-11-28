-- ============================================================================
-- Migration 001: Core Users & Roles
-- ============================================================================
-- Tables: users, user_profiles, user_keys, roles
-- 
-- ID Strategy:
-- - users.id: UUID (Auth0 requirement, app generates via Uuid::now_v7())
-- - user_profiles: UUID FK to users (no separate id)
-- - user_keys.id: BIGINT (DB generates, not used in app code)
-- - roles.id: BIGINT (DB generates via unique_rowid(), app uses RETURNING)
-- ============================================================================

-- ============================================================================
-- USERS (UUID - Auth0 integration)
-- ============================================================================

CREATE TABLE users (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    auth0_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    federated_from VARCHAR(255),  -- NULL = local, otherwise federation source
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_auth0_id ON users(auth0_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_federated_from ON users(federated_from);

-- ============================================================================
-- USER PROFILES (UUID FK - extends users)
-- ============================================================================

CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255),
    picture TEXT,
    bio TEXT,
    location VARCHAR(255),
    website TEXT,
    cover_image TEXT,
    avatar_url TEXT,  -- Alias for picture (some code uses this)
    is_public BOOLEAN DEFAULT true,
    follower_count INT DEFAULT 0,
    following_count INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_user_profiles_name ON user_profiles(name);
CREATE INDEX idx_user_profiles_is_public ON user_profiles(is_public);

-- ============================================================================
-- USER KEYS (BIGINT - DB generates, not used in app)
-- ============================================================================

CREATE TABLE user_keys (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key_type VARCHAR(100) NOT NULL,
    key_value TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_user_keys_user_id ON user_keys(user_id);
CREATE INDEX idx_user_keys_type ON user_keys(key_type);

-- ============================================================================
-- ROLES (BIGINT - DB generates, app uses RETURNING)
-- ============================================================================

CREATE TABLE roles (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    permissions JSONB,
    is_default BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_roles_name ON roles(name);
CREATE INDEX idx_roles_is_default ON roles(is_default);
