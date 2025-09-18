-- PostgreSQL initialization script for Docker container
-- Creates the test database needed for comprehensive testing

-- Create test database for integration tests
CREATE DATABASE community_manager_test;

-- Grant all privileges to dev user on test database
GRANT ALL PRIVILEGES ON DATABASE community_manager_test TO dev;

-- Connect to test database and ensure dev user has schema permissions
\c community_manager_test
GRANT ALL PRIVILEGES ON SCHEMA public TO dev;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO dev;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO dev;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON TABLES TO dev;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON SEQUENCES TO dev;