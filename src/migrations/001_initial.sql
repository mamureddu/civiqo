-- Initial database setup with user management and core tables

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "postgis";

-- Custom types
CREATE TYPE membership_status AS ENUM ('pending', 'active', 'suspended', 'banned');
CREATE TYPE business_category AS ENUM ('food', 'retail', 'services', 'healthcare', 'education', 'technology', 'manufacturing', 'agriculture', 'arts', 'recreation', 'other');
CREATE TYPE poll_type AS ENUM ('single_choice', 'multiple_choice', 'ranked_choice', 'yes_no', 'rating');
CREATE TYPE poll_status AS ENUM ('draft', 'active', 'completed', 'cancelled');
CREATE TYPE decision_type AS ENUM ('simple', 'consensus', 'majority', 'super_majority', 'unanimous');
CREATE TYPE decision_status AS ENUM ('pending', 'in_progress', 'approved', 'rejected', 'expired');
CREATE TYPE decision_vote_type AS ENUM ('approve', 'reject', 'abstain');
CREATE TYPE room_type AS ENUM ('general', 'announcement', 'topic', 'direct_message', 'group');
CREATE TYPE participant_role AS ENUM ('admin', 'moderator', 'member');

-- Users table (synced from Auth0)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    auth0_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User profiles
CREATE TABLE user_profiles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255),
    avatar_url TEXT,
    bio TEXT,
    location VARCHAR(255),
    website TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id)
);

-- Roles for RBAC
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    permissions JSONB NOT NULL DEFAULT '[]',
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Communities
CREATE TABLE communities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    slug VARCHAR(255) UNIQUE NOT NULL,
    is_public BOOLEAN DEFAULT TRUE,
    requires_approval BOOLEAN DEFAULT FALSE,
    created_by UUID REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Community settings
CREATE TABLE community_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    community_id UUID REFERENCES communities(id) ON DELETE CASCADE,
    max_members INTEGER,
    allow_business_listings BOOLEAN DEFAULT TRUE,
    governance_rules JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(community_id)
);

-- Community geographic boundaries
CREATE TABLE community_boundaries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    community_id UUID REFERENCES communities(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    boundary_data JSONB NOT NULL, -- GeoJSON polygon
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Community memberships
CREATE TABLE community_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    community_id UUID REFERENCES communities(id) ON DELETE CASCADE,
    role_id UUID REFERENCES roles(id) ON DELETE RESTRICT,
    status membership_status DEFAULT 'pending',
    joined_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, community_id)
);

-- Indexes for performance
CREATE INDEX idx_users_auth0_id ON users(auth0_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_communities_slug ON communities(slug);
CREATE INDEX idx_communities_public ON communities(is_public);
CREATE INDEX idx_community_members_user_id ON community_members(user_id);
CREATE INDEX idx_community_members_community_id ON community_members(community_id);
CREATE INDEX idx_community_members_status ON community_members(status);

-- Update timestamps trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_user_profiles_updated_at BEFORE UPDATE ON user_profiles FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_roles_updated_at BEFORE UPDATE ON roles FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_communities_updated_at BEFORE UPDATE ON communities FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_community_settings_updated_at BEFORE UPDATE ON community_settings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_community_members_updated_at BEFORE UPDATE ON community_members FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();