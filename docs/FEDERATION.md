# Federation Architecture

**Status**: Planned  
**Version**: 1.0  
**Last Updated**: November 25, 2025

---

## Overview

Civiqo supports a **federated architecture** to overcome digital sovereignty issues. Communities can:

1. **Self-host** their own instance on their own servers
2. **Federate** with the main Civiqo aggregator at civiqo.com
3. Maintain **data ownership** while being discoverable globally

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                     civiqo.com (Aggregator)                      │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │ Local       │  │ Federated   │  │ Federated   │              │
│  │ Community A │  │ Community B │  │ Community C │              │
│  │ (hosted)    │  │ (external)  │  │ (external)  │              │
│  └─────────────┘  └──────┬──────┘  └──────┬──────┘              │
│                          │                 │                     │
│                    HTMX Proxy         HTMX Proxy                │
│                          │                 │                     │
└──────────────────────────┼─────────────────┼─────────────────────┘
                           │                 │
                    ┌──────▼──────┐   ┌──────▼──────┐
                    │ community-b │   │ community-c │
                    │ .example.com│   │ .org        │
                    │             │   │             │
                    │ Self-Hosted │   │ Self-Hosted │
                    │ Instance    │   │ Instance    │
                    └─────────────┘   └─────────────┘
```

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

## Federation Protocol

### Discovery Endpoint
```
GET /api/federation/info
```

Returns instance metadata:
```json
{
  "instance_url": "https://community.example.com",
  "instance_name": "Example Community",
  "api_version": "v1",
  "public_key": "...",
  "supported_features": ["htmx", "api", "webhooks"]
}
```

### Community List
```
GET /api/federation/communities
```

Returns public communities:
```json
{
  "communities": [
    {
      "id": 123,
      "name": "Example Community",
      "slug": "example",
      "description": "...",
      "member_count": 1500,
      "is_public": true
    }
  ]
}
```

### HTMX Fragments
```
GET /api/federation/communities/:id/htmx/:fragment
```

Returns embeddable HTMX content for:
- `detail` - Community detail view
- `members` - Member list
- `posts` - Recent posts
- `sidebar` - Sidebar widget

---

## Aggregator API

### Register Federation
```
POST /api/aggregator/register
Content-Type: application/json

{
  "instance_url": "https://community.example.com",
  "public_key": "...",
  "community_id": 123,
  "community_name": "Example Community",
  "community_slug": "example"
}
```

### Verify Federation
```
POST /api/aggregator/verify
Content-Type: application/json

{
  "community_id": 123,
  "verification_token": "..."
}
```

### Proxy Requests
```
GET /api/aggregator/proxy/:community_id/*path
```

Proxies requests to federated instance with:
- Content sanitization
- URL rewriting
- Caching

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
