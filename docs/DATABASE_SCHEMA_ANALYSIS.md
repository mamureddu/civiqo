# Database Schema Analysis: UUID vs BIGINT

**Date**: November 25, 2025  
**Database**: CockroachDB Cloud  
**Analysis**: Performance optimization for primary and foreign keys

---

## 📊 **Executive Summary**

### Current State
- **Total Tables**: 26
- **UUID Primary Keys**: 24 tables
- **UUID Foreign Keys**: 100+ references
- **Performance Impact**: Moderate (UUIDs are 16 bytes vs BIGINT 8 bytes)

### Recommendation
**Convert to BIGINT with SERIAL/SEQUENCE** for better retrieval performance, EXCEPT:
- ✅ **Keep UUID**: `users.id` (comes from Auth0 as UUID - cannot change)
- ✅ **Keep UUID**: `user_profiles.user_id` (foreign key to users.id)
- ✅ **Keep UUID**: `user_keys.user_id` (foreign key to users.id)
- ✅ **Keep UUID**: All other `user_id` references (foreign key to users.id)

---

## 🔍 **Detailed Analysis by Table**

### ✅ **KEEP AS UUID (Auth0 Integration)**

**users.id** - UUID
- **Reason**: Comes from Auth0 as UUID, cannot be changed
- **Impact**: All user_id foreign keys must remain UUID
- **Tables affected**: 15+ tables with user_id references

```sql
-- Auth0 provides UUID format
-- Cannot change without breaking authentication
```

---

### ❌ **CONVERT TO BIGINT (Performance Optimization)**

#### 1. **communities** Table
```sql
-- Current
id UUID PRIMARY KEY,
created_by UUID,  -- Foreign key to users (keep UUID)

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
created_by UUID,  -- Keep as UUID (foreign key to users)
```

**Why**: 
- Primary key used frequently in queries
- 16 bytes (UUID) → 8 bytes (BIGINT) = 50% smaller
- Faster index lookups
- Better cache locality

---

#### 2. **businesses** Table
```sql
-- Current
id UUID PRIMARY KEY,
community_id UUID,  -- Foreign key to communities
owner_id UUID,      -- Foreign key to users (keep UUID)

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
community_id BIGINT,  -- Foreign key to communities (change)
owner_id UUID,        -- Keep as UUID (foreign key to users)
```

---

#### 3. **community_members** Table
```sql
-- Current
id UUID PRIMARY KEY,
user_id UUID,           -- Foreign key to users (keep UUID)
community_id UUID,      -- Foreign key to communities
role_id UUID,           -- Foreign key to roles

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
user_id UUID,           -- Keep as UUID (foreign key to users)
community_id BIGINT,    -- Foreign key to communities (change)
role_id BIGINT,         -- Foreign key to roles (change)
```

---

#### 4. **chat_rooms** Table
```sql
-- Current
id UUID PRIMARY KEY,
community_id UUID,  -- Foreign key to communities
created_by UUID,    -- Foreign key to users (keep UUID)

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
community_id BIGINT,  -- Foreign key to communities (change)
created_by UUID,      -- Keep as UUID (foreign key to users)
```

---

#### 5. **proposals** Table
```sql
-- Current
id UUID PRIMARY KEY,
community_id UUID,  -- Foreign key to communities
created_by UUID,    -- Foreign key to users (keep UUID)

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
community_id BIGINT,  -- Foreign key to communities (change)
created_by UUID,      -- Keep as UUID (foreign key to users)
```

---

#### 6. **decisions** Table
```sql
-- Current
id UUID PRIMARY KEY,
community_id UUID,  -- Foreign key to communities
created_by UUID,    -- Foreign key to users (keep UUID)

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
community_id BIGINT,  -- Foreign key to communities (change)
created_by UUID,      -- Keep as UUID (foreign key to users)
```

---

#### 7. **polls** Table
```sql
-- Current
id UUID PRIMARY KEY,
community_id UUID,  -- Foreign key to communities
created_by UUID,    -- Foreign key to users (keep UUID)

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
community_id BIGINT,  -- Foreign key to communities (change)
created_by UUID,      -- Keep as UUID (foreign key to users)
```

---

#### 8. **votes** Table
```sql
-- Current
id UUID PRIMARY KEY,
poll_id UUID,   -- Foreign key to polls
user_id UUID,   -- Foreign key to users (keep UUID)

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
poll_id BIGINT,  -- Foreign key to polls (change)
user_id UUID,    -- Keep as UUID (foreign key to users)
```

---

#### 9. **proposal_comments** Table
```sql
-- Current
id UUID PRIMARY KEY,
proposal_id UUID,  -- Foreign key to proposals
user_id UUID,      -- Foreign key to users (keep UUID)
parent_id UUID,    -- Self-reference to proposal_comments

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
proposal_id BIGINT,  -- Foreign key to proposals (change)
user_id UUID,        -- Keep as UUID (foreign key to users)
parent_id BIGINT,    -- Self-reference (change)
```

---

#### 10. **decision_votes** Table
```sql
-- Current
decision_id UUID,  -- Foreign key to decisions
user_id UUID,      -- Foreign key to users (keep UUID)

-- Recommended
decision_id BIGINT,  -- Foreign key to decisions (change)
user_id UUID,        -- Keep as UUID (foreign key to users)
```

---

#### 11. **room_participants** Table
```sql
-- Current
id UUID PRIMARY KEY,
room_id UUID,   -- Foreign key to chat_rooms
user_id UUID,   -- Foreign key to users (keep UUID)

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
room_id BIGINT,  -- Foreign key to chat_rooms (change)
user_id UUID,    -- Keep as UUID (foreign key to users)
```

---

#### 12. **roles** Table
```sql
-- Current
id UUID PRIMARY KEY,

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
```

---

#### 13. **temp_offline_messages** Table
```sql
-- Current
id UUID PRIMARY KEY,
recipient_id UUID,  -- Foreign key to users (keep UUID)
sender_id UUID,     -- Foreign key to users (keep UUID)
room_id UUID,       -- Foreign key to chat_rooms

-- Recommended
id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
recipient_id UUID,  -- Keep as UUID (foreign key to users)
sender_id UUID,     -- Keep as UUID (foreign key to users)
room_id BIGINT,     -- Foreign key to chat_rooms (change)
```

---

#### 14. **community_boundaries** Table
```sql
-- Current
community_id UUID,  -- Foreign key to communities

-- Recommended
community_id BIGINT,  -- Foreign key to communities (change)
```

---

#### 15. **business_hours** Table
```sql
-- Current
business_id UUID,  -- Foreign key to businesses

-- Recommended
business_id BIGINT,  -- Foreign key to businesses (change)
```

---

#### 16. **business_images** Table
```sql
-- Current
business_id UUID,  -- Foreign key to businesses

-- Recommended
business_id BIGINT,  -- Foreign key to businesses (change)
```

---

#### 17. **business_products** Table
```sql
-- Current
business_id UUID,  -- Foreign key to businesses

-- Recommended
business_id BIGINT,  -- Foreign key to businesses (change)
```

---

#### 18. **active_connections** Table
```sql
-- Current
user_id UUID,  -- Foreign key to users (keep UUID)
room_id UUID,  -- Foreign key to chat_rooms

-- Recommended
user_id UUID,  -- Keep as UUID (foreign key to users)
room_id BIGINT,  -- Foreign key to chat_rooms (change)
```

---

## 📋 **Summary Table**

| Table | Current ID | Recommended | Foreign Keys to Change |
|-------|-----------|------------|------------------------|
| communities | UUID | BIGINT | - |
| businesses | UUID | BIGINT | community_id |
| community_members | UUID | BIGINT | community_id, role_id |
| chat_rooms | UUID | BIGINT | community_id |
| proposals | UUID | BIGINT | community_id |
| decisions | UUID | BIGINT | community_id |
| polls | UUID | BIGINT | community_id |
| votes | UUID | BIGINT | poll_id |
| proposal_comments | UUID | BIGINT | proposal_id, parent_id |
| decision_votes | - | BIGINT | decision_id |
| room_participants | UUID | BIGINT | room_id |
| roles | UUID | BIGINT | - |
| temp_offline_messages | UUID | BIGINT | room_id |
| community_boundaries | - | BIGINT | community_id |
| business_hours | - | BIGINT | business_id |
| business_images | - | BIGINT | business_id |
| business_products | - | BIGINT | business_id |
| active_connections | - | BIGINT | room_id |
| **users** | **UUID** | **KEEP UUID** | **N/A** |
| **user_profiles** | UUID | **KEEP UUID** | **user_id** |
| **user_keys** | UUID | **KEEP UUID** | **user_id** |

---

## 🚀 **Migration Strategy**

### Phase 1: Create New Tables with BIGINT
```sql
-- Create new tables with BIGINT primary keys
-- Use CockroachDB's unique_rowid() for auto-increment

CREATE TABLE communities_new (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    slug VARCHAR(255) NOT NULL,
    is_public BOOLEAN DEFAULT true,
    requires_approval BOOLEAN DEFAULT false,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### Phase 2: Migrate Data
```sql
-- Copy data from old table to new table
INSERT INTO communities_new (id, name, description, slug, is_public, requires_approval, created_by, created_at, updated_at)
SELECT id::BIGINT, name, description, slug, is_public, requires_approval, created_by, created_at, updated_at
FROM communities;
```

### Phase 3: Update Foreign Keys
```sql
-- Update all foreign key references
ALTER TABLE businesses ADD COLUMN community_id_new BIGINT;
UPDATE businesses SET community_id_new = community_id::BIGINT;
ALTER TABLE businesses DROP COLUMN community_id;
ALTER TABLE businesses RENAME COLUMN community_id_new TO community_id;
```

### Phase 4: Drop Old Tables
```sql
DROP TABLE communities;
ALTER TABLE communities_new RENAME TO communities;
```

---

## 📊 **Performance Impact**

### Storage Savings
- **UUID**: 16 bytes per ID
- **BIGINT**: 8 bytes per ID
- **Savings**: 50% reduction in index size

### Example: communities table with 10,000 records
- **UUID Primary Key Index**: ~160 KB
- **BIGINT Primary Key Index**: ~80 KB
- **Savings**: ~80 KB per index

### Query Performance
- **Faster index lookups**: Smaller key size = better cache locality
- **Faster JOINs**: Smaller foreign key comparisons
- **Better pagination**: Smaller offset calculations

---

## ⚠️ **Important Notes**

### What CANNOT Change
1. **users.id** - Must remain UUID (Auth0 integration)
2. **All user_id foreign keys** - Must remain UUID (references users.id)
3. **user_profiles.user_id** - Must remain UUID (references users.id)
4. **user_keys.user_id** - Must remain UUID (references users.id)

### What CAN Change
1. **All other primary keys** - Convert to BIGINT
2. **All other foreign keys** - Convert to BIGINT
3. **Self-referencing keys** - Convert to BIGINT

---

## 🔧 **CockroachDB Commands**

### Check Current Schema
```sql
SELECT table_name, column_name, data_type 
FROM information_schema.columns 
WHERE table_schema = 'public'
ORDER BY table_name, ordinal_position;
```

### Generate Migration Script
```sql
-- Generate ALTER TABLE statements
SELECT 
    'ALTER TABLE ' || table_name || ' ALTER COLUMN id SET DEFAULT unique_rowid();' 
FROM information_schema.tables 
WHERE table_schema = 'public' 
AND table_name NOT IN ('users', 'user_profiles', 'user_keys', '_sqlx_migrations');
```

### Verify BIGINT Conversion
```sql
SELECT table_name, column_name, data_type 
FROM information_schema.columns 
WHERE data_type = 'bigint' 
ORDER BY table_name;
```

---

## 📅 **Recommended Timeline**

### Immediate (Phase 2b)
- Document this analysis
- Plan migration strategy
- Create backup

### Short-term (After Phase 2b)
- Create new tables with BIGINT
- Migrate data
- Update foreign keys
- Test thoroughly

### Long-term (Next Release)
- Drop old tables
- Update application code
- Monitor performance improvements

---

## 💡 **Conclusion**

**Recommendation**: Convert all non-user primary and foreign keys from UUID to BIGINT using CockroachDB's `unique_rowid()` function.

**Benefits**:
- ✅ 50% reduction in index size
- ✅ Faster query performance
- ✅ Better cache locality
- ✅ Reduced storage costs

**Constraints**:
- ✅ Keep users.id as UUID (Auth0 requirement)
- ✅ Keep all user_id references as UUID
- ✅ Maintain referential integrity

**Effort**: Medium (requires careful migration planning)

---

**Next Step**: Implement migration in Phase 3 or later release cycle.
