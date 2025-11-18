// lib.rs - Library interface for testing
use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use shared::{database::Database, auth::Auth0Config};

// Re-export modules
pub mod config;
pub mod handlers;
// pub mod middleware; // Disabled - using tower-sessions instead

// Re-export main types from main.rs
pub use config::Config;

pub type AppState = Arc<ApiState>;

pub struct ApiState {
    pub db: Database,
    pub config: Config,
    pub auth_config: Auth0Config,
}

/// Create a simple application router for testing
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .with_state(state)
}