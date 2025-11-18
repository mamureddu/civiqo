use axum::{
    http::StatusCode,
    response::{Json, Response, IntoResponse},
};
use serde_json::json;
use shared::error::AppError;
use tracing::{error, warn};

pub async fn handle_error(err: AppError) -> Response {
    let (status, error_message, should_log_details) = match &err {
        AppError::Auth(_) => {
            (StatusCode::UNAUTHORIZED, "Authentication failed".to_string(), false)
        }
        AppError::Authorization(_) => {
            (StatusCode::FORBIDDEN, "Access denied".to_string(), false)
        }
        AppError::Validation(msg) => {
            (StatusCode::BAD_REQUEST, format!("Invalid input: {}", sanitize_error_message(msg)), false)
        }
        AppError::NotFound(_) => {
            (StatusCode::NOT_FOUND, "Resource not found".to_string(), false)
        }
        AppError::Conflict(_) => {
            (StatusCode::CONFLICT, "Resource conflict".to_string(), false)
        }
        AppError::Database(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".to_string(), true)
        }
        AppError::ExternalService(_) => {
            (StatusCode::BAD_GATEWAY, "External service unavailable".to_string(), true)
        }
        AppError::Config(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error".to_string(), true)
        }
        AppError::Internal(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string(), true)
        }
        AppError::Serialization(_) => {
            (StatusCode::UNPROCESSABLE_ENTITY, "Invalid request format".to_string(), false)
        }
        AppError::Crypto(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Cryptographic error".to_string(), true)
        }
        AppError::RateLimit(_) => {
            (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded".to_string(), false)
        }
    };

    // Log sensitive errors with full details, but only show generic messages to users
    if should_log_details {
        error!("Internal error: {:?}", err);
    } else {
        warn!("Client error: {}", error_message);
    }

    let body = Json(json!({
        "success": false,
        "error": error_message,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }));

    (status, body).into_response()
}

// Sanitize error messages to prevent information leakage
fn sanitize_error_message(msg: &str) -> String {
    // Remove potentially sensitive information from error messages
    let sanitized = msg
        .replace("SQLSTATE", "Database error")
        .replace("Connection refused", "Service unavailable")
        .replace("timeout", "Service temporarily unavailable");

    // Truncate very long error messages
    if sanitized.len() > 200 {
        format!("{}...", &sanitized[..197])
    } else {
        sanitized
    }
}