# Database Schema Documentation

This document provides a comprehensive overview of the Community Manager database schema.

## Overview

- **Database**: CockroachDB Serverless (PostgreSQL-compatible)
- **Total Tables**: 22
- **Migrations**: 6 migration files in `backend/migrations/`
- **ORM**: SQLx with raw SQL queries

## Table of Contents

1. [User Management](#user-management)
2. [Community Management](#community-management)
3. [Business Management](#business-management)
4. [Governance](#governance)
5. [Chat System](#chat-system)
6. [Enums](#enums)
7. [Indexes](#indexes)

---

## User Management

### `users`
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

**Indexes:**
- `idx_users_auth0_id` on `auth0_id`
- `idx_users_email` on `email`
- `idx_users_location` (spatial) on `location`

---

## Community Management

### `communities`
Main community table.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Community unique identifier |
| `name` | VARCHAR(255) | NOT NULL | Community name |
| `description` | TEXT | | Community description |
| `location` | GEOGRAPHY(POINT) | NOT NULL | Community location (PostGIS) |
| `radius_meters` | INTEGER | NOT NULL | Geographic radius in meters |
| `owner_id` | UUID | FK → users(id) | Community owner |
| `visibility` | community_visibility | DEFAULT 'public' | Public/private status |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |
| `updated_at` | TIMESTAMPTZ | DEFAULT NOW() | Last update timestamp |

**Indexes:**
- `idx_communities_location` (spatial) on `location`
- `idx_communities_owner` on `owner_id`

### `community_members`
Community membership and roles.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Membership unique identifier |
| `community_id` | UUID | FK → communities(id) | Community reference |
| `user_id` | UUID | FK → users(id) | User reference |
| `role_id` | UUID | FK → roles(id) | Role reference |
| `joined_at` | TIMESTAMPTZ | DEFAULT NOW() | Join timestamp |

**Constraints:**
- UNIQUE(`community_id`, `user_id`) - One membership per user per community

**Indexes:**
- `idx_community_members_community` on `community_id`
- `idx_community_members_user` on `user_id`

### `roles`
Role definitions with permissions.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Role unique identifier |
| `name` | VARCHAR(50) | UNIQUE, NOT NULL | Role name |
| `description` | TEXT | | Role description |
| `permissions` | JSONB | NOT NULL | Permissions array |
| `is_system` | BOOLEAN | DEFAULT false | System-defined role flag |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |

**Default Roles:**
- `owner` - Full control
- `socio` - Active member
- `investor` - Financial contributor
- `affiliate` - Partner
- `supporter` - Basic member

---

## Business Management

### `businesses`
Local business profiles.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Business unique identifier |
| `community_id` | UUID | FK → communities(id) | Community reference |
| `owner_id` | UUID | FK → users(id) | Business owner |
| `name` | VARCHAR(255) | NOT NULL | Business name |
| `description` | TEXT | | Business description |
| `category` | business_category | NOT NULL | Business category |
| `location` | GEOGRAPHY(POINT) | NOT NULL | Business location |
| `address` | TEXT | | Physical address |
| `phone` | VARCHAR(50) | | Contact phone |
| `email` | VARCHAR(255) | | Contact email |
| `website` | TEXT | | Business website |
| `verified` | BOOLEAN | DEFAULT false | Verification status |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |
| `updated_at` | TIMESTAMPTZ | DEFAULT NOW() | Last update timestamp |

**Indexes:**
- `idx_businesses_community` on `community_id`
- `idx_businesses_location` (spatial) on `location`
- `idx_businesses_category` on `category`

### `products`
Business products/services.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Product unique identifier |
| `business_id` | UUID | FK → businesses(id) | Business reference |
| `name` | VARCHAR(255) | NOT NULL | Product name |
| `description` | TEXT | | Product description |
| `price` | DECIMAL(10,2) | | Product price |
| `currency` | VARCHAR(3) | DEFAULT 'EUR' | Currency code |
| `available` | BOOLEAN | DEFAULT true | Availability status |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |
| `updated_at` | TIMESTAMPTZ | DEFAULT NOW() | Last update timestamp |

### `business_hours`
Operating hours for businesses.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Hours unique identifier |
| `business_id` | UUID | FK → businesses(id) | Business reference |
| `day_of_week` | INTEGER | CHECK (0-6) | Day (0=Sunday) |
| `open_time` | TIME | | Opening time |
| `close_time` | TIME | | Closing time |
| `is_closed` | BOOLEAN | DEFAULT false | Closed flag |

**Constraints:**
- UNIQUE(`business_id`, `day_of_week`)

### `business_images`
Business photo gallery.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Image unique identifier |
| `business_id` | UUID | FK → businesses(id) | Business reference |
| `url` | TEXT | NOT NULL | Image URL (S3) |
| `caption` | TEXT | | Image caption |
| `is_primary` | BOOLEAN | DEFAULT false | Primary image flag |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Upload timestamp |

---

## Governance

### `polls`
Community polls and voting.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Poll unique identifier |
| `community_id` | UUID | FK → communities(id) | Community reference |
| `creator_id` | UUID | FK → users(id) | Poll creator |
| `title` | VARCHAR(255) | NOT NULL | Poll title |
| `description` | TEXT | | Poll description |
| `options` | JSONB | NOT NULL | Poll options array |
| `poll_type` | poll_type | NOT NULL | Poll type |
| `status` | poll_status | DEFAULT 'active' | Poll status |
| `starts_at` | TIMESTAMPTZ | NOT NULL | Start timestamp |
| `ends_at` | TIMESTAMPTZ | NOT NULL | End timestamp |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |

**Indexes:**
- `idx_polls_community` on `community_id`
- `idx_polls_status` on `status`

### `votes`
Individual poll votes.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Vote unique identifier |
| `poll_id` | UUID | FK → polls(id) | Poll reference |
| `user_id` | UUID | FK → users(id) | Voter reference |
| `option_index` | INTEGER | NOT NULL | Selected option index |
| `voted_at` | TIMESTAMPTZ | DEFAULT NOW() | Vote timestamp |

**Constraints:**
- UNIQUE(`poll_id`, `user_id`) - One vote per user per poll

### `decisions`
Community decisions and proposals.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Decision unique identifier |
| `community_id` | UUID | FK → communities(id) | Community reference |
| `creator_id` | UUID | FK → users(id) | Decision creator |
| `title` | VARCHAR(255) | NOT NULL | Decision title |
| `description` | TEXT | NOT NULL | Decision description |
| `decision_type` | decision_type | NOT NULL | Decision type |
| `status` | decision_status | DEFAULT 'proposed' | Decision status |
| `required_votes` | INTEGER | | Votes needed |
| `votes_for` | INTEGER | DEFAULT 0 | Votes in favor |
| `votes_against` | INTEGER | DEFAULT 0 | Votes against |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |
| `decided_at` | TIMESTAMPTZ | | Decision timestamp |

### `decision_votes`
Votes on decisions.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Vote unique identifier |
| `decision_id` | UUID | FK → decisions(id) | Decision reference |
| `user_id` | UUID | FK → users(id) | Voter reference |
| `vote` | BOOLEAN | NOT NULL | true=for, false=against |
| `comment` | TEXT | | Vote comment |
| `voted_at` | TIMESTAMPTZ | DEFAULT NOW() | Vote timestamp |

**Constraints:**
- UNIQUE(`decision_id`, `user_id`)

### `decision_comments`
Comments on decisions.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Comment unique identifier |
| `decision_id` | UUID | FK → decisions(id) | Decision reference |
| `user_id` | UUID | FK → users(id) | Commenter reference |
| `content` | TEXT | NOT NULL | Comment content |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Comment timestamp |

---

## Chat System

### `chat_rooms`
Chat room definitions.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Room unique identifier |
| `community_id` | UUID | FK → communities(id) | Community reference |
| `name` | VARCHAR(255) | | Room name (optional for DMs) |
| `room_type` | room_type | NOT NULL | community/direct/group |
| `is_encrypted` | BOOLEAN | DEFAULT false | E2EE flag |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp |

**Indexes:**
- `idx_chat_rooms_community` on `community_id`

### `chat_participants`
Room membership.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Participant unique identifier |
| `room_id` | UUID | FK → chat_rooms(id) | Room reference |
| `user_id` | UUID | FK → users(id) | User reference |
| `joined_at` | TIMESTAMPTZ | DEFAULT NOW() | Join timestamp |
| `last_read_at` | TIMESTAMPTZ | | Last read timestamp |

**Constraints:**
- UNIQUE(`room_id`, `user_id`)

### `user_keys`
E2EE public keys.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Key unique identifier |
| `user_id` | UUID | FK → users(id) | User reference |
| `public_key` | TEXT | NOT NULL | Public key (base64) |
| `key_type` | VARCHAR(50) | NOT NULL | Key algorithm |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Key creation timestamp |
| `expires_at` | TIMESTAMPTZ | | Key expiration |

### `active_connections`
WebSocket connection tracking.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Connection unique identifier |
| `user_id` | UUID | FK → users(id) | User reference |
| `connection_id` | VARCHAR(255) | UNIQUE, NOT NULL | WebSocket connection ID |
| `instance_id` | VARCHAR(255) | NOT NULL | Server instance ID |
| `connected_at` | TIMESTAMPTZ | DEFAULT NOW() | Connection timestamp |
| `last_ping_at` | TIMESTAMPTZ | DEFAULT NOW() | Last heartbeat |

**Indexes:**
- `idx_active_connections_user` on `user_id`
- `idx_active_connections_instance` on `instance_id`

### `offline_messages`
Message queue for offline users.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | UUID | PRIMARY KEY | Message unique identifier |
| `recipient_id` | UUID | FK → users(id) | Recipient reference |
| `room_id` | UUID | FK → chat_rooms(id) | Room reference |
| `sender_id` | UUID | FK → users(id) | Sender reference |
| `content` | TEXT | NOT NULL | Message content (encrypted) |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | Message timestamp |
| `delivered_at` | TIMESTAMPTZ | | Delivery timestamp |

**Indexes:**
- `idx_offline_messages_recipient` on `recipient_id`

---

## Enums

### `community_visibility`
- `public` - Visible to all
- `private` - Invite-only

### `business_category`
- `restaurant`
- `retail`
- `service`
- `health`
- `education`
- `entertainment`
- `other`

### `poll_type`
- `simple` - Single choice
- `multiple` - Multiple choices
- `ranked` - Ranked voting

### `poll_status`
- `draft` - Not yet active
- `active` - Currently running
- `closed` - Voting ended
- `cancelled` - Cancelled

### `decision_type`
- `proposal` - General proposal
- `budget` - Budget allocation
- `policy` - Policy change
- `election` - Role election

### `decision_status`
- `proposed` - Under discussion
- `voting` - Active voting
- `approved` - Approved
- `rejected` - Rejected
- `implemented` - Implemented

### `room_type`
- `community` - Community-wide chat
- `direct` - Direct message (1-on-1)
- `group` - Group chat

---

## Triggers

### `update_updated_at_column`
Automatically updates `updated_at` timestamp on row modification.

Applied to:
- `users`
- `communities`
- `businesses`
- `products`

### `cleanup_old_offline_messages`
Automatically deletes delivered messages older than 30 days.

---

## Relationships Diagram

```
users
  ├─→ communities (owner_id)
  ├─→ community_members (user_id)
  ├─→ businesses (owner_id)
  ├─→ polls (creator_id)
  ├─→ votes (user_id)
  ├─→ decisions (creator_id)
  ├─→ decision_votes (user_id)
  ├─→ decision_comments (user_id)
  ├─→ chat_participants (user_id)
  ├─→ user_keys (user_id)
  ├─→ active_connections (user_id)
  └─→ offline_messages (recipient_id, sender_id)

communities
  ├─→ community_members (community_id)
  ├─→ businesses (community_id)
  ├─→ polls (community_id)
  ├─→ decisions (community_id)
  └─→ chat_rooms (community_id)

businesses
  ├─→ products (business_id)
  ├─→ business_hours (business_id)
  └─→ business_images (business_id)

polls
  └─→ votes (poll_id)

decisions
  ├─→ decision_votes (decision_id)
  └─→ decision_comments (decision_id)

chat_rooms
  ├─→ chat_participants (room_id)
  └─→ offline_messages (room_id)
```

---

## Migration Files

1. **001_initial.sql** - Users, communities, roles, base enums
2. **002_business.sql** - Business management tables
3. **003_governance.sql** - Polls, decisions, voting
4. **004_chat.sql** - Chat rooms, E2EE, WebSocket tracking
5. **005_*.sql** - Future migrations
6. **006_*.sql** - Future migrations

---

## Notes

- All timestamps use `TIMESTAMPTZ` for timezone awareness
- Geographic data uses PostGIS `GEOGRAPHY(POINT)` type
- UUIDs are used for all primary keys
- Foreign keys have `ON DELETE CASCADE` where appropriate
- Indexes are optimized for common query patterns
- JSONB is used for flexible schema (permissions, poll options)

---

**Last Updated**: November 18, 2025
