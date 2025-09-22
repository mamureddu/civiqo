# Authentication System - Community Manager

## ⚠️ IMPORTANT: Authentication Architecture

**The Community Manager project uses NextAuth.js with Auth0 as the authentication provider. We do NOT use the @auth0/nextjs-auth0 SDK.**

## Technology Stack

- **Frontend Authentication**: NextAuth.js v4.24.11
- **Authentication Provider**: Auth0 (Auth0Provider from next-auth/providers/auth0)
- **Backend Authentication**: JWT validation with Auth0 JWKS
- **Session Management**: NextAuth.js SessionProvider

## Configuration

### Frontend (NextAuth.js Setup)

**Location**: `frontend/src/app/api/auth/[...nextauth]/route.ts`

```typescript
import NextAuth from 'next-auth'
import Auth0Provider from 'next-auth/providers/auth0'

const handler = NextAuth({
  providers: [
    Auth0Provider({
      clientId: process.env.AUTH0_CLIENT_ID!,
      clientSecret: process.env.AUTH0_CLIENT_SECRET!,
      issuer: `https://${process.env.AUTH0_DOMAIN}`,
    })
  ],
  callbacks: {
    async jwt({ token, account, profile }) {
      if (account) {
        token.accessToken = account.access_token
        token.idToken = account.id_token
      }
      return token
    },
    async session({ session, token }) {
      session.accessToken = token.accessToken
      session.idToken = token.idToken
      return session
    },
  },
  pages: {
    signIn: '/auth/signin',
    signOut: '/auth/signout',
    error: '/auth/error',
  },
})
```

### Frontend Provider Setup

**Location**: `frontend/src/components/providers/AuthProvider.tsx`

```typescript
import { SessionProvider } from 'next-auth/react'

export default function AuthProvider({ children }: AuthProviderProps) {
  return <SessionProvider>{children}</SessionProvider>
}
```

### Frontend Usage

**In components**:
```typescript
import { useSession, signIn, signOut } from 'next-auth/react'

function MyComponent() {
  const { data: session, status } = useSession()
  const user = session?.user

  if (status === 'loading') return <Loading />
  if (!session) return <button onClick={() => signIn('auth0')}>Sign In</button>

  return (
    <div>
      <p>Welcome {user?.name}</p>
      <button onClick={() => signOut()}>Sign Out</button>
    </div>
  )
}
```

## Backend Authentication

### JWT Validation

**Location**: `backend/shared/src/auth/mod.rs`

The backend validates JWT tokens from Auth0 using JWKS (JSON Web Key Set):

```rust
pub struct JwtValidator {
    pub client: reqwest::Client,
    pub auth0_domain: String,
    pub auth0_audience: String,
}

impl JwtValidator {
    pub async fn validate_token(&self, token: &str) -> Result<Claims> {
        // Validates JWT against Auth0 JWKS endpoint
        // Returns Claims struct with user information
    }
}
```

### API Gateway Integration

**Location**: `backend/api-gateway/src/middleware/auth.rs`

```rust
pub async fn extract_user(state: &AppState, headers: &HeaderMap) -> Result<AuthenticatedUser> {
    // Extracts Bearer token from Authorization header
    // Validates JWT with Auth0
    // Returns authenticated user information
}
```

## Environment Variables

### Frontend (.env.local)
```bash
AUTH0_CLIENT_ID=your_auth0_client_id
AUTH0_CLIENT_SECRET=your_auth0_client_secret
AUTH0_DOMAIN=your_domain.eu.auth0.com
NEXTAUTH_URL=http://localhost:3000
NEXTAUTH_SECRET=your_nextauth_secret
```

### Backend (.env)
```bash
AUTH0_DOMAIN=your_domain.eu.auth0.com
AUTH0_AUDIENCE=your_auth0_audience
AUTH0_CLIENT_ID=your_auth0_client_id
AUTH0_CLIENT_SECRET=your_auth0_client_secret
```

## Authentication Flow

1. **User Login**: User clicks sign in → NextAuth.js redirects to Auth0
2. **Auth0 Authentication**: User authenticates with Auth0
3. **Callback**: Auth0 redirects back to `/api/auth/callback/auth0`
4. **Session Creation**: NextAuth.js creates session with access tokens
5. **API Requests**: Frontend sends JWT in Authorization header to backend
6. **Backend Validation**: Backend validates JWT with Auth0 JWKS endpoint
7. **Protected Resources**: Backend returns user-specific data

## Current Status

✅ **Frontend Setup**: Complete with NextAuth.js + Auth0Provider
✅ **Backend JWT Validation**: Complete with JWKS validation
✅ **Environment Configuration**: Auth0 tenant configured
✅ **Session Management**: NextAuth.js SessionProvider integrated
✅ **API Integration**: JWT extraction and validation middleware ready

## DO NOT USE

❌ **@auth0/nextjs-auth0**: This is NOT used in this project
❌ **Auth0 SDK directly**: We use NextAuth.js as the wrapper
❌ **Custom JWT libraries**: NextAuth.js handles all JWT operations

## Troubleshooting

If you see errors about `@auth0/nextjs-auth0/client`:
1. Clear Next.js cache: `rm -rf .next`
2. Verify imports use `next-auth/react`
3. Restart development server

## References

- [NextAuth.js Documentation](https://next-auth.js.org/)
- [Auth0 Provider Setup](https://next-auth.js.org/providers/auth0)
- [Auth0 JWT Validation](https://auth0.com/docs/secure/tokens/json-web-tokens)