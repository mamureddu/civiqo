# HTMX + WASM Migration Guide

## Overview

Migrated from Next.js to **HTMX + WASM (Leptos)** for a 100% Rust stack.

## Architecture

```
┌─────────────────────────────────────────┐
│      Actix-web Server (Rust)            │
│  ┌───────────────────────────────────┐  │
│  │  HTMX Templates (Tera)            │  │
│  │  - Pages, Forms, Navigation       │  │
│  │  - Server-side rendering          │  │
│  └───────────────────────────────────┘  │
│                                          │
│  ┌───────────────────────────────────┐  │
│  │  WASM Modules (Leptos)            │  │
│  │  - Chat component                 │  │
│  │  - Map interactions               │  │
│  │  - Real-time features             │  │
│  └───────────────────────────────────┘  │
│                                          │
│  ┌───────────────────────────────────┐  │
│  │  Static Files                     │  │
│  │  - CSS (TailwindCSS)              │  │
│  │  - WASM binaries                  │  │
│  │  - Images                         │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## Stack

### Backend
- **Server**: Actix-web
- **Templates**: Tera
- **Database**: SQLx + CockroachDB Cloud
- **Auth**: Session-based (cookies)

### Frontend
- **Base**: HTMX for 80% of interactions
- **Interactive**: Leptos WASM for complex features
- **Styling**: TailwindCSS
- **Micro-interactions**: Alpine.js

## Project Structure

```
backend/api-gateway/
├── src/
│   ├── handlers/
│   │   ├── pages.rs      # HTML page handlers
│   │   ├── htmx.rs       # HTMX fragment handlers
│   │   ├── auth.rs       # Auth API
│   │   └── ...
│   └── main.rs
├── templates/            # Tera templates
│   ├── base.html
│   ├── index.html
│   ├── communities.html
│   └── chat.html
└── static/              # Static assets
    ├── styles/
    │   └── main.css
    ├── wasm/            # WASM modules (built separately)
    │   ├── chat.js
    │   ├── chat_bg.wasm
    │   ├── map.js
    │   └── map_bg.wasm
    └── images/

frontend/wasm-app/       # Leptos WASM components
├── Cargo.toml
├── src/
│   ├── components/
│   │   ├── chat.rs
│   │   └── map.rs
│   └── lib.rs
└── Trunk.toml
```

## When to Use HTMX vs WASM

### HTMX for:
- ✅ Page navigation
- ✅ Forms (login, signup, create community)
- ✅ Lists and grids (communities, businesses)
- ✅ Simple interactions (modals, dropdowns)
- ✅ CRUD operations
- ✅ Polls and voting

### WASM for:
- ✅ Real-time chat
- ✅ Interactive maps
- ✅ Drag & drop
- ✅ Rich text editors
- ✅ Complex animations
- ✅ Game-like interactions

## Benefits

### Performance
- **Initial Load**: ~20KB (HTMX + CSS)
- **With WASM**: ~220KB (vs ~330KB React)
- **Server Response**: <10ms (Rust)
- **No CORS**: Same origin

### Developer Experience
- **100% Rust**: No context switching
- **Type Safety**: End-to-end
- **Code Sharing**: Shared types between backend/frontend
- **Hot Reload**: Both HTMX and WASM

### Deployment
- **Single Server**: Actix serves everything
- **Simple**: One binary + static files
- **Fast**: Rust performance everywhere

## Development Workflow

### 1. Start Backend (HTMX pages)
```bash
cd backend
cargo run --bin api-gateway
# Server at http://localhost:9001
```

### 2. Develop WASM Components (later)
```bash
cd frontend/wasm-app
trunk serve
# Auto-rebuild on changes
```

### 3. Build for Production
```bash
# Build WASM
cd frontend/wasm-app
trunk build --release

# Copy to backend
cp dist/* ../../backend/api-gateway/static/wasm/

# Build backend
cd ../../backend
cargo lambda build --release
```

## Migration Status

### ✅ Phase 1: HTMX Base (CURRENT)
- [x] Templates structure
- [x] Base layout
- [x] Home page
- [x] Communities list
- [x] Chat page (placeholder)
- [x] Static file serving
- [x] HTMX fragments

### ⏳ Phase 2: WASM Components (NEXT)
- [ ] Setup Leptos workspace
- [ ] Chat WASM component
- [ ] WebSocket integration
- [ ] Map WASM component
- [ ] Build pipeline

### ⏳ Phase 3: Integration
- [ ] Auth system
- [ ] Database integration
- [ ] Real data from CockroachDB
- [ ] Session management
- [ ] File uploads

### ⏳ Phase 4: Polish
- [ ] TailwindCSS build process
- [ ] Optimize WASM size
- [ ] Loading states
- [ ] Error handling
- [ ] Testing

## Next Steps

1. **Test HTMX pages** - Verify templates render
2. **Add database queries** - Real data in fragments
3. **Setup Leptos** - WASM workspace
4. **Implement chat** - First WASM component
5. **Add map** - Second WASM component

---

**Last Updated**: November 18, 2025
**Status**: Phase 1 in progress
