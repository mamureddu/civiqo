# Repository Context & Analysis

## Overview
**Project**: Community Manager
**Tech Stack**:
- **Backend**: Rust (Axum, Tokio)
- **Database**: PostgreSQL / CockroachDB (SQLx)
- **Frontend**: Server-Side Rendered (Tera Templates)
- **Interactivity**: HTMX + Alpine.js
- **Styling**: TailwindCSS (currently via CDN) + Custom CSS (`main.css`)
- **Auth**: OAuth2 (Auth0)

## Current State (based on `docs/NEXT_STEPS.md` & Codebase)

### ✅ Completed Features
- **Authentication**: OAuth2 code exchange, user sync, session management, logout.
- **Basic UI**:
    - `base.html` layout with navigation and footer.
    - Login/Logout flow.
    - Dashboard with user profile and communities list.
    - Protected routes using `AuthUser` extractor.

### 🎨 UI/UX & Branding Status
- **Current Styling**:
    - **TailwindCSS** configured with Civiqo Palette (`civiqo-blue`, `civiqo-teal`, etc.).
    - **Custom CSS** (`src/server/static/styles/main.css`) aligned with brand guidelines.
- **Branding**:
    - **Name**: "Civiqo"
    - **Primary Color**: Civiqo Blue (`#3B7FBA`)
    - **Font**: Nunito (Brand) + Inter (UI)
    - **Logo**: Civiqo Symbol + Wordmark implemented in CSS.
    - **Guidelines**: Strict adherence to [BRAND_GUIDELINES.md](BRAND_GUIDELINES.md).

## Integration Points for Brand ID
To implement the new Brand ID and UI guidelines, the following areas will need updates:

1.  **Design Tokens**:
    - Define the new color palette, typography, and spacing in a structured way.
    - Since Tailwind is used, this likely means creating a `tailwind.config.js` to customize the theme (colors, fonts) instead of relying on the default CDN configuration.

2.  **Global Styles**:
    - Update `src/server/static/styles/main.css` to include new global variables (CSS custom properties) if needed.
    - Import new fonts (e.g., from Google Fonts) in `base.html`.

3.  **Components**:
    - Update existing component classes (buttons, cards, navbars) to match the new guidelines.
    - Refactor hardcoded Tailwind classes (e.g., `bg-indigo-600`) to use semantic names or the new theme colors (e.g., `bg-primary`).

4.  **Templates**:
    - Update `base.html` to include the new logo and favicon.
    - Adjust layout structure if the brand guidelines dictate a different grid or spacing system.

## Next Steps (Pre-Brand Integration)
- [ ] Receive Brand ID assets (Colors, Fonts, Logo, Component specs).
- [ ] Set up a proper Tailwind build process (optional but recommended for customization) or configure the CDN script with custom theme.
- [ ] Apply new styles to `base.html` and `main.css`.
