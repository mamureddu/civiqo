# Database Migrations

This directory contains SQL migrations for the Community Manager database schema.

## Migration Files

1. **001_initial.sql** - Core user management and community tables
   - Users and user profiles (synced from Auth0)
   - Communities and community settings
   - Role-based access control (RBAC)
   - Community memberships and boundaries
   - Basic indexes and triggers

2. **002_business.sql** - Business directory and local commerce
   - Business profiles with geographic data
   - Product/service catalogs
   - Business hours and images
   - Category-based organization

3. **003_governance.sql** - Democratic decision-making tools
   - Polls with multiple voting types
   - Formal decision processes
   - Proposal system with comments
   - Vote encryption and privacy

4. **004_chat.sql** - Real-time messaging (E2EE-focused)
   - Chat rooms and participants
   - User public key storage
   - Connection tracking for stateless services
   - Minimal temporary message storage

5. **005_seed_data.sql** - Development seed data
   - Default roles and permissions
   - Sample community and user
   - Demo business and chat room

## Database Features

### PostGIS Integration
- Geographic boundaries for communities
- Location-based business search
- Spatial indexes for performance

### Encryption Support
- Public key storage for E2EE chat
- Vote encryption for privacy
- Hash-based vote verification

### Performance Optimizations
- Strategic indexes on frequently queried columns
- Spatial indexes for geographic queries
- Automatic timestamp updates via triggers

### Data Integrity
- Foreign key constraints
- Check constraints for data validation
- Unique constraints where appropriate

## Running Migrations

### Using sqlx-cli
```bash
cd backend
sqlx migrate run --source migrations
```

### Using the setup script
```bash
./scripts/setup.sh  # Includes migration setup
```

### Manual execution
```bash
psql -d community_manager -f migrations/001_initial.sql
psql -d community_manager -f migrations/002_business.sql
# ... continue for all files
```

## Development Database

The migrations include sample data for development:
- Demo community with sample user
- Default roles and permissions
- Sample business listing
- General chat room

### Accessing the Development Database
- **Host**: localhost:5432
- **Database**: community_manager
- **Username**: dev
- **Password**: dev123
- **Admin UI**: http://localhost:8080 (Adminer)

## Schema Overview

### Core Entities
- **Users**: Auth0-synced user accounts
- **Communities**: Geographic-based communities
- **Businesses**: Local commerce directory
- **Governance**: Polls, decisions, proposals
- **Chat**: Real-time messaging rooms

### Key Relationships
- Users belong to communities with specific roles
- Businesses are owned by users within communities
- Governance tools operate within community context
- Chat rooms are community-specific

## Security Considerations

### E2EE Chat Design
- No message content stored in database
- Only public keys and metadata stored
- Temporary offline message queue (24h TTL)
- Connection tracking for stateless routing

### Privacy Features
- Encrypted poll votes with verification hashes
- Anonymous voting options
- Role-based access control
- Geographic data privacy options

## Performance Notes

### Indexing Strategy
- Foreign key indexes for join performance
- Status/type indexes for filtering
- Geographic indexes for location queries
- Timestamp indexes for time-based queries

### Cleanup Procedures
- Automatic cleanup of expired connections
- TTL-based message expiration
- Soft delete patterns where appropriate

## Future Enhancements

- Audit logging tables
- Advanced governance features
- Multi-language support
- Advanced geographic features
- Performance monitoring tables