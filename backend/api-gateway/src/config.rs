use serde::Deserialize;
use shared::error::{AppError, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub cors_origins: String,
    pub development_mode: bool,
    pub s3_bucket: String,
    pub s3_region: String,
    pub aws_region: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| AppError::Config("DATABASE_URL not set".to_string()))?,
            cors_origins: std::env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            development_mode: std::env::var("DEVELOPMENT_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            s3_bucket: std::env::var("S3_BUCKET")
                .map_err(|_| AppError::Config("S3_BUCKET not set".to_string()))?,
            s3_region: std::env::var("S3_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
            aws_region: std::env::var("AWS_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }

    pub fn from_test() -> Self {
        Self {
            database_url: "postgresql://test:test@localhost:5433/test".to_string(),
            cors_origins: "http://localhost:3000".to_string(),
            development_mode: true,
            s3_bucket: "test-bucket".to_string(),
            s3_region: "us-east-1".to_string(),
            aws_region: "us-east-1".to_string(),
            log_level: "debug".to_string(),
        }
    }
}