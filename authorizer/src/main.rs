use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Lambda Authorizer Event from API Gateway
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthorizerEvent {
    #[serde(rename = "type")]
    event_type: String,
    method_arn: String,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    query_string_parameters: HashMap<String, String>,
    #[serde(default)]
    authorization_token: Option<String>, // For TOKEN authorizers
}

/// IAM Policy Statement
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PolicyStatement {
    action: String,
    effect: String, // "Allow" or "Deny"
    resource: String,
}

/// IAM Policy Document
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PolicyDocument {
    version: String,
    statement: Vec<PolicyStatement>,
}

/// Lambda Authorizer Response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthorizerResponse {
    principal_id: String,
    policy_document: PolicyDocument,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<HashMap<String, Value>>,
}

/// User context to inject into request
/// This data will be available in ALL backend Lambda functions
/// without needing to call Auth0 or database again!
#[derive(Debug, Serialize, Clone)]
struct UserContext {
    // Identity
    user_id: String,
    email: String,
    username: String,
    
    // Authorization
    roles: Vec<String>,
    permissions: Vec<String>,
    
    // Profile (optional)
    picture: Option<String>,
    name: Option<String>,
    
    // Metadata
    email_verified: bool,
    created_at: Option<String>,
    last_login: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(event: LambdaEvent<AuthorizerEvent>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    
    tracing::info!("Authorizer invoked for method: {}", event.method_arn);
    
    // Extract token from Authorization header or query parameter
    let token = extract_token(&event)?;
    
    // Validate token and get user info
    match validate_token(&token).await {
        Ok(user_info) => {
            let user_id = user_info.user_id.clone();
            tracing::info!("Token valid for user: {}", user_id);
            
            // Generate Allow policy
            let response = generate_policy(
                &user_id,
                "Allow",
                &event.method_arn,
                Some(user_info),
            );
            
            Ok(serde_json::to_value(response)?)
        }
        Err(e) => {
            tracing::warn!("Token validation failed: {}", e);
            
            // Generate Deny policy
            let response = generate_policy(
                "unauthorized",
                "Deny",
                &event.method_arn,
                None,
            );
            
            Ok(serde_json::to_value(response)?)
        }
    }
}

/// Extract token from request
fn extract_token(event: &AuthorizerEvent) -> Result<String, Error> {
    // Try Authorization header first (TOKEN authorizer)
    if let Some(token) = &event.authorization_token {
        return Ok(token.clone());
    }
    
    // Try Authorization header from headers map (REQUEST authorizer)
    if let Some(auth_header) = event.headers.get("authorization")
        .or_else(|| event.headers.get("Authorization"))
    {
        // Remove "Bearer " prefix if present
        let token = auth_header
            .strip_prefix("Bearer ")
            .unwrap_or(auth_header);
        return Ok(token.to_string());
    }
    
    // Try query parameter
    if let Some(token) = event.query_string_parameters.get("token") {
        return Ok(token.clone());
    }
    
    Err("No authorization token found".into())
}

/// Validate token and return user info with ALL user data
/// This is called ONCE per user (then cached), so it's OK to fetch everything
async fn validate_token(token: &str) -> Result<UserContext, Error> {
    // STRATEGY 1: JWT Validation (fastest - no external calls)
    // Decode and validate JWT locally
    if let Ok(user_context) = validate_jwt_token(token).await {
        return Ok(user_context);
    }
    
    // STRATEGY 2: Auth0 /userinfo (if JWT validation fails or for OAuth tokens)
    if let Ok(user_context) = fetch_auth0_userinfo(token).await {
        return Ok(user_context);
    }
    
    // STRATEGY 3: Database lookup (for custom session tokens)
    if let Ok(user_context) = fetch_user_from_database(token).await {
        return Ok(user_context);
    }
    
    Err("Invalid token".into())
}

/// Validate JWT token locally (fastest - no network calls)
async fn validate_jwt_token(token: &str) -> Result<UserContext, Error> {
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    
    // TODO: Load from environment
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    
    #[derive(Debug, serde::Deserialize)]
    struct Claims {
        sub: String,              // user_id
        email: String,
        name: Option<String>,
        picture: Option<String>,
        email_verified: Option<bool>,
        #[serde(default)]
        roles: Vec<String>,
        #[serde(default)]
        permissions: Vec<String>,
        exp: usize,
        iat: Option<usize>,
    }
    
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    
    let claims = token_data.claims;
    
    // Extract username from email or name
    let username = claims.name.clone()
        .or_else(|| claims.email.split('@').next().map(String::from))
        .unwrap_or_else(|| "user".to_string());
    
    Ok(UserContext {
        user_id: claims.sub,
        email: claims.email,
        username,
        roles: claims.roles,
        permissions: claims.permissions,
        picture: claims.picture,
        name: claims.name,
        email_verified: claims.email_verified.unwrap_or(false),
        created_at: claims.iat.map(|iat| {
            chrono::DateTime::from_timestamp(iat as i64, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default()
        }),
        last_login: Some(chrono::Utc::now().to_rfc3339()),
    })
}

/// Fetch user info from Auth0 /userinfo endpoint
async fn fetch_auth0_userinfo(token: &str) -> Result<UserContext, Error> {
    let auth0_domain = std::env::var("AUTH0_DOMAIN")
        .unwrap_or_else(|_| "your-tenant.auth0.com".to_string());
    
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://{}/userinfo", auth0_domain))
        .bearer_auth(token)
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err("Auth0 userinfo failed".into());
    }
    
    #[derive(serde::Deserialize)]
    struct Auth0UserInfo {
        sub: String,
        email: String,
        name: Option<String>,
        picture: Option<String>,
        email_verified: Option<bool>,
        #[serde(default)]
        #[serde(rename = "https://myapp.com/roles")]
        roles: Vec<String>,
        #[serde(default)]
        #[serde(rename = "https://myapp.com/permissions")]
        permissions: Vec<String>,
        created_at: Option<String>,
        updated_at: Option<String>,
    }
    
    let user_info: Auth0UserInfo = response.json().await?;
    
    let username = user_info.name.clone()
        .or_else(|| user_info.email.split('@').next().map(String::from))
        .unwrap_or_else(|| "user".to_string());
    
    Ok(UserContext {
        user_id: user_info.sub,
        email: user_info.email,
        username,
        roles: user_info.roles,
        permissions: user_info.permissions,
        picture: user_info.picture,
        name: user_info.name,
        email_verified: user_info.email_verified.unwrap_or(false),
        created_at: user_info.created_at,
        last_login: Some(chrono::Utc::now().to_rfc3339()),
    })
}

/// Fetch user from database (for custom session tokens)
async fn fetch_user_from_database(_token: &str) -> Result<UserContext, Error> {
    // TODO: Implement database lookup
    // This would query your database to get user info based on session token
    
    Err("Database lookup not implemented".into())
}

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

/// Generate IAM policy response
fn generate_policy(
    principal_id: &str,
    effect: &str,
    resource: &str,
    user_context: Option<UserContext>,
) -> AuthorizerResponse {
    // CRITICAL: Use wildcard for ALL routes to enable proper caching!
    // Format: arn:aws:execute-api:region:account-id:api-id/stage/*/*
    // Without wildcard, cache breaks when user calls different endpoints
    
    let wildcard_resource = extract_wildcard_resource(resource);
    
    let policy_document = PolicyDocument {
        version: "2012-10-17".to_string(),
        statement: vec![PolicyStatement {
            action: "execute-api:Invoke".to_string(),
            effect: effect.to_string(),
            resource: wildcard_resource,
        }],
    };
    
    // Convert user context to HashMap for API Gateway
    // ALL this data will be available in backend Lambda without additional calls!
    let context = user_context.map(|user| {
        let mut map = HashMap::new();
        
        // Identity (always present)
        map.insert("userId".to_string(), json!(user.user_id));
        map.insert("email".to_string(), json!(user.email));
        map.insert("username".to_string(), json!(user.username));
        
        // Authorization (always present)
        map.insert("roles".to_string(), json!(user.roles.join(",")));
        map.insert("permissions".to_string(), json!(user.permissions.join(",")));
        
        // Profile (optional)
        if let Some(picture) = user.picture {
            map.insert("picture".to_string(), json!(picture));
        }
        if let Some(name) = user.name {
            map.insert("name".to_string(), json!(name));
        }
        
        // Metadata
        map.insert("emailVerified".to_string(), json!(user.email_verified));
        if let Some(created_at) = user.created_at {
            map.insert("createdAt".to_string(), json!(created_at));
        }
        if let Some(last_login) = user.last_login {
            map.insert("lastLogin".to_string(), json!(last_login));
        }
        
        map
    });
    
    AuthorizerResponse {
        principal_id: principal_id.to_string(),
        policy_document,
        context,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_token_from_bearer() {
        let mut event = AuthorizerEvent {
            event_type: "TOKEN".to_string(),
            method_arn: "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/GET/users".to_string(),
            headers: HashMap::new(),
            query_string_parameters: HashMap::new(),
            authorization_token: None,
        };
        
        event.headers.insert("Authorization".to_string(), "Bearer test_token_123".to_string());
        
        let token = extract_token(&event).unwrap();
        assert_eq!(token, "test_token_123");
    }
    
    #[test]
    fn test_extract_wildcard_resource() {
        // Test normal ARN
        let resource = "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/GET/users";
        let wildcard = extract_wildcard_resource(resource);
        assert_eq!(wildcard, "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/*/*");
        
        // Test different method and path
        let resource2 = "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/POST/communities";
        let wildcard2 = extract_wildcard_resource(resource2);
        assert_eq!(wildcard2, "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/*/*");
        
        // Both should produce the SAME wildcard (critical for caching!)
        assert_eq!(wildcard, wildcard2);
    }
    
    #[test]
    fn test_generate_allow_policy() {
        let user = UserContext {
            user_id: "user-123".to_string(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            roles: vec!["admin".to_string()],
            permissions: vec!["read:posts".to_string()],
            picture: None,
            name: Some("Test User".to_string()),
            email_verified: true,
            created_at: None,
            last_login: None,
        };
        
        let response = generate_policy(
            "user-123",
            "Allow",
            "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/GET/users",
            Some(user),
        );
        
        assert_eq!(response.principal_id, "user-123");
        assert_eq!(response.policy_document.statement[0].effect, "Allow");
        
        // CRITICAL: Resource must be wildcard for caching!
        assert_eq!(
            response.policy_document.statement[0].resource,
            "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/*/*"
        );
        
        assert!(response.context.is_some());
    }
    
    #[test]
    fn test_policy_same_for_all_routes() {
        // Generate policy for /users endpoint
        let policy1 = generate_policy(
            "user-123",
            "Allow",
            "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/GET/users",
            None,
        );
        
        // Generate policy for /communities endpoint
        let policy2 = generate_policy(
            "user-123",
            "Allow",
            "arn:aws:execute-api:us-east-1:123456789012:abcdef123/prod/POST/communities",
            None,
        );
        
        // CRITICAL: Both policies must have the SAME resource (wildcard)
        // This ensures cache works across all endpoints!
        assert_eq!(
            policy1.policy_document.statement[0].resource,
            policy2.policy_document.statement[0].resource
        );
    }
}
