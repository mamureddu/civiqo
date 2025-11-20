# 🔐 AWS Lambda Authorizer con Caching

## 🎯 Cos'è un Lambda Authorizer?

Un **Lambda Authorizer** (ex Custom Authorizer) è una funzione Lambda che API Gateway chiama **prima** di invocare le tue API per:
1. **Autenticare** la richiesta (validare token/credenziali)
2. **Autorizzare** l'accesso (generare IAM policy)
3. **Iniettare contesto** utente nella richiesta

### 🚀 Vantaggi Chiave

1. **Caching Automatico** - API Gateway cachea le policy fino a 1 ora
2. **Separazione Logica** - Auth separato dal business logic
3. **Riutilizzabile** - Un authorizer per tutte le API
4. **Performance** - Riduce latenza e costi Lambda
5. **Scalabile** - Gestito da AWS, nessun server da mantenere

## 📊 Come Funziona

### Flow Completo

```
1. Client → API Gateway
   Headers: Authorization: Bearer <token>

2. API Gateway → Lambda Authorizer
   Event: { token, methodArn, headers, ... }

3. Lambda Authorizer valida token
   - Verifica JWT signature
   - Chiama Auth0 /userinfo
   - Query database per session
   - Controlla Redis cache

4. Lambda Authorizer → API Gateway
   Response: {
     principalId: "user-123",
     policyDocument: { ... IAM policy ... },
     context: { userId, email, roles, ... }
   }

5. API Gateway cachea la policy
   Cache Key: token
   TTL: 300-3600 secondi

6. API Gateway → Backend Lambda
   Event + context.authorizer: {
     userId: "user-123",
     email: "user@example.com",
     roles: "admin,user"
   }

7. Backend Lambda → Client
   Response con dati
```

### 🔑 Cache Key

Il **cache key** determina quando riutilizzare una policy cachata:

**TOKEN Authorizer:**
```
Cache Key = Authorization header value
```

**REQUEST Authorizer:**
```
Cache Key = Combinazione di:
- Authorization header
- Query parameters
- Stage variables
- Context variables
```

### ⏱️ Cache TTL

- **Min**: 0 secondi (no cache)
- **Max**: 3600 secondi (1 ora)
- **Default**: 300 secondi (5 minuti)

**Considerazioni:**
- TTL più lungo = meno invocazioni Lambda = costi ridotti
- TTL più corto = revoche token più rapide = più sicuro

## 🏗️ Implementazione

### 1. Lambda Authorizer Function

Ho creato `services/authorizer/src/main.rs`:

```rust
// Valida token e genera IAM policy
async fn handler(event: AuthorizerEvent) -> Result<Value, Error> {
    let token = extract_token(&event)?;
    
    match validate_token(&token).await {
        Ok(user_info) => {
            // Generate Allow policy with user context
            generate_policy(&user_info.user_id, "Allow", &event.method_arn, Some(user_info))
        }
        Err(_) => {
            // Generate Deny policy
            generate_policy("unauthorized", "Deny", &event.method_arn, None)
        }
    }
}
```

### 2. IAM Policy Response

```json
{
  "principalId": "user-123",
  "policyDocument": {
    "Version": "2012-10-17",
    "Statement": [
      {
        "Action": "execute-api:Invoke",
        "Effect": "Allow",
        "Resource": "arn:aws:execute-api:region:account:api-id/stage/*/*"
      }
    ]
  },
  "context": {
    "userId": "user-123",
    "email": "user@example.com",
    "username": "mario",
    "roles": "admin,user"
  }
}
```

### ⚠️ **CRITICO: Wildcard Resource per Caching!**

**PROBLEMA COMUNE:**
```
❌ SBAGLIATO:
Resource: "arn:aws:execute-api:region:account:api-id/stage/GET/users"

Cosa succede:
1. User chiama GET /users → cache policy per GET /users
2. User chiama POST /communities → 403! Cache non valida per questo endpoint
3. Cache rotta! Ogni endpoint richiede nuova invocazione Lambda
```

**SOLUZIONE:**
```
✅ CORRETTO:
Resource: "arn:aws:execute-api:region:account:api-id/stage/*/*"

Cosa succede:
1. User chiama GET /users → cache policy per TUTTE le route
2. User chiama POST /communities → usa cache! Nessuna invocazione Lambda
3. Cache funziona per tutti gli endpoint!
```

**Implementazione:**
```rust
fn extract_wildcard_resource(resource: &str) -> String {
    // Input:  arn:aws:execute-api:us-east-1:123:api-id/prod/GET/users
    // Output: arn:aws:execute-api:us-east-1:123:api-id/prod/*/*
    
    let parts: Vec<&str> = resource.split('/').collect();
    format!("{}/*/*", parts[..2].join("/"))
}
```

**Perché è importante:**
- ✅ Cache funziona per **tutti** gli endpoint
- ✅ Una sola invocazione Lambda per utente
- ✅ 99% riduzione costi
- ✅ Latenza minima

**Quando NON usare wildcard:**
- Se hai bisogno di autorizzazione granulare per route (raro)
- In quel caso, disabilita il caching

### 3. Accesso al Context nelle Lambda Backend

```rust
// Nel tuo handler API
#[derive(Deserialize)]
struct RequestContext {
    authorizer: AuthorizerContext,
}

#[derive(Deserialize)]
struct AuthorizerContext {
    #[serde(rename = "userId")]
    user_id: String,
    email: String,
    username: String,
    roles: String,
}

async fn handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    // Accedi al context dell'authorizer
    let user_id = event.request_context.authorizer.user_id;
    let email = event.request_context.authorizer.email;
    
    // Usa i dati utente
    create_community(&user_id, &payload).await?;
    
    Ok(response)
}
```

## 🎨 Pattern di Utilizzo

### Pattern 1: Full Authorization

L'authorizer fa **tutto**:
- Valida token
- Controlla permessi specifici per route/metodo
- Genera policy granulare

```rust
async fn validate_token(token: &str, method_arn: &str) -> Result<UserContext, Error> {
    let user = decode_jwt(token)?;
    
    // Check specific permissions based on method
    if method_arn.contains("/admin/") && !user.roles.contains("admin") {
        return Err("Insufficient permissions".into());
    }
    
    Ok(user)
}
```

### Pattern 2: Authentication Only (CONSIGLIATO)

L'authorizer fa solo **autenticazione**:
- Valida token
- Inietta user context
- Policy sempre Allow per tutte le route
- Backend Lambda fa autorizzazione granulare

```rust
async fn validate_token(token: &str) -> Result<UserContext, Error> {
    // Solo validazione token
    let user = decode_jwt(token)?;
    Ok(user)
}

// Policy sempre Allow con wildcard
fn generate_policy(user: &UserContext) -> AuthorizerResponse {
    AuthorizerResponse {
        principal_id: user.user_id.clone(),
        policy_document: PolicyDocument {
            statement: vec![PolicyStatement {
                effect: "Allow",
                resource: "arn:aws:execute-api:*:*:*/*/*", // Wildcard!
            }],
        },
        context: Some(user_context_map),
    }
}

// Backend Lambda fa authorization
async fn create_community(event: ApiGatewayProxyRequest) -> Result<Response, Error> {
    let user_id = event.request_context.authorizer.user_id;
    let roles = event.request_context.authorizer.roles.split(',').collect::<Vec<_>>();
    
    // Check permissions here
    if !roles.contains(&"admin") {
        return Err("Forbidden".into());
    }
    
    // ... create community ...
}
```

**Perché Pattern 2 è migliore:**
- ✅ Massimo caching (policy uguale per tutti)
- ✅ Flessibilità (cambi permessi senza invalidare cache)
- ✅ Meno complessità nell'authorizer
- ✅ Authorization logic vicina al business logic

## 🔧 Configurazione API Gateway

### Serverless Framework

```yaml
functions:
  authorizer:
    handler: authorizer.handler
    runtime: rust
    
  api:
    handler: api.handler
    events:
      - http:
          path: /communities
          method: post
          authorizer:
            name: authorizer
            resultTtlInSeconds: 3600  # Cache 1 hour
            identitySource: method.request.header.Authorization
            type: token
```

### AWS SAM

```yaml
Resources:
  AuthorizerFunction:
    Type: AWS::Serverless::Function
    Properties:
      CodeUri: ./authorizer
      Handler: bootstrap
      Runtime: provided.al2
      
  ApiGateway:
    Type: AWS::Serverless::Api
    Properties:
      Auth:
        DefaultAuthorizer: LambdaAuthorizer
        Authorizers:
          LambdaAuthorizer:
            FunctionArn: !GetAtt AuthorizerFunction.Arn
            Identity:
              Header: Authorization
              ReauthorizeEvery: 3600  # Cache TTL
```

### Terraform

```hcl
resource "aws_api_gateway_authorizer" "lambda" {
  name                   = "lambda-authorizer"
  rest_api_id            = aws_api_gateway_rest_api.api.id
  authorizer_uri         = aws_lambda_function.authorizer.invoke_arn
  authorizer_credentials = aws_iam_role.invocation_role.arn
  
  # Cache settings
  authorizer_result_ttl_in_seconds = 3600
  identity_source                  = "method.request.header.Authorization"
  type                             = "TOKEN"
}
```

## 🧪 Testing

### Test Locale con cargo-lambda

```bash
# Build authorizer
cd services/authorizer
cargo lambda build --release

# Test con evento mock
cargo lambda invoke authorizer --data-file test-event.json
```

**test-event.json:**
```json
{
  "type": "TOKEN",
  "methodArn": "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/GET/users",
  "authorizationToken": "Bearer valid_token_123"
}
```

### Test su AWS

```bash
# Deploy
cargo lambda deploy authorizer

# Test via API Gateway
curl -H "Authorization: Bearer test_token" \
  https://api-id.execute-api.region.amazonaws.com/prod/communities
```

### Verificare Cache

```bash
# Prima richiesta - invoca Lambda
curl -w "\nTime: %{time_total}s\n" \
  -H "Authorization: Bearer token123" \
  https://api.example.com/communities
# Time: 0.250s (cold start + validation)

# Seconda richiesta - usa cache
curl -w "\nTime: %{time_total}s\n" \
  -H "Authorization: Bearer token123" \
  https://api.example.com/communities
# Time: 0.050s (cached!)

# Terza richiesta con token diverso - invoca Lambda
curl -w "\nTime: %{time_total}s\n" \
  -H "Authorization: Bearer different_token" \
  https://api.example.com/communities
# Time: 0.200s (new cache entry)
```

## 📊 Performance e Costi

### Senza Cache

```
1000 requests/min × 60 min = 60,000 requests/hour
60,000 Lambda invocations × $0.0000002 = $0.012/hour
```

### Con Cache (TTL 1 hour)

```
Unique users: 100
100 Lambda invocations × $0.0000002 = $0.00002/hour
Risparmio: 99.8%!
```

### Latenza

```
Senza cache:
- Cold start: 200-500ms
- Warm: 50-100ms

Con cache:
- Hit: 5-10ms
- Miss: 50-100ms
```

## 🔒 Security Best Practices

### 1. Validazione Token Robusta

```rust
async fn validate_token(token: &str) -> Result<UserContext, Error> {
    // Validate JWT signature
    let validation = Validation::new(Algorithm::RS256);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    
    // Check expiration
    if token_data.claims.exp < current_timestamp() {
        return Err("Token expired".into());
    }
    
    // Check issuer
    if token_data.claims.iss != "https://your-domain.auth0.com/" {
        return Err("Invalid issuer".into());
    }
    
    Ok(UserContext::from(token_data.claims))
}
```

### 2. Cache TTL Appropriato

```
High security APIs: 300s (5 min)
Normal APIs: 1800s (30 min)
Read-only APIs: 3600s (1 hour)
```

### 3. Flush Cache su Breach

```bash
# Flush all cached policies
aws apigateway flush-stage-authorizers-cache \
  --rest-api-id api-id \
  --stage-name prod
```

### 4. Logging e Monitoring

```rust
tracing::info!(
    user_id = %user.user_id,
    method_arn = %event.method_arn,
    "Authorization successful"
);

// CloudWatch Metrics
cloudwatch.put_metric_data(
    namespace: "Authorizer",
    metric_name: "AuthSuccess",
    value: 1.0,
);
```

## 🎯 Migrazione dal Sistema Attuale

### Attuale (Axum Extractors)

```rust
pub async fn create_community(
    AuthUser(user): AuthUser,  // Extractor
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCommunityRequest>,
) -> Result<Json<Response>, StatusCode> {
    // ...
}
```

### Con Lambda Authorizer

```rust
pub async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>
) -> Result<ApiGatewayProxyResponse, Error> {
    // User context già validato dall'authorizer
    let user_id = event.request_context.authorizer.user_id;
    let email = event.request_context.authorizer.email;
    
    // Parse body
    let payload: CreateCommunityRequest = serde_json::from_str(&event.body)?;
    
    // Business logic
    create_community(&user_id, &payload).await?;
    
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        body: serde_json::to_string(&response)?,
        ..Default::default()
    })
}
```

## 📚 Risorse

- [AWS Lambda Authorizers Docs](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-use-lambda-authorizer.html)
- [Alex DeBrie's Guide](https://www.alexdebrie.com/posts/lambda-custom-authorizers/)
- [cargo-lambda](https://www.cargo-lambda.info/)

## 🎉 Vantaggi per il Tuo Progetto

1. **Performance**: Cache riduce latenza del 90%
2. **Costi**: 99% meno invocazioni Lambda
3. **Scalabilità**: Gestito da AWS, auto-scaling
4. **Manutenibilità**: Auth logic centralizzata
5. **Flessibilità**: Cambi auth senza toccare API

**Pronto per implementare!** 🚀
