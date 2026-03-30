use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
};
use shared::{
    models::{ApiResponse, UpdateUserProfileRequest, UserWithProfile},
    error::{AppError, Result},
};
use crate::{AppState, middleware::auth::extract_user};

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
            u.email as "email!",
            u.provider as "provider!",
            u.provider_id as "provider_id?",
            u.created_at as "created_at!",
            u.updated_at as "updated_at!",
            p.id as "profile_id?",
            p.name as "profile_name?",
            p.picture as "profile_picture?",
            p.bio as "profile_bio?",
            p.location as "profile_location?",
            p.website as "profile_website?",
            p.created_at as "profile_created_at?",
            p.updated_at as "profile_updated_at?"
        FROM users u
        LEFT JOIN user_profiles p ON u.id = p.user_id
        WHERE u.id = $1
        "#,
        user.user_id
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
            u.email as "email!",
            u.provider as "provider!",
            u.provider_id as "provider_id?",
            u.created_at as "created_at!",
            u.updated_at as "updated_at!",
            p.id as "profile_id?", p.name as "profile_name?",
            p.picture as "profile_picture?", p.bio as "profile_bio?",
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