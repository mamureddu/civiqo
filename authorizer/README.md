# 🔐 Lambda Authorizer

AWS Lambda Authorizer function per autenticazione e autorizzazione con caching.

## 📦 Struttura

```
authorizer/
├── src/
│   └── main.rs          # Lambda handler
├── Cargo.toml           # Dipendenze
├── README.md            # Questa guida
└── CACHING_WARNING.md   # ⚠️ IMPORTANTE: Guida al caching
```

## 🚀 Build e Deploy

### Build Locale

```bash
# Build per AWS Lambda (ARM64)
cargo lambda build --release --arm64

# Build per test locale
cargo build --release
```

### Test Locale

```bash
# Test con cargo-lambda
cargo lambda invoke authorizer --data-file test-event.json

# Run tests
cargo test
```

### Deploy su AWS

```bash
# Deploy con cargo-lambda
cargo lambda deploy authorizer

# Deploy con SAM
sam build
sam deploy --guided

# Deploy con Serverless Framework
serverless deploy
```

## 🔧 Configurazione

### Variabili d'Ambiente

```bash
# .env per test locale
AUTH0_DOMAIN=your-tenant.auth0.com
AUTH0_AUDIENCE=your-api-identifier
JWT_SECRET=your-jwt-secret
```

### API Gateway Configuration

**Serverless Framework (`serverless.yml`):**

```yaml
functions:
  authorizer:
    handler: bootstrap
    runtime: provided.al2
    architecture: arm64
    
  api:
    handler: api
    events:
      - http:
          path: /{proxy+}
          method: ANY
          authorizer:
            name: authorizer
            resultTtlInSeconds: 3600  # Cache 1 hour
            identitySource: method.request.header.Authorization
            type: token
```

**AWS SAM (`template.yaml`):**

```yaml
Resources:
  AuthorizerFunction:
    Type: AWS::Serverless::Function
    Properties:
      CodeUri: ./authorizer
      Handler: bootstrap
      Runtime: provided.al2
      Architectures:
        - arm64
      
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
              ReauthorizeEvery: 3600
```

**Terraform:**

```hcl
resource "aws_lambda_function" "authorizer" {
  filename         = "authorizer.zip"
  function_name    = "api-authorizer"
  role            = aws_iam_role.lambda_role.arn
  handler         = "bootstrap"
  runtime         = "provided.al2"
  architectures   = ["arm64"]
}

resource "aws_api_gateway_authorizer" "lambda" {
  name                   = "lambda-authorizer"
  rest_api_id            = aws_api_gateway_rest_api.api.id
  authorizer_uri         = aws_lambda_function.authorizer.invoke_arn
  authorizer_result_ttl_in_seconds = 3600
  identity_source        = "method.request.header.Authorization"
  type                   = "TOKEN"
}
```

## 📝 Event Format

### Input Event (TOKEN Authorizer)

```json
{
  "type": "TOKEN",
  "methodArn": "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/GET/users",
  "authorizationToken": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### Output Response

```json
{
  "principalId": "user-123",
  "policyDocument": {
    "Version": "2012-10-17",
    "Statement": [
      {
        "Action": "execute-api:Invoke",
        "Effect": "Allow",
        "Resource": "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/*/*"
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

## ⚠️ IMPORTANTE: Caching

**Leggi `CACHING_WARNING.md` per evitare errori critici!**

La policy DEVE usare wildcard `/*/*` per funzionare correttamente con il caching:

```rust
// ✅ CORRETTO
resource: "arn:aws:execute-api:region:account:api-id/stage/*/*"

// ❌ SBAGLIATO - Cache rotta!
resource: "arn:aws:execute-api:region:account:api-id/stage/GET/users"
```

## 🧪 Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
# Test con evento mock
cargo lambda invoke authorizer --data-file test-event.json

# Test con token reale
cargo lambda invoke authorizer --data-ascii '{
  "type": "TOKEN",
  "methodArn": "arn:aws:execute-api:us-east-1:123:api/prod/GET/users",
  "authorizationToken": "Bearer YOUR_TOKEN_HERE"
}'
```

### Test su AWS

```bash
# Deploy
cargo lambda deploy authorizer

# Test via API Gateway
curl -H "Authorization: Bearer test_token" \
  https://api-id.execute-api.region.amazonaws.com/prod/users
```

## 📊 Monitoring

### CloudWatch Logs

```bash
# View logs
aws logs tail /aws/lambda/authorizer --follow

# Filter errors
aws logs filter-log-events \
  --log-group-name /aws/lambda/authorizer \
  --filter-pattern "ERROR"
```

### CloudWatch Metrics

```bash
# Invocations count
aws cloudwatch get-metric-statistics \
  --namespace AWS/Lambda \
  --metric-name Invocations \
  --dimensions Name=FunctionName,Value=authorizer \
  --start-time $(date -u -d '1 hour ago' --iso-8601) \
  --end-time $(date -u --iso-8601) \
  --period 3600 \
  --statistics Sum
```

## 🔒 Security

### JWT Validation

L'authorizer valida:
- ✅ Signature JWT
- ✅ Expiration time
- ✅ Issuer
- ✅ Audience

### Best Practices

1. **Cache TTL**: 300-3600 secondi (5 min - 1 ora)
2. **Token Rotation**: Implementa refresh token
3. **Logging**: Log tentativi di accesso falliti
4. **Monitoring**: Alert su spike di 401/403

## 📚 Documentazione

- [CACHING_WARNING.md](./CACHING_WARNING.md) - ⚠️ Guida critica al caching
- [../LAMBDA_AUTHORIZER_GUIDE.md](../LAMBDA_AUTHORIZER_GUIDE.md) - Guida completa
- [AWS Lambda Authorizers](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-use-lambda-authorizer.html)
- [cargo-lambda](https://www.cargo-lambda.info/)

## 🎯 Quick Start

```bash
# 1. Build
cd authorizer
cargo lambda build --release --arm64

# 2. Test locale
cargo lambda invoke authorizer --data-file test-event.json

# 3. Deploy
cargo lambda deploy authorizer

# 4. Configure API Gateway
# Vedi sezione "Configurazione" sopra

# 5. Test
curl -H "Authorization: Bearer token" https://your-api.com/endpoint
```

## 💡 Tips

- **Cold Start**: ~200ms, warm: ~50ms
- **Memory**: 128MB sufficiente
- **Timeout**: 5 secondi raccomandato
- **Costo**: ~$0.0000002 per invocazione
- **Cache**: Riduce costi del 90-99%

## 🚀 Performance

```
Senza cache:
- 1000 req/min = 1000 invocazioni
- Costo: $0.012/ora
- Latenza: 50-200ms

Con cache (TTL 1h):
- 1000 req/min = 100 invocazioni (una per utente)
- Costo: $0.00002/ora
- Latenza: 5-10ms (cached)
- Risparmio: 99%!
```
