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
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(AppError::Auth("test".to_string()), 401, "AUTH_ERROR"; "auth error")]
    #[test_case(AppError::Authorization("test".to_string()), 403, "AUTHORIZATION_ERROR"; "authorization error")]
    #[test_case(AppError::NotFound("test".to_string()), 404, "NOT_FOUND"; "not found error")]
    #[test_case(AppError::Validation("test".to_string()), 400, "VALIDATION_ERROR"; "validation error")]
    #[test_case(AppError::Conflict("test".to_string()), 400, "CONFLICT"; "conflict error")]
    #[test_case(AppError::Serialization(serde_json::Error::from(serde_json::de::Error::custom("test"))), 422, "SERIALIZATION_ERROR"; "serialization error")]
    #[test_case(AppError::ExternalService("test".to_string()), 502, "EXTERNAL_SERVICE_ERROR"; "external service error")]
    #[test_case(AppError::Config("test".to_string()), 500, "CONFIG_ERROR"; "config error")]
    #[test_case(AppError::Crypto("test".to_string()), 500, "CRYPTO_ERROR"; "crypto error")]
    #[test_case(AppError::Internal(anyhow::Error::msg("test")), 500, "INTERNAL_ERROR"; "internal error")]
    fn test_error_status_codes_and_codes(error: AppError, expected_status: u16, expected_code: &str) {
        assert_eq!(error.status_code(), expected_status);
        assert_eq!(error.error_code(), expected_code);
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
        assert_eq!(auth_error.to_string(), "Authentication error: Invalid token");

        let not_found_error = AppError::NotFound("User not found".to_string());
        assert_eq!(not_found_error.to_string(), "Not found: User not found");

        let validation_error = AppError::Validation("Email is required".to_string());
        assert_eq!(validation_error.to_string(), "Validation error: Email is required");
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