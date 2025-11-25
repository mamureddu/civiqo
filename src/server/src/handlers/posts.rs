use axum::{
    extract::{Path, State, Query},
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
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub content_type: Option<String>,
    pub media_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: String,
    pub community_id: String,
    pub author_id: String,
    pub author_email: String,
    pub title: String,
    pub content: String,
    pub content_type: String,
    pub media_url: Option<String>,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub view_count: i64,
    pub reaction_count: i64,
    pub comment_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct PostsListResponse {
    pub posts: Vec<PostResponse>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Deserialize)]
pub struct PostsQueryParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 20 }

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

// ============================================================================
// Posts Endpoints
// ============================================================================

/// Create a new post in a community
pub async fn create_post(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PostResponse>>), StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let title = payload.title.trim();
    let content = payload.content.trim();
    
    if title.is_empty() {
        return Ok((StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false, data: None,
            message: Some("Title cannot be empty".to_string()),
        })));
    }
    
    if content.is_empty() {
        return Ok((StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false, data: None,
            message: Some("Content cannot be empty".to_string()),
        })));
    }

    let is_member: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active')"
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| { tracing::error!("Failed to check membership: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    if !is_member {
        return Ok((StatusCode::FORBIDDEN, Json(ApiResponse {
            success: false, data: None,
            message: Some("You must be a community member to create posts".to_string()),
        })));
    }

    let content_type = payload.content_type.unwrap_or_else(|| "markdown".to_string());
    let post_id = Uuid::now_v7();

    let result = sqlx::query(
        "INSERT INTO posts (id, community_id, author_id, title, content, content_type, media_url, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW()) RETURNING created_at, updated_at"
    )
    .bind(post_id).bind(community_id).bind(user_uuid)
    .bind(title).bind(content).bind(&content_type).bind(&payload.media_url)
    .fetch_one(&state.db.pool)
    .await;

    match result {
        Ok(row) => {
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
            let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
            tracing::info!("User {} created post {} in community {}", user.user_id, post_id, community_id);
            
            Ok((StatusCode::CREATED, Json(ApiResponse {
                success: true,
                data: Some(PostResponse {
                    id: post_id.to_string(),
                    community_id: community_id.to_string(),
                    author_id: user_uuid.to_string(),
                    author_email: user.email.clone(),
                    title: title.to_string(),
                    content: content.to_string(),
                    content_type,
                    media_url: payload.media_url,
                    is_pinned: false, is_locked: false, view_count: 0,
                    reaction_count: 0, comment_count: 0,
                    created_at: created_at.format("%Y-%m-%d %H:%M").to_string(),
                    updated_at: updated_at.format("%Y-%m-%d %H:%M").to_string(),
                }),
                message: Some("Post created successfully".to_string()),
            })))
        }
        Err(e) => {
            tracing::error!("Failed to create post: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
                success: false, data: None,
                message: Some("Failed to create post".to_string()),
            })))
        }
    }
}

/// List posts in a community
pub async fn list_posts(
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
    Query(params): Query<PostsQueryParams>,
    OptionalAuthUser(user): OptionalAuthUser,
) -> Result<Json<ApiResponse<PostsListResponse>>, StatusCode> {
    let offset = (params.page - 1) * params.limit;

    let community: Option<(bool,)> = sqlx::query_as("SELECT is_public FROM communities WHERE id = $1")
        .bind(community_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| { tracing::error!("Failed to fetch community: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    let is_public = match community {
        Some((public,)) => public,
        None => return Ok(Json(ApiResponse { success: false, data: None, message: Some("Community not found".to_string()) })),
    };

    if !is_public {
        if let Some(ref auth_user) = user {
            let user_uuid = Uuid::parse_str(&auth_user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let is_member: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active')"
            ).bind(community_id).bind(user_uuid).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            if !is_member {
                return Ok(Json(ApiResponse { success: false, data: None, message: Some("Access denied".to_string()) }));
            }
        } else {
            return Ok(Json(ApiResponse { success: false, data: None, message: Some("Access denied".to_string()) }));
        }
    }

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE community_id = $1")
        .bind(community_id).fetch_one(&state.db.pool).await.map_err(|e| { tracing::error!("Failed to count: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    let rows = sqlx::query(
        "SELECT p.id, p.community_id, p.author_id, u.email as author_email, p.title, p.content, p.content_type, p.media_url,
                p.is_pinned, p.is_locked, p.view_count, p.created_at, p.updated_at,
                COALESCE((SELECT COUNT(*) FROM reactions WHERE post_id = p.id), 0) as reaction_count,
                COALESCE((SELECT COUNT(*) FROM comments WHERE post_id = p.id), 0) as comment_count
         FROM posts p JOIN users u ON p.author_id = u.id WHERE p.community_id = $1
         ORDER BY p.is_pinned DESC, p.created_at DESC LIMIT $2 OFFSET $3"
    ).bind(community_id).bind(params.limit as i64).bind(offset as i64)
    .fetch_all(&state.db.pool).await.map_err(|e| { tracing::error!("Failed to fetch posts: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    let posts: Vec<PostResponse> = rows.iter().map(|row| PostResponse {
        id: row.get::<Uuid, _>("id").to_string(),
        community_id: row.get::<Uuid, _>("community_id").to_string(),
        author_id: row.get::<Uuid, _>("author_id").to_string(),
        author_email: row.get::<String, _>("author_email"),
        title: row.get("title"), content: row.get("content"),
        content_type: row.get::<Option<String>, _>("content_type").unwrap_or_else(|| "markdown".to_string()),
        media_url: row.get("media_url"),
        is_pinned: row.get::<Option<bool>, _>("is_pinned").unwrap_or(false),
        is_locked: row.get::<Option<bool>, _>("is_locked").unwrap_or(false),
        view_count: row.get::<Option<i64>, _>("view_count").unwrap_or(0),
        reaction_count: row.get::<Option<i64>, _>("reaction_count").unwrap_or(0),
        comment_count: row.get::<Option<i64>, _>("comment_count").unwrap_or(0),
        created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").format("%Y-%m-%d %H:%M").to_string(),
    }).collect();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(PostsListResponse {
            posts, total, page: params.page, limit: params.limit,
            has_next: (offset + params.limit) < total as u32, has_prev: params.page > 1,
        }),
        message: None,
    }))
}

/// Get a single post
pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
    OptionalAuthUser(user): OptionalAuthUser,
) -> Result<Json<ApiResponse<PostResponse>>, StatusCode> {
    let row = sqlx::query(
        "SELECT p.id, p.community_id, p.author_id, u.email as author_email, p.title, p.content, p.content_type, p.media_url,
                p.is_pinned, p.is_locked, p.view_count, p.created_at, p.updated_at, c.is_public as community_is_public,
                COALESCE((SELECT COUNT(*) FROM reactions WHERE post_id = p.id), 0) as reaction_count,
                COALESCE((SELECT COUNT(*) FROM comments WHERE post_id = p.id), 0) as comment_count
         FROM posts p JOIN users u ON p.author_id = u.id JOIN communities c ON p.community_id = c.id WHERE p.id = $1"
    ).bind(post_id).fetch_optional(&state.db.pool).await.map_err(|e| { tracing::error!("Failed: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ApiResponse { success: false, data: None, message: Some("Post not found".to_string()) })),
    };

    let community_is_public: bool = row.get("community_is_public");
    let community_id: Uuid = row.get("community_id");

    if !community_is_public {
        if let Some(ref auth_user) = user {
            let user_uuid = Uuid::parse_str(&auth_user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let is_member: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active')"
            ).bind(community_id).bind(user_uuid).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            if !is_member { return Ok(Json(ApiResponse { success: false, data: None, message: Some("Access denied".to_string()) })); }
        } else {
            return Ok(Json(ApiResponse { success: false, data: None, message: Some("Access denied".to_string()) }));
        }
    }

    let _ = sqlx::query("UPDATE posts SET view_count = view_count + 1 WHERE id = $1").bind(post_id).execute(&state.db.pool).await;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(PostResponse {
            id: row.get::<Uuid, _>("id").to_string(),
            community_id: row.get::<Uuid, _>("community_id").to_string(),
            author_id: row.get::<Uuid, _>("author_id").to_string(),
            author_email: row.get::<String, _>("author_email"),
            title: row.get("title"), content: row.get("content"),
            content_type: row.get::<Option<String>, _>("content_type").unwrap_or_else(|| "markdown".to_string()),
            media_url: row.get("media_url"),
            is_pinned: row.get::<Option<bool>, _>("is_pinned").unwrap_or(false),
            is_locked: row.get::<Option<bool>, _>("is_locked").unwrap_or(false),
            view_count: row.get::<Option<i64>, _>("view_count").unwrap_or(0) + 1,
            reaction_count: row.get::<Option<i64>, _>("reaction_count").unwrap_or(0),
            comment_count: row.get::<Option<i64>, _>("comment_count").unwrap_or(0),
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
            updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").format("%Y-%m-%d %H:%M").to_string(),
        }),
        message: None,
    }))
}

/// Update a post (author only)
pub async fn update_post(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post: Option<(Uuid,)> = sqlx::query_as("SELECT author_id FROM posts WHERE id = $1")
        .bind(post_id).fetch_optional(&state.db.pool).await.map_err(|e| { tracing::error!("Failed: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    let author_id = match post {
        Some((author,)) => author,
        None => return Ok(Json(ApiResponse { success: false, data: None, message: Some("Post not found".to_string()) })),
    };

    if author_id != user_uuid {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("Only the author can update this post".to_string()) }));
    }

    if payload.title.is_none() && payload.content.is_none() {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("No fields to update".to_string()) }));
    }

    let title = payload.title.as_deref().map(|t| t.trim()).filter(|t| !t.is_empty());
    let content = payload.content.as_deref().map(|c| c.trim()).filter(|c| !c.is_empty());

    if title.is_some() {
        sqlx::query("UPDATE posts SET title = $1, updated_at = NOW() WHERE id = $2")
            .bind(title).bind(post_id).execute(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if content.is_some() {
        sqlx::query("UPDATE posts SET content = $1, updated_at = NOW() WHERE id = $2")
            .bind(content).bind(post_id).execute(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tracing::info!("User {} updated post {}", user.user_id, post_id);
    Ok(Json(ApiResponse { success: true, data: None, message: Some("Post updated".to_string()) }))
}

/// Delete a post (author or admin)
pub async fn delete_post(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post: Option<(Uuid, Uuid)> = sqlx::query_as("SELECT author_id, community_id FROM posts WHERE id = $1")
        .bind(post_id).fetch_optional(&state.db.pool).await.map_err(|e| { tracing::error!("Failed: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    let (author_id, community_id) = match post {
        Some(p) => p,
        None => return Ok(Json(ApiResponse { success: false, data: None, message: Some("Post not found".to_string()) })),
    };

    let is_author = author_id == user_uuid;
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members cm JOIN roles r ON cm.role_id = r.id WHERE cm.community_id = $1 AND cm.user_id = $2 AND r.name IN ('admin', 'owner'))"
    ).bind(community_id).bind(user_uuid).fetch_one(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_author && !is_admin {
        return Ok(Json(ApiResponse { success: false, data: None, message: Some("Permission denied".to_string()) }));
    }

    sqlx::query("DELETE FROM posts WHERE id = $1").bind(post_id).execute(&state.db.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tracing::info!("User {} deleted post {}", user.user_id, post_id);
    Ok(Json(ApiResponse { success: true, data: None, message: Some("Post deleted".to_string()) }))
}
