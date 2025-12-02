-- ============================================================================
-- RESET DATABASE FOR SINGLE COMMUNITY REFACTOR
-- ============================================================================
-- WARNING: This script DELETES ALL DATA
-- Run this before applying 009_single_community.sql
-- ============================================================================

-- Disable foreign key checks temporarily (CockroachDB way)
SET sql_safe_updates = false;

-- ============================================================================
-- DELETE ALL DATA (in correct order due to foreign keys)
-- ============================================================================

-- Analytics & Moderation
DELETE FROM analytics_events WHERE 1=1;
DELETE FROM moderation_queue WHERE 1=1;
DELETE FROM audit_logs WHERE 1=1;

-- Social
DELETE FROM user_follows WHERE 1=1;
DELETE FROM notifications WHERE 1=1;

-- Chat
DELETE FROM chat_messages WHERE 1=1;
DELETE FROM chat_room_members WHERE 1=1;
DELETE FROM chat_rooms WHERE 1=1;

-- Governance
DELETE FROM votes WHERE 1=1;
DELETE FROM proposal_options WHERE 1=1;
DELETE FROM proposals WHERE 1=1;

-- Content
DELETE FROM reactions WHERE 1=1;
DELETE FROM comments WHERE 1=1;
DELETE FROM post_media WHERE 1=1;
DELETE FROM posts WHERE 1=1;

-- Business
DELETE FROM order_items WHERE 1=1;
DELETE FROM orders WHERE 1=1;
DELETE FROM business_reviews WHERE 1=1;
DELETE FROM business_products WHERE 1=1;
DELETE FROM business_hours WHERE 1=1;
DELETE FROM businesses WHERE 1=1;

-- Communities
DELETE FROM community_boundaries WHERE 1=1;
DELETE FROM community_members WHERE 1=1;
DELETE FROM communities WHERE 1=1;

-- Users (keep structure, delete data)
DELETE FROM user_keys WHERE 1=1;
DELETE FROM user_profiles WHERE 1=1;
DELETE FROM users WHERE 1=1;

-- Re-enable safe updates
SET sql_safe_updates = true;

-- ============================================================================
-- VERIFY CLEANUP
-- ============================================================================

SELECT 'communities' as table_name, COUNT(*) as count FROM communities
UNION ALL
SELECT 'users', COUNT(*) FROM users
UNION ALL
SELECT 'posts', COUNT(*) FROM posts
UNION ALL
SELECT 'proposals', COUNT(*) FROM proposals;

