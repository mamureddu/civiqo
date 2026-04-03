// Temporary stub implementations to get compilation working
use axum::response::Json;
use serde_json::json;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "community-manager-api",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
