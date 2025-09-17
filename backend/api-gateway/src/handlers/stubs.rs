// Temporary stub implementations to get compilation working
use axum::{extract::State, response::Json};
use serde_json::json;
use shared::error::Result;
use crate::AppState;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "community-manager-api",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn root() -> Json<serde_json::Value> {
    Json(json!({
        "service": "Community Manager API",
        "version": "0.1.0",
        "status": "running"
    }))
}
