use std::sync::Arc;

use shared::{
    auth::JwtService,
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

    /// JWT authentication service
    pub jwt_service: JwtService,

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
        jwt_service: JwtService,
    ) -> Self {
        // Create message router (without AWS clients — local mode)
        let message_router = Arc::new(MessageRouter::new(
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
            message_router.clone(),
            config.max_connections,
            config.heartbeat_interval_seconds,
        ));

        Self {
            database,
            config,
            jwt_service,
            connection_manager,
            message_router,
            message_validator,
            rate_limiter,
        }
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn jwt_service(&self) -> &JwtService {
        &self.jwt_service
    }

    pub fn connection_manager(&self) -> &Arc<ConnectionManager> {
        &self.connection_manager
    }

    pub fn message_router(&self) -> &Arc<MessageRouter> {
        &self.message_router
    }

    pub fn message_validator(&self) -> &Arc<MessageValidator> {
        &self.message_validator
    }

    pub fn rate_limiter(&self) -> &Arc<RateLimiter> {
        &self.rate_limiter
    }
}
