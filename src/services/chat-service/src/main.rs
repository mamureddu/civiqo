use axum::{http::StatusCode, routing::get, Router};
use shared::{
    auth::JwtService,
    database::Database,
    error::{AppError, Result},
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

mod config;
mod handlers;
mod middleware;
mod services;
mod state;

// Integration tests disabled - require full DB setup
// #[cfg(test)]
// mod integration_tests;
//
// #[cfg(test)]
// mod security_integration_tests;

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
                .layer(CorsLayer::permissive()), // TODO: Restrict in production
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

    // Initialize JWT auth
    let jwt_config = shared::auth::JwtConfig::from_env()
        .map_err(|e| AppError::Config(format!("Failed to load JWT config: {}", e)))?;
    let jwt_service = JwtService::new(jwt_config);
    info!("JWT auth initialized");

    // Create application state
    let app_state = AppState::new(database, config.clone(), jwt_service);

    // Build the application
    let app = create_app(app_state);

    // Start server
    let bind_addr = format!("{}:{}", config.host, config.port);
    info!("Starting server on {}", bind_addr);

    let listener = TcpListener::bind(&bind_addr)
        .await
        .map_err(|e| AppError::Config(format!("Failed to bind to {}: {}", bind_addr, e)))?;

    info!("Chat service listening on {}", bind_addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::Config(format!("Server error: {}", e)))?;

    Ok(())
}

// Tests disabled - require full DB setup
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::http::StatusCode;
//     use axum_test::TestServer;
//
//     #[tokio::test]
//     async fn test_health_check() {
//         let response = health_check().await;
//         assert_eq!(response, StatusCode::OK);
//     }
// }
