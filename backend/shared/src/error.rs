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