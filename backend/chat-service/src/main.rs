use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::Response,
    routing::get,
    Router,
};
use shared::{
    auth::AuthState,
    database::Database,
    error::{AppError, Result},
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, warn};

mod config;
mod handlers;
mod middleware;
mod services;
mod state;

use config::Config;
use handlers::websocket::websocket_handler;
use state::AppState;

/// Health check endpoint
async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Create the application router
fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ws", get(websocket_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()) // TODO: Restrict in production
        )
        .with_state(state)
}

/// Main application entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "chat_service=debug,shared=debug".into()),
        )
        .init();

    info!("Starting Community Manager Chat Service");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded successfully");

    // Initialize database connection
    let database = Database::connect(&config.database_url).await?;
    info!("Database connection established");

    // Initialize AWS clients
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest()).load().await;
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);
    let sns_client = aws_sdk_sns::Client::new(&aws_config);
    info!("AWS clients initialized");

    // Initialize Auth state
    let auth0_config = shared::auth::Auth0Config::from_env()
        .map_err(|e| AppError::Config(format!("Failed to load Auth0 config: {}", e)))?;

    let auth_state = AuthState::new(&auth0_config);
    info!("Auth0 state initialized");

    // Create application state
    let app_state = AppState::new(
        database,
        config.clone(),
        auth_state,
        sqs_client,
        sns_client,
    );

    // Build the application
    let app = create_app(app_state);

    // Check if running in Lambda environment
    if std::env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok() {
        info!("Running in AWS Lambda environment");
        // Lambda runtime setup would go here
        // For now, we'll implement local development mode
        todo!("Lambda runtime not yet implemented")
    } else {
        // Local development mode
        let bind_addr = format!("{}:{}", config.host, config.port);
        info!("Starting server on {}", bind_addr);

        let listener = TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| AppError::Config(format!("Failed to bind to {}: {}", bind_addr, e)))?;

        info!("Chat service listening on {}", bind_addr);

        axum::serve(listener, app)
            .await
            .map_err(|e| AppError::Config(format!("Server error: {}", e)))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, StatusCode::OK);
    }
}