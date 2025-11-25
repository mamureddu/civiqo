# 📋 Agent 2 Planning: Phase 2 Views - Posts & Comments UI

**Date**: November 25, 2025  
**Role**: Agent 2 (Tech Lead)  
**Task**: Plan Views implementation for Phase 2

---

## 🎯 Objective

Complete the **View (V)** layer for Phase 2 Posts & Comments system.

**Already Done:**
- ✅ Model: `posts`, `comments`, `reactions` tables
- ✅ Controller: 13 API endpoints in `posts.rs`, `comments.rs`, `reactions.rs`
- ✅ Tests: 12 integration tests

**To Do:**
- ⏳ Pages: Full page templates
- ⏳ Fragments: Reusable UI components
- ⏳ Page Handlers: Server-side rendering handlers
- ⏳ Routes: Wire pages to main.rs

---

## 📁 Files to Create

### 1. Pages (Full Templates)

#### `templates/pages/community_posts.html`
**Purpose**: List all posts in a community  
**Route**: GET /communities/:id/posts  
**Features**:
- Header with community name
- "Create Post" button (if member)
- List of post cards
- Pagination
- Sort options (newest, popular)

#### `templates/pages/post_detail.html`
**Purpose**: Single post with comments  
**Route**: GET /posts/:id  
**Features**:
- Full post content
- Author info
- Reactions
- Comments section
- Comment form (if member)
- Edit/Delete buttons (if author/admin)

#### `templates/pages/create_post.html`
**Purpose**: Form to create new post  
**Route**: GET /communities/:id/posts/new  
**Features**:
- Title input
- Content textarea (markdown)
- Media upload (optional)
- Submit button
- Cancel button

### 2. Fragments (Reusable Components)

#### `templates/fragments/post-card.html`
**Purpose**: Card for post in list view  
**Used in**: community_posts.html, dashboard  
**Content**:
```html
<div class="post-card">
    <div class="post-header">
        <img src="{{ author.avatar }}" class="avatar">
        <span class="author-name">{{ author.name }}</span>
        <span class="post-date">{{ created_at }}</span>
    </div>
    <h3 class="post-title">{{ title }}</h3>
    <p class="post-excerpt">{{ content | truncate(200) }}</p>
    <div class="post-footer">
        <span class="reactions">{{ reaction_count }} reactions</span>
        <span class="comments">{{ comment_count }} comments</span>
        <a href="/posts/{{ id }}">Read more</a>
    </div>
</div>
```

#### `templates/fragments/post-full.html`
**Purpose**: Full post content  
**Used in**: post_detail.html  
**Content**: Complete post with all details

#### `templates/fragments/comment-item.html`
**Purpose**: Single comment  
**Used in**: post_detail.html, comment-thread.html  
**Content**:
```html
<div class="comment" id="comment-{{ id }}">
    <div class="comment-header">
        <img src="{{ author.avatar }}" class="avatar-sm">
        <span class="author-name">{{ author.name }}</span>
        <span class="comment-date">{{ created_at }}</span>
        {% if is_edited %}<span class="edited">(edited)</span>{% endif %}
    </div>
    <div class="comment-content">{{ content }}</div>
    <div class="comment-actions">
        <button hx-post="/api/posts/{{ post_id }}/comments" 
                hx-vals='{"parent_id": "{{ id }}"}'>Reply</button>
        {% if is_author %}
        <button hx-put="/api/comments/{{ id }}">Edit</button>
        <button hx-delete="/api/comments/{{ id }}">Delete</button>
        {% endif %}
    </div>
    <!-- Nested replies -->
    <div class="replies" id="replies-{{ id }}">
        {% for reply in replies %}
            {% include "fragments/comment-item.html" %}
        {% endfor %}
    </div>
</div>
```

#### `templates/fragments/comment-form.html`
**Purpose**: Form to add comment  
**Used in**: post_detail.html  
**Content**:
```html
<form hx-post="/api/posts/{{ post_id }}/comments"
      hx-target="#comments-list"
      hx-swap="afterbegin">
    <textarea name="content" placeholder="Write a comment..." required></textarea>
    <input type="hidden" name="parent_id" value="{{ parent_id }}">
    <button type="submit">Post Comment</button>
</form>
```

#### `templates/fragments/reaction-buttons.html`
**Purpose**: Reaction buttons  
**Used in**: post_detail.html, post-card.html  
**Content**:
```html
<div class="reactions" hx-get="/api/posts/{{ post_id }}/reactions" hx-trigger="load">
    <button hx-post="/api/posts/{{ post_id }}/reactions" 
            hx-vals='{"reaction_type": "like"}'
            class="{% if user_reaction == 'like' %}active{% endif %}">
        👍 {{ like_count }}
    </button>
    <button hx-post="/api/posts/{{ post_id }}/reactions"
            hx-vals='{"reaction_type": "heart"}'
            class="{% if user_reaction == 'heart' %}active{% endif %}">
        ❤️ {{ heart_count }}
    </button>
    <!-- More reaction types -->
</div>
```

#### `templates/fragments/post-form.html`
**Purpose**: Form to create/edit post  
**Used in**: create_post.html, edit modal  
**Content**:
```html
<form hx-post="/api/communities/{{ community_id }}/posts"
      hx-target="body"
      hx-swap="innerHTML">
    <input type="text" name="title" placeholder="Post title" required>
    <textarea name="content" placeholder="Write your post..." required></textarea>
    <select name="content_type">
        <option value="markdown">Markdown</option>
        <option value="text">Plain Text</option>
    </select>
    <input type="url" name="media_url" placeholder="Media URL (optional)">
    <button type="submit">Create Post</button>
</form>
```

---

## 🔧 Page Handlers to Add

### In `handlers/pages.rs`

```rust
/// Community posts page
pub async fn community_posts(
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
    OptionalAuthUser(user): OptionalAuthUser,
) -> Result<Response, AppError> {
    // Fetch community info
    // Fetch posts with pagination
    // Check if user is member
    // Render template
}

/// Post detail page
pub async fn post_detail(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
    OptionalAuthUser(user): OptionalAuthUser,
) -> Result<Response, AppError> {
    // Fetch post with author
    // Fetch comments
    // Fetch reactions
    // Check user permissions
    // Render template
}

/// Create post page
pub async fn create_post_page(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
) -> Result<Response, AppError> {
    // Check user is member
    // Render form template
}
```

---

## 🛤️ Routes to Add

### In `main.rs`

```rust
// Posts pages
.route("/communities/:id/posts", get(pages::community_posts))
.route("/communities/:id/posts/new", get(pages::create_post_page))
.route("/posts/:id", get(pages::post_detail))
```

---

## 🎨 Brand Compliance

### Colors (from Civiqo Brand Book)
- **Primary**: `#57C98A` - Buttons, links, success
- **Secondary**: `#3B7FBA` - Headers, navigation
- **Accent**: `#EF6F5E` - Alerts, reactions, errors
- **Neutral**: `#F5F5F5` - Backgrounds
- **Text**: `#333333` - Body text

### Typography
- **Headings**: Font-weight 600-700
- **Body**: Font-weight 400
- **Small**: Font-size 0.875rem

### Spacing
- **Card padding**: 1rem (16px)
- **Section margin**: 2rem (32px)
- **Element gap**: 0.5rem (8px)

---

## ✅ Acceptance Criteria

### Pages
- [ ] `community_posts.html` renders correctly
- [ ] `post_detail.html` shows post and comments
- [ ] `create_post.html` form works
- [ ] All pages use base.html layout
- [ ] Brand colors applied correctly

### Fragments
- [ ] `post-card.html` displays post summary
- [ ] `comment-item.html` supports nesting
- [ ] `reaction-buttons.html` updates via HTMX
- [ ] All fragments are reusable

### Functionality
- [ ] Create post works (member only)
- [ ] View posts works (public/member based on community)
- [ ] Comments work with threading
- [ ] Reactions update in real-time
- [ ] Edit/delete work for authors

### HTMX Integration
- [ ] Forms submit via HTMX
- [ ] Reactions update without page reload
- [ ] Comments load dynamically
- [ ] Proper loading states

---

## 📊 Implementation Order

1. **Fragments first** (reusable components)
   - post-card.html
   - comment-item.html
   - reaction-buttons.html
   - post-form.html
   - comment-form.html

2. **Pages second** (use fragments)
   - community_posts.html
   - post_detail.html
   - create_post.html

3. **Handlers third** (server-side logic)
   - community_posts handler
   - post_detail handler
   - create_post_page handler

4. **Routes last** (wire everything)
   - Add routes to main.rs
   - Test all pages

---

## ⏱️ Time Estimate

| Task | Time |
|------|------|
| Fragments (5) | 1.5 hours |
| Pages (3) | 1.5 hours |
| Handlers (3) | 1 hour |
| Routes + Testing | 0.5 hours |
| **Total** | **4.5 hours** |

---

## 🚀 Ready for Agent 1

**Agent 1 can now implement:**
1. Create fragments in `templates/fragments/`
2. Create pages in `templates/pages/` (or root templates/)
3. Add handlers to `handlers/pages.rs`
4. Wire routes in `main.rs`
5. Test all functionality

**Start with**: `post-card.html` fragment

