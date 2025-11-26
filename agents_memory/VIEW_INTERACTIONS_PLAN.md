# View Interaction Tests - Implementation Plan

**Date**: November 26, 2025
**Status**: Planning Phase (Agent 2)
**Total HTMX Interactions**: 40

---

## 📋 All HTMX Interactions by View

### 1. **index.html** (Homepage)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 1 | `hx-get` | `/api/communities/recent` | Load recent communities on page load |

### 2. **dashboard.html** (User Dashboard)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 2 | `hx-get` | `/api/user/communities` | Load user's communities |
| 3 | `hx-get` | `/api/user/activity` | Load user's recent activity |

### 3. **communities.html** (Communities List)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 4 | `hx-get` | `/api/communities/search` | Search communities (form) |
| 5 | `hx-get` | `/api/communities/list` | Load communities list |

### 4. **community_detail.html** (Community Detail)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 6 | `hx-get` | `/api/communities/:id/feed` | Load community feed |

### 5. **community_posts.html** (Community Posts)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 7 | `hx-get` | `/communities/:id/posts?sort=newest` | Sort by newest |
| 8 | `hx-get` | `/communities/:id/posts?sort=popular` | Sort by popular |
| 9 | `hx-get` | `/communities/:id/posts?sort=discussed` | Sort by discussed |

### 6. **create_community.html** (Create Community Form)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 10 | `hx-post` | `/api/communities` | Create new community |

### 7. **post_detail.html** (Post Detail)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 11 | `hx-delete` | `/api/posts/:id` | Delete post |

### 8. **businesses.html** (Businesses List)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 12 | `hx-get` | `/api/businesses/search` | Search businesses |
| 13 | `hx-get` | `/api/businesses/list` | Load businesses list |

### 9. **business_detail.html** (Business Detail)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 14 | `hx-get` | `/api/businesses/:id/posts` | Load business posts |
| 15 | `hx-get` | `/api/businesses/:id/reviews` | Load business reviews |

### 10. **governance.html** (Governance)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 16 | `hx-get` | `/api/governance/proposals` | Load proposals |

### 11. **poi.html** (Points of Interest)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 17 | `hx-get` | `/api/poi/nearby` | Load nearby POIs |

### 12. **chat.html** (Chat)
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 18 | `hx-get` | `/api/chat/:room_id/header` | Load chat room header |

---

## 📋 Fragment Interactions

### 13. **fragments/community-card.html**
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 19 | `hx-post` | `/api/communities/:id/join` | Join public community |
| 20 | `hx-post` | `/api/communities/:id/request-join` | Request to join private |

### 14. **fragments/join-button.html**
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 21 | `hx-post` | `/api/communities/:id/leave` | Leave community |
| 22 | `hx-post` | `/api/communities/:id/join` | Join community |
| 23 | `hx-post` | `/api/communities/:id/request-join` | Request join |

### 15. **fragments/members-list.html**
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 24 | `hx-post` | `/api/communities/:id/promote/:user_id` | Promote member |
| 25 | `hx-post` | `/api/communities/:id/demote/:user_id` | Demote member |
| 26 | `hx-delete` | `/api/communities/:id/members/:user_id` | Remove member |
| 27 | `hx-get` | `/api/communities/:id/members?page=X` | Pagination prev |
| 28 | `hx-get` | `/api/communities/:id/members?page=X` | Pagination next |

### 16. **fragments/post-form.html**
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 29 | `hx-put` | `/api/posts/:id` | Update post |
| 30 | `hx-post` | `/api/communities/:id/posts` | Create post |

### 17. **fragments/comment-form.html**
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 31 | `hx-post` | `/api/posts/:id/comments` | Create comment |
| 32 | `hx-get` | `/api/empty` | Cancel/clear form |

### 18. **fragments/comment-item.html**
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 33 | `hx-get` | `/api/comments/:id/reply-form` | Get reply form |
| 34 | `hx-get` | `/api/comments/:id/edit-form` | Get edit form |
| 35 | `hx-delete` | `/api/comments/:id` | Delete comment |

### 19. **fragments/reaction-buttons.html**
| # | Type | Endpoint | Description |
|---|------|----------|-------------|
| 36 | `hx-post` | `/api/posts/:id/reactions` | Add like reaction |
| 37 | `hx-post` | `/api/posts/:id/reactions` | Add heart reaction |
| 38 | `hx-post` | `/api/posts/:id/reactions` | Add celebrate reaction |
| 39 | `hx-post` | `/api/posts/:id/reactions` | Add thinking reaction |
| 40 | `hx-delete` | `/api/posts/:id/reactions` | Remove reaction |

---

## 🎯 Implementation Strategy

### Phase 1: Create Test File Structure
Create `src/server/tests/view_interactions_test.rs` with:
- Test module organization by view
- Helper functions for HTTP requests
- HTML parsing utilities

### Phase 2: Stub Tests (All 40)
Create stub tests for each interaction:
```rust
#[tokio::test]
#[ignore] // Remove when implemented
async fn test_interaction_XX_description() {
    // TODO: Implement
    todo!("Implement test for endpoint X")
}
```

### Phase 3: Implement Tests by Priority

**Priority 1 - Core User Flows (Critical)**:
- [ ] #10 Create community
- [ ] #19, #22 Join community
- [ ] #21 Leave community
- [ ] #30 Create post
- [ ] #31 Create comment
- [ ] #36-40 Reactions

**Priority 2 - Navigation & Lists**:
- [ ] #1 Recent communities (homepage)
- [ ] #4, #5 Communities search/list
- [ ] #7-9 Post sorting
- [ ] #27, #28 Members pagination

**Priority 3 - Admin Operations**:
- [ ] #24, #25 Promote/demote member
- [ ] #26 Remove member
- [ ] #11 Delete post
- [ ] #35 Delete comment

**Priority 4 - Secondary Features**:
- [ ] #2, #3 Dashboard data
- [ ] #6 Community feed
- [ ] #12-17 Businesses, governance, POI, chat

### Phase 4: Verification
- Run all tests: `cargo test view_interaction --workspace`
- Verify 40/40 tests passing
- Check coverage matches HTMX attributes

---

## 📁 Test File Structure

```
src/server/tests/
├── view_interactions_test.rs     # Main test file
│   ├── mod homepage_tests
│   ├── mod dashboard_tests
│   ├── mod communities_tests
│   ├── mod posts_tests
│   ├── mod comments_tests
│   ├── mod reactions_tests
│   ├── mod membership_tests
│   └── mod admin_tests
```

---

## ✅ Acceptance Criteria

1. **40 test functions** created (one per HTMX interaction)
2. **All tests pass** when run with `cargo test view_interaction`
3. **Tests verify**:
   - HTTP response status codes
   - HTML response contains expected elements
   - Error scenarios handled correctly
4. **No stub tests remaining** (all `#[ignore]` removed)

---

## 🚀 Next Steps

1. Agent 1: Create test file with all 40 stub tests
2. Agent 1: Implement Priority 1 tests (core user flows)
3. Agent 2: Review and verify test coverage
4. Agent 1: Implement remaining tests
5. Agent 2: Final verification

**Estimated Time**: 4-6 hours total
