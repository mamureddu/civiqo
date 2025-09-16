use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use reqwest;
use std::collections::HashMap;
use crate::models::Claims;
use crate::error::{AppError, Result};

pub mod middleware;

#[derive(Debug, Clone)]
pub struct Auth0Config {
    pub domain: String,
    pub audience: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Auth0Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            domain: std::env::var("AUTH0_DOMAIN")
                .map_err(|_| AppError::Config("AUTH0_DOMAIN not set".to_string()))?,
            audience: std::env::var("AUTH0_AUDIENCE")
                .map_err(|_| AppError::Config("AUTH0_AUDIENCE not set".to_string()))?,
            client_id: std::env::var("AUTH0_CLIENT_ID")
                .map_err(|_| AppError::Config("AUTH0_CLIENT_ID not set".to_string()))?,
            client_secret: std::env::var("AUTH0_CLIENT_SECRET")
                .map_err(|_| AppError::Config("AUTH0_CLIENT_SECRET not set".to_string()))?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwk {
    pub kty: String,
    pub kid: String,
    pub r#use: Option<String>,
    pub n: String,
    pub e: String,
    pub x5c: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct JwtValidator {
    pub auth0_domain: String,
    pub audience: String,
    pub validation: Validation,
}

impl JwtValidator {
    pub fn new(config: &Auth0Config) -> Self {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&format!("https://{}/", config.domain)]);

        Self {
            auth0_domain: config.domain.clone(),
            audience: config.audience.clone(),
            validation,
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<Claims> {
        // Get the key ID from the token header
        let header = jsonwebtoken::decode_header(token)
            .map_err(|e| AppError::Auth(format!("Invalid JWT header: {}", e)))?;

        let kid = header.kid.ok_or_else(|| {
            AppError::Auth("JWT header missing key ID".to_string())
        })?;

        // Fetch JWKS from Auth0
        let jwks = self.fetch_jwks().await?;

        // Find the key with matching kid
        let jwk = jwks.keys
            .iter()
            .find(|key| key.kid == kid)
            .ok_or_else(|| AppError::Auth("Key not found in JWKS".to_string()))?;

        // Create decoding key from the JWK
        let decoding_key = self.jwk_to_decoding_key(jwk)?;

        // Validate the token
        let token_data = decode::<Claims>(token, &decoding_key, &self.validation)
            .map_err(|e| AppError::Auth(format!("Invalid JWT: {}", e)))?;

        Ok(token_data.claims)
    }

    async fn fetch_jwks(&self) -> Result<Jwks> {
        let jwks_url = format!("https://{}/.well-known/jwks.json", self.auth0_domain);

        let response = reqwest::get(&jwks_url)
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to fetch JWKS: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::ExternalService(
                "Failed to fetch JWKS from Auth0".to_string()
            ));
        }

        let jwks: Jwks = response
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Invalid JWKS response: {}", e)))?;

        Ok(jwks)
    }

    fn jwk_to_decoding_key(&self, jwk: &Jwk) -> Result<DecodingKey> {
        // For RSA keys, we need to construct the public key from n and e
        use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

        if jwk.kty != "RSA" {
            return Err(AppError::Auth("Only RSA keys are supported".to_string()));
        }

        // If x5c is present, use the first certificate
        if let Some(ref x5c) = jwk.x5c {
            if let Some(cert) = x5c.first() {
                let cert_der = URL_SAFE_NO_PAD.decode(cert)
                    .map_err(|e| AppError::Auth(format!("Invalid certificate encoding: {}", e)))?;

                return DecodingKey::from_rsa_der(&cert_der)
                    .map_err(|e| AppError::Auth(format!("Invalid RSA certificate: {}", e)));
            }
        }

        // Fallback: construct RSA public key from n and e parameters
        // This is more complex and requires additional cryptographic libraries
        // For now, return an error suggesting using x5c
        Err(AppError::Auth(
            "RSA key construction from n/e not implemented. Please use x5c in JWKS.".to_string()
        ))
    }

    // Development-only: simplified validation for testing
    #[cfg(feature = "development")]
    pub fn validate_token_dev(&self, token: &str, secret: &str) -> Result<Claims> {
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&self.audience]);

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| AppError::Auth(format!("Invalid JWT: {}", e)))?;

        Ok(token_data.claims)
    }
}

// Extract token from Authorization header
pub fn extract_bearer_token(auth_header: &str) -> Result<&str> {
    if auth_header.starts_with("Bearer ") {
        Ok(&auth_header[7..])
    } else {
        Err(AppError::Auth("Invalid Authorization header format".to_string()))
    }
}

// Check if user has required role in community
pub fn has_community_role(claims: &Claims, community_id: uuid::Uuid, required_role: &str) -> bool {
    claims.community_roles.iter().any(|role| {
        role.community_id == community_id && role.role == required_role
    })
}

// Check if user has any of the required permissions
pub fn has_permission(claims: &Claims, community_id: uuid::Uuid, permission: &str) -> bool {
    claims.community_roles.iter().any(|role| {
        role.community_id == community_id && role.permissions.contains(&permission.to_string())
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
    pub auth0_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub claims: Claims,
}