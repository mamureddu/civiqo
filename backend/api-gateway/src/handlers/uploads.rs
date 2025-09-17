use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use shared::{
    models::ApiResponse,
    error::{AppError, Result},
};
use crate::{AppState, middleware::auth::extract_user};

#[derive(Deserialize, Validate)]
pub struct PresignedUrlRequest {
    #[validate(regex(path = "ALLOWED_MIME_TYPES", message = "Invalid file type"))]
    pub file_type: String,
    #[validate(range(min = 1, max = 10485760, message = "File size must be between 1 byte and 10MB"))]
    pub file_size: i64,
    #[validate(regex(path = "ALLOWED_UPLOAD_TYPES", message = "Invalid upload type"))]
    pub upload_type: String, // "avatar", "business_image", etc.
}

// Compile-time validation patterns
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ALLOWED_MIME_TYPES: Regex = Regex::new(r"^(image/jpeg|image/png|image/webp|image/gif)$").unwrap();
    static ref ALLOWED_UPLOAD_TYPES: Regex = Regex::new(r"^(avatar|business_image|community_banner)$").unwrap();
}

#[derive(Serialize)]
pub struct PresignedUrlResponse {
    pub upload_url: String,
    pub file_url: String,
    pub expires_in: i64,
}

pub async fn get_presigned_url(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<PresignedUrlRequest>,
) -> Result<Json<ApiResponse<PresignedUrlResponse>>> {
    // Authenticate user
    let user = extract_user(&state, &headers).await?;

    // Validate input
    request.validate()
        .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

    // Additional security checks
    if request.file_size > 10 * 1024 * 1024 { // 10MB limit
        return Err(AppError::Validation("File too large".to_string()));
    }

    // Generate secure filename with user ID prefix to prevent conflicts
    let filename = format!(
        "{}/{}/{}_{}",
        user.user_id,
        request.upload_type,
        uuid::Uuid::new_v4(),
        match request.file_type.as_str() {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/webp" => "webp",
            "image/gif" => "gif",
            _ => return Err(AppError::Validation("Unsupported file type".to_string())),
        }
    );

    // Placeholder implementation - in production this would generate real presigned URLs
    let response = PresignedUrlResponse {
        upload_url: format!("https://placeholder-upload-url.com/{}", filename),
        file_url: format!("https://{}.s3.{}.amazonaws.com/{}",
                          state.config.s3_bucket, state.config.s3_region, filename),
        expires_in: 3600,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Upload functionality not implemented yet".to_string()),
        error: None,
    }))
}