-- Add auth0_id column to users table for Auth0 integration
-- This allows us to link Auth0 users with local database users

-- Add auth0_id column (unique identifier from Auth0)
ALTER TABLE users ADD COLUMN IF NOT EXISTS auth0_id VARCHAR(255) UNIQUE;

-- Add last_login timestamp
ALTER TABLE users ADD COLUMN IF NOT EXISTS last_login TIMESTAMP;

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_auth0_id ON users(auth0_id);

-- Add comment for documentation
COMMENT ON COLUMN users.auth0_id IS 'Auth0 user identifier (sub claim from JWT)';
COMMENT ON COLUMN users.last_login IS 'Timestamp of last successful login';
