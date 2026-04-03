use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::Auth(_) => 401,
            Self::Authorization(_) => 403,
            Self::NotFound(_) => 404,
            Self::Validation(_) | Self::Conflict(_) => 400,
            Self::Database(_) | Self::Internal(_) => 500,
            Self::Serialization(_) => 422,
            Self::ExternalService(_) => 502,
            Self::Config(_) => 500,
            Self::Crypto(_) => 500,
            Self::RateLimit(_) => 429,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Auth(_) => "AUTH_ERROR",
            Self::Authorization(_) => "AUTHORIZATION_ERROR",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::Conflict(_) => "CONFLICT",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Serialization(_) => "SERIALIZATION_ERROR",
            Self::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            Self::Config(_) => "CONFIG_ERROR",
            Self::Crypto(_) => "CRYPTO_ERROR",
            Self::RateLimit(_) => "RATE_LIMIT_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = Json(json!({
            "success": false,
            "error": {
                "code": self.error_code(),
                "message": self.to_string()
            }
        }));

        (status_code, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes_and_codes() {
        // Test auth error
        let error = AppError::Auth("test".to_string());
        assert_eq!(error.status_code(), 401);
        assert_eq!(error.error_code(), "AUTH_ERROR");

        // Test authorization error
        let error = AppError::Authorization("test".to_string());
        assert_eq!(error.status_code(), 403);
        assert_eq!(error.error_code(), "AUTHORIZATION_ERROR");

        // Test not found error
        let error = AppError::NotFound("test".to_string());
        assert_eq!(error.status_code(), 404);
        assert_eq!(error.error_code(), "NOT_FOUND");

        // Test validation error
        let error = AppError::Validation("test".to_string());
        assert_eq!(error.status_code(), 400);
        assert_eq!(error.error_code(), "VALIDATION_ERROR");

        // Test conflict error
        let error = AppError::Conflict("test".to_string());
        assert_eq!(error.status_code(), 400);
        assert_eq!(error.error_code(), "CONFLICT");

        // Test external service error
        let error = AppError::ExternalService("test".to_string());
        assert_eq!(error.status_code(), 502);
        assert_eq!(error.error_code(), "EXTERNAL_SERVICE_ERROR");

        // Test config error
        let error = AppError::Config("test".to_string());
        assert_eq!(error.status_code(), 500);
        assert_eq!(error.error_code(), "CONFIG_ERROR");

        // Test crypto error
        let error = AppError::Crypto("test".to_string());
        assert_eq!(error.status_code(), 500);
        assert_eq!(error.error_code(), "CRYPTO_ERROR");

        // Test internal error
        let error = AppError::Internal(anyhow::Error::msg("test"));
        assert_eq!(error.status_code(), 500);
        assert_eq!(error.error_code(), "INTERNAL_ERROR");
    }

    #[test]
    fn test_database_error_conversion() {
        let sql_error = sqlx::Error::RowNotFound;
        let app_error = AppError::from(sql_error);

        assert_eq!(app_error.status_code(), 500);
        assert_eq!(app_error.error_code(), "DATABASE_ERROR");

        match app_error {
            AppError::Database(_) => (),
            _ => panic!("Expected Database error"),
        }
    }

    #[test]
    fn test_serde_error_conversion() {
        let json_str = r#"{"invalid": json}"#;
        let serde_error = serde_json::from_str::<serde_json::Value>(json_str);

        if let Err(err) = serde_error {
            let app_error = AppError::from(err);

            assert_eq!(app_error.status_code(), 422);
            assert_eq!(app_error.error_code(), "SERIALIZATION_ERROR");

            match app_error {
                AppError::Serialization(_) => (),
                _ => panic!("Expected Serialization error"),
            }
        }
    }

    #[test]
    fn test_anyhow_error_conversion() {
        let anyhow_error = anyhow::Error::msg("Test internal error");
        let app_error = AppError::from(anyhow_error);

        assert_eq!(app_error.status_code(), 500);
        assert_eq!(app_error.error_code(), "INTERNAL_ERROR");

        match app_error {
            AppError::Internal(_) => (),
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_error_display() {
        let auth_error = AppError::Auth("Invalid token".to_string());
        assert_eq!(
            auth_error.to_string(),
            "Authentication error: Invalid token"
        );

        let not_found_error = AppError::NotFound("User not found".to_string());
        assert_eq!(not_found_error.to_string(), "Not found: User not found");

        let validation_error = AppError::Validation("Email is required".to_string());
        assert_eq!(
            validation_error.to_string(),
            "Validation error: Email is required"
        );
    }

    #[test]
    fn test_error_debug() {
        let config_error = AppError::Config("Missing DATABASE_URL".to_string());
        let debug_str = format!("{:?}", config_error);

        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("Missing DATABASE_URL"));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_result() -> Result<String> {
            Ok("success".to_string())
        }

        let result = returns_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_result_error() {
        fn returns_error() -> Result<String> {
            Err(AppError::NotFound("Resource not found".to_string()))
        }

        let result = returns_error();
        assert!(result.is_err());

        if let Err(AppError::NotFound(msg)) = result {
            assert_eq!(msg, "Resource not found");
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[test]
    fn test_rate_limit_error_handling() {
        let rate_limit_error = AppError::RateLimit("Message rate limit exceeded".to_string());

        // Test specific rate limit error properties
        assert_eq!(rate_limit_error.status_code(), 429);
        assert_eq!(rate_limit_error.error_code(), "RATE_LIMIT_ERROR");

        let error_msg = rate_limit_error.to_string();
        assert!(error_msg.contains("Rate limit exceeded"));
        assert!(error_msg.contains("Message rate limit exceeded"));
    }

    #[test]
    fn test_error_chaining() {
        // Test that we can chain errors properly
        let original_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let anyhow_error = anyhow::Error::from(original_error);
        let app_error = AppError::from(anyhow_error);

        match app_error {
            AppError::Internal(inner) => {
                let error_chain = format!("{:?}", inner);
                assert!(error_chain.contains("File not found"));
            }
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_rate_limit_error() {
        let error = AppError::RateLimit("Too many requests".to_string());
        assert_eq!(error.status_code(), 429);
        assert_eq!(error.error_code(), "RATE_LIMIT_ERROR");
        assert_eq!(error.to_string(), "Rate limit exceeded: Too many requests");
    }

    #[test]
    fn test_custom_error_messages() {
        let errors = vec![
            AppError::Auth("JWT expired".to_string()),
            AppError::Authorization("Insufficient permissions".to_string()),
            AppError::NotFound("User ID 123 not found".to_string()),
            AppError::Validation("Invalid email format".to_string()),
            AppError::Conflict("Username already exists".to_string()),
            AppError::ExternalService("Auth0 API unavailable".to_string()),
            AppError::Config("Missing S3_BUCKET environment variable".to_string()),
            AppError::Crypto("Key generation failed".to_string()),
            AppError::RateLimit("Rate limit exceeded".to_string()),
        ];

        for error in errors {
            let status = error.status_code();
            let code = error.error_code();
            let message = error.to_string();

            // All should have valid HTTP status codes
            assert!(status >= 400 && status < 600);

            // All should have non-empty error codes
            assert!(!code.is_empty());

            // All should have meaningful messages
            assert!(!message.is_empty());

            println!("Error: {} -> Status: {}, Code: {}", message, status, code);
        }
    }
}
