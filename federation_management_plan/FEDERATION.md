# Federation Architecture

**Status**: Planned  
**Version**: 2.0  
**Last Updated**: November 25, 2025

---

## Overview

Civiqo supports a **federated architecture** to overcome digital sovereignty issues. Communities can:

1. **Self-host** their own instance on their own servers
2. **Federate** with the main Civiqo aggregator at civiqo.com
3. Maintain **data ownership** while being discoverable globally
4. **Cryptographically verify** their identity via public key authentication

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    civiqo.com (Aggregator)                       │
│                                                                  │
│  Security: CSP script-src 'self' | Cookies: SameSite=Strict     │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │ Local       │  │ Direct HTMX │  │ Direct HTMX │              │
│  │ Community   │  │ ✓ Verified  │  │ ⚠ Community │              │
│  └─────────────┘  └──────┬──────┘  └──────┬──────┘              │
│                          │                 │                     │
└──────────────────────────┼─────────────────┼─────────────────────┘
                           │ (no proxy!)     │
                    ┌──────▼──────┐   ┌──────▼──────┐
                    │ remote1.com │   │ remote2.org │
                    │ CORS: ✓     │   │ CORS: ✓     │
                    │ Public Key  │   │ Public Key  │
                    │ HTTPS: ✓    │   │ HTTPS: ✓    │
                    └─────────────┘   └─────────────┘
```

### Key Design Principles

1. **No Proxy**: Direct HTMX requests to federated instances (zero server overhead)
2. **Security via CSP**: `script-src 'self'` blocks malicious scripts from federated content
3. **Cookie Protection**: `SameSite=Strict` prevents cross-origin cookie access
4. **Public Key Verification**: Each instance has unique Ed25519 keypair
5. **Pre-Verification**: Email + Domain verification required before key issuance

---

## Modes of Operation

### Mode 1: Hosted (Default)
- Community data stored on civiqo.com
- No self-hosting required
- Full platform features available

### Mode 2: Self-Hosted + Federated
- Community runs on your own server
- Registered with civiqo.com aggregator
- Appears in global community listings
- Data stays on your server

### Mode 3: Self-Hosted (Standalone)
- Community runs on your own server
- NOT registered with aggregator
- Completely independent operation
- Can federate later if desired

---

## Registration Flow

### Step 1: Request Federation
```
POST /api/aggregator/request
Content-Type: application/json

{
  "hosting_url": "https://community.example.com",
  "admin_email": "admin@example.com",
  "organization_name": "Example Org",
  "description": "Community for example purposes"
}
```

Response:
```json
{
  "request_id": 123,
  "status": "pending_verification",
  "domain_token": "abc123...",
  "next_steps": [
    "Check email for verification link",
    "Add DNS TXT: civiqo-verify=abc123...",
    "Or place file at /.well-known/civiqo-verify.txt"
  ]
}
```

### Step 2: Email Verification
```
GET /api/aggregator/verify-email?token=xxx&request_id=123
```

### Step 3: Domain Verification (Optional but speeds approval)
Either:
- Add DNS TXT record: `civiqo-verify=<domain_token>`
- Place file at: `https://your-domain.com/.well-known/civiqo-verify.txt`

```
POST /api/aggregator/verify-domain
{ "request_id": 123 }
```

### Step 4: Approval & Key Issuance
- **Auto-approve**: Email ✓ + Domain ✓ → Keys issued (trust: community)
- **Manual review**: Email ✓ only → Admin reviews → Keys issued

Private key sent via email. Store in `.env`:
```
FEDERATION_PRIVATE_KEY=base64_encoded_ed25519_private_key
```

### Step 5: Activation
```
POST /api/aggregator/activate
{
  "federation_id": 456,
  "signed_challenge": "base64_signature_of_challenge"
}
```

---

## Ongoing Verification

### Challenge-Response
Aggregator periodically sends challenges:
```
POST https://your-instance.com/api/federation/sign-challenge
{ "challenge": "random_nonce" }
```

Instance responds with signature:
```json
{ "signature": "base64_ed25519_signature" }
```

Aggregator verifies with stored public key.

### Key Rotation
Sign new public key with old private key:
```
POST /api/aggregator/rotate-key
{
  "federation_id": 456,
  "new_public_key": "base64_new_public_key",
  "signature": "base64_signature_of_new_key_with_old_key"
}
```

---

## Direct HTMX (No Proxy)

Federated content loaded directly from remote instances:

```html
<!-- On civiqo.com -->
<div hx-get="https://remote-community.com/api/htmx/community/card"
     hx-trigger="load"
     hx-on:htmx:responseError="this.innerHTML='⚠️ Unavailable'">
</div>
```

### Why No Proxy?
- **Zero server overhead** on civiqo.com
- **True decentralization** - data never touches aggregator
- **Faster** - direct connection to source
- **Scalable** - no bottleneck

### Security (Without Proxy)
- **CSP**: `script-src 'self'` blocks external scripts
- **Cookies**: `SameSite=Strict` prevents cross-origin access
- **HTMX Config**: `allowScriptTags: false`
- **CORS**: Remote must allow civiqo.com origin

---

## Security

### Key Exchange
- Ed25519 public/private key pairs
- Keys generated during instance setup
- Public key shared during registration

### Request Signing
- All federation requests signed with private key
- Aggregator verifies with public key
- Prevents request tampering

### Content Sanitization
- All proxied HTML sanitized
- Script tags removed
- Dangerous attributes stripped
- URLs validated

### Rate Limiting
- Per-instance rate limits
- Automatic suspension on abuse
- Configurable thresholds

---

## Database Schema

### communities table additions
```sql
is_self_hosted BOOLEAN DEFAULT false
hosting_url VARCHAR(500)
api_endpoint VARCHAR(500)
federation_key VARCHAR(255)
federation_status VARCHAR(50) -- 'local', 'pending', 'active', 'suspended'
last_sync_at TIMESTAMP WITH TIME ZONE
```

### federation_registry table
```sql
id BIGINT PRIMARY KEY
community_id BIGINT REFERENCES communities(id)
remote_url VARCHAR(500)
api_version VARCHAR(20)
public_key TEXT
verified_at TIMESTAMP WITH TIME ZONE
health_status VARCHAR(50)
last_health_check TIMESTAMP WITH TIME ZONE
```

---

## Health Checks

The aggregator performs periodic health checks:

1. **Frequency**: Every 5 minutes
2. **Endpoint**: `GET /api/federation/info`
3. **Timeout**: 2 seconds
4. **Failure threshold**: 3 consecutive failures

### Health Statuses
- `healthy` - Instance responding normally
- `degraded` - Slow responses or partial failures
- `unhealthy` - Not responding
- `suspended` - Manually suspended

---

## Configuration

### Environment Variables

```bash
# Federation mode: aggregator, self_hosted, standalone
FEDERATION_MODE=self_hosted

# Aggregator URL (for self-hosted instances)
AGGREGATOR_URL=https://civiqo.com

# Instance URL (public URL of this instance)
INSTANCE_URL=https://community.example.com

# Key paths
FEDERATION_PRIVATE_KEY=/path/to/private.key
FEDERATION_PUBLIC_KEY=/path/to/public.key
```

---

## See Also

- [Self-Hosting Guide](SELF_HOSTING.md)
- [API Documentation](API_GUIDE.md)
- [Security Guide](SECURITY.md)
