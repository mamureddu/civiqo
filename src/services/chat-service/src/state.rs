use std::sync::Arc;

use shared::{
    auth::AuthState,
    database::Database,
};

use crate::{
    config::Config,
    services::{
        connection_manager::ConnectionManager,
        message_router::MessageRouter,
        message_validator::MessageValidator,
        rate_limiter::RateLimiter,
    },
};

/// Shared application state for the chat service
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub database: Database,

    /// Service configuration
    pub config: Config,

    /// Auth0 authentication state
    pub auth_state: AuthState,

    /// WebSocket connection manager
    pub connection_manager: Arc<ConnectionManager>,

    /// Message routing service
    pub message_router: Arc<MessageRouter>,

    /// Message validation service
    pub message_validator: Arc<MessageValidator>,

    /// Rate limiting service
    pub rate_limiter: Arc<RateLimiter>,
}

impl AppState {
    /// Create new application state
    pub fn new(
        database: Database,
        config: Config,
        auth_state: AuthState,
        sqs_client: aws_sdk_sqs::Client,
        sns_client: aws_sdk_sns::Client,
    ) -> Self {
        // Create message router
        let message_router = Arc::new(MessageRouter::new(
            sqs_client,
            sns_client,
            config.sqs_queue_url.clone(),
            config.sns_topic_arn.clone(),
            config.message_ttl_seconds,
        ));

        // Create message validator
        let message_validator = Arc::new(MessageValidator::new(config.max_message_size));

        // Create rate limiter
        let rate_limiter = Arc::new(RateLimiter::new(
            config.rate_limit_messages_per_minute,
            config.rate_limit_typing_per_minute,
        ));

        // Create connection manager
        let connection_manager = Arc::new(ConnectionManager::new(
            // database.clone(),  // Uncomment when implementing persistent storage
            message_router.clone(),
            config.max_connections,
            config.heartbeat_interval_seconds,
        ));

        Self {
            database,
            config,
            auth_state,
            connection_manager,
            message_router,
            message_validator,
            rate_limiter,
        }
    }

    /// Get database reference
    pub fn database(&self) -> &Database {
        &self.database
    }

    /// Get configuration reference
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get auth state reference
    pub fn auth_state(&self) -> &AuthState {
        &self.auth_state
    }

    /// Get connection manager reference
    pub fn connection_manager(&self) -> &Arc<ConnectionManager> {
        &self.connection_manager
    }

    /// Get message router reference
    pub fn message_router(&self) -> &Arc<MessageRouter> {
        &self.message_router
    }

    /// Get message validator reference
    pub fn message_validator(&self) -> &Arc<MessageValidator> {
        &self.message_validator
    }

    /// Get rate limiter reference
    pub fn rate_limiter(&self) -> &Arc<RateLimiter> {
        &self.rate_limiter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_getters() {
        // Test that getter methods exist and have correct return types
        // This is a compile-time test to ensure the interface is correct

        // Mock values for testing compilation
        let _database_ref: fn(&AppState) -> &Database = AppState::database;
        let _config_ref: fn(&AppState) -> &Config = AppState::config;
        let _auth_ref: fn(&AppState) -> &AuthState = AppState::auth_state;
        let _conn_ref: fn(&AppState) -> &Arc<ConnectionManager> = AppState::connection_manager;
        let _router_ref: fn(&AppState) -> &Arc<MessageRouter> = AppState::message_router;
    }
}