-- Migration 009: Add created_at and updated_at to community_members
-- 
-- This migration adds standard timestamp columns to community_members table
-- to maintain consistency with other tables in the schema.
--
-- The original schema only had joined_at, but created_at and updated_at
-- are standard columns that should be present for audit purposes.

-- Add created_at column with default value (CockroachDB compatible)
ALTER TABLE community_members 
ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ DEFAULT now();

-- Add updated_at column with default value
ALTER TABLE community_members 
ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT now();
