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
use crate::auth::AuthUser;

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate a URL-friendly slug from a community name
fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            ' ' => '-',
            _ if c.is_ascii_punctuation() => '-',
            _ => '\0', // Will be filtered out
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
        .trim_matches('-')
        .to_string()
}

/// Ensure slug is unique by appending number if needed
async fn ensure_unique_slug(
    base_slug: &str,
    pool: &sqlx::PgPool,
) -> Result<String, StatusCode> {
    let mut slug = base_slug.to_string();
    let mut counter = 1;
    
    // Check if slug exists, if so append number
    while sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM communities WHERE slug = $1)"
    )
    .bind(&slug)
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        slug = format!("{}-{}", base_slug, counter);
        counter += 1;
        
        // Prevent infinite loop
        if counter > 1000 {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    Ok(slug)
}

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
    pub is_public: Option<bool>,
    pub requires_approval: Option<bool>,
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
    // Validate input
    if payload.name.trim().is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Community name cannot be empty".to_string()),
        }));
    }
    
    if payload.name.len() > 255 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Community name too long (max 255 characters)".to_string()),
        }));
    }
    
    if let Some(ref description) = payload.description {
        if description.len() > 2000 {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Description too long (max 2000 characters)".to_string()),
            }));
        }
    }
    
    // Parse authenticated user ID
    let creator_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Generate unique slug
    let base_slug = generate_slug(&payload.name);
    let slug = ensure_unique_slug(&base_slug, &state.db.pool).await?;
    
    // Start transaction for atomic community creation + admin membership
    let mut tx = state.db.pool.begin().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let community_id = Uuid::new_v4();
    
    // Insert community
    let community_result = sqlx::query(
        "INSERT INTO communities (id, name, description, slug, is_public, requires_approval, created_by) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id, created_at"
    )
    .bind(community_id)
    .bind(&payload.name.trim())
    .bind(&payload.description)
    .bind(&slug)
    .bind(payload.is_public.unwrap_or(true))
    .bind(payload.requires_approval.unwrap_or(false))
    .bind(creator_id)
    .fetch_one(&mut *tx)
    .await;
    
    let community_row = match community_result {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("Failed to create community: {}", e);
            tx.rollback().await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // Get default admin role (or create if doesn't exist)
    let admin_role = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM roles WHERE name = 'admin' LIMIT 1"
    )
    .fetch_one(&mut *tx)
    .await;
    
    let admin_role_id = match admin_role {
        Ok(role_id) => role_id,
        Err(_) => {
            // Create default admin role if it doesn't exist
            let new_role_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO roles (id, name, description, permissions, is_default) 
                 VALUES ($1, 'admin', 'Community administrator', '[\"manage_community\", \"manage_members\", \"manage_posts\"]', FALSE)"
            )
            .bind(new_role_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create admin role: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            new_role_id
        }
    };
    
    // Add creator as admin member
    let membership_result = sqlx::query(
        "INSERT INTO community_members (id, user_id, community_id, role_id, status, joined_at) 
         VALUES ($1, $2, $3, $4, 'active', NOW())"
    )
    .bind(Uuid::new_v4())
    .bind(creator_id)
    .bind(community_id)
    .bind(admin_role_id)
    .execute(&mut *tx)
    .await;
    
    match membership_result {
        Ok(_) => {
            // Commit transaction
            tx.commit().await
                .map_err(|e| {
                    tracing::error!("Failed to commit transaction: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            
            tracing::info!("Community '{}' created successfully by user {}", payload.name, user.user_id);
            
            let community = CommunityResponse {
                id: community_id.to_string(),
                name: payload.name.trim().to_string(),
                description: payload.description,
                created_at: community_row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                    .format("%Y-%m-%d %H:%M").to_string(),
            };
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(community),
                message: Some("Community created successfully".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to add creator as member: {}", e);
            tx.rollback().await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    AuthUser(_user): AuthUser, // Requires authentication
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
