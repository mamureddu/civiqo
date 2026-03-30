use axum::http::HeaderMap;
use shared::{
    auth::{extract_bearer_token, JwtService, JwtConfig, AuthenticatedUser},
    error::{AppError, Result},
};
use crate::AppState;
use uuid::Uuid;

/// Extract and validate user from Authorization header
pub async fn extract_user(state: &AppState, headers: &HeaderMap) -> Result<AuthenticatedUser> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Auth("Missing Authorization header".to_string()))?;

    let token = extract_bearer_token(auth_header)?;

    let jwt_config = JwtConfig::from_env()?;
    let jwt_service = JwtService::new(jwt_config);
    let claims = jwt_service.validate_token(token)?;

    // Parse UUID from claims.sub (now contains user UUID directly)
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))?;

    // Look up user in database by UUID
    let user = sqlx::query!(
        "SELECT id FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::Auth("User not found in database".to_string()))?;

    Ok(AuthenticatedUser {
        user_id: user.id,
        email: claims.email.clone(),
        name: claims.name.clone(),
        claims,
    })
}

/// Check if user has permission for a specific community
pub async fn check_community_permission(
    state: &AppState,
    user_id: Uuid,
    community_id: Uuid,
    required_permission: &str,
) -> Result<bool> {
    // Permission mapping based on role ENUM
    // owner/admin: all permissions
    // moderator: manage_content, moderate
    // member: read, write, vote
    let allowed_roles = match required_permission {
        "all" | "manage_members" | "manage_settings" => vec!["owner", "admin"],
        "manage_content" | "moderate" => vec!["owner", "admin", "moderator"],
        "read" | "write" | "vote" => vec!["owner", "admin", "moderator", "member"],
        _ => vec!["owner", "admin"], // Default to admin-only for unknown permissions
    };
    
    let has_permission: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM community_members
            WHERE user_id = $1 AND community_id = $2 AND status = 'active'
            AND role::text = ANY($3)
        )"
    )
    .bind(user_id)
    .bind(community_id)
    .bind(&allowed_roles)
    .fetch_one(&state.db.pool)
    .await?;

    Ok(has_permission)
}

/// Check if user is a member of a community
pub async fn check_community_membership(
    state: &AppState,
    user_id: Uuid,
    community_id: Uuid,
) -> Result<bool> {
    let is_member = sqlx::query!(
        "SELECT 1 as is_member FROM community_members WHERE user_id = $1 AND community_id = $2 AND status = 'active'",
        user_id,
        community_id
    )
    .fetch_optional(&state.db.pool)
    .await?
    .is_some();

    Ok(is_member)
}