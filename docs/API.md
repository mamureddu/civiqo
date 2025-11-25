# 📡 API & Database Guide

## Overview

Complete guide to the Community Manager REST API and database schema, including endpoints, data models, and integration patterns.

- **Database**: CockroachDB Cloud (PostgreSQL-compatible)
- **Total Tables**: 22
- **Migrations**: 6 migration files
- **ORM**: SQLx with raw SQL queries
- **API Style**: RESTful JSON endpoints
- **Authentication**: Session-based with Auth0

## Database Connection

### Configuration

```bash
# Database
DATABASE_URL=postgresql://community-manager:***@community-manager-dev-18546.j77.aws-eu-central-1.cockroachlabs.cloud:26257/community-manager?sslmode=verify-full

# Optional DB tuning
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=5
DB_ACQUIRE_TIMEOUT_SECONDS=8
```

### Connection Setup

```rust
// In server/src/main.rs
let database = Database::new(&config.database_url).await?;
let state = Arc::new(AppState { db: database });

// Auto-run migrations
sqlx::migrate!("./migrations").run(&database.pool).await?;
```

### SQLx Offline Mode

```bash
# Update cached queries
cargo sqlx prepare --workspace

# Run tests without DB connection
SQLX_OFFLINE=true cargo test --workspace
```

## Database Schema

### User Management

#### `users` table
Core user table with authentication and profile information.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | User unique identifier |
| `auth0_id` | VARCHAR(255) | UNIQUE, NOT NULL | Auth0 user ID |
| `email` | VARCHAR(255) | UNIQUE, NOT NULL | User email |
| `username` | VARCHAR(50) | UNIQUE, NOT NULL | Unique username |
| `full_name` | VARCHAR(255) | | User's full name |
| `avatar_url` | TEXT | | Profile picture URL |
| `bio` | TEXT | | User biography |
| `location` | GEOGRAPHY(POINT) | | User location (PostGIS) |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Account creation timestamp |
| `updated_at` | TIMESTAMPTZ | DEFAULT NOW() | Last update timestamp |

#### `user_profiles` table
Extended profile information.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `user_id` | UUID | PRIMARY KEY, FK → users.id | User reference |
| `display_name` | VARCHAR(100) | | Display name |
| `website` | TEXT | | Personal website |
| `social_links` | JSONB | | Social media links |
| `preferences` | JSONB | | User preferences |

### Community Management

#### `communities` table
Community information and settings.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Community unique identifier |
| `name` | VARCHAR(100) | NOT NULL | Community name |
| `slug` | VARCHAR(100) | UNIQUE, NOT NULL | URL-friendly name |
| `description` | TEXT | | Community description |
| `category` | VARCHAR(50) | | Community category |
| `location` | GEOGRAPHY(POINT) | | Community location |
| `created_by` | UUID | FK → users.id | Creator user ID |
| `is_private` | BOOLEAN | DEFAULT false | Privacy setting |
| `member_count` | INTEGER | DEFAULT 0 | Cached member count |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |
| `updated_at` | TIMESTAMPTZ | DEFAULT NOW() | Last update timestamp |

#### `community_members` table
Community membership relationships.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Membership unique identifier |
| `community_id` | UUID | FK → communities.id | Community reference |
| `user_id` | UUID | FK → users.id | User reference |
| `role` | VARCHAR(20) | DEFAULT 'member' | Member role |
| `joined_at` | TIMESTAMPTZ | DEFAULT NOW() | Join timestamp |

### Content Management

#### `posts` table
Community posts and discussions.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Post unique identifier |
| `community_id` | UUID | FK → communities.id | Community reference |
| `author_id` | UUID | FK → users.id | Author user ID |
| `title` | VARCHAR(200) | NOT NULL | Post title |
| `content` | TEXT | NOT NULL | Post content |
| `type` | VARCHAR(20) | DEFAULT 'discussion' | Post type |
| `pinned` | BOOLEAN | DEFAULT false | Pin status |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |
| `updated_at` | TIMESTAMPTZ | DEFAULT NOW() | Last update timestamp |

#### `comments` table
Comments on posts.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Comment unique identifier |
| `post_id` | UUID | FK → posts.id | Post reference |
| `author_id` | UUID | FK → users.id | Author user ID |
| `content` | TEXT | NOT NULL | Comment content |
| `parent_id` | UUID | FK → comments.id | Parent comment (for replies) |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |

### Business Directory

#### `businesses` table
Local business listings.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Business unique identifier |
| `name` | VARCHAR(100) | NOT NULL | Business name |
| `slug` | VARCHAR(100) | UNIQUE, NOT NULL | URL-friendly name |
| `description` | TEXT | | Business description |
| `category` | VARCHAR(50) | | Business category |
| `address` | TEXT | | Street address |
| `location` | GEOGRAPHY(POINT) | | Geographic location |
| `phone` | VARCHAR(20) | | Phone number |
| `website` | TEXT | | Website URL |
| `email` | VARCHAR(255) | | Contact email |
| `owner_id` | UUID | FK → users.id | Business owner |
| `verified` | BOOLEAN | DEFAULT false | Verification status |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |

### Governance System

#### `proposals` table
Community proposals for voting.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Proposal unique identifier |
| `community_id` | UUID | FK → communities.id | Community reference |
| `author_id` | UUID | FK → users.id | Author user ID |
| `title` | VARCHAR(200) | NOT NULL | Proposal title |
| `description` | TEXT | NOT NULL | Proposal description |
| `type` | VARCHAR(20) | DEFAULT 'poll' | Proposal type |
| `status` | VARCHAR(20) | DEFAULT 'active' | Proposal status |
| `starts_at` | TIMESTAMPTZ | | Voting start time |
| `ends_at` | TIMESTAMPTZ | | Voting end time |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |

#### `votes` table
User votes on proposals.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Vote unique identifier |
| `proposal_id` | UUID | FK → proposals.id | Proposal reference |
| `user_id` | UUID | FK → users.id | Voter user ID |
| `choice` | VARCHAR(50) | NOT NULL | Vote choice |
| `voted_at` | TIMESTAMPTZ | DEFAULT NOW() | Vote timestamp |

### Chat System

#### `chat_rooms` table
Chat rooms for communities.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Room unique identifier |
| `community_id` | UUID | FK → communities.id | Community reference |
| `name` | VARCHAR(100) | NOT NULL | Room name |
| `type` | VARCHAR(20) | DEFAULT 'public' | Room type |
| `created_by` | UUID | FK → users.id | Creator user ID |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |

#### `chat_messages` table
Messages in chat rooms.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Message unique identifier |
| `room_id` | UUID | FK → chat_rooms.id | Room reference |
| `author_id` | UUID | FK → users.id | Author user ID |
| `content` | TEXT | NOT NULL | Message content |
| `message_type` | VARCHAR(20) | DEFAULT 'text' | Message type |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |

## REST API Endpoints

### Authentication End

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/auth/login` | GET | No | Redirect to Auth0 login |
| `/auth/callback` | GET | No | Handle Auth0 OAuth2 callback |
| `/auth/logout` | POST | No | Logout and clear session |
| `/auth/me` | GET | Yes | Get current user info |

### Users

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/users` | GET | No | List all users |
| `/api/users` | POST | No | Create new user |
| `/api/users/:id` | GET | No | Get user by ID |
| `/api/users/:id` | PUT | Yes | Update user profile |
| `/api/users/:id` | DELETE | Yes | Delete user account |

#### Create User Example

```bash
curl -X POST http://localhost:9001/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "mario",
    "email": "mario@example.com",
    "password": "password123"
  }'
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "mario",
    "email": "mario@example.com",
    "created_at": "2025-11-20 06:00"
  },
  "message": "User created successfully"
}
```

### Communities

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/communities` | GET | No | List all communities |
| `/api/communities` | POST | Yes | Create new community |
| `/api/communities/:id` | GET | No | Get community by ID |
| `/api/communities/:id` | PUT | Yes | Update community (owner only) |
| `/api/communities/:id` | DELETE | Yes | Delete community (owner only) |
| `/api/communities/:id/join` | POST | Yes | Join community |
| `/api/communities/:id/leave` | POST | Yes | Leave community |
| `/api/communities/:id/members` | GET | No | List community members |

#### Create Community Example

```bash
curl -X POST http://localhost:9001/api/communities \
  -H "Content-Type: application/json" \
  -H "Cookie: community_manager_session=..." \
  -d '{
    "name": "Tech Enthusiasts",
    "slug": "tech-enthusiasts",
    "description": "A community for tech lovers",
    "category": "technology",
    "is_private": false
  }'
```

### Posts

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/communities/:id/posts` | GET | No | List community posts |
| `/api/communities/:id/posts` | POST | Yes | Create new post |
| `/api/posts/:id` | GET | No | Get post by ID |
| `/api/posts/:id` | PUT | Yes | Update post (author only) |
| `/api/posts/:id` | DELETE | Yes | Delete post (author/mod) |

#### Create Post Example

```bash
curl -X POST http://localhost:9001/api/communities/550e8400-e29b-41d4-a716-446655440000/posts \
  -H "Content-Type: application/json" \
  -H "Cookie: community_manager_session=..." \
  -d '{
    "title": "Welcome to our community!",
    "content": "This is our first post in this amazing community.",
    "type": "discussion"
  }'
```

### Comments

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/posts/:id/comments` | GET | No | List post comments |
| `/api/posts/:id/comments` | POST | Yes | Create new comment |
| `/api/comments/:id` | PUT | Yes | Update comment (author only) |
| `/api/comments/:id` | DELETE | Yes | Delete comment (author/mod) |

### Business Directory

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/businesses` | GET | No | List all businesses |
| `/api/businesses` | POST | Yes | Create new business |
| `/api/businesses/:id` | GET | No | Get business by ID |
| `/api/businesses/:id` | PUT | Yes | Update business (owner only) |
| `/api/businesses/:id` | DELETE | Yes | Delete business (owner only) |
| `/api/businesses/search` | GET | No | Search businesses by location/category |

### Governance

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/communities/:id/proposals` | GET | No | List community proposals |
| `/api/communities/:id/proposals` | POST | Yes | Create new proposal |
| `/api/proposals/:id/vote` | POST | Yes | Vote on proposal |
| `/api/proposals/:id/results` | GET | No | Get proposal results |

### Chat System

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/communities/:id/rooms` | GET | No | List community chat rooms |
| `/api/communities/:id/rooms` | POST | Yes | Create new chat room |
| `/api/rooms/:id/messages` | GET | No | Get room messages |
| `/api/rooms/:id/messages` | POST | Yes | Send message to room |

## HTMX End

### Dynamic UI Endpoints

| Endpoint | Method | Auth Required | Description |
|----------|--------|---------------|-------------|
| `/api/nav` | GET | No | Navigation fragment |
| `/api/user/communities` | GET | Yes | User's communities list |
| `/api/user/activity` | GET | Yes | User's recent activity |
| `/api/communities/recent` | GET | No | Recent communities fragment |
| `/api/communities/list` | GET | No | Communities list fragment |

## Database Queries

### Common Query Patterns

#### User Communities with Member Count

```sql
SELECT c.id, c.name, c.description, c.created_at, 
       COUNT(DISTINCT m.user_id) as member_count
FROM communities c
LEFT JOIN community_members m ON c.id = m.community_id
WHERE c.created_by = $1
GROUP BY c.id, c.name, c.description, c.created_at
ORDER BY c.created_at DESC
LIMIT 10
```

#### Recent Activity from User's Communities

```sql
SELECT p.id, p.title, p.community_id, c.name as community_name, p.created_at
FROM posts p
JOIN communities c ON p.community_id = c.id
WHERE c.created_by = $1
ORDER BY p.created_at DESC
LIMIT 5
```

#### Geographic Search for Businesses

```sql
SELECT id, name, description, address, category,
       ST_Distance(location, ST_GeomFromText($1, 4326)) as distance_meters
FROM businesses
WHERE ST_DWithin(location, ST_GeomFromText($1, 4326), 5000) -- 5km radius
ORDER BY distance_meters
LIMIT 20
```

### Performance Optimizations

#### Connection Pooling

```rust
// In shared/src/database.rs
let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(10)
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(8))
    .connect(&database_url)
    .await?;
```

#### Query Optimization

- Use `LIMIT` for pagination
- Add appropriate indexes on foreign keys
- Use `EXPLAIN ANALYZE` for query tuning
- Leverage CockroachDB's distributed nature

## Error Handling

### Standard Error Response Format

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": {
      "field": "email",
      "reason": "Invalid email format"
    }
  }
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `VALIDATION_ERROR` | 400 | Input validation failed |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Permission denied |
| `NOT_FOUND` | 404 | Resource not found |
| `CONFLICT` | 409 | Resource already exists |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |

## Testing

### Database Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) {
        // Test user creation with real database
    }

    #[sqlx::test]
    async fn test_community_membership(pool: PgPool) {
        // Test community membership logic
    }
}
```

### API Integration Tests

```bash
# Test authentication flow
curl -c cookies.txt -X POST http://localhost:9001/auth/login

# Test protected endpoint
curl -b cookies.txt http://localhost:9001/api/user/communities

# Test API with invalid data
curl -X POST http://localhost:9001/api/users \
  -H "Content-Type: application/json" \
  -d '{"invalid": "data"}'
```

## Migration Management

### Creating New Migrations

```bash
# Create new migration
sqlx migrate add create_new_table

# Run migrations
sqlx migrate run

# Rollback migration
sqlx migrate revert
```

### Migration Example

```sql
-- migrations/007_add_user_preferences.sql
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    theme VARCHAR(20) DEFAULT 'light',
    language VARCHAR(10) DEFAULT 'en',
    email_notifications BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_user_preferences_user_id ON user_preferences(user_id);
```

---

**Last Updated**: November 25, 2025  
**Related Files**: 
- `src/migrations/` - Database migration files
- `src/server/src/handlers/`` - API route handlers
- `src/shared/src/database.rs` - Database connection logic
