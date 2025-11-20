# 🎯 Context Injection - User Data Optimization

## 💡 Il Problema

**Approccio tradizionale (inefficiente):**
```
1. User fa richiesta → API Gateway
2. API Gateway → Authorizer (valida token)
3. Authorizer → API Gateway (Allow/Deny)
4. API Gateway → Backend Lambda
5. Backend Lambda → Auth0 /userinfo (fetch user data) ❌
6. Backend Lambda → Database (fetch roles/permissions) ❌
7. Backend Lambda → Response

Problemi:
- 2+ chiamate esterne per ogni richiesta
- Latenza: +200-500ms
- Costi: chiamate Auth0 + DB
- Complessità: ogni Lambda deve gestire auth
```

## ✅ La Soluzione: Context Injection

**Approccio ottimizzato:**
```
1. User fa richiesta → API Gateway
2. API Gateway → Authorizer (valida token + fetch TUTTO)
3. Authorizer → Auth0/DB (UNA VOLTA, poi cached 1h)
4. Authorizer → API Gateway (Allow + USER CONTEXT)
5. API Gateway → Backend Lambda (con context già popolato) ✅
6. Backend Lambda → Response (nessuna chiamata esterna!)

Vantaggi:
- 1 sola chiamata esterna (cached per 1h)
- Latenza: -90%
- Costi: -99%
- Semplicità: context già disponibile
```

## 📦 Dati Iniettati nel Context

### Identity (sempre presente)
```json
{
  "userId": "auth0|123456789",
  "email": "user@example.com",
  "username": "mario"
}
```

### Authorization (sempre presente)
```json
{
  "roles": "admin,user,moderator",
  "permissions": "read:posts,write:posts,delete:posts"
}
```

### Profile (opzionale)
```json
{
  "name": "Mario Rossi",
  "picture": "https://cdn.auth0.com/avatars/mr.png"
}
```

### Metadata
```json
{
  "emailVerified": true,
  "createdAt": "2024-01-15T10:30:00Z",
  "lastLogin": "2025-11-20T10:15:00Z"
}
```

## 🔧 Come Usare il Context nelle Lambda Backend

### Rust (Lambda Runtime)

```rust
use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct RequestContext {
    authorizer: AuthorizerContext,
}

#[derive(Debug, Deserialize)]
struct AuthorizerContext {
    #[serde(rename = "userId")]
    user_id: String,
    email: String,
    username: String,
    roles: String,
    permissions: String,
    picture: Option<String>,
    name: Option<String>,
    #[serde(rename = "emailVerified")]
    email_verified: String,
    #[serde(rename = "createdAt")]
    created_at: Option<String>,
    #[serde(rename = "lastLogin")]
    last_login: Option<String>,
}

async fn handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<Response, Error> {
    // Extract user context (NO external calls needed!)
    let user_id = &event.payload.request_context.authorizer.user_id;
    let email = &event.payload.request_context.authorizer.email;
    let roles: Vec<&str> = event.payload.request_context.authorizer.roles.split(',').collect();
    let permissions: Vec<&str> = event.payload.request_context.authorizer.permissions.split(',').collect();
    
    // Check permissions
    if !permissions.contains(&"write:posts") {
        return Err("Forbidden".into());
    }
    
    // Use user data
    let post = create_post(user_id, &payload).await?;
    
    Ok(Response {
        status_code: 200,
        body: serde_json::to_string(&post)?,
    })
}
```

### Rust (Axum - se usi Axum su Lambda)

```rust
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};

#[derive(Debug, Clone)]
struct UserContext {
    user_id: String,
    email: String,
    username: String,
    roles: Vec<String>,
    permissions: Vec<String>,
}

// Middleware per estrarre context dall'evento Lambda
async fn extract_user_context(
    event: &ApiGatewayProxyRequest,
) -> Result<UserContext, StatusCode> {
    let auth = &event.request_context.authorizer;
    
    Ok(UserContext {
        user_id: auth.user_id.clone(),
        email: auth.email.clone(),
        username: auth.username.clone(),
        roles: auth.roles.split(',').map(String::from).collect(),
        permissions: auth.permissions.split(',').map(String::from).collect(),
    })
}

// Handler con context già estratto
async fn create_post(
    Extension(user): Extension<UserContext>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<Post>, StatusCode> {
    // User context già disponibile!
    if !user.permissions.contains(&"write:posts".to_string()) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let post = Post {
        author_id: user.user_id,
        author_email: user.email,
        title: payload.title,
        content: payload.content,
    };
    
    Ok(Json(post))
}
```

### Node.js

```javascript
exports.handler = async (event) => {
    // Extract user context
    const { authorizer } = event.requestContext;
    
    const userId = authorizer.userId;
    const email = authorizer.email;
    const roles = authorizer.roles.split(',');
    const permissions = authorizer.permissions.split(',');
    
    // Check permissions
    if (!permissions.includes('write:posts')) {
        return {
            statusCode: 403,
            body: JSON.stringify({ error: 'Forbidden' })
        };
    }
    
    // Use user data (no external calls!)
    const post = await createPost({
        authorId: userId,
        authorEmail: email,
        ...body
    });
    
    return {
        statusCode: 200,
        body: JSON.stringify(post)
    };
};
```

### Python

```python
def lambda_handler(event, context):
    # Extract user context
    authorizer = event['requestContext']['authorizer']
    
    user_id = authorizer['userId']
    email = authorizer['email']
    roles = authorizer['roles'].split(',')
    permissions = authorizer['permissions'].split(',')
    
    # Check permissions
    if 'write:posts' not in permissions:
        return {
            'statusCode': 403,
            'body': json.dumps({'error': 'Forbidden'})
        }
    
    # Use user data (no external calls!)
    post = create_post(
        author_id=user_id,
        author_email=email,
        **body
    )
    
    return {
        'statusCode': 200,
        'body': json.dumps(post)
    }
```

## 🎯 Pattern di Utilizzo

### 1. Authorization Check

```rust
// Check if user has specific role
let roles: Vec<&str> = context.authorizer.roles.split(',').collect();
if !roles.contains(&"admin") {
    return Err(StatusCode::FORBIDDEN);
}

// Check if user has specific permission
let permissions: Vec<&str> = context.authorizer.permissions.split(',').collect();
if !permissions.contains(&"delete:posts") {
    return Err(StatusCode::FORBIDDEN);
}
```

### 2. Audit Logging

```rust
// Log user action with full context
tracing::info!(
    user_id = %context.authorizer.user_id,
    email = %context.authorizer.email,
    action = "create_post",
    "User created post"
);
```

### 3. Resource Ownership

```rust
// Check if user owns resource
let post = get_post(post_id).await?;
if post.author_id != context.authorizer.user_id {
    return Err(StatusCode::FORBIDDEN);
}
```

### 4. User Enrichment

```rust
// Add user info to response
let response = PostResponse {
    id: post.id,
    title: post.title,
    author: AuthorInfo {
        id: context.authorizer.user_id.clone(),
        username: context.authorizer.username.clone(),
        email: context.authorizer.email.clone(),
        picture: context.authorizer.picture.clone(),
    },
};
```

## 📊 Performance Comparison

### Without Context Injection

```
Request 1:
- Authorizer: 100ms (validate token)
- Backend: 200ms (fetch user from Auth0) + 50ms (business logic)
- Total: 350ms

Request 2 (same user):
- Authorizer: 10ms (cached)
- Backend: 200ms (fetch user from Auth0 again!) + 50ms
- Total: 260ms

1000 requests = 260,000ms = 260 seconds
```

### With Context Injection

```
Request 1:
- Authorizer: 200ms (validate + fetch user)
- Backend: 50ms (context already available!)
- Total: 250ms

Request 2 (same user):
- Authorizer: 10ms (cached with context!)
- Backend: 50ms (context from cache)
- Total: 60ms

1000 requests = 60,000ms = 60 seconds
Improvement: 76% faster!
```

## 🔒 Security Considerations

### 1. Context Tampering

**Problema**: API Gateway context è read-only per le Lambda, ma potrebbe essere modificato se esposto.

**Soluzione**: Mai esporre il context direttamente agli utenti. Usalo solo server-side.

### 2. Sensitive Data

**Problema**: Il context è loggato da CloudWatch.

**Soluzione**: Non includere dati sensibili (password, token, etc.) nel context.

```rust
// ❌ SBAGLIATO
map.insert("password".to_string(), json!(user.password));
map.insert("creditCard".to_string(), json!(user.credit_card));

// ✅ CORRETTO
map.insert("userId".to_string(), json!(user.user_id));
map.insert("email".to_string(), json!(user.email));
```

### 3. Context Size Limit

**Problema**: API Gateway ha un limite di 10KB per il context.

**Soluzione**: Includi solo dati essenziali. Per dati grandi, usa riferimenti.

```rust
// ❌ SBAGLIATO - troppo grande
map.insert("fullProfile".to_string(), json!(user.full_profile)); // 50KB

// ✅ CORRETTO - solo ID
map.insert("userId".to_string(), json!(user.user_id));
// Backend può fetchare full profile se necessario
```

## 🎉 Vantaggi Riassuntivi

1. **Performance**: 76% più veloce
2. **Costi**: 99% meno chiamate esterne
3. **Semplicità**: Context già disponibile
4. **Scalabilità**: Caching automatico
5. **Manutenibilità**: Logica auth centralizzata
6. **Debugging**: Context loggato automaticamente

## 📚 Best Practices

1. ✅ **Includi solo dati essenziali** nel context
2. ✅ **Usa nomi camelCase** per compatibilità JSON
3. ✅ **Valida permissions** nel backend, non solo nell'authorizer
4. ✅ **Logga user actions** con context per audit
5. ✅ **Testa con context mock** per unit tests
6. ❌ **Non includere dati sensibili** (password, token)
7. ❌ **Non superare 10KB** di context size
8. ❌ **Non fare chiamate esterne** nel backend per dati già nel context

**Context injection = Ottimizzazione massima!** 🚀
