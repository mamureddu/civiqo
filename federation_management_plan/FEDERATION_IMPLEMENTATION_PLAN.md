# Federation Implementation Plan

**Agent 1**: Follow this step-by-step guide  
**Total Estimated Time**: ~10 hours

---

## Phase 1: Database Schema (1 hour)

### Step 1.1: Create Migration
**File**: `src/migrations/009_add_federation_support.sql`

```sql
-- Federation requests (before keys issued)
CREATE TABLE federation_requests (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    hosting_url VARCHAR(500) NOT NULL,
    admin_email VARCHAR(255) NOT NULL,
    organization_name VARCHAR(255),
    description TEXT,
    status VARCHAR(20) DEFAULT 'pending',
    email_token VARCHAR(255),
    email_token_expires TIMESTAMP WITH TIME ZONE,
    email_verified_at TIMESTAMP WITH TIME ZONE,
    domain_token VARCHAR(255),
    domain_verified_at TIMESTAMP WITH TIME ZONE,
    reviewed_by VARCHAR(255),
    reviewed_at TIMESTAMP WITH TIME ZONE,
    review_notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Federation instances (after verification)
CREATE TABLE federation_instances (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    request_id BIGINT REFERENCES federation_requests(id),
    hosting_url VARCHAR(500) NOT NULL UNIQUE,
    admin_email VARCHAR(255) NOT NULL,
    organization_name VARCHAR(255),
    public_key TEXT NOT NULL,
    key_created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    key_rotated_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20) DEFAULT 'pending_activation',
    trust_level VARCHAR(20) DEFAULT 'community',
    last_verified_at TIMESTAMP WITH TIME ZONE,
    verification_failures INT DEFAULT 0,
    reported_version VARCHAR(20),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Add to communities
ALTER TABLE communities ADD COLUMN is_self_hosted BOOLEAN DEFAULT false;
ALTER TABLE communities ADD COLUMN hosting_url VARCHAR(500);
ALTER TABLE communities ADD COLUMN federation_instance_id BIGINT REFERENCES federation_instances(id);
ALTER TABLE communities ADD COLUMN api_version VARCHAR(20);

-- Indexes
CREATE INDEX idx_federation_requests_status ON federation_requests(status);
CREATE INDEX idx_federation_instances_status ON federation_instances(status);
CREATE INDEX idx_communities_federation ON communities(is_self_hosted);
```

- [ ] Create migration file
- [ ] Run: `sqlx migrate run`
- [ ] Regenerate: `cargo sqlx prepare`

---

## Phase 2: Request & Email Verification (2 hours)

### Step 2.1: Create Federation Handler
**File**: `src/server/src/handlers/federation.rs`

Endpoints to implement:
- `POST /api/aggregator/request` - Submit federation request
- `GET /api/aggregator/verify-email` - Verify email token
- `POST /api/aggregator/verify-domain` - Trigger domain check

### Step 2.2: Add Routes
**File**: `src/server/src/main.rs`

```rust
.route("/api/aggregator/request", post(federation::request_federation))
.route("/api/aggregator/verify-email", get(federation::verify_email))
.route("/api/aggregator/verify-domain", post(federation::verify_domain))
```

- [ ] Create federation.rs handler
- [ ] Implement request_federation
- [ ] Implement verify_email
- [ ] Implement verify_domain (DNS TXT + .well-known file)
- [ ] Add routes

---

## Phase 3: Key Issuance & Activation (2 hours)

### Step 3.1: Key Generation
After verification passes, generate Ed25519 keypair:
- Store public key in `federation_instances`
- Send private key to admin via email

### Step 3.2: Activation Endpoint
**Endpoint**: `POST /api/aggregator/activate`

Instance signs challenge with private key to prove possession.

### Step 3.3: Key Rotation
**Endpoint**: `POST /api/aggregator/rotate-key`

Sign new public key with old private key.

- [ ] Implement approve_and_issue_keys
- [ ] Implement activate endpoint
- [ ] Implement rotate_key endpoint
- [ ] Add email sending for keys

---

## Phase 4: Ongoing Verification (1.5 hours)

### Step 4.1: Sign Challenge Endpoint (Self-Hosted)
**File**: `src/server/src/handlers/federation.rs`
**Endpoint**: `POST /api/federation/sign-challenge`

```rust
pub async fn sign_challenge(
    Json(req): Json<ChallengeRequest>,
) -> Json<SignChallengeResponse> {
    let private_key = env::var("FEDERATION_PRIVATE_KEY")?;
    let signature = sign(req.challenge, private_key);
    Json(SignChallengeResponse { signature })
}
```

### Step 4.2: Health Check Job
Background job that periodically verifies instances.

- [ ] Implement sign_challenge endpoint
- [ ] Implement verify_instance_health
- [ ] Add background job (or cron endpoint)

---

## Phase 5: Security Headers (30 min)

### Step 5.1: CSP Header
**File**: `src/server/src/main.rs`

```rust
.layer(SetResponseHeaderLayer::if_not_present(
    header::CONTENT_SECURITY_POLICY,
    HeaderValue::from_static("script-src 'self'")
))
```

### Step 5.2: Cookie Security
Verify cookies have:
- `SameSite=Strict`
- `HttpOnly=true`
- `Secure=true`

- [ ] Add CSP header
- [ ] Verify cookie settings

---

## Phase 6: HTMX Config (30 min)

### Step 6.1: Base Template
**File**: `src/server/templates/base.html`

```html
<head>
    <meta name="htmx-config" content='{"allowScriptTags": false}'>
</head>
```

### Step 6.2: CORS on Self-Hosted
**File**: `src/server/src/main.rs` (self-hosted mode)

```rust
.layer(CorsLayer::new().allow_origin(Any).allow_methods([Method::GET, Method::POST]))
```

- [ ] Add HTMX config
- [ ] Add CORS layer (conditional on federation mode)

---

## Phase 7: UI Updates (2 hours)

### Step 7.1: Community List Template
**File**: `src/server/templates/communities/list.html`

```html
{% for community in communities %}
    {% if community.is_self_hosted %}
        <div hx-get="{{ community.hosting_url }}/api/htmx/community/card"
             hx-trigger="load"
             hx-on:htmx:responseError="this.innerHTML='⚠️ Unavailable'">
        </div>
        {% if community.api_version < current_version %}
            <span class="badge old">v{{ community.api_version }}</span>
        {% endif %}
    {% else %}
        <div hx-get="/api/htmx/community/{{ community.id }}/card" hx-trigger="load"></div>
    {% endif %}
{% endfor %}
```

### Step 7.2: Trust Badges
```html
{% if community.trust_level == "verified" %}
    <span class="badge verified">✓ Verified</span>
{% elif community.trust_level == "community" %}
    <span class="badge community">Community</span>
{% else %}
    <span class="badge unverified">⚠️ Unverified</span>
{% endif %}
```

### Step 7.3: Admin Dashboard
Create admin page to review pending federation requests.

- [ ] Update community list template
- [ ] Add trust badges
- [ ] Create admin review page

---

## Phase 8: Testing (1.5 hours)

### Step 8.1: Integration Tests
**File**: `src/server/tests/federation_test.rs`

- [ ] Test request_federation
- [ ] Test email verification
- [ ] Test domain verification
- [ ] Test key issuance
- [ ] Test activation
- [ ] Test key rotation
- [ ] Test sign_challenge

---

## Dependencies to Add

```toml
# Cargo.toml
ed25519-dalek = "2.0"
base64 = "0.21"
```

---

## Verification Checklist

- [ ] Migration applied
- [ ] Request endpoint works
- [ ] Email verification works
- [ ] Domain verification works
- [ ] Keys issued after verification
- [ ] Activation works
- [ ] Key rotation works
- [ ] Health checks work
- [ ] CSP header present
- [ ] Cookies secure
- [ ] HTMX config set
- [ ] CORS on self-hosted
- [ ] UI shows trust badges
- [ ] Admin can review requests
- [ ] All tests passing
