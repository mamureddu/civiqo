# 🗺️ **Project Roadmap - Community Manager (Federated Architecture)**

**Version**: 2.0 - Federated Edition  
**Last Updated**: November 25, 2025  
**Architecture**: Distributed Federation with Central Aggregator  
**Total Estimated Time**: 12-16 weeks

---

## 📊 **Current Status**

### ✅ Completed
- [x] MVC Project Structure
- [x] CockroachDB Cloud Integration
- [x] Migrations Applied (7+)
- [x] Axum Server with HTMX
- [x] Auth0 Handlers (login, callback, logout)
- [x] Lambda Authorizer with Context Injection
- [x] REST API Endpoints (users, communities, posts)
- [x] Tera Templates for Main Pages
- [x] 205+ Tests Passing
- [x] Complete Documentation
- [x] UUIDv7 for Community IDs (Federation-Ready)
- [x] Community Membership System (5 endpoints)
- [x] Public/Private Communities with Join Requests
- [x] Community Discovery (my, trending)
- [x] Owner/Admin Management (transfer, promote, demote)
- [x] HTMX Fragments (card, join-button, members)
- [x] Brand Compliance (Civiqo Colors)

### 🚧 In Progress
- [ ] Federation Architecture Implementation
- [ ] Multi-instance Communication Protocol
- [ ] Cross-instance Data Synchronization

### ⏳ To Do
- [ ] Business Features
- [ ] Governance Features
- [ ] Chat Real-time WebSocket
- [ ] Complete Frontend
- [ ] End-to-end Testing
- [ ] Production Deployment

---

## 🎯 **PHASE 1: Core Community Features (COMPLETED ✅)**

### Status: ✅ DONE
All core community management features are implemented and tested:
- ✅ Community CRUD (create, read, update, delete)
- ✅ Membership Management (join, leave, list, roles)
- ✅ Public/Private Communities
- ✅ Join Requests & Approval Workflow
- ✅ Discovery Endpoints
- ✅ Admin Management (transfer, promote, demote)
- ✅ HTMX Fragments
- ✅ Brand Compliance

**Metrics**:
- 18 API Endpoints Implemented
- 41 Integration Tests
- 100% Brand Compliance
- Zero Security Vulnerabilities

---

## 🎯 **PHASE 2: Federation Architecture (2-3 weeks)**

### 2.1 Database Schema for Federation ⚡ PRIORITY HIGH
**Objective**: Add federation tables and fields

**Tasks**:
- [ ] Create `federation_requests` table
  - Tracks federation requests before verification
  - Stores email verification tokens
  - Stores domain verification status
- [ ] Create `federation_instances` table
  - Stores verified federation instances
  - Manages public keys for signature verification
  - Tracks trust levels and verification status
- [ ] Add federation fields to `communities`
  - `is_self_hosted` (boolean)
  - `hosting_url` (VARCHAR)
  - `federation_instance_id` (FK)
  - `api_version` (VARCHAR)
- [ ] Create indexes for performance

**Time Estimate**: 1 day  
**Dependencies**: None  
**Output**: Federation-ready database schema

### 2.2 Federation Request & Verification ⚡ PRIORITY HIGH
**Objective**: Implement federation request workflow

**Endpoints**:
- `POST /api/aggregator/request` - Submit federation request
- `GET /api/aggregator/verify-email` - Verify email token
- `POST /api/aggregator/verify-domain` - Verify domain ownership

**Tasks**:
- [ ] Create `federation.rs` handler
- [ ] Implement email verification flow
  - Generate verification token
  - Send email with verification link
  - Validate token on callback
- [ ] Implement domain verification
  - DNS TXT record check
  - `.well-known/civiqo.json` file check
  - Async verification job
- [ ] Add error handling and logging

**Time Estimate**: 2 days  
**Dependencies**: 2.1  
**Output**: Federation request workflow

### 2.3 Key Issuance & Management ⚡ PRIORITY HIGH
**Objective**: Generate and manage Ed25519 keypairs

**Tasks**:
- [ ] Generate Ed25519 keypair after verification
  - Store public key in `federation_instances`
  - Send private key to admin via secure email
- [ ] Implement activation endpoint
  - Instance signs challenge with private key
  - Proves possession of private key
- [ ] Implement key rotation
  - Sign new public key with old private key
  - Seamless rotation without downtime
- [ ] Add key expiration and renewal

**Time Estimate**: 1.5 days  
**Dependencies**: 2.2  
**Output**: Secure key management system

### 2.4 Instance Health & Verification ⚡ PRIORITY HIGH
**Objective**: Ongoing verification of federation instances

**Tasks**:
- [ ] Implement `sign_challenge` endpoint (self-hosted)
  - Instances sign challenges to prove they're alive
  - Verify signatures with stored public keys
- [ ] Create background verification job
  - Periodic health checks (every 24 hours)
  - Update `last_verified_at` timestamp
  - Track verification failures
- [ ] Implement trust scoring
  - Increase trust after successful verifications
  - Decrease trust after failures
  - Auto-suspend after threshold

**Time Estimate**: 1.5 days  
**Dependencies**: 2.3  
**Output**: Continuous instance verification

**MILESTONE 2**: ✅ Federation Infrastructure Complete

---

## 🎯 **PHASE 3: Multi-Instance Communication (2-3 weeks)**

### 3.1 Federation Protocol Implementation ⚡ PRIORITY HIGH
**Objective**: Implement secure inter-instance communication

**Tasks**:
- [ ] Define federation protocol
  - Request/response format (JSON)
  - Signature scheme (Ed25519)
  - Timestamp validation
  - Nonce to prevent replay attacks
- [ ] Implement request signing
  - Sign all outgoing federation requests
  - Include timestamp and nonce
  - Verify signatures on incoming requests
- [ ] Add request verification middleware
  - Validate signatures
  - Check timestamps (within 5 minutes)
  - Verify nonce uniqueness
  - Rate limiting per instance

**Time Estimate**: 2 days  
**Dependencies**: 2.4  
**Output**: Secure federation protocol

### 3.2 Cross-Instance Community Sync ⚡ PRIORITY HIGH
**Objective**: Synchronize communities across instances

**Endpoints** (Aggregator):
- `POST /api/federation/sync-community` - Receive community update
- `GET /api/federation/community/:id` - Fetch community from instance
- `POST /api/federation/community/:id/members` - Sync members

**Endpoints** (Self-Hosted):
- `GET /api/federation/community/:id` - Expose community
- `GET /api/federation/community/:id/members` - Expose members

**Tasks**:
- [ ] Implement community sync from self-hosted to aggregator
  - Fetch community data from self-hosted instance
  - Verify signature and timestamp
  - Store in aggregator database
  - Update `federation_instance_id`
- [ ] Implement member sync
  - Fetch member list from self-hosted
  - Sync member roles and status
  - Handle member additions/removals
- [ ] Handle conflicts
  - Last-write-wins strategy
  - Timestamp-based resolution
  - Log conflicts for review

**Time Estimate**: 2 days  
**Dependencies**: 3.1  
**Output**: Cross-instance community synchronization

### 3.3 Federated Search & Discovery ⚡ PRIORITY MEDIUM
**Objective**: Search across all federated instances

**Endpoints**:
- `GET /api/communities/federated-search?q=query` - Search all instances
- `GET /api/communities/federated-trending` - Trending across federation

**Tasks**:
- [ ] Implement federated search
  - Query local communities
  - Query each federated instance in parallel
  - Aggregate and rank results
  - Cache results (5 minute TTL)
- [ ] Implement federated trending
  - Aggregate member counts from all instances
  - Sort by popularity
  - Cache results (1 hour TTL)
- [ ] Add pagination and filtering
  - Limit results per instance
  - Filter by trust level
  - Filter by instance region

**Time Estimate**: 1.5 days  
**Dependencies**: 3.2  
**Output**: Federated discovery system

### 3.4 Data Consistency & Conflict Resolution ⚡ PRIORITY MEDIUM
**Objective**: Maintain consistency across federation

**Tasks**:
- [ ] Implement conflict detection
  - Detect concurrent updates
  - Log conflicts with full context
  - Alert admins of conflicts
- [ ] Implement conflict resolution
  - Last-write-wins (default)
  - Manual resolution UI for admins
  - Audit trail of all resolutions
- [ ] Implement data reconciliation
  - Periodic full sync (weekly)
  - Detect missing data
  - Repair inconsistencies

**Time Estimate**: 1.5 days  
**Dependencies**: 3.3  
**Output**: Conflict resolution system

**MILESTONE 3**: ✅ Multi-Instance Communication Complete

---

## 🎯 **PHASE 4: Advanced Federation Features (2-3 weeks)**

### 4.1 Federated User Profiles ⚡ PRIORITY MEDIUM
**Objective**: User profiles work across federation

**Tasks**:
- [ ] Extend user profile schema
  - Add `federation_instance_id`
  - Add `remote_user_id`
  - Add `profile_sync_status`
- [ ] Implement profile sync
  - Sync profile from self-hosted to aggregator
  - Handle profile updates
  - Maintain profile consistency
- [ ] Implement federated user search
  - Search users across instances
  - Display instance badge
  - Show trust level

**Time Estimate**: 1.5 days  
**Dependencies**: 3.2  
**Output**: Federated user profiles

### 4.2 Federated Posts & Comments ⚡ PRIORITY MEDIUM
**Objective**: Posts and comments work across federation

**Tasks**:
- [ ] Extend posts schema
  - Add `federation_instance_id`
  - Add `remote_post_id`
  - Add `sync_status`
- [ ] Implement post sync
  - Sync posts from self-hosted to aggregator
  - Handle post updates and deletions
  - Maintain referential integrity
- [ ] Implement comment federation
  - Comments on federated posts
  - Cross-instance comment threads
  - Notification system

**Time Estimate**: 2 days  
**Dependencies**: 4.1  
**Output**: Federated posts and comments

### 4.3 Federated Governance ⚡ PRIORITY LOW
**Objective**: Governance features work across federation

**Tasks**:
- [ ] Extend proposals schema
  - Add `federation_instance_id`
  - Add `remote_proposal_id`
- [ ] Implement proposal federation
  - Sync proposals across instances
  - Cross-instance voting
  - Aggregate results
- [ ] Implement decision federation
  - Sync decisions across instances
  - Maintain decision consistency

**Time Estimate**: 1.5 days  
**Dependencies**: 4.2  
**Output**: Federated governance

### 4.4 Federated Notifications ⚡ PRIORITY MEDIUM
**Objective**: Notifications work across federation

**Tasks**:
- [ ] Implement cross-instance notifications
  - Notify users on aggregator when activity on self-hosted
  - Notify users on self-hosted when activity on aggregator
  - Real-time WebSocket support
- [ ] Implement notification preferences
  - Per-instance notification settings
  - Notification filtering
  - Notification aggregation

**Time Estimate**: 1.5 days  
**Dependencies**: 4.3  
**Output**: Federated notifications

**MILESTONE 4**: ✅ Advanced Federation Features Complete

---

## 🎯 **PHASE 5: Business Features (2-3 weeks)**

### 5.1 Business Entities 💼
**Objective**: Complete business system

**Tasks**:
- [ ] CRUD business entities
- [ ] Business profiles
- [ ] Products/Services catalog
- [ ] Business verification
- [ ] Business analytics
- [ ] UI business dashboard

**Time Estimate**: 2 days  
**Dependencies**: Phase 4  
**Output**: Business features complete

### 5.2 Transactions & Orders 💰
**Objective**: Transaction system

**Tasks**:
- [ ] Order management
- [ ] Transaction history
- [ ] Payment integration (Stripe/PayPal)
- [ ] Invoicing
- [ ] Refunds
- [ ] UI transactions

**Time Estimate**: 2 days  
**Dependencies**: 5.1  
**Output**: Transaction system

### 5.3 Reviews & Ratings ⭐
**Objective**: Review system

**Tasks**:
- [ ] Review system
- [ ] Rating aggregation
- [ ] Review moderation
- [ ] Verified purchase badges
- [ ] UI reviews

**Time Estimate**: 1 day  
**Dependencies**: 5.2  
**Output**: Review system complete

**MILESTONE 5**: ✅ Business Features Complete

---

## 🎯 **PHASE 6: Governance Features (2-3 weeks)**

### 6.1 Proposals System 🗳️
**Objective**: Governance proposals

**Tasks**:
- [ ] Create proposals
- [ ] Proposal types (text, poll, budget)
- [ ] Proposal lifecycle
- [ ] Discussion threads
- [ ] UI proposals

**Time Estimate**: 1.5 days  
**Dependencies**: Phase 5  
**Output**: Proposals complete

### 6.2 Voting System 🗳️
**Objective**: Voting mechanism

**Tasks**:
- [ ] Voting types (simple, weighted, quadratic)
- [ ] Vote counting
- [ ] Vote verification
- [ ] UI voting

**Time Estimate**: 1.5 days  
**Dependencies**: 6.1  
**Output**: Voting system

### 6.3 Decision Making 📋
**Objective**: Decision implementation

**Tasks**:
- [ ] Decision tracking
- [ ] Implementation status
- [ ] Outcome reporting
- [ ] UI decisions

**Time Estimate**: 1 day  
**Dependencies**: 6.2  
**Output**: Decision system

**MILESTONE 6**: ✅ Governance Features Complete

---

## 🎯 **PHASE 7: Chat & Real-time (1-2 weeks)**

### 7.1 WebSocket Chat 💬
**Objective**: Real-time chat

**Tasks**:
- [ ] WebSocket server setup
- [ ] Chat room management
- [ ] Message persistence
- [ ] UI chat

**Time Estimate**: 1.5 days  
**Dependencies**: Phase 6  
**Output**: Chat system

### 7.2 Real-time Notifications 🔔
**Objective**: Real-time notifications

**Tasks**:
- [ ] Notification delivery
- [ ] Notification preferences
- [ ] UI notifications

**Time Estimate**: 1 day  
**Dependencies**: 7.1  
**Output**: Notifications

### 7.3 Presence System 👥
**Objective**: User presence tracking

**Tasks**:
- [ ] Online/offline status
- [ ] Activity tracking
- [ ] UI presence

**Time Estimate**: 0.5 days  
**Dependencies**: 7.2  
**Output**: Presence system

**MILESTONE 7**: ✅ Chat & Real-time Complete

---

## 🎯 **PHASE 8: Advanced Features (1-2 weeks)**

### 8.1 Analytics & Reporting 📊
- Community analytics
- User engagement metrics
- Business analytics
- Admin dashboard

**Time Estimate**: 1.5 days

### 8.2 Admin Dashboard 🎛️
- Community management
- User management
- Federation management
- System health monitoring

**Time Estimate**: 1.5 days

### 8.3 Moderation Tools 🛡️
- Content moderation
- User moderation
- Appeal system
- Audit logs

**Time Estimate**: 1 day

**MILESTONE 8**: ✅ Advanced Features Complete

---

## 🎯 **PHASE 9: Deployment & Polish (1-2 weeks)**

### 9.1 End-to-end Testing
- Integration tests
- Load testing
- Security testing
- Federated testing

**Time Estimate**: 1.5 days

### 9.2 Performance Optimization
- Database optimization
- Caching strategy
- CDN integration
- Load balancing

**Time Estimate**: 1 day

### 9.3 Production Deployment
- Infrastructure setup
- CI/CD pipeline
- Monitoring & alerting
- Backup & recovery

**Time Estimate**: 1 day

**MILESTONE 9**: ✅ Production Ready

---

## 📈 **Timeline Summary**

| Phase | Duration | Status |
|-------|----------|--------|
| **Phase 1: Core Communities** | 2-3 weeks | ✅ COMPLETE |
| **Phase 2: Federation Architecture** | 2-3 weeks | 🚧 IN PROGRESS |
| **Phase 3: Multi-Instance Communication** | 2-3 weeks | ⏳ PENDING |
| **Phase 4: Advanced Federation** | 2-3 weeks | ⏳ PENDING |
| **Phase 5: Business Features** | 2-3 weeks | ⏳ PENDING |
| **Phase 6: Governance** | 2-3 weeks | ⏳ PENDING |
| **Phase 7: Chat & Real-time** | 1-2 weeks | ⏳ PENDING |
| **Phase 8: Advanced Features** | 1-2 weeks | ⏳ PENDING |
| **Phase 9: Deployment & Polish** | 1-2 weeks | ⏳ PENDING |

**Total Estimated Time**: 12-16 weeks

---

## 🏗️ **Architecture Overview**

### Central Aggregator
- Hosts main instance
- Manages federation requests
- Verifies instances
- Issues keypairs
- Aggregates data from federation

### Self-Hosted Instances
- Independent communities
- Sign requests with private key
- Sync data to aggregator
- Maintain local autonomy
- Optional federation

### Communication
- Signed requests (Ed25519)
- Timestamp validation
- Nonce for replay prevention
- Rate limiting
- Async verification

---

## 🔐 **Security Considerations**

- ✅ Ed25519 signatures for all federation requests
- ✅ Email verification for federation requests
- ✅ Domain verification (DNS + .well-known)
- ✅ Key rotation support
- ✅ Trust scoring system
- ✅ Rate limiting per instance
- ✅ Audit logging for all federation activities
- ✅ Conflict detection and resolution

---

## 📚 **Documentation**

- `federation_management_plan/FEDERATION_IMPLEMENTATION_PLAN.md` - Detailed implementation steps
- `federation_management_plan/FEDERATION_TASK_CONTEXT.md` - Task context and requirements
- `federation_management_plan/FEDERATION.md` - Architecture overview
- `docs/ARCHITECTURE.md` - System architecture
- `docs/API_GUIDE.md` - API documentation

---

## 🎯 **Next Steps**

1. **Immediate** (This Week):
   - [ ] Review federation architecture
   - [ ] Plan Phase 2 implementation
   - [ ] Set up federation development environment

2. **Short Term** (Next 2 Weeks):
   - [ ] Implement Phase 2 (Federation Architecture)
   - [ ] Test federation request workflow
   - [ ] Verify key management system

3. **Medium Term** (Next 4-6 Weeks):
   - [ ] Implement Phase 3-4 (Multi-Instance Communication)
   - [ ] Implement federated search and discovery
   - [ ] Test cross-instance synchronization

4. **Long Term** (Weeks 7-16):
   - [ ] Implement remaining phases
   - [ ] Complete feature set
   - [ ] Production deployment

---

**Status**: Ready for Phase 2 Implementation 🚀

