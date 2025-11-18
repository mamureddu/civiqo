-- Initial seed data for development

-- Insert default roles
INSERT INTO roles (id, name, description, permissions, is_default) VALUES
    (uuid_generate_v4(), 'owner', 'Community Owner', '["all"]', false),
    (uuid_generate_v4(), 'admin', 'Community Administrator', '["manage_community", "manage_members", "manage_business", "create_polls", "manage_governance"]', false),
    (uuid_generate_v4(), 'moderator', 'Community Moderator', '["manage_members", "moderate_chat", "create_polls"]', false),
    (uuid_generate_v4(), 'member', 'Community Member', '["view_community", "participate_chat", "vote", "create_business"]', true),
    (uuid_generate_v4(), 'viewer', 'Community Viewer', '["view_community"]', false);

-- Sample community (for development/testing)
DO $$
DECLARE
    sample_user_id UUID := uuid_generate_v4();
    sample_community_id UUID := uuid_generate_v4();
    owner_role_id UUID;
    member_role_id UUID;
    general_room_id UUID := uuid_generate_v4();
BEGIN
    -- Get role IDs
    SELECT id INTO owner_role_id FROM roles WHERE name = 'owner';
    SELECT id INTO member_role_id FROM roles WHERE name = 'member';

    -- Create sample user (this would normally come from Auth0)
    INSERT INTO users (id, auth0_id, email) VALUES
        (sample_user_id, 'auth0|sample_user', 'demo@example.com');

    INSERT INTO user_profiles (user_id, name, bio) VALUES
        (sample_user_id, 'Demo User', 'Sample user for development');

    -- Create sample community
    INSERT INTO communities (id, name, description, slug, is_public, requires_approval, created_by) VALUES
        (sample_community_id, 'Demo Community', 'A sample community for development and testing', 'demo-community', true, false, sample_user_id);

    -- Add community settings
    INSERT INTO community_settings (community_id, allow_business_listings, governance_rules) VALUES
        (sample_community_id, true, '{"voting_period_days": 7, "quorum_percentage": 50}');

    -- Add creator as owner
    INSERT INTO community_members (user_id, community_id, role_id, status, joined_at) VALUES
        (sample_user_id, sample_community_id, owner_role_id, 'active', NOW());

    -- Create general chat room
    INSERT INTO chat_rooms (id, community_id, name, description, room_type, is_private, created_by) VALUES
        (general_room_id, sample_community_id, 'General', 'General community discussion', 'general', false, sample_user_id);

    -- Add creator to general room
    INSERT INTO room_participants (room_id, user_id, role, joined_at) VALUES
        (general_room_id, sample_user_id, 'admin', NOW());

    -- Sample business
    INSERT INTO businesses (community_id, owner_id, name, description, category, website, phone, email, address, latitude, longitude, is_verified, is_active) VALUES
        (sample_community_id, sample_user_id, 'Demo Coffee Shop', 'A cozy local coffee shop serving organic coffee and pastries', 'food', 'https://democoffee.example.com', '+1-555-0123', 'hello@democoffee.example.com', '123 Main St, Demo City', 40.7128, -74.0060, true, true);

    RAISE NOTICE 'Sample data created successfully';
    RAISE NOTICE 'Sample community ID: %', sample_community_id;
    RAISE NOTICE 'Sample user ID: %', sample_user_id;
END $$;