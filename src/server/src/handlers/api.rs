use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::pages::AppState;
use crate::auth::{AuthUser, OptionalAuthUser};

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommunityRequest {
    pub name: String,
    pub description: Option<String>,
    pub created_by: String, // user_id
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub author_id: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct CommunityResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

// ============================================================================
// User Endpoints
// ============================================================================

/// Create a new user
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // Hash password (in production, use bcrypt or argon2)
    let password_hash = format!("hashed_{}", payload.password);
    
    let user_id = Uuid::new_v4();
    
    let result = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash) 
         VALUES ($1, $2, $3, $4)"
    )
    .bind(user_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .execute(&state.db.pool)
    .await;
    
    match result {
        Ok(_) => {
            let user = UserResponse {
                id: user_id.to_string(),
                username: payload.username,
                email: payload.email,
                created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string(),
            };
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(user),
                message: Some("User created successfully".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get all users
pub async fn get_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    use sqlx::Row;
    
    let users = sqlx::query("SELECT id, username, email, created_at FROM users ORDER BY created_at DESC")
        .fetch_all(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let users_data: Vec<UserResponse> = users.iter().map(|row| {
        UserResponse {
            id: row.get::<Uuid, _>("id").to_string(),
            username: row.get::<String, _>("username"),
            email: row.get::<String, _>("email"),
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        }
    }).collect();
    
    Ok(Json(users_data))
}

// ============================================================================
// Community Endpoints
// ============================================================================

/// Create a new community (PROTECTED - requires authentication)
pub async fn create_community(
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCommunityRequest>,
) -> Result<Json<ApiResponse<CommunityResponse>>, StatusCode> {
    let community_id = Uuid::new_v4();
    let creator_id = Uuid::parse_str(&payload.created_by)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let result = sqlx::query(
        "INSERT INTO communities (id, name, description, created_by) 
         VALUES ($1, $2, $3, $4)"
    )
    .bind(community_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(creator_id)
    .execute(&state.db.pool)
    .await;
    
    match result {
        Ok(_) => {
            let community = CommunityResponse {
                id: community_id.to_string(),
                name: payload.name,
                description: payload.description,
                created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string(),
            };
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(community),
                message: Some("Community created successfully".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to create community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get all communities
pub async fn get_communities(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CommunityResponse>>, StatusCode> {
    use sqlx::Row;
    
    let communities = sqlx::query(
        "SELECT id, name, description, created_at FROM communities ORDER BY created_at DESC"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let communities_data: Vec<CommunityResponse> = communities.iter().map(|row| {
        CommunityResponse {
            id: row.get::<Uuid, _>("id").to_string(),
            name: row.get::<String, _>("name"),
            description: row.get::<Option<String>, _>("description"),
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        }
    }).collect();
    
    Ok(Json(communities_data))
}

// ============================================================================
// Post Endpoints
// ============================================================================

/// Create a new post in a community (PROTECTED - requires authentication)
pub async fn create_post(
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<PostResponse>>, StatusCode> {
    let post_id = Uuid::new_v4();
    let community_uuid = Uuid::parse_str(&community_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let author_uuid = Uuid::parse_str(&payload.author_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let result = sqlx::query(
        "INSERT INTO posts (id, community_id, author_id, title, content) 
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(post_id)
    .bind(community_uuid)
    .bind(author_uuid)
    .bind(&payload.title)
    .bind(&payload.content)
    .execute(&state.db.pool)
    .await;
    
    match result {
        Ok(_) => {
            let post = PostResponse {
                id: post_id.to_string(),
                title: payload.title,
                content: payload.content,
                created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string(),
            };
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(post),
                message: Some("Post created successfully".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to create post: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get posts for a community
pub async fn get_posts(
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
) -> Result<Json<Vec<PostResponse>>, StatusCode> {
    use sqlx::Row;
    
    let community_uuid = Uuid::parse_str(&community_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let posts = sqlx::query(
        "SELECT id, title, content, created_at FROM posts 
         WHERE community_id = $1 
         ORDER BY created_at DESC"
    )
    .bind(community_uuid)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let posts_data: Vec<PostResponse> = posts.iter().map(|row| {
        PostResponse {
            id: row.get::<Uuid, _>("id").to_string(),
            title: row.get::<String, _>("title"),
            content: row.get::<String, _>("content"),
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        }
    }).collect();
    
    Ok(Json(posts_data))
}
