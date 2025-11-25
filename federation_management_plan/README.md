# Federation Management Plan

**Status**: Future Implementation  
**Priority**: After core community features are stable

This directory contains detailed planning for the federation feature, which will allow:
- Self-hosted Civiqo instances
- Multiple communities per host
- Federation between instances
- Shared authentication (optional)

## Contents

- `FEDERATION_TASK_CONTEXT.md` - Full requirements and acceptance criteria
- `FEDERATION_IMPLEMENTATION_PLAN.md` - Step-by-step implementation guide
- `FEDERATION.md` - Architecture documentation

## Key Concepts

1. **Multi-tenant**: Each host can run multiple communities
2. **Federation**: Communities can federate with civiqo.com aggregator
3. **Auth Flexibility**: Local auth OR federated civiqo auth
4. **Direct HTMX**: No proxy layer, direct requests to federated instances
5. **Public Key Verification**: Ed25519 for instance authenticity

## When to Implement

Implement federation AFTER:
- [ ] Core community CRUD is stable
- [ ] Community UI is complete
- [ ] User management is working
- [ ] Basic moderation tools exist

## Current Focus

Building communities with **federation-ready architecture**:
- Clean separation of concerns
- Configurable auth layer
- HTMX endpoints that can be called cross-origin
- No hardcoded assumptions about single-host deployment
