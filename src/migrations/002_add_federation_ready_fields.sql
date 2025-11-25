-- Migration: Add federation-ready fields
-- Date: November 25, 2025
-- Purpose: Prepare schema for future federation without implementing it yet

-- ============================================================================
-- Community Codes
-- ============================================================================
-- Each community gets a unique code for federation identification
-- Format: {instance_prefix}_{random} (e.g., "cvq_abc123")
-- This allows individual communities to federate independently

-- Add code column (nullable initially, app will populate on create)
ALTER TABLE communities ADD COLUMN IF NOT EXISTS code VARCHAR(20) UNIQUE;

-- ============================================================================
-- User Federation Source
-- ============================================================================
-- Tracks where a user authenticated from:
-- NULL = local auth (this instance's Auth0)
-- "civiqo.com" = federated from main civiqo
-- "other.instance.com" = federated from another instance
--
-- Since user IDs are UUIDs, collision is extremely unlikely (~1 in 2^122)
-- Both local and federated users can coexist in the same table

ALTER TABLE users ADD COLUMN IF NOT EXISTS federated_from VARCHAR(255);

-- Index for filtering by auth source
CREATE INDEX IF NOT EXISTS idx_users_federated_from ON users(federated_from);

-- ============================================================================
-- Notes
-- ============================================================================
-- This migration prepares the schema for federation but doesn't implement it.
-- Federation features will be added later in federation_management_plan/.
--
-- Key concepts:
-- 1. Community codes allow per-community federation
-- 2. federated_from allows dual-auth (local + federated users)
-- 3. UUID user IDs work across instances (collision unlikely)
--
-- Code generation happens in application:
-- fn generate_community_code() -> String {
--     let prefix = env::var("INSTANCE_PREFIX").unwrap_or("cvq".into());
--     format!("{}_{}", prefix, random_string(8))
-- }
