# Implementation Plan: Federation & Self-Hosted Communities

**Agent 1**: Follow this step-by-step guide  
**Update**: Check off items as completed

---

## Phase 1: Database & Models

### Step 1.1: Create Migration
**File**: `src/migrations/009_add_federation_support.sql`

```sql
-- Add federation fields to communities table
ALTER TABLE communities ADD COLUMN IF NOT EXISTS is_self_hosted BOOLEAN DEFAULT false;
ALTER TABLE communities ADD COLUMN IF NOT EXISTS hosting_url VARCHAR(500);
ALTER TABLE communities ADD COLUMN IF NOT EXISTS api_endpoint VARCHAR(500);
ALTER TABLE communities ADD COLUMN IF NOT EXISTS federation_key VARCHAR(255);
ALTER TABLE communities ADD COLUMN IF NOT EXISTS federation_status VARCHAR(50) DEFAULT 'local';
ALTER TABLE communities ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMP WITH TIME ZONE;

-- Create index on federation_status for filtering
CREATE INDEX IF NOT EXISTS idx_communities_federation_status ON communities(federation_status);
CREATE INDEX IF NOT EXISTS idx_communities_is_self_hosted ON communities(is_self_hosted);

-- Create federation registry table
CREATE TABLE IF NOT EXISTS federation_registry (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    community_id BIGINT NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    remote_url VARCHAR(500) NOT NULL,
    api_version VARCHAR(20) DEFAULT 'v1',
    public_key TEXT,
    verified_at TIMESTAMP WITH TIME ZONE,
    health_status VARCHAR(50) DEFAULT 'unknown',
    last_health_check TIMESTAMP WITH TIME ZONE,
    error_count INT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(community_id)
);

CREATE INDEX IF NOT EXISTS idx_federation_registry_health ON federation_registry(health_status);
CREATE INDEX IF NOT EXISTS idx_federation_registry_community ON federation_registry(community_id);
```

- [ ] Create migration file
- [ ] Run migration: `sqlx migrate run`
- [ ] Regenerate SQLx cache: `cargo sqlx prepare`

### Step 1.2: Update Community Models
**File**: `src/server/src/handlers/api.rs`

Add to `CreateCommunityRequest`:
```rust
pub is_self_hosted: Option<bool>,
pub hosting_url: Option<String>,
```

Add to `CommunityResponse`:
```rust
pub is_self_hosted: bool,
pub hosting_url: Option<String>,
pub federation_status: String,
```

- [ ] Update CreateCommunityRequest
- [ ] Update CommunityResponse
- [ ] Update create_community handler
- [ ] Update get_community_detail handler

### Step 1.3: Create Federation Models
**File**: `src/server/src/models/federation.rs`

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct FederationInfo {
    pub instance_url: String,
    pub instance_name: String,
    pub api_version: String,
    pub public_key: String,
    pub supported_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FederationRegistration {
    pub instance_url: String,
    pub public_key: String,
    pub community_id: i64,
    pub community_name: String,
    pub community_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FederatedCommunity {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub member_count: i64,
    pub created_at: String,
}
```

- [ ] Create federation.rs
- [ ] Add to models/mod.rs
- [ ] Add to lib.rs exports

---

## Phase 2: Federation Protocol Endpoints

### Step 2.1: Create Federation Handler
**File**: `src/server/src/handlers/federation.rs`

```rust
/// GET /api/federation/info
/// Returns instance metadata for federation discovery
pub async fn federation_info() -> Json<FederationInfo>

/// GET /api/federation/communities
/// Returns list of public communities for federation
pub async fn federation_communities() -> Json<Vec<FederatedCommunity>>

/// GET /api/federation/communities/:id
/// Returns single community data for federation
pub async fn federation_community_detail(Path(id): Path<i64>) -> Json<FederatedCommunity>

/// GET /api/federation/communities/:id/htmx/:fragment
/// Returns HTMX fragment for embedding
pub async fn federation_htmx_fragment(
    Path((id, fragment)): Path<(i64, String)>
) -> Html<String>
```

- [ ] Create federation.rs handler file
- [ ] Implement federation_info
- [ ] Implement federation_communities
- [ ] Implement federation_community_detail
- [ ] Implement federation_htmx_fragment
- [ ] Add routes to main.rs

### Step 2.2: Add Federation Routes
**File**: `src/server/src/main.rs`

```rust
// Federation Protocol
.route("/api/federation/info", get(federation::federation_info))
.route("/api/federation/communities", get(federation::federation_communities))
.route("/api/federation/communities/:id", get(federation::federation_community_detail))
.route("/api/federation/communities/:id/htmx/:fragment", get(federation::federation_htmx_fragment))
```

- [ ] Add federation routes
- [ ] Import federation handler module

---

## Phase 3: Aggregator Endpoints

### Step 3.1: Create Aggregator Handler
**File**: `src/server/src/handlers/aggregator.rs`

```rust
/// GET /api/aggregator/communities
/// Returns all communities (local + federated)
pub async fn aggregator_communities() -> Json<Vec<CommunityListItem>>

/// POST /api/aggregator/register
/// Accept federation registration request
pub async fn aggregator_register(
    Json(registration): Json<FederationRegistration>
) -> Result<Json<ApiResponse<()>>, StatusCode>

/// POST /api/aggregator/verify
/// Verify federation key
pub async fn aggregator_verify(
    Json(verification): Json<FederationVerification>
) -> Result<Json<ApiResponse<()>>, StatusCode>

/// GET /api/aggregator/proxy/:community_id/*path
/// Proxy requests to federated instance
pub async fn aggregator_proxy(
    Path((community_id, path)): Path<(i64, String)>
) -> Result<Response, StatusCode>

/// DELETE /api/aggregator/unregister/:community_id
/// Remove federation
pub async fn aggregator_unregister(
    Path(community_id): Path<i64>
) -> Result<Json<ApiResponse<()>>, StatusCode>
```

- [ ] Create aggregator.rs handler file
- [ ] Implement aggregator_communities
- [ ] Implement aggregator_register
- [ ] Implement aggregator_verify
- [ ] Implement aggregator_proxy
- [ ] Implement aggregator_unregister
- [ ] Add routes to main.rs

---

## Phase 4: HTMX Proxy Layer

### Step 4.1: Create Proxy Service
**File**: `src/server/src/services/proxy.rs`

```rust
pub struct ProxyService {
    client: reqwest::Client,
    cache: Cache<String, String>,
}

impl ProxyService {
    /// Fetch and cache federated content
    pub async fn fetch_htmx(&self, url: &str) -> Result<String, ProxyError>
    
    /// Sanitize HTML content
    pub fn sanitize_html(&self, html: &str) -> String
    
    /// Rewrite URLs to use proxy
    pub fn rewrite_urls(&self, html: &str, community_id: i64) -> String
}
```

- [ ] Create proxy.rs service
- [ ] Implement HTTP client with timeout
- [ ] Implement caching (use moka or similar)
- [ ] Implement HTML sanitization
- [ ] Implement URL rewriting

### Step 4.2: Create Content Sanitizer
**File**: `src/server/src/services/content_sanitizer.rs`

```rust
pub fn sanitize_htmx_content(html: &str) -> String {
    // Remove script tags
    // Remove dangerous attributes (onclick, onerror, etc.)
    // Allow only safe HTMX attributes
    // Validate URLs
}
```

- [ ] Create content_sanitizer.rs
- [ ] Implement HTML sanitization
- [ ] Add tests for sanitization

---

## Phase 5: Self-Hosting Package

### Step 5.1: Configuration
**File**: `src/server/src/config.rs` (update)

```rust
pub struct FederationConfig {
    pub enabled: bool,
    pub mode: FederationMode, // Aggregator, SelfHosted, Standalone
    pub aggregator_url: Option<String>,
    pub instance_url: String,
    pub private_key_path: Option<String>,
    pub public_key_path: Option<String>,
}

pub enum FederationMode {
    Aggregator,   // civiqo.com
    SelfHosted,   // Federated with aggregator
    Standalone,   // Not federated
}
```

- [ ] Add FederationConfig
- [ ] Load from environment variables
- [ ] Validate configuration

### Step 5.2: Health Check Endpoint
**File**: `src/server/src/handlers/health.rs` (update)

```rust
/// GET /health/federation
/// Returns federation health status
pub async fn federation_health() -> Json<FederationHealth>
```

- [ ] Add federation health endpoint
- [ ] Include connectivity to aggregator
- [ ] Include last sync status

---

## Phase 6: UI Updates

### Step 6.1: Community Creation Form
**File**: `src/server/templates/communities/create.html`

Add fields:
- [ ] Self-hosted checkbox
- [ ] Hosting URL input (shown when self-hosted)
- [ ] Federation status display

### Step 6.2: Community List
**File**: `src/server/templates/communities/list.html`

Add:
- [ ] Federation status badge
- [ ] "External" indicator for federated communities
- [ ] Health status indicator

### Step 6.3: Community Detail
**File**: `src/server/templates/communities/detail.html`

Add:
- [ ] Federation info section
- [ ] "Hosted on" link for self-hosted
- [ ] Sync status

---

## Phase 7: Testing

### Step 7.1: Integration Tests
**File**: `src/server/tests/federation_test.rs`

- [ ] Test federation info endpoint
- [ ] Test federation communities endpoint
- [ ] Test aggregator registration
- [ ] Test proxy functionality
- [ ] Test content sanitization

### Step 7.2: Security Tests

- [ ] Test SQL injection in federation endpoints
- [ ] Test XSS prevention in proxy
- [ ] Test rate limiting
- [ ] Test invalid federation keys

---

## Phase 8: Documentation

### Step 8.1: Federation Guide
**File**: `docs/FEDERATION.md`

- [ ] Overview of federation architecture
- [ ] API documentation
- [ ] Security considerations
- [ ] Troubleshooting

### Step 8.2: Self-Hosting Guide
**File**: `docs/SELF_HOSTING.md`

- [ ] Prerequisites
- [ ] Installation steps
- [ ] Configuration
- [ ] Federation registration
- [ ] Maintenance

---

## Verification Checklist

- [ ] All migrations applied
- [ ] All endpoints working
- [ ] Proxy caching functional
- [ ] Content sanitization tested
- [ ] Health checks running
- [ ] UI updates complete
- [ ] All tests passing
- [ ] Documentation complete

---

## Estimated Timeline

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| Phase 1: Database & Models | 2-3 hours | None |
| Phase 2: Federation Protocol | 4-6 hours | Phase 1 |
| Phase 3: Aggregator | 4-6 hours | Phase 2 |
| Phase 4: HTMX Proxy | 3-4 hours | Phase 3 |
| Phase 5: Self-Hosting | 4-6 hours | Phase 4 |
| Phase 6: UI Updates | 2-3 hours | Phase 5 |
| Phase 7: Testing | 2-3 hours | Phase 6 |
| Phase 8: Documentation | 2-3 hours | Phase 7 |

**Total Estimated Time**: 23-34 hours (3-4 days)
