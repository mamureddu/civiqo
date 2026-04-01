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
use crate::auth::AuthUser;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub post_id: String,
    pub author_id: String,
    pub author_email: String,
    pub parent_id: Option<String>,
    pub content: String,
    pub is_edited: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CommentsListResponse {
    pub comments: Vec<CommentResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

// ============================================================================
// Comments Endpoints
// ============================================================================

/// Create a comment on a post
pub async fn create_comment(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::response::Response, StatusCode> {
    let payload: CreateCommentRequest = {
        let ct = headers.get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("");
        if ct.contains("application/json") {
            serde_json::from_slice(&body).map_err(|_| StatusCode::BAD_REQUEST)?
        } else {
            serde_urlencoded::from_bytes(&body).map_err(|_| StatusCode::BAD_REQUEST)?
        }
    };
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content = payload.content.trim();
    if content.is_empty() {
        return Ok((StatusCode::BAD_REQUEST, Json(ApiResponse::<CommentResponse> {
            success: false, data: None, message: Some("Content cannot be empty".to_string()),
        })).into_response());
    }

    // Get post and check if locked
    let post: Option<(Uuid, bool)> = sqlx::query_as(
        "SELECT community_id, COALESCE(is_locked, false) FROM posts WHERE id = $1"
    ).bind(post_id).fetch_optional(&state.db.pool).await.map_err(|e| {
        tracing::error!("Failed to fetch post: {}", e); StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let (community_id, is_locked) = match post {
        Some(p) => p,
        None => return Ok((StatusCode::NOT_FOUND, Json(ApiResponse::<CommentResponse> {
            success: false, data: None, message: Some("Post not found".to_string()),
        })).into_response()),
    };

    if is_locked {
        return Ok((StatusCode::FORBIDDEN, Json(ApiResponse::<CommentResponse> {
            success: false, data: None, message: Some("Post is locked".to_string()),
        })).into_response());
    }

    // Check membership
    let is_member: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active')"
    ).bind(community_id).bind(user_uuid).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_member {
        return Ok((StatusCode::FORBIDDEN, Json(ApiResponse::<CommentResponse> {
            success: false, data: None, message: Some("Must be a member to comment".to_string()),
        })).into_response());
    }

    // Validate parent_id
    let parent_uuid: Option<Uuid> = if let Some(ref pid) = payload.parent_id {
        if let Ok(p) = Uuid::parse_str(pid) {
            let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM comments WHERE id = $1 AND post_id = $2)")
                .bind(p).bind(post_id).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            if !exists {
                return Ok((StatusCode::NOT_FOUND, Json(ApiResponse::<CommentResponse> {
                    success: false, data: None, message: Some("Parent comment not found".to_string()),
                })).into_response());
            }
            Some(p)
        } else { None }
    } else { None };

    let comment_id = Uuid::now_v7();

    let result = sqlx::query(
        "INSERT INTO comments (id, post_id, author_id, parent_id, content, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING created_at, updated_at"
    ).bind(comment_id).bind(post_id).bind(user_uuid).bind(parent_uuid).bind(content)
    .fetch_one(&state.db.pool).await;

    match result {
        Ok(row) => {
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
            let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
            tracing::info!("User {} created comment {} on post {}", user.user_id, comment_id, post_id);

            // If HTMX request, return HTML fragment
            let is_htmx = headers.get("hx-request").is_some();
            if is_htmx {
                let mut ctx = tera::Context::new();
                let mut comment_map = std::collections::HashMap::new();
                comment_map.insert("id", comment_id.to_string());
                comment_map.insert("post_id", post_id.to_string());
                comment_map.insert("content", content.to_string());
                comment_map.insert("author_email", user.email.clone());
                comment_map.insert("author_name", user.name.clone().unwrap_or(user.email.clone()));
                comment_map.insert("created_at", created_at.format("%Y-%m-%d %H:%M").to_string());
                comment_map.insert("is_edited", "false".to_string());
                ctx.insert("comment", &comment_map);
                ctx.insert("is_author", &true);
                ctx.insert("is_member", &true);
                ctx.insert("depth", &0);

                let html = state.tera.render("fragments/comment-item.html", &ctx)
                    .map_err(|e| {
                        tracing::error!("Failed to render comment item: {}", e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                return Ok(Html(html).into_response());
            }

            Ok((StatusCode::CREATED, Json(ApiResponse {
                success: true,
                data: Some(CommentResponse {
                    id: comment_id.to_string(),
                    post_id: post_id.to_string(),
                    author_id: user_uuid.to_string(),
                    author_email: user.email.clone(),
                    parent_id: parent_uuid.map(|p| p.to_string()),
                    content: content.to_string(),
                    is_edited: false,
                    created_at: created_at.format("%Y-%m-%d %H:%M").to_string(),
                    updated_at: updated_at.format("%Y-%m-%d %H:%M").to_string(),
                }),
                message: Some("Comment created".to_string()),
            })).into_response())
        }
        Err(e) => {
            tracing::error!("Failed to create comment: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<CommentResponse> {
                success: false, data: None, message: Some("Failed to create comment".to_string()),
            })).into_response())
        }
    }
}

/// List comments on a post
pub async fn list_comments(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<ApiResponse<CommentsListResponse>>, StatusCode> {
    // Check post exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
        .bind(post_id).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("Post not found".to_string()) }));
    }

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM comments WHERE post_id = $1")
        .bind(post_id).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let rows = sqlx::query(
        "SELECT c.id, c.post_id, c.author_id, u.email as author_email, c.parent_id, c.content,
                COALESCE(c.is_edited, false) as is_edited, c.created_at, c.updated_at
         FROM comments c JOIN users u ON c.author_id = u.id
         WHERE c.post_id = $1 ORDER BY c.created_at ASC"
    ).bind(post_id).fetch_all(&state.db.pool).await.map_err(|e| {
        tracing::error!("Failed to fetch comments: {}", e); StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let comments: Vec<CommentResponse> = rows.iter().map(|row| CommentResponse {
        id: row.get::<Uuid, _>("id").to_string(),
        post_id: row.get::<Uuid, _>("post_id").to_string(),
        author_id: row.get::<Uuid, _>("author_id").to_string(),
        author_email: row.get::<String, _>("author_email"),
        parent_id: row.get::<Option<Uuid>, _>("parent_id").map(|p| p.to_string()),
        content: row.get("content"),
        is_edited: row.get("is_edited"),
        created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").format("%Y-%m-%d %H:%M").to_string(),
    }).collect();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(CommentsListResponse { comments, total }),
        message: None,
    }))
}

/// Update a comment (author only)
pub async fn update_comment(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(comment_id): Path<Uuid>,
    Json(payload): Json<UpdateCommentRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content = payload.content.trim();
    if content.is_empty() {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("Content cannot be empty".to_string()) }));
    }

    let comment: Option<(Uuid,)> = sqlx::query_as("SELECT author_id FROM comments WHERE id = $1")
        .bind(comment_id).fetch_optional(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let author_id = match comment {
        Some((a,)) => a,
        None => return Ok(Json(ApiResponse { success: false, data: None, message: Some("Comment not found".to_string()) })),
    };

    if author_id != user_uuid {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("Only author can update".to_string()) }));
    }

    sqlx::query("UPDATE comments SET content = $1, is_edited = true, updated_at = NOW() WHERE id = $2")
        .bind(content).bind(comment_id).execute(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("User {} updated comment {}", user.user_id, comment_id);
    Ok(Json(ApiResponse { success: true, data: None, message: Some("Comment updated".to_string()) }))
}

/// Delete a comment (author or admin)
pub async fn delete_comment(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(comment_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let comment: Option<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT c.author_id, p.community_id FROM comments c JOIN posts p ON c.post_id = p.id WHERE c.id = $1"
    ).bind(comment_id).fetch_optional(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (author_id, community_id) = match comment {
        Some(c) => c,
        None => return Ok(Json(ApiResponse { success: false, data: None, message: Some("Comment not found".to_string()) })),
    };

    let is_author = author_id == user_uuid;
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND role IN ('admin', 'owner'))"
    ).bind(community_id).bind(user_uuid).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_author && !is_admin {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("Permission denied".to_string()) }));
    }

    sqlx::query("DELETE FROM comments WHERE id = $1").bind(comment_id).execute(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tracing::info!("User {} deleted comment {}", user.user_id, comment_id);
    Ok(Json(ApiResponse { success: true, data: None, message: Some("Comment deleted".to_string()) }))
}
