use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use sqlx::Row;

use crate::handlers::pages::AppState;
use crate::auth::{AuthUser, OptionalAuthUser};

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct AddReactionRequest {
    pub reaction_type: String,
}

#[derive(Debug, Serialize)]
pub struct ReactionCountResponse {
    pub reaction_type: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct ReactionsListResponse {
    pub reactions: Vec<ReactionCountResponse>,
    pub user_reaction: Option<String>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

// Valid reaction types
const VALID_REACTIONS: &[&str] = &["like", "upvote", "heart", "celebrate", "laugh", "sad"];

// ============================================================================
// Reactions Endpoints
// ============================================================================

/// Add or update a reaction to a post
pub async fn add_reaction(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<AddReactionRequest>,
) -> Result<Json<ApiResponse<ReactionsListResponse>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let reaction_type = payload.reaction_type.trim().to_lowercase();
    
    if !VALID_REACTIONS.contains(&reaction_type.as_str()) {
        return Ok(Json(ApiResponse {
            success: false, data: None,
            message: Some(format!("Invalid reaction type. Valid types: {:?}", VALID_REACTIONS)),
        }));
    }

    // Check post exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
        .bind(post_id).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("Post not found".to_string()) }));
    }

    // Upsert reaction (insert or update)
    sqlx::query(
        "INSERT INTO reactions (post_id, user_id, reaction_type, created_at)
         VALUES ($1, $2, $3, NOW())
         ON CONFLICT (post_id, user_id) DO UPDATE SET reaction_type = $3"
    ).bind(post_id).bind(user_uuid).bind(&reaction_type)
    .execute(&state.db.pool).await.map_err(|e| {
        tracing::error!("Failed to add reaction: {}", e); StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!("User {} reacted {} to post {}", user.user_id, reaction_type, post_id);

    // Return updated reaction counts
    get_reactions_internal(&state, post_id, Some(user_uuid)).await
}

/// Remove a reaction from a post
pub async fn remove_reaction(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<ApiResponse<ReactionsListResponse>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query("DELETE FROM reactions WHERE post_id = $1 AND user_id = $2")
        .bind(post_id).bind(user_uuid).execute(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("No reaction to remove".to_string()) }));
    }

    tracing::info!("User {} removed reaction from post {}", user.user_id, post_id);

    // Return updated reaction counts
    get_reactions_internal(&state, post_id, Some(user_uuid)).await
}

/// List reactions on a post
pub async fn list_reactions(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
    OptionalAuthUser(user): OptionalAuthUser,
) -> Result<Json<ApiResponse<ReactionsListResponse>>, StatusCode> {
    // Check post exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
        .bind(post_id).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("Post not found".to_string()) }));
    }

    let user_uuid = user.and_then(|u| Uuid::parse_str(&u.user_id).ok());
    get_reactions_internal(&state, post_id, user_uuid).await
}

/// Internal helper to get reactions
async fn get_reactions_internal(
    state: &Arc<AppState>,
    post_id: Uuid,
    user_uuid: Option<Uuid>,
) -> Result<Json<ApiResponse<ReactionsListResponse>>, StatusCode> {
    // Get reaction counts grouped by type
    let rows = sqlx::query(
        "SELECT reaction_type, COUNT(*) as count FROM reactions WHERE post_id = $1 GROUP BY reaction_type ORDER BY count DESC"
    ).bind(post_id).fetch_all(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let reactions: Vec<ReactionCountResponse> = rows.iter().map(|row| ReactionCountResponse {
        reaction_type: row.get("reaction_type"),
        count: row.get("count"),
    }).collect();

    let total: i64 = reactions.iter().map(|r| r.count).sum();

    // Get user's reaction if authenticated
    let user_reaction: Option<String> = if let Some(uid) = user_uuid {
        sqlx::query_scalar("SELECT reaction_type FROM reactions WHERE post_id = $1 AND user_id = $2")
            .bind(post_id).bind(uid).fetch_optional(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        None
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(ReactionsListResponse { reactions, user_reaction, total }),
        message: None,
    }))
}
