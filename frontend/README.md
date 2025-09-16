# Frontend Application - Next.js 14 + Material UI

Modern web application built with Next.js 14 and Material UI for rapid development and professional design.

## Tech Stack
- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **UI Library**: Material UI v5
- **Styling**: Material UI styled components + Emotion
- **Authentication**: Auth0 Next.js SDK
- **State Management**: Zustand + React Query (TanStack Query)
- **Maps**: React Leaflet with Material UI integration
- **Icons**: Material UI Icons + Heroicons
- **Charts**: Material UI X Charts (for governance analytics)

## Project Structure

### src/app/ - Next.js App Router
```
src/app/
├── layout.tsx              # Root layout with Material UI theme
├── page.tsx                # Home page
├── globals.css             # Global styles
├── (auth)/                 # Authentication group
│   ├── login/page.tsx      # Login page
│   ├── signup/page.tsx     # Registration
│   └── callback/page.tsx   # Auth0 callback
├── communities/            # Community management
│   ├── page.tsx            # Community discovery
│   ├── create/page.tsx     # Create community
│   └── [id]/               # Dynamic community pages
├── businesses/             # Business directory
├── governance/             # Polls and voting
└── chat/                   # Real-time messaging
```

### src/components/ - Component Library
```
src/components/
├── ui/                     # Basic UI primitives
│   ├── ThemeProvider.tsx   # Material UI theme wrapper
│   ├── LoadingButton.tsx   # Custom loading states
│   ├── ConfirmDialog.tsx   # Reusable confirmation dialogs
│   └── DataTable.tsx       # Material UI DataGrid wrapper
├── layouts/                # Page layouts
│   ├── AppLayout.tsx       # Main layout with AppBar/Drawer
│   ├── CommunityLayout.tsx # Community-specific navigation
│   └── AuthLayout.tsx      # Authentication layout
├── community/              # Community features
├── business/               # Business directory
├── chat/                   # Real-time messaging
└── governance/             # Democratic tools
```

### src/lib/ - Utilities and Configuration
```
src/lib/
├── theme.ts                # Material UI theme configuration
├── auth.ts                 # Auth0 configuration
├── api.ts                  # API client with React Query
├── websocket.ts            # WebSocket client for chat
├── crypto.ts               # Client-side E2EE utilities
└── utils.ts                # Common helper functions
```

## Material UI Integration

### Theme Configuration
Custom theme with community-focused color palette:
```typescript
const theme = createTheme({
  palette: {
    primary: { main: '#1976d2' },    // Community blue
    secondary: { main: '#dc004e' },   // Governance red
    success: { main: '#2e7d32' },     // Business green
  },
});
```

### Component Customization
- Custom Material UI component variants
- Consistent spacing and typography
- Responsive breakpoints
- Dark mode support (future)

### Key Material UI Components Used
- **Navigation**: AppBar, Drawer, Tabs, Breadcrumbs
- **Data Display**: DataGrid, Card, List, Timeline, Chip, Badge
- **Inputs**: TextField, Select, DatePicker, Autocomplete
- **Feedback**: Dialog, Snackbar, Progress, Skeleton
- **Layout**: Container, Grid, Stack, Box

## Development

### Setup
```bash
cd frontend
npm install
npm run dev
```

### Material UI Development
- Use Material UI components instead of building custom ones
- Follow Material Design guidelines
- Leverage Material UI's theming system
- Use Material UI X for advanced components (DataGrid, Charts)

### Key Features
- **Responsive Design**: Mobile-first with Material UI breakpoints
- **Accessibility**: Built-in a11y with Material UI
- **Performance**: Server-side rendering with Next.js 14
- **Type Safety**: Full TypeScript integration

## Authentication Flow
1. Auth0 Universal Login
2. JWT token management
3. Protected routes with Material UI loading states
4. Role-based UI components

## State Management
- **Global State**: Zustand stores
- **Server State**: React Query for API data
- **Form State**: React Hook Form with Material UI integration
- **UI State**: Material UI built-in state management

## Real-time Features
- WebSocket connection for chat
- Material UI components for message display
- Optimistic updates with React Query
- Connection status indicators

## Deployment
- **Development**: `npm run dev`
- **Build**: `npm run build`
- **Production**: Vercel deployment (zero-config)

## Performance Optimizations
- Next.js 14 App Router with RSC
- Material UI tree shaking
- Image optimization with Next.js
- Bundle splitting and lazy loading