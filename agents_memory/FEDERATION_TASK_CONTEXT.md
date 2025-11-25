# Task Context: Federation & Self-Hosted Communities

**Date**: November 25, 2025  
**Agent 2 Planning**: Complete  
**Status**: Ready for Agent 1 Implementation

---

## 🎯 Objectives

Implement federated architecture to overcome digital sovereignty issues:

1. **Self-Hosted Communities**: Allow communities to run on their own servers
2. **Direct HTMX Federation**: No proxy - direct HTMX requests to federated instances
3. **Public Key Verification**: Cryptographic proof of instance authenticity
4. **Pre-Verification**: Email + Domain verification before issuing keys
5. **Trust Levels**: Community, Verified, Partner tiers

---

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    civiqo.com (Aggregator)                       │
│                                                                  │
│  CSP: script-src 'self'    Cookies: SameSite=Strict             │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │ Local       │  │ Direct HTMX │  │ Direct HTMX │              │
│  │ Community   │  │ from remote1│  │ from remote2│              │
│  └─────────────┘  └──────┬──────┘  └──────┬──────┘              │
│                          │                 │                     │
└──────────────────────────┼─────────────────┼─────────────────────┘
                           │                 │
                    ┌──────▼──────┐   ┌──────▼──────┐
                    │ remote1.com │   │ remote2.org │
                    │ CORS: ✓     │   │ CORS: ✓     │
                    │ Keys: ✓     │   │ Keys: ✓     │
                    └─────────────┘   └─────────────┘
```

### Key Design Decisions

1. **No Proxy Layer**: Direct HTMX requests to federated instances
2. **Security via CSP**: `script-src 'self'` blocks malicious scripts
3. **Cookie Protection**: `SameSite=Strict` prevents cross-origin cookie access
4. **Public Key Auth**: Each instance has unique keypair for verification
5. **Pre-Verification**: Must verify email + domain before getting keys

---

## ✅ Acceptance Criteria

### Functional
- [ ] Communities can be marked as self-hosted with hosting URL
- [ ] Pre-verification: Email verification required
- [ ] Pre-verification: Domain verification (DNS TXT or .well-known file)
- [ ] Public key issued only after verification
- [ ] Instances can sign challenges with private key
- [ ] Aggregator verifies signatures with stored public key
- [ ] Key rotation via signing new key with old key
- [ ] Trust levels displayed: Community, Verified, Partner
- [ ] Version warnings for old instances
- [ ] Direct HTMX from federated instances (no proxy)
- [ ] Graceful fallback when instance offline

### Non-Functional
- [ ] Direct HTMX requests (no server overhead)
- [ ] CSP: `script-src 'self'` for security
- [ ] Cookies: `SameSite=Strict` protection
- [ ] HTTPS required for all federation
- [ ] Zero compilation errors
- [ ] All tests passing

---

## 🔑 Key Requirements

### Database Schema

**communities table additions**:
```sql
is_self_hosted BOOLEAN DEFAULT false
hosting_url VARCHAR(500)           -- e.g., https://community.example.com
api_endpoint VARCHAR(500)          -- e.g., https://community.example.com/api
federation_key VARCHAR(255)        -- Unique verification key
federation_status VARCHAR(50)      -- 'local', 'pending', 'active', 'suspended'
last_sync_at TIMESTAMP WITH TIME ZONE
```

**New table: federation_registry**:
```sql
id BIGINT PRIMARY KEY
community_id BIGINT REFERENCES communities(id)
remote_url VARCHAR(500) NOT NULL
api_version VARCHAR(20) DEFAULT 'v1'
public_key TEXT
verified_at TIMESTAMP WITH TIME ZONE
health_status VARCHAR(50) DEFAULT 'unknown'
last_health_check TIMESTAMP WITH TIME ZONE
created_at, updated_at TIMESTAMP WITH TIME ZONE
```

### API Endpoints

**Federation Protocol** (implemented by all instances):
- `GET /api/federation/info` - Instance metadata
- `GET /api/federation/communities` - List communities (public data only)
- `GET /api/federation/communities/:id` - Single community data
- `GET /api/federation/communities/:id/htmx/:fragment` - HTMX fragment

**Aggregator** (civiqo.com only):
- `GET /api/aggregator/communities` - All communities (local + federated)
- `POST /api/aggregator/register` - Accept federation registration
- `POST /api/aggregator/verify` - Verify federation key
- `GET /api/aggregator/proxy/:community_id/*` - Proxy requests to federated instance
- `DELETE /api/aggregator/unregister/:community_id` - Remove federation

### HTMX Proxy Requirements

1. **Transparent Proxying**: Federated HTMX fragments appear native
2. **Content Sanitization**: Strip dangerous scripts/styles
3. **URL Rewriting**: Rewrite internal links to use proxy
4. **Caching**: Cache federated content (configurable TTL)
5. **Fallback**: Show "Community offline" placeholder when unavailable

### Security Requirements

1. **Key Exchange**: Ed25519 public/private key pairs
2. **Request Signing**: Sign federation requests
3. **Content Validation**: Sanitize HTML, validate JSON
4. **Rate Limiting**: Per-instance rate limits
5. **CORS**: Strict origin validation
6. **TLS**: Require HTTPS for federation

---

## 📊 Technical Approach

### Architecture Modes

**Mode 1: Hosted (default)**
- Community data stored on civiqo.com
- `is_self_hosted = false`
- `hosting_url = NULL`

**Mode 2: Self-Hosted + Federated**
- Community data on external server
- Registered with civiqo.com aggregator
- `is_self_hosted = true`
- `hosting_url = 'https://community.example.com'`
- `federation_status = 'active'`

**Mode 3: Self-Hosted (standalone)**
- Community data on external server
- NOT registered with aggregator
- `is_self_hosted = true`
- `federation_status = 'local'`

### Federation Flow

```
1. Self-hosted instance starts
2. Admin initiates federation registration
3. Instance sends POST /api/aggregator/register with:
   - instance_url
   - public_key
   - community metadata
4. Aggregator verifies instance is reachable
5. Aggregator stores in federation_registry
6. Aggregator sets federation_status = 'pending'
7. Admin approves (or auto-approve based on policy)
8. federation_status = 'active'
9. Community appears in aggregator listings
```

### HTMX Proxy Flow

```
1. User visits civiqo.com/communities/:federated_id
2. Server detects is_self_hosted = true
3. Server proxies request to hosting_url/api/federation/communities/:id/htmx/detail
4. Response sanitized and cached
5. HTMX fragment returned to browser
6. Internal links rewritten to use proxy
```

---

## 🚨 Important Constraints

1. **Backward Compatibility**: Existing communities unaffected
2. **Graceful Degradation**: Offline instances don't break UI
3. **Data Ownership**: Federated data stays on source server
4. **Privacy**: Only public data shared via federation
5. **Performance**: Caching required for acceptable latency

---

## ⚡ Performance Targets

- Federation info request: < 100ms
- Community list (cached): < 50ms
- HTMX proxy (cached): < 100ms
- HTMX proxy (uncached): < 500ms
- Health check: < 2s timeout

---

## 🔗 Related Files

### To Create
- `src/migrations/009_add_federation_support.sql`
- `src/server/src/handlers/federation.rs`
- `src/server/src/handlers/aggregator.rs`
- `src/server/src/models/federation.rs`
- `src/server/src/services/federation_client.rs`
- `src/server/src/services/proxy.rs`
- `src/server/src/services/content_sanitizer.rs`
- `docs/FEDERATION.md`
- `docs/SELF_HOSTING.md`

### To Modify
- `src/server/src/handlers/api.rs` - Add federation fields to community endpoints
- `src/server/src/main.rs` - Add federation routes
- `src/server/src/models/community.rs` - Add federation fields
- `src/server/templates/communities/*.html` - Federation status UI

---

## 📝 Implementation Phases

### Phase 1: Database & Models (2-3 hours)
- [ ] Create migration 009_add_federation_support.sql
- [ ] Update Community struct with federation fields
- [ ] Create FederationRegistry model
- [ ] Update CreateCommunityRequest/Response

### Phase 2: Federation Protocol (4-6 hours)
- [ ] Implement /api/federation/* endpoints
- [ ] Create federation info endpoint
- [ ] Create community data endpoints
- [ ] Add HTMX fragment endpoint

### Phase 3: Aggregator (4-6 hours)
- [ ] Implement /api/aggregator/* endpoints
- [ ] Registration flow
- [ ] Verification flow
- [ ] Health check background job

### Phase 4: HTMX Proxy (3-4 hours)
- [ ] Create proxy handler
- [ ] Content sanitization
- [ ] URL rewriting
- [ ] Caching layer
- [ ] Fallback handling

### Phase 5: Self-Hosting Package (4-6 hours)
- [ ] Configuration for self-hosted mode
- [ ] Standalone deployment scripts
- [ ] Federation registration CLI
- [ ] Health check endpoints

### Phase 6: UI Updates (2-3 hours)
- [ ] Community creation form updates
- [ ] Federation status indicators
- [ ] Aggregator view updates

### Phase 7: Testing & Documentation (2-3 hours)
- [ ] Integration tests
- [ ] Security tests
- [ ] Federation documentation
- [ ] Self-hosting guide

---

## 📚 Notes

- Consider ActivityPub compatibility for future interop
- May need WebSocket support for real-time federation updates
- Consider CDN for caching federated static assets
- Plan for federation key rotation
