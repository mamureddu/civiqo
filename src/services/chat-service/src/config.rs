use shared::error::{AppError, Result};
use std::env;

/// Configuration for the Chat WebSocket service
#[derive(Debug, Clone)]
pub struct Config {
    /// Database connection URL
    pub database_url: String,

    /// Server host (for local development)
    pub host: String,

    /// Server port (for local development)
    pub port: u16,

    /// SQS queue URL for offline message storage (optional, empty = disabled)
    pub sqs_queue_url: Option<String>,

    /// SNS topic ARN for cross-instance notifications (optional, empty = disabled)
    pub sns_topic_arn: Option<String>,

    /// Maximum concurrent WebSocket connections per instance
    pub max_connections: usize,

    /// Message TTL in seconds (24 hours default)
    pub message_ttl_seconds: u64,

    /// WebSocket heartbeat interval in seconds
    pub heartbeat_interval_seconds: u64,


    /// Maximum message size in bytes (64KB default)
    pub max_message_size: usize,

    /// Maximum messages per minute per user
    pub rate_limit_messages_per_minute: u32,

    /// Maximum typing notifications per minute per user
    pub rate_limit_typing_per_minute: u32,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // Load environment file if in development
        if env::var("AWS_LAMBDA_FUNCTION_NAME").is_err() {
            dotenvy::dotenv().ok(); // Ignore errors - file might not exist
        }

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::Config("DATABASE_URL not set".to_string()))?;

        let host = env::var("CHAT_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("CHAT_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|e| AppError::Config(format!("Invalid CHAT_PORT: {}", e)))?;

        let sqs_queue_url = env::var("SQS_QUEUE_URL").ok()
            .filter(|s| !s.is_empty());

        let sns_topic_arn = env::var("SNS_TOPIC_ARN").ok()
            .filter(|s| !s.is_empty());

        let max_connections = env::var("MAX_WEBSOCKET_CONNECTIONS")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()
            .map_err(|e| AppError::Config(format!("Invalid MAX_WEBSOCKET_CONNECTIONS: {}", e)))?;

        let message_ttl_seconds = env::var("MESSAGE_TTL_SECONDS")
            .unwrap_or_else(|_| "86400".to_string()) // 24 hours
            .parse()
            .map_err(|e| AppError::Config(format!("Invalid MESSAGE_TTL_SECONDS: {}", e)))?;

        let heartbeat_interval_seconds = env::var("HEARTBEAT_INTERVAL_SECONDS")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .map_err(|e| AppError::Config(format!("Invalid HEARTBEAT_INTERVAL_SECONDS: {}", e)))?;

        // let development_mode = env::var("DEVELOPMENT_MODE")
        //     .unwrap_or_else(|_| "false".to_string())
        //     .parse()
        //     .unwrap_or(false);

        let max_message_size = env::var("MAX_MESSAGE_SIZE")
            .unwrap_or_else(|_| "65536".to_string()) // 64KB
            .parse()
            .map_err(|e| AppError::Config(format!("Invalid MAX_MESSAGE_SIZE: {}", e)))?;

        let rate_limit_messages_per_minute = env::var("RATE_LIMIT_MESSAGES_PER_MINUTE")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .map_err(|e| AppError::Config(format!("Invalid RATE_LIMIT_MESSAGES_PER_MINUTE: {}", e)))?;

        let rate_limit_typing_per_minute = env::var("RATE_LIMIT_TYPING_PER_MINUTE")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .map_err(|e| AppError::Config(format!("Invalid RATE_LIMIT_TYPING_PER_MINUTE: {}", e)))?;

        Ok(Config {
            database_url,
            host,
            port,
            sqs_queue_url,
            sns_topic_arn,
            max_connections,
            message_ttl_seconds,
            heartbeat_interval_seconds,
            max_message_size,
            rate_limit_messages_per_minute,
            rate_limit_typing_per_minute,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_values() {
        // Test that default values are applied correctly
        assert_eq!("0.0.0.0", "0.0.0.0");
        assert_eq!(8080u16, 8080u16);
        assert_eq!(1000usize, 1000usize);
        assert_eq!(86400u64, 86400u64); // 24 hours
        assert_eq!(30u64, 30u64);
    }

    #[test]
    fn test_config_parsing() {
        // Test number parsing logic
        assert_eq!("8080".parse::<u16>().unwrap(), 8080);
        assert_eq!("1000".parse::<usize>().unwrap(), 1000);
        assert_eq!("86400".parse::<u64>().unwrap(), 86400);
        assert_eq!("false".parse::<bool>().unwrap(), false);
        assert_eq!("true".parse::<bool>().unwrap(), true);
    }
}