# 🗺️ **Project Roadmap - Community Manager (MVC Complete)**

**Version**: 4.1 - MVC Architecture  
**Last Updated**: November 26, 2025  
**Architecture**: Single Instance (Federation-Ready for Phase 9)  
**Total Estimated Time**: 10-14 weeks (+ 2-3 weeks for Phase 9 Federation)

---

## 📐 **MVC Architecture Overview**

Ogni fase include implementazione completa di:
- **Model (M)**: Database schema, migrations, SQLx queries
- **View (V)**: HTMX templates, UI fragments, pages
- **Controller (C)**: API handlers, business logic, validation

```
src/
├── migrations/           # Model - Database schema
├── server/
│   ├── src/handlers/    # Controller - API handlers
│   └── templates/       # View - HTMX templates
│       ├── pages/       # Full pages
│       └── fragments/   # Reusable UI components
└── shared/              # Shared models and utilities
```

---

## 📊 **Current Status**

### ✅ Completed
- [x] **Phase 1**: Core Communities (M+V+C Complete)
- [x] **Phase 2**: Posts & Comments (M+V+C Complete) ✅

### ⏳ To Do
- [ ] **Phase 3**: User Profiles & Search
- [ ] **Phase 4-8**: Full MVC Implementation
- [ ] **Phase 9**: Federation (BONUS)

---

## 🎯 **PHASE 1: Core Community Features ✅ COMPLETED**

### Model (M) ✅
- ✅ `communities` table (UUIDv7 PK)
- ✅ `community_members` table
- ✅ `roles` table
- ✅ Foreign keys and indexes
- ✅ Migration: `001_initial_schema_with_bigint.sql`

### View (V) ✅
- ✅ `templates/fragments/community-card.html`
- ✅ `templates/fragments/join-button.html`
- ✅ `templates/fragments/members-list.html`
- ✅ Brand compliance (Civiqo colors)

### Controller (C) ✅
- ✅ `handlers/api.rs` - 18 endpoints
- ✅ Community CRUD
- ✅ Membership management
- ✅ Join requests & approval
- ✅ Admin management

### Tests ✅
- ✅ 21 integration tests
- ✅ 100% passing

---

## 🎯 **PHASE 2: Posts & Comments System (2-3 weeks)**

### 2.1 Model (M) ✅ DONE

#### Database Schema
```sql
-- posts table
CREATE TABLE posts (
    id UUID PRIMARY KEY,
    community_id UUID REFERENCES communities(id),
    author_id UUID REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    content_type VARCHAR(20) DEFAULT 'markdown',
    media_url VARCHAR(500),
    is_pinned BOOLEAN DEFAULT false,
    is_locked BOOLEAN DEFAULT false,
    view_count BIGINT DEFAULT 0,
    created_at, updated_at
);

-- comments table
CREATE TABLE comments (
    id UUID PRIMARY KEY,
    post_id UUID REFERENCES posts(id),
    author_id UUID REFERENCES users(id),
    parent_id UUID REFERENCES comments(id),  -- threading
    content TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT false,
    created_at, updated_at
);

-- reactions table
CREATE TABLE reactions (
    id BIGINT PRIMARY KEY,
    post_id UUID REFERENCES posts(id),
    user_id UUID REFERENCES users(id),
    reaction_type VARCHAR(20) NOT NULL,
    UNIQUE(post_id, user_id)
);
```

#### Tasks ✅
- [x] Migration: `004_posts_comments_reactions.sql`
- [x] Indexes for performance
- [x] CASCADE deletes

### 2.2 View (V) ✅ DONE

#### Pages ✅
- [x] `templates/community_posts.html` - Lista posts di una community
- [x] `templates/post_detail.html` - Dettaglio post con commenti
- [x] `templates/create_post.html` - Form creazione post
- [x] `templates/community_detail.html` - Integrato con posts (tabs, new post button)

#### Fragments ✅
- [x] `templates/fragments/post-card.html` - Card singolo post
- [x] `templates/fragments/post-form.html` - Form creazione/modifica post
- [x] `templates/fragments/comment-item.html` - Singolo commento (con nesting)
- [x] `templates/fragments/comment-form.html` - Form commento
- [x] `templates/fragments/reaction-buttons.html` - Pulsanti reazioni

#### HTMX Endpoints ✅
- [x] `/htmx/comments/:id/reply-form` - Form per rispondere
- [x] `/htmx/comments/:id/edit-form` - Form per modificare
- [x] `/htmx/communities/:id/feed` - Feed posts community

#### Brand Compliance ✅
- [x] Primary color: `#57C98A` (buttons, links)
- [x] Secondary color: `#3B7FBA` (headers)
- [x] Accent color: `#EF6F5E` (alerts, reactions)
- [x] Gradient headers: `from-[#57C98A] to-[#3B7FBA]`

### 2.3 Controller (C) ✅ DONE

#### Handlers
- [x] `handlers/posts.rs`
  - `create_post` - POST /api/communities/:id/posts
  - `list_posts` - GET /api/communities/:id/posts
  - `get_post` - GET /api/posts/:id
  - `update_post` - PUT /api/posts/:id
  - `delete_post` - DELETE /api/posts/:id

- [x] `handlers/comments.rs`
  - `create_comment` - POST /api/posts/:id/comments
  - `list_comments` - GET /api/posts/:id/comments
  - `update_comment` - PUT /api/comments/:id
  - `delete_comment` - DELETE /api/comments/:id

- [x] `handlers/reactions.rs`
  - `add_reaction` - POST /api/posts/:id/reactions
  - `remove_reaction` - DELETE /api/posts/:id/reactions
  - `list_reactions` - GET /api/posts/:id/reactions

#### Page Handlers ✅
- [x] `handlers/pages.rs`:
  - `community_detail` - GET /communities/:id (with posts integration)
  - `post_detail` - GET /posts/:id
  - `create_post_page` - GET /communities/:id/posts/new

### 2.4 Tests ✅ DONE
- [x] 12 integration tests (posts CRUD)
- [x] 37 view interaction tests
- [x] Comments threading tests
- [x] Reactions tests
- [x] Cascade delete tests
- [x] 189 total tests passing

### Phase 2 Completion Checklist ✅
- [x] **Model**: Database schema complete
- [x] **View**: Templates and fragments complete
- [x] **Controller**: API handlers complete
- [x] **Tests**: Integration tests passing
- [x] **Integration**: Posts integrated in community_detail page

---

## ✅ **PHASE 3: User Profiles & Search - COMPLETED**

**Completion Date**: November 27, 2025
**Status**: FULLY IMPLEMENTED

### 3.1 Model (M) ✅ DONE

#### Database Schema (Migration 010)
- [x] `user_profiles` extended (cover_image, is_public, avatar_url, follower_count, following_count)
- [x] `user_follows` table with no_self_follow constraint
- [x] `notifications` table with actor, target, type
- [x] Full-text search indexes on users, communities, posts

### 3.2 View (V) ✅ DONE

#### Pages
- [x] `templates/profile.html` - Profilo utente con tabs
- [x] `templates/profile_edit.html` - Modifica profilo
- [x] `templates/search.html` - Risultati ricerca con filtri
- [x] `templates/notifications.html` - Notifiche con filtri
- [x] `templates/404.html` - Pagina errore 404
- [x] `templates/500.html` - Pagina errore 500

#### Fragments
- [x] `templates/fragments/user-card.html` - Card utente
- [x] `templates/fragments/follow-button.html` - Pulsante follow/unfollow
- [x] `templates/fragments/notifications-list.html` - Lista notifiche
- [x] `templates/fragments/empty-state.html` - Empty state riutilizzabile
- [x] `templates/fragments/toast.html` - Sistema toast globale
- [x] `templates/fragments/welcome-modal.html` - Onboarding modal
- [x] `templates/fragments/profile-completion-banner.html` - Banner profilo

### 3.3 Controller (C) ✅ DONE

#### Page Handlers
- [x] `user_profile` - GET /users/:id
- [x] `edit_profile_page` - GET /users/:id/edit
- [x] `search_page` - GET /search
- [x] `notifications` - GET /notifications
- [x] `not_found` - 404 fallback
- [x] `internal_error` - 500 handler

#### API Handlers
- [x] `update_profile` - PUT /api/users/:id
- [x] `follow_user` - POST /api/users/:id/follow
- [x] `unfollow_user` - POST /api/users/:id/unfollow
- [x] `dismiss_welcome` - POST /api/users/dismiss-welcome
- [x] `dismiss_profile_banner` - POST /api/users/dismiss-profile-banner

#### HTMX Handlers
- [x] `user_posts` - GET /htmx/users/:id/posts
- [x] `user_profile_communities` - GET /htmx/users/:id/communities
- [x] `user_followers` - GET /htmx/users/:id/followers
- [x] `user_following` - GET /htmx/users/:id/following
- [x] `follow_button` - GET /htmx/users/:id/follow-button
- [x] `search_results` - GET /htmx/search
- [x] `notifications_list` - GET /htmx/notifications/list
- [x] `notifications_dropdown` - GET /htmx/notifications
- [x] `mark_notification_read` - POST /htmx/notifications/:id/read
- [x] `mark_all_notifications_read` - POST /htmx/notifications/mark-all-read

### 3.4 Tests ✅ DONE
- [x] 25 test cases in `phase3_user_profiles_test.rs`
- [x] User profile tests
- [x] Follow/unfollow tests
- [x] Search tests
- [x] Notification tests
- [x] HTMX fragment tests

### Phase 3 Completion Checklist ✅
- [x] **Model**: Database schema complete (Migration 010)
- [x] **View**: Templates and fragments complete
- [x] **Controller**: API and page handlers complete
- [x] **Tests**: Test suite created
- [x] **UX**: Agent UX approved (9.2/10)

---

## 🎯 **PHASE 4: Business Features (2-3 weeks)**

### 4.1 Model (M)

#### Database Schema
```sql
-- businesses table
CREATE TABLE businesses (
    id UUID PRIMARY KEY,
    owner_id UUID REFERENCES users(id),
    community_id UUID REFERENCES communities(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    address TEXT,
    phone VARCHAR(50),
    email VARCHAR(255),
    website VARCHAR(255),
    logo_url VARCHAR(500),
    cover_url VARCHAR(500),
    is_verified BOOLEAN DEFAULT false,
    rating_avg DECIMAL(3,2) DEFAULT 0,
    review_count INT DEFAULT 0,
    created_at, updated_at
);

-- business_hours table
CREATE TABLE business_hours (
    id BIGINT PRIMARY KEY,
    business_id UUID REFERENCES businesses(id),
    day_of_week INT,
    open_time TIME,
    close_time TIME,
    is_closed BOOLEAN DEFAULT false
);

-- products table
CREATE TABLE products (
    id UUID PRIMARY KEY,
    business_id UUID REFERENCES businesses(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10,2),
    currency VARCHAR(3) DEFAULT 'EUR',
    image_url VARCHAR(500),
    is_available BOOLEAN DEFAULT true,
    stock_quantity INT,
    created_at, updated_at
);

-- reviews table
CREATE TABLE reviews (
    id UUID PRIMARY KEY,
    business_id UUID REFERENCES businesses(id),
    user_id UUID REFERENCES users(id),
    rating INT CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(255),
    content TEXT,
    created_at, updated_at,
    UNIQUE(business_id, user_id)
);

-- orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    business_id UUID REFERENCES businesses(id),
    user_id UUID REFERENCES users(id),
    status VARCHAR(50) DEFAULT 'pending',
    total_amount DECIMAL(10,2),
    currency VARCHAR(3) DEFAULT 'EUR',
    notes TEXT,
    created_at, updated_at
);

-- order_items table
CREATE TABLE order_items (
    id BIGINT PRIMARY KEY,
    order_id UUID REFERENCES orders(id),
    product_id UUID REFERENCES products(id),
    quantity INT,
    unit_price DECIMAL(10,2),
    total_price DECIMAL(10,2)
);
```

### 4.2 View (V)

#### Pages
- [ ] `templates/pages/businesses.html` - Lista business
- [ ] `templates/pages/business-detail.html` - Dettaglio business
- [ ] `templates/pages/create-business.html` - Crea business
- [ ] `templates/pages/edit-business.html` - Modifica business
- [ ] `templates/pages/products.html` - Lista prodotti
- [ ] `templates/pages/product-detail.html` - Dettaglio prodotto
- [ ] `templates/pages/cart.html` - Carrello
- [ ] `templates/pages/checkout.html` - Checkout
- [ ] `templates/pages/orders.html` - I miei ordini
- [ ] `templates/pages/order-detail.html` - Dettaglio ordine

#### Fragments
- [ ] `templates/fragments/business-card.html`
- [ ] `templates/fragments/business-header.html`
- [ ] `templates/fragments/business-hours.html`
- [ ] `templates/fragments/product-card.html`
- [ ] `templates/fragments/product-grid.html`
- [ ] `templates/fragments/review-card.html`
- [ ] `templates/fragments/review-form.html`
- [ ] `templates/fragments/rating-stars.html`
- [ ] `templates/fragments/cart-item.html`
- [ ] `templates/fragments/order-summary.html`
- [ ] `templates/fragments/order-status.html`

### 4.3 Controller (C)

#### API Handlers
- [ ] `handlers/businesses.rs`
- [ ] `handlers/products.rs`
- [ ] `handlers/reviews.rs`
- [ ] `handlers/orders.rs`

#### Page Handlers
- [ ] Business pages
- [ ] Product pages
- [ ] Order pages

### 4.4 Tests
- [ ] Business CRUD tests
- [ ] Product tests
- [ ] Review tests
- [ ] Order flow tests

---

## 🎯 **PHASE 5: Governance & Voting (2-3 weeks)**

### 5.1 Model (M)

#### Database Schema
```sql
-- proposals table
CREATE TABLE proposals (
    id UUID PRIMARY KEY,
    community_id UUID REFERENCES communities(id),
    author_id UUID REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    proposal_type VARCHAR(50), -- 'text', 'poll', 'budget'
    status VARCHAR(50) DEFAULT 'draft',
    voting_start TIMESTAMP,
    voting_end TIMESTAMP,
    quorum_required INT,
    created_at, updated_at
);

-- proposal_options table (for polls)
CREATE TABLE proposal_options (
    id BIGINT PRIMARY KEY,
    proposal_id UUID REFERENCES proposals(id),
    option_text VARCHAR(255),
    vote_count INT DEFAULT 0
);

-- votes table
CREATE TABLE votes (
    id BIGINT PRIMARY KEY,
    proposal_id UUID REFERENCES proposals(id),
    user_id UUID REFERENCES users(id),
    option_id BIGINT REFERENCES proposal_options(id),
    vote_weight DECIMAL(10,2) DEFAULT 1,
    created_at TIMESTAMP,
    UNIQUE(proposal_id, user_id)
);

-- decisions table
CREATE TABLE decisions (
    id UUID PRIMARY KEY,
    proposal_id UUID REFERENCES proposals(id),
    community_id UUID REFERENCES communities(id),
    outcome VARCHAR(50),
    implementation_status VARCHAR(50) DEFAULT 'pending',
    notes TEXT,
    decided_at TIMESTAMP,
    implemented_at TIMESTAMP
);
```

### 5.2 View (V)

#### Pages
- [ ] `templates/pages/proposals.html` - Lista proposte
- [ ] `templates/pages/proposal-detail.html` - Dettaglio proposta
- [ ] `templates/pages/create-proposal.html` - Crea proposta
- [ ] `templates/pages/voting.html` - Pagina votazione
- [ ] `templates/pages/decisions.html` - Decisioni prese

#### Fragments
- [ ] `templates/fragments/proposal-card.html`
- [ ] `templates/fragments/proposal-status.html`
- [ ] `templates/fragments/voting-options.html`
- [ ] `templates/fragments/vote-results.html`
- [ ] `templates/fragments/vote-progress.html`
- [ ] `templates/fragments/decision-card.html`

### 5.3 Controller (C)

#### API Handlers
- [ ] `handlers/proposals.rs`
- [ ] `handlers/votes.rs`
- [ ] `handlers/decisions.rs`

### 5.4 Tests
- [ ] Proposal lifecycle tests
- [ ] Voting tests
- [ ] Decision tests

---

## 🎯 **PHASE 6: Chat & Real-time (1-2 weeks)**

### 6.1 Model (M)

#### Database Schema
```sql
-- chat_rooms table (già esistente, estendere)
-- messages table
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    room_id UUID REFERENCES chat_rooms(id),
    sender_id UUID REFERENCES users(id),
    content TEXT NOT NULL,
    message_type VARCHAR(20) DEFAULT 'text',
    is_edited BOOLEAN DEFAULT false,
    created_at TIMESTAMP
);

-- message_reads table
CREATE TABLE message_reads (
    id BIGINT PRIMARY KEY,
    message_id UUID REFERENCES messages(id),
    user_id UUID REFERENCES users(id),
    read_at TIMESTAMP,
    UNIQUE(message_id, user_id)
);
```

### 6.2 View (V)

#### Pages
- [ ] `templates/pages/chat.html` - Chat principale
- [ ] `templates/pages/chat-room.html` - Singola chat room

#### Fragments
- [ ] `templates/fragments/chat-sidebar.html`
- [ ] `templates/fragments/chat-room-list.html`
- [ ] `templates/fragments/chat-message.html`
- [ ] `templates/fragments/chat-input.html`
- [ ] `templates/fragments/online-users.html`
- [ ] `templates/fragments/typing-indicator.html`

### 6.3 Controller (C)

#### WebSocket Handlers
- [ ] `handlers/websocket.rs`
  - Connection management
  - Message broadcasting
  - Presence updates

#### API Handlers
- [ ] `handlers/chat.rs`
  - Room management
  - Message history
  - Read receipts

### 6.4 Tests
- [ ] WebSocket connection tests
- [ ] Message delivery tests
- [ ] Presence tests

---

## 🎯 **PHASE 7: Advanced Features & Analytics (1-2 weeks)**

### 7.1 Model (M)

#### Database Schema
```sql
-- analytics_events table
CREATE TABLE analytics_events (
    id BIGINT PRIMARY KEY,
    event_type VARCHAR(100),
    user_id UUID REFERENCES users(id),
    community_id UUID REFERENCES communities(id),
    metadata JSONB,
    created_at TIMESTAMP
);

-- moderation_queue table
CREATE TABLE moderation_queue (
    id UUID PRIMARY KEY,
    content_type VARCHAR(50),
    content_id UUID,
    reported_by UUID REFERENCES users(id),
    reason VARCHAR(255),
    status VARCHAR(50) DEFAULT 'pending',
    moderator_id UUID REFERENCES users(id),
    resolution TEXT,
    created_at, resolved_at
);

-- audit_logs table
CREATE TABLE audit_logs (
    id BIGINT PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    action VARCHAR(100),
    target_type VARCHAR(50),
    target_id UUID,
    old_value JSONB,
    new_value JSONB,
    ip_address VARCHAR(45),
    created_at TIMESTAMP
);
```

### 7.2 View (V)

#### Pages
- [ ] `templates/pages/admin/dashboard.html`
- [ ] `templates/pages/admin/users.html`
- [ ] `templates/pages/admin/communities.html`
- [ ] `templates/pages/admin/moderation.html`
- [ ] `templates/pages/admin/analytics.html`
- [ ] `templates/pages/admin/audit-log.html`

#### Fragments
- [ ] `templates/fragments/admin/stats-card.html`
- [ ] `templates/fragments/admin/chart.html`
- [ ] `templates/fragments/admin/user-row.html`
- [ ] `templates/fragments/admin/moderation-item.html`
- [ ] `templates/fragments/admin/audit-entry.html`

### 7.3 Controller (C)

#### API Handlers
- [ ] `handlers/admin.rs`
- [ ] `handlers/analytics.rs`
- [ ] `handlers/moderation.rs`

### 7.4 Tests
- [ ] Admin access tests
- [ ] Analytics tests
- [ ] Moderation workflow tests

---

## 🎯 **PHASE 8: Deployment & Polish (1-2 weeks)**

### 8.1 Testing
- [ ] End-to-end tests
- [ ] Load testing
- [ ] Security audit
- [ ] Performance testing

### 8.2 Optimization
- [ ] Database query optimization
- [ ] Caching (Redis)
- [ ] CDN for static assets
- [ ] Image optimization

### 8.3 Deployment
- [ ] CI/CD pipeline
- [ ] Monitoring (Prometheus/Grafana)
- [ ] Logging (structured logs)
- [ ] Backup strategy

### 8.4 Documentation
- [ ] API documentation (OpenAPI)
- [ ] User guide
- [ ] Admin guide
- [ ] Developer guide

---

## 🎯 **PHASE 9: FEDERATION (BONUS - 2-3 weeks)**

### ⚠️ Implementato DOPO che Phases 1-8 sono complete

### 9.1 Model (M)
- [ ] `federation_requests` table
- [ ] `federation_instances` table
- [ ] Federation fields su tabelle esistenti

### 9.2 View (V)
- [ ] `templates/pages/admin/federation.html`
- [ ] `templates/fragments/federation-instance.html`
- [ ] `templates/fragments/federation-request.html`

### 9.3 Controller (C)
- [ ] `handlers/federation.rs`
- [ ] Key management
- [ ] Instance verification
- [ ] Data sync

---

## 📈 **Timeline Summary**

| Phase | M | V | C | Tests | Duration | Status |
|-------|---|---|---|-------|----------|--------|
| **1** | ✅ | ✅ | ✅ | ✅ | 2-3w | ✅ DONE |
| **2** | ✅ | ✅ | ✅ | ✅ | 2-3w | ✅ DONE |
| **3** | ⏳ | ⏳ | ⏳ | ⏳ | 2-3w | ⏳ TODO |
| **4** | ⏳ | ⏳ | ⏳ | ⏳ | 2-3w | ⏳ TODO |
| **5** | ⏳ | ⏳ | ⏳ | ⏳ | 2-3w | ⏳ TODO |
| **6** | ⏳ | ⏳ | ⏳ | ⏳ | 1-2w | ⏳ TODO |
| **7** | ⏳ | ⏳ | ⏳ | ⏳ | 1-2w | ⏳ TODO |
| **8** | - | - | - | ✅ | 1-2w | ⏳ TODO |
| **9** | ⏳ | ⏳ | ⏳ | ⏳ | 2-3w | 🎁 BONUS |

---

## 🎯 **Next Steps**

### Immediate: Start Phase 3 - User Profiles & Search
1. Create migration for user_profiles extension
2. Implement profile pages (view, edit)
3. Implement follow/unfollow functionality
4. Add global search endpoint
5. Create notifications system

### Known Issues to Fix
- [ ] `chat-service` schema mismatch (room_id: Uuid vs bigint) - Phase 6

---

**Status**: ✅ Phase 2 Complete - Ready for Phase 3

🚀 **Target**: Complete MVC for each phase before moving to next

