use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use shared::{
    models::{ApiResponse, UpdateUserProfileRequest, UserWithProfile},
    error::{AppError, Result},
};
use crate::{AppState, middleware::auth::extract_user};

#[derive(Serialize, Deserialize, Validate)]
pub struct SyncUserRequest {
    #[validate(length(min = 1, max = 100, message = "Auth0 ID must be 1-100 characters"))]
    pub auth0_id: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: Option<String>,
    #[validate(url(message = "Invalid picture URL format"))]
    pub picture: Option<String>,
}

/// Get current user information
pub async fn get_current_user(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<UserWithProfile>>> {
    let user = extract_user(&state, &headers).await?;

    // Fetch user with profile from database
    let user_with_profile = sqlx::query_as!(
        UserWithProfile,
        r#"
        SELECT
            u.id as "id!",
            u.auth0_id as "auth0_id!",
            u.email as "email!",
            u.created_at as "created_at!",
            u.updated_at as "updated_at!",
            p.id as "profile_id?",
            p.name as "profile_name?",
            p.avatar_url as "profile_avatar_url?",
            p.bio as "profile_bio?",
            p.location as "profile_location?",
            p.website as "profile_website?",
            p.created_at as "profile_created_at?",
            p.updated_at as "profile_updated_at?"
        FROM users u
        LEFT JOIN user_profiles p ON u.id = p.user_id
        WHERE u.auth0_id = $1
        "#,
        user.auth0_id
    )
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(user_with_profile),
        message: None,
        error: None,
    }))
}

/// Sync user from Auth0 (create or update)
pub async fn sync_user_from_auth0(
    State(state): State<AppState>,
    Json(request): Json<SyncUserRequest>,
) -> Result<Json<ApiResponse<UserWithProfile>>> {
    // Validate input
    request.validate()
        .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;
    // Insert or update user
    let user = sqlx::query!(
        r#"
        INSERT INTO users (auth0_id, email, created_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        ON CONFLICT (auth0_id)
        DO UPDATE SET
            email = EXCLUDED.email,
            updated_at = NOW()
        RETURNING id, auth0_id, email, created_at, updated_at
        "#,
        request.auth0_id,
        request.email
    )
    .fetch_one(&state.db.pool)
    .await?;

    // Create or update profile if name or picture provided
    if request.name.is_some() || request.picture.is_some() {
        sqlx::query!(
            r#"
            INSERT INTO user_profiles (user_id, name, avatar_url, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            ON CONFLICT (user_id)
            DO UPDATE SET
                name = COALESCE(EXCLUDED.name, user_profiles.name),
                avatar_url = COALESCE(EXCLUDED.avatar_url, user_profiles.avatar_url),
                updated_at = NOW()
            "#,
            user.id,
            request.name,
            request.picture
        )
        .execute(&state.db.pool)
        .await?;
    }

    // Fetch the complete user with profile
    let user_with_profile = sqlx::query_as!(
        UserWithProfile,
        r#"
        SELECT
            u.id as "id!",
            u.auth0_id as "auth0_id!",
            u.email as "email!",
            u.created_at as "created_at!",
            u.updated_at as "updated_at!",
            p.id as "profile_id?", p.name as "profile_name?",
            p.avatar_url as "profile_avatar_url?", p.bio as "profile_bio?",
            p.location as "profile_location?", p.website as "profile_website?",
            p.created_at as "profile_created_at?", p.updated_at as "profile_updated_at?"
        FROM users u
        LEFT JOIN user_profiles p ON u.id = p.user_id
        WHERE u.id = $1
        "#,
        user.id
    )
    .fetch_one(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(user_with_profile),
        message: Some("User synced successfully".to_string()),
        error: None,
    }))
}

/// Update user profile
pub async fn update_user_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<UpdateUserProfileRequest>,
) -> Result<Json<ApiResponse<UserWithProfile>>> {
    let user = extract_user(&state, &headers).await?;

    // Update user profile
    sqlx::query!(
        r#"
        INSERT INTO user_profiles (user_id, name, bio, location, website, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        ON CONFLICT (user_id)
        DO UPDATE SET
            name = COALESCE(EXCLUDED.name, user_profiles.name),
            bio = COALESCE(EXCLUDED.bio, user_profiles.bio),
            location = COALESCE(EXCLUDED.location, user_profiles.location),
            website = COALESCE(EXCLUDED.website, user_profiles.website),
            updated_at = NOW()
        "#,
        user.user_id,
        request.name,
        request.bio,
        request.location,
        request.website
    )
    .execute(&state.db.pool)
    .await?;

    // Fetch updated user with profile
    let user_with_profile = sqlx::query_as!(
        UserWithProfile,
        r#"
        SELECT
            u.id as "id!",
            u.auth0_id as "auth0_id!",
            u.email as "email!",
            u.created_at as "created_at!",
            u.updated_at as "updated_at!",
            p.id as "profile_id?", p.name as "profile_name?",
            p.avatar_url as "profile_avatar_url?", p.bio as "profile_bio?",
            p.location as "profile_location?", p.website as "profile_website?",
            p.created_at as "profile_created_at?", p.updated_at as "profile_updated_at?"
        FROM users u
        LEFT JOIN user_profiles p ON u.id = p.user_id
        WHERE u.id = $1
        "#,
        user.user_id
    )
    .fetch_one(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(user_with_profile),
        message: Some("Profile updated successfully".to_string()),
        error: None,
    }))
}