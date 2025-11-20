# ⚠️ CRITICAL: Lambda Authorizer Caching Pitfall

## 🚨 Il Problema

**Errore comune che rompe il caching:**

```rust
// ❌ SBAGLIATO - Policy specifica per endpoint
PolicyStatement {
    effect: "Allow",
    resource: "arn:aws:execute-api:us-east-1:123:api-id/prod/GET/users"
}
```

### Cosa Succede

```
1. User fa login → token: "abc123"

2. User chiama GET /users
   → Authorizer invocato
   → Policy generata: Allow GET /users
   → Cache salvata: key="abc123", value="Allow GET /users"
   
3. User chiama POST /communities
   → API Gateway controlla cache: key="abc123"
   → Cache trovata: "Allow GET /users"
   → Endpoint richiesto: POST /communities
   → Policy non copre POST /communities
   → 403 FORBIDDEN! ❌
   
4. User chiama GET /businesses
   → Stesso problema: 403 FORBIDDEN! ❌
   
5. Ogni endpoint diverso = 403 errore!
```

## ✅ La Soluzione

**Policy con wildcard per TUTTE le route:**

```rust
// ✅ CORRETTO - Policy per tutti gli endpoint
PolicyStatement {
    effect: "Allow",
    resource: "arn:aws:execute-api:us-east-1:123:api-id/prod/*/*"
                                                            ↑↑↑↑
                                                         WILDCARD!
}
```

### Cosa Succede

```
1. User fa login → token: "abc123"

2. User chiama GET /users
   → Authorizer invocato
   → Policy generata: Allow prod/*/*
   → Cache salvata: key="abc123", value="Allow prod/*/*"
   
3. User chiama POST /communities
   → API Gateway controlla cache: key="abc123"
   → Cache trovata: "Allow prod/*/*"
   → Endpoint richiesto: POST /communities
   → Policy copre prod/*/* (include POST /communities)
   → ✅ SUCCESS! Nessuna invocazione Lambda!
   
4. User chiama GET /businesses
   → Cache hit! ✅
   
5. User chiama DELETE /posts/123
   → Cache hit! ✅
   
6. Tutti gli endpoint funzionano con una sola invocazione!
```

## 🔧 Implementazione

### Funzione per Estrarre Wildcard

```rust
/// Extract wildcard resource ARN for caching across all routes
/// Input:  arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/GET/users
/// Output: arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/*/*
fn extract_wildcard_resource(resource: &str) -> String {
    // Split ARN: arn:aws:execute-api:region:account:api-id/stage/method/path
    let parts: Vec<&str> = resource.split('/').collect();
    
    if parts.len() >= 2 {
        // Keep only: arn:aws:execute-api:region:account:api-id/stage
        // Add wildcard: /*/*
        format!("{}/*/*", parts[..2].join("/"))
    } else {
        // Fallback: use original resource (shouldn't happen)
        tracing::warn!("Invalid resource ARN format: {}", resource);
        resource.to_string()
    }
}
```

### Uso nella Policy

```rust
fn generate_policy(
    principal_id: &str,
    effect: &str,
    resource: &str,
    user_context: Option<UserContext>,
) -> AuthorizerResponse {
    // CRITICAL: Convert to wildcard!
    let wildcard_resource = extract_wildcard_resource(resource);
    
    let policy_document = PolicyDocument {
        version: "2012-10-17".to_string(),
        statement: vec![PolicyStatement {
            action: "execute-api:Invoke".to_string(),
            effect: effect.to_string(),
            resource: wildcard_resource,  // ← Wildcard, non resource originale!
        }],
    };
    
    AuthorizerResponse {
        principal_id: principal_id.to_string(),
        policy_document,
        context: user_context.map(|u| convert_to_map(u)),
    }
}
```

## 🧪 Test per Verificare

```rust
#[test]
fn test_policy_same_for_all_routes() {
    // Generate policy for /users endpoint
    let policy1 = generate_policy(
        "user-123",
        "Allow",
        "arn:aws:execute-api:us-east-1:123:api/prod/GET/users",
        None,
    );
    
    // Generate policy for /communities endpoint
    let policy2 = generate_policy(
        "user-123",
        "Allow",
        "arn:aws:execute-api:us-east-1:123:api/prod/POST/communities",
        None,
    );
    
    // CRITICAL: Both policies must have the SAME resource (wildcard)
    assert_eq!(
        policy1.policy_document.statement[0].resource,
        policy2.policy_document.statement[0].resource
    );
    
    // Both should be: arn:aws:execute-api:us-east-1:123:api/prod/*/*
    assert!(policy1.policy_document.statement[0].resource.ends_with("/*/*"));
}
```

## 📊 Impatto Performance

### Senza Wildcard (Cache Rotta)

```
Scenario: 100 utenti, 10 endpoint diversi

Invocazioni Lambda:
- 100 utenti × 10 endpoint = 1,000 invocazioni
- Cache inutile!

Costo:
- 1,000 invocazioni × $0.0000002 = $0.0002

Latenza:
- Ogni richiesta: 50-200ms (Lambda invocation)
```

### Con Wildcard (Cache Funzionante)

```
Scenario: 100 utenti, 10 endpoint diversi

Invocazioni Lambda:
- 100 utenti × 1 invocazione = 100 invocazioni
- Cache funziona per tutti gli endpoint!

Costo:
- 100 invocazioni × $0.0000002 = $0.00002
- Risparmio: 90%!

Latenza:
- Prima richiesta: 50-200ms (Lambda invocation)
- Richieste successive: 5-10ms (cache hit)
- Risparmio: 95%!
```

## 🎯 Best Practices

### ✅ DO

1. **Sempre usa wildcard** per il resource nella policy
2. **Testa con endpoint diversi** per verificare cache
3. **Monitora invocazioni Lambda** per confermare caching
4. **Fai authorization granulare** nel backend Lambda, non nell'authorizer

### ❌ DON'T

1. **Non usare resource specifico** (es. `/GET/users`)
2. **Non fare authorization granulare** nell'authorizer
3. **Non assumere** che cache funzioni senza testare
4. **Non dimenticare** di aggiornare policy quando aggiungi route

## 🔍 Come Verificare

### CloudWatch Logs

```bash
# Conta invocazioni authorizer
aws logs filter-log-events \
  --log-group-name /aws/lambda/authorizer \
  --start-time $(date -u -d '1 hour ago' +%s)000 \
  --query 'events[*].message' \
  | grep "Authorization successful" \
  | wc -l
```

### CloudWatch Metrics

```bash
# Vedi invocations count
aws cloudwatch get-metric-statistics \
  --namespace AWS/Lambda \
  --metric-name Invocations \
  --dimensions Name=FunctionName,Value=authorizer \
  --start-time $(date -u -d '1 hour ago' --iso-8601) \
  --end-time $(date -u --iso-8601) \
  --period 3600 \
  --statistics Sum
```

### Test Manuale

```bash
# Prima richiesta - invoca Lambda
time curl -H "Authorization: Bearer token123" \
  https://api.example.com/users
# Time: ~200ms

# Seconda richiesta stesso endpoint - cache hit
time curl -H "Authorization: Bearer token123" \
  https://api.example.com/users
# Time: ~10ms

# CRITICAL: Terza richiesta ENDPOINT DIVERSO - deve essere cache hit!
time curl -H "Authorization: Bearer token123" \
  https://api.example.com/communities
# Time: ~10ms ✅ (se wildcard corretto)
# Time: ~200ms ❌ (se wildcard mancante - cache rotta!)
```

## 📚 Riferimenti

- [AWS API Gateway Authorizer Caching](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-caching.html)
- [Understanding API Gateway Authorizer Caching](https://tmmr.uk/post/api-gateway/api-gateway-auth-caching/)
- [Lambda Authorizer Best Practices](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-use-lambda-authorizer.html)

## 🎉 Riepilogo

**Ricorda:**
1. ⚠️ **Policy specifica = Cache rotta**
2. ✅ **Policy wildcard = Cache funzionante**
3. 🚀 **Wildcard = 90% costi in meno + 95% latenza in meno**

**Codice chiave:**
```rust
// Sempre converti a wildcard!
let wildcard = extract_wildcard_resource(resource);
// arn:aws:execute-api:region:account:api-id/stage/*/*
```

**Non dimenticare mai il wildcard!** 🎯
