use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    response::{Json, IntoResponse, Html},
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
const VALID_REACTIONS: &[&str] = &["like", "upvote", "heart", "celebrate", "laugh", "sad", "thinking"];

// ============================================================================
// Reactions Endpoints
// ============================================================================

/// Add or update a reaction to a post (accepts JSON or form-encoded from HTMX)
pub async fn add_reaction(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::response::Response, StatusCode> {
    let payload: AddReactionRequest = {
        let content_type = headers.get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        if content_type.contains("application/json") {
            serde_json::from_slice(&body).map_err(|_| StatusCode::BAD_REQUEST)?
        } else {
            serde_urlencoded::from_bytes(&body).map_err(|_| StatusCode::BAD_REQUEST)?
        }
    };
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let reaction_type = payload.reaction_type.trim().to_lowercase();

    if !VALID_REACTIONS.contains(&reaction_type.as_str()) {
        return Ok(Json(ApiResponse::<ReactionsListResponse> {
            success: false, data: None,
            message: Some(format!("Invalid reaction type. Valid types: {:?}", VALID_REACTIONS)),
        }).into_response());
    }

    // Check post exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
        .bind(post_id).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists {
        return Ok(Json(ApiResponse::<ReactionsListResponse> { success: false, data: None, message: Some("Post not found".to_string()) }).into_response());
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

    // If HTMX request, return HTML fragment
    let is_htmx = headers.get("hx-request").is_some();
    if is_htmx {
        let html = render_reaction_buttons_html(&state, post_id, Some(user_uuid)).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Html(html).into_response());
    }

    // Otherwise return JSON
    let result = get_reactions_internal(&state, post_id, Some(user_uuid)).await?;
    Ok(result.into_response())
}

/// Remove a reaction from a post
pub async fn remove_reaction(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<axum::response::Response, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("DELETE FROM reactions WHERE post_id = $1 AND user_id = $2")
        .bind(post_id).bind(user_uuid).execute(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("User {} removed reaction from post {}", user.user_id, post_id);

    if headers.get("hx-request").is_some() {
        let html = render_reaction_buttons_html(&state, post_id, Some(user_uuid)).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Html(html).into_response());
    }

    let result = get_reactions_internal(&state, post_id, Some(user_uuid)).await?;
    Ok(result.into_response())
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

/// Render reaction buttons as HTML fragment for HTMX swap
async fn render_reaction_buttons_html(
    state: &Arc<AppState>,
    post_id: Uuid,
    user_uuid: Option<Uuid>,
) -> Result<String, StatusCode> {
    let rows = sqlx::query(
        "SELECT reaction_type, COUNT(*) as count FROM reactions WHERE post_id = $1 GROUP BY reaction_type"
    ).bind(post_id).fetch_all(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut reactions = std::collections::HashMap::new();
    for row in &rows {
        let rt: String = row.get("reaction_type");
        let count: i64 = row.get("count");
        reactions.insert(rt, count);
    }

    let user_reaction: Option<String> = if let Some(uid) = user_uuid {
        sqlx::query_scalar("SELECT reaction_type FROM reactions WHERE post_id = $1 AND user_id = $2")
            .bind(post_id).bind(uid).fetch_optional(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        None
    };

    let mut ctx = tera::Context::new();
    ctx.insert("post_id", &post_id.to_string());
    ctx.insert("reactions", &reactions);
    ctx.insert("user_reaction", &user_reaction);

    state.tera.render("fragments/reaction-buttons.html", &ctx)
        .map_err(|e| {
            tracing::error!("Failed to render reaction buttons: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
