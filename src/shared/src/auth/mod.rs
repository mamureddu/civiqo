use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use crate::models::{Claims, CommunityRole};
use crate::error::{AppError, Result};

/// JWT configuration for self-issued tokens
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub expiry_hours: u64,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            secret: std::env::var("JWT_SECRET")
                .map_err(|_| AppError::Config("JWT_SECRET not set (min 32 bytes)".to_string()))?,
            issuer: std::env::var("JWT_ISSUER")
                .unwrap_or_else(|_| "civiqo".to_string()),
            expiry_hours: std::env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
        })
    }
}

/// JWT service for issuing and validating tokens (HS256)
#[derive(Clone)]
pub struct JwtService {
    config: JwtConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl std::fmt::Debug for JwtService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtService")
            .field("config", &self.config)
            .finish()
    }
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.secret.as_bytes());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&config.issuer]);
        validation.set_audience(&["civiqo-api"]);

        Self {
            config,
            encoding_key,
            decoding_key,
            validation,
        }
    }

    /// Issue a new JWT token for a user
    pub fn issue_token(
        &self,
        user_id: uuid::Uuid,
        email: &str,
        name: Option<&str>,
        community_roles: Vec<CommunityRole>,
    ) -> Result<String> {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::hours(self.config.expiry_hours as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            aud: "civiqo-api".to_string(),
            iss: self.config.issuer.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            email: Some(email.to_string()),
            name: name.map(|n| n.to_string()),
            community_roles,
        };

        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| AppError::Auth(format!("Failed to issue JWT: {}", e)))
    }

    /// Validate a JWT token and return its claims
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|e| AppError::Auth(format!("Invalid JWT: {}", e)))?;
        Ok(token_data.claims)
    }

    /// Refresh a token — issue a new one with updated expiry
    pub fn refresh_token(&self, claims: &Claims) -> Result<String> {
        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))?;

        self.issue_token(
            user_id,
            claims.email.as_deref().unwrap_or(""),
            claims.name.as_deref(),
            claims.community_roles.clone(),
        )
    }

    /// Get the configured expiry in seconds (for API responses)
    pub fn expiry_seconds(&self) -> u64 {
        self.config.expiry_hours * 3600
    }
}

/// Extract token from Authorization header
pub fn extract_bearer_token(auth_header: &str) -> Result<&str> {
    if auth_header.starts_with("Bearer ") {
        Ok(&auth_header[7..])
    } else {
        Err(AppError::Auth("Invalid Authorization header format".to_string()))
    }
}

/// Check if user has required role in community
pub fn has_community_role(claims: &Claims, community_id: uuid::Uuid, required_role: &str) -> bool {
    claims.community_roles.iter().any(|role| {
        role.community_id == community_id && role.role == required_role
    })
}

/// Check if user has any of the required permissions
pub fn has_permission(claims: &Claims, community_id: uuid::Uuid, permission: &str) -> bool {
    claims.community_roles.iter().any(|role| {
        role.community_id == community_id && role.permissions.contains(&permission.to_string())
    })
}

/// Authenticated user extracted from JWT
#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
    pub email: Option<String>,
    pub name: Option<String>,
    pub claims: Claims,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CommunityRole;
    use rstest::*;

    #[fixture]
    fn jwt_config() -> JwtConfig {
        JwtConfig {
            secret: "test-secret-that-is-at-least-32-bytes-long".to_string(),
            issuer: "civiqo".to_string(),
            expiry_hours: 24,
        }
    }

    #[fixture]
    fn jwt_service(jwt_config: JwtConfig) -> JwtService {
        JwtService::new(jwt_config)
    }

    #[rstest]
    fn test_issue_and_validate_token(jwt_service: JwtService) {
        let user_id = uuid::Uuid::new_v4();
        let email = "test@example.com";
        let name = Some("Test User");
        let roles = vec![CommunityRole {
            community_id: uuid::Uuid::new_v4(),
            role: "admin".to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
        }];

        let token = jwt_service.issue_token(user_id, email, name, roles.clone())
            .expect("Should issue token");

        let claims = jwt_service.validate_token(&token)
            .expect("Should validate token");

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, Some(email.to_string()));
        assert_eq!(claims.name, Some("Test User".to_string()));
        assert_eq!(claims.iss, "civiqo");
        assert_eq!(claims.aud, "civiqo-api");
        assert_eq!(claims.community_roles.len(), 1);
        assert_eq!(claims.community_roles[0].role, "admin");
    }

    #[rstest]
    fn test_validate_invalid_token(jwt_service: JwtService) {
        let result = jwt_service.validate_token("invalid-token");
        assert!(result.is_err());
    }

    #[rstest]
    fn test_validate_token_wrong_secret(jwt_service: JwtService) {
        let user_id = uuid::Uuid::new_v4();
        let token = jwt_service.issue_token(user_id, "test@example.com", None, vec![])
            .expect("Should issue token");

        let wrong_config = JwtConfig {
            secret: "completely-different-secret-that-is-32-bytes".to_string(),
            issuer: "civiqo".to_string(),
            expiry_hours: 24,
        };
        let wrong_service = JwtService::new(wrong_config);

        let result = wrong_service.validate_token(&token);
        assert!(result.is_err());
    }

    #[rstest]
    fn test_refresh_token(jwt_service: JwtService) {
        let user_id = uuid::Uuid::new_v4();
        let token = jwt_service.issue_token(user_id, "test@example.com", Some("Test"), vec![])
            .expect("Should issue token");

        let claims = jwt_service.validate_token(&token).expect("Should validate");
        let refreshed = jwt_service.refresh_token(&claims).expect("Should refresh");

        let new_claims = jwt_service.validate_token(&refreshed).expect("Should validate refreshed");
        assert_eq!(new_claims.sub, user_id.to_string());
        assert!(new_claims.iat >= claims.iat);
    }

    #[rstest]
    fn test_expiry_seconds(jwt_service: JwtService) {
        assert_eq!(jwt_service.expiry_seconds(), 24 * 3600);
    }

    #[test]
    fn test_extract_bearer_token_success() {
        let result = extract_bearer_token("Bearer token123").expect("Should extract token");
        assert_eq!(result, "token123");
    }

    #[test]
    fn test_extract_bearer_token_failure() {
        assert!(extract_bearer_token("Invalid token123").is_err());
        assert!(extract_bearer_token("Bear token123").is_err());
        assert!(extract_bearer_token("").is_err());
    }

    #[test]
    fn test_has_community_role() {
        let community_id = uuid::Uuid::new_v4();
        let claims = Claims {
            sub: uuid::Uuid::new_v4().to_string(),
            aud: "civiqo-api".to_string(),
            iss: "civiqo".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
            iat: chrono::Utc::now().timestamp(),
            email: Some("test@example.com".to_string()),
            name: None,
            community_roles: vec![CommunityRole {
                community_id,
                role: "admin".to_string(),
                permissions: vec!["read".to_string(), "write".to_string()],
            }],
        };

        assert!(has_community_role(&claims, community_id, "admin"));
        assert!(!has_community_role(&claims, community_id, "member"));
        assert!(!has_community_role(&claims, uuid::Uuid::new_v4(), "admin"));
    }

    #[test]
    fn test_has_permission() {
        let community_id = uuid::Uuid::new_v4();
        let claims = Claims {
            sub: uuid::Uuid::new_v4().to_string(),
            aud: "civiqo-api".to_string(),
            iss: "civiqo".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
            iat: chrono::Utc::now().timestamp(),
            email: None,
            name: None,
            community_roles: vec![CommunityRole {
                community_id,
                role: "admin".to_string(),
                permissions: vec!["read".to_string(), "write".to_string()],
            }],
        };

        assert!(has_permission(&claims, community_id, "read"));
        assert!(has_permission(&claims, community_id, "write"));
        assert!(!has_permission(&claims, community_id, "delete"));
        assert!(!has_permission(&claims, uuid::Uuid::new_v4(), "read"));
    }
}
