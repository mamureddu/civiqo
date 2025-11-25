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
// Communities List/View Response Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityListResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub is_public: bool,
    pub member_count: i64,
    pub created_at: String,
    pub user_role: Option<String>, // User's role in this community (if member)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityDetailResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub is_public: bool,
    pub requires_approval: bool,
    pub member_count: i64,
    pub posts_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub user_role: Option<String>, // User's role in this community (if member)
    pub is_member: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunitiesListResponse {
    pub communities: Vec<CommunityListResponse>,
    pub total_count: i64,
    pub page: u32,
    pub limit: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Deserialize)]
pub struct CommunitiesQueryParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter: Option<String>, // "public", "my", "all"
    #[serde(default)]
    pub sort: Option<String>, // "created", "name", "members"
}

fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 20 }

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
    // Enhanced validation with specific error messages
    let trimmed_name = payload.name.trim();
    
    if trimmed_name.is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Community name is required and cannot be empty or only whitespace".to_string()),
        }));
    }
    
    if trimmed_name.len() < 3 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Community name must be at least 3 characters long".to_string()),
        }));
    }
    
    if trimmed_name.len() > 255 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Community name is too long (maximum 255 characters, currently {} characters)".to_string()),
        }));
    }
    
    // Check for invalid characters that would create empty slugs
    if !trimmed_name.chars().any(|c| c.is_alphanumeric()) {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Community name must contain at least one letter or number".to_string()),
        }));
    }
    
    if let Some(ref description) = payload.description {
        let trimmed_desc = description.trim();
        if !trimmed_desc.is_empty() && trimmed_desc.len() > 2000 {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Description is too long (maximum 2000 characters, currently {} characters)".to_string()),
            }));
        }
    }
    
    // Parse authenticated user ID with better error handling
    let creator_id = Uuid::parse_str(&user.user_id)
        .map_err(|e| {
            tracing::error!("Invalid user ID format for authenticated user: {} - Error: {}", user.user_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Generate unique slug with better error handling
    let base_slug = generate_slug(trimmed_name);
    if base_slug.is_empty() {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Community name cannot be converted to a valid URL slug. Please use letters, numbers, and basic punctuation.".to_string()),
        }));
    }
    
    let slug = ensure_unique_slug(&base_slug, &state.db.pool).await
        .map_err(|e| {
            tracing::error!("Failed to generate unique slug for community '{}': {}", trimmed_name, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Start transaction for atomic community creation + admin membership
    let mut tx = state.db.pool.begin().await
        .map_err(|e| {
            tracing::error!("Failed to begin database transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let community_id = Uuid::new_v4();
    
    // Insert community with detailed error handling
    let community_result = sqlx::query(
        "INSERT INTO communities (id, name, description, slug, is_public, requires_approval, created_by) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id, created_at"
    )
    .bind(community_id)
    .bind(trimmed_name)
    .bind(&payload.description.as_ref().map(|d| d.trim()).filter(|s| !s.is_empty()))
    .bind(&slug)
    .bind(payload.is_public.unwrap_or(true))
    .bind(payload.requires_approval.unwrap_or(false))
    .bind(creator_id)
    .fetch_one(&mut *tx)
    .await;
    
    let community_row = match community_result {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("Failed to create community '{}': {}", trimmed_name, e);
            
            // Check for specific database errors
            let error_msg = if e.to_string().contains("duplicate key") {
                "A community with this name or similar URL already exists. Please choose a different name."
            } else if e.to_string().contains("violates check constraint") {
                "Community name contains invalid characters. Please use letters, numbers, spaces, and basic punctuation."
            } else {
                "Failed to create community due to a database error. Please try again."
            };
            
            tx.rollback().await
                .map_err(|rollback_err| {
                    tracing::error!("Failed to rollback transaction after community creation error: {}", rollback_err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some(error_msg.to_string()),
            }));
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
            let role_creation_result = sqlx::query(
                "INSERT INTO roles (id, name, description, permissions, is_default) 
                 VALUES ($1, 'admin', 'Community administrator', '[\"manage_community\", \"manage_members\", \"manage_posts\"]', FALSE)"
            )
            .bind(new_role_id)
            .execute(&mut *tx)
            .await;
            
            if let Err(e) = role_creation_result {
                tracing::error!("Failed to create admin role: {}", e);
                tx.rollback().await
                    .map_err(|rollback_err| {
                        tracing::error!("Failed to rollback transaction after admin role creation error: {}", rollback_err);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                return Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to set up community permissions. Please try again.".to_string()),
                }));
            }
            new_role_id
        }
    };
    
    // Add creator as admin member with detailed error handling
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
                    tracing::error!("Failed to commit transaction for community '{}': {}", trimmed_name, e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            
            tracing::info!("Community '{}' (slug: {}) created successfully by user {}", trimmed_name, slug, user.user_id);
            
            let community = CommunityResponse {
                id: community_id.to_string(),
                name: trimmed_name.to_string(),
                description: payload.description.as_ref().map(|d| d.trim().to_string()).filter(|s| !s.is_empty()),
                created_at: community_row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                    .format("%Y-%m-%d %H:%M").to_string(),
            };
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(community),
                message: Some(format!("Community '{}' created successfully! You are now the administrator.", trimmed_name)),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to add creator as admin member for community '{}': {}", trimmed_name, e);
            tx.rollback().await
                .map_err(|rollback_err| {
                    tracing::error!("Failed to rollback transaction after membership creation error: {}", rollback_err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Community was created but failed to assign administrator permissions. Please contact support.".to_string()),
            }))
        }
    }
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

// ============================================================================
// Communities Endpoints
// ============================================================================

/// Get communities list with pagination, search, and filtering
/// Combines public communities and user's memberships
pub async fn get_communities(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CommunitiesQueryParams>,
    OptionalAuthUser(user): OptionalAuthUser, // Optional auth for enhanced results
) -> Result<Json<ApiResponse<CommunitiesListResponse>>, StatusCode> {
    use sqlx::Row;
    
    tracing::info!("📋 get_communities called - user: {:?}, page: {}, limit: {}, search: {:?}", 
        user.as_ref().map(|u| &u.user_id), params.page, params.limit, params.search);
    
    let offset = (params.page - 1) * params.limit;
    let search_param = params.search.as_deref().unwrap_or("");
    
    let sort_clause = match params.sort.as_deref() {
        Some("name") => "ORDER BY c.name ASC",
        Some("members") => "ORDER BY member_count DESC",
        Some("created") => "ORDER BY c.created_at DESC",
        _ => "ORDER BY c.created_at DESC", // Default
    };
    
    // Build query based on filter type - ALWAYS use parameterized queries
    let (query, count_query) = if let Some(ref user) = user {
        // Authenticated user - can see public + their memberships
        let main_query = format!(
            "SELECT c.id, c.name, c.description, c.slug, c.is_public, c.created_at,
                    COUNT(DISTINCT m.user_id) as member_count,
                    CASE WHEN m_user.user_id IS NOT NULL THEN m_user.role ELSE NULL END as user_role
             FROM communities c
             LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
             LEFT JOIN community_members m_user ON c.id = m_user.community_id AND m_user.user_id = $1
             WHERE (c.is_public = true OR m_user.user_id IS NOT NULL)
             AND ($2 = '' OR c.name ILIKE '%' || $2 || '%' OR c.description ILIKE '%' || $2 || '%')
             GROUP BY c.id, c.name, c.description, c.slug, c.is_public, c.created_at, m_user.role
             {} 
             LIMIT $3 OFFSET $4",
            sort_clause
        );
        
        let count_query = format!(
            "SELECT COUNT(DISTINCT c.id) as total
             FROM communities c
             LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
             LEFT JOIN community_members m_user ON c.id = m_user.community_id AND m_user.user_id = $1
             WHERE (c.is_public = true OR m_user.user_id IS NOT NULL)
             AND ($2 = '' OR c.name ILIKE '%' || $2 || '%' OR c.description ILIKE '%' || $2 || '%')"
        );
        
        (main_query, count_query)
    } else {
        // Unauthenticated user - can only see public communities
        let main_query = format!(
            "SELECT c.id, c.name, c.description, c.slug, c.is_public, c.created_at,
                    COUNT(DISTINCT m.user_id) as member_count,
                    NULL as user_role
             FROM communities c
             LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
             WHERE c.is_public = true
             AND ($1 = '' OR c.name ILIKE '%' || $1 || '%' OR c.description ILIKE '%' || $1 || '%')
             GROUP BY c.id, c.name, c.description, c.slug, c.is_public, c.created_at
             {} 
             LIMIT $2 OFFSET $3",
            sort_clause
        );
        
        let count_query = format!(
            "SELECT COUNT(DISTINCT c.id) as total
             FROM communities c
             WHERE c.is_public = true
             AND ($1 = '' OR c.name ILIKE '%' || $1 || '%' OR c.description ILIKE '%' || $1 || '%')"
        );
        
        (main_query, count_query)
    };
    
    tracing::info!("Executing communities query with user_id: {:?}", user.as_ref().map(|u| &u.user_id));
    
    // Execute query with proper parameter binding
    let communities = if let Some(ref _user) = user {
        sqlx::query(&query)
            .bind(&_user.user_id)
            .bind(search_param)
            .bind(params.limit as i64)
            .bind(offset as i64)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch communities: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    } else {
        sqlx::query(&query)
            .bind(search_param)
            .bind(params.limit as i64)
            .bind(offset as i64)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch communities: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };
    
    // Execute count query with proper parameter binding
    let total_count: i64 = if let Some(ref user) = user {
        sqlx::query_scalar(&count_query)
            .bind(&user.user_id)
            .bind(search_param)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to count communities: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    } else {
        sqlx::query_scalar(&count_query)
            .bind(search_param)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to count communities: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };
    
    // Convert to response format
    let communities_data: Vec<CommunityListResponse> = communities.iter().map(|row| {
        CommunityListResponse {
            id: row.get::<uuid::Uuid, _>("id").to_string(),
            name: row.get::<String, _>("name"),
            description: row.get::<Option<String>, _>("description"),
            slug: row.get::<String, _>("slug"),
            is_public: row.get::<bool, _>("is_public"),
            member_count: row.get::<i64, _>("member_count"),
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                .format("%Y-%m-%d %H:%M").to_string(),
            user_role: row.get::<Option<String>, _>("user_role"),
        }
    }).collect();
    
    let has_next = (params.page * params.limit) < total_count as u32;
    let has_prev = params.page > 1;
    
    let response = CommunitiesListResponse {
        communities: communities_data,
        total_count,
        page: params.page,
        limit: params.limit,
        has_next,
        has_prev,
    };
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some(format!("Found {} communities", total_count)),
    }))
}

/// Get community details by ID or slug
pub async fn get_community_detail(
    State(state): State<Arc<AppState>>,
    Path(community_id_or_slug): Path<String>,
    OptionalAuthUser(user): OptionalAuthUser, // Optional auth for member status
) -> Result<Json<ApiResponse<CommunityDetailResponse>>, StatusCode> {
    use sqlx::Row;
    
    // Try to parse as UUID first, otherwise treat as slug
    let community_uuid = uuid::Uuid::parse_str(&community_id_or_slug);
    
    // Build query based on whether we have UUID or slug - ALWAYS use parameterized queries
    let (query, _param_count) = if community_uuid.is_ok() {
        let main_query = format!(
            "SELECT c.id, c.name, c.description, c.slug, c.is_public, c.requires_approval, 
                    c.created_at, c.updated_at,
                    COUNT(DISTINCT m.user_id) as member_count,
                    COUNT(DISTINCT p.id) as posts_count,
                    CASE WHEN m_user.user_id IS NOT NULL THEN m_user.role ELSE NULL END as user_role,
                    CASE WHEN m_user.user_id IS NOT NULL THEN true ELSE false END as is_member
             FROM communities c
             LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
             LEFT JOIN posts p ON c.id = p.community_id
             LEFT JOIN community_members m_user ON c.id = m_user.community_id AND m_user.user_id = ${}
             WHERE c.id = ${} AND (c.is_public = true OR m_user.user_id IS NOT NULL)
             GROUP BY c.id, c.name, c.description, c.slug, c.is_public, c.requires_approval, 
                      c.created_at, c.updated_at, m_user.role, m_user.user_id",
            "$1", "$2"
        );
        (main_query, 2)
    } else {
        let main_query = format!(
            "SELECT c.id, c.name, c.description, c.slug, c.is_public, c.requires_approval, 
                    c.created_at, c.updated_at,
                    COUNT(DISTINCT m.user_id) as member_count,
                    COUNT(DISTINCT p.id) as posts_count,
                    CASE WHEN m_user.user_id IS NOT NULL THEN m_user.role ELSE NULL END as user_role,
                    CASE WHEN m_user.user_id IS NOT NULL THEN true ELSE false END as is_member
             FROM communities c
             LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
             LEFT JOIN posts p ON c.id = p.community_id
             LEFT JOIN community_members m_user ON c.id = m_user.community_id AND m_user.user_id = ${}
             WHERE c.slug = ${} AND (c.is_public = true OR m_user.user_id IS NOT NULL)
             GROUP BY c.id, c.name, c.description, c.slug, c.is_public, c.requires_approval, 
                      c.created_at, c.updated_at, m_user.role, m_user.user_id",
            "$1", "$2"
        );
        (main_query, 2)
    };
    
    tracing::info!("Executing community detail query: {}", query);
    
    let community = if let Some(ref user) = user {
        sqlx::query(&query)
            .bind(&community_id_or_slug)
            .bind(&user.user_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch community detail: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    } else {
        // For unauthenticated users, bind NULL for user_id
        sqlx::query(&query)
            .bind(&community_id_or_slug)
            .bind::<Option<String>>(None)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch community detail: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };
    
    let community_row = match community {
        Some(row) => row,
        None => {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Community not found or access denied".to_string()),
            }));
        }
    };
    
    let response = CommunityDetailResponse {
        id: community_row.get::<uuid::Uuid, _>("id").to_string(),
        name: community_row.get::<String, _>("name"),
        description: community_row.get::<Option<String>, _>("description"),
        slug: community_row.get::<String, _>("slug"),
        is_public: community_row.get::<bool, _>("is_public"),
        requires_approval: community_row.get::<bool, _>("requires_approval"),
        member_count: community_row.get::<i64, _>("member_count"),
        posts_count: community_row.get::<i64, _>("posts_count"),
        created_at: community_row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
            .format("%Y-%m-%d %H:%M").to_string(),
        updated_at: community_row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at")
            .format("%Y-%m-%d %H:%M").to_string(),
        user_role: community_row.get::<Option<String>, _>("user_role"),
        is_member: community_row.get::<bool, _>("is_member"),
    };
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Community details retrieved successfully".to_string()),
    }))
}

/// Update an existing community (PROTECTED - owner only)
pub async fn update_community(
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
    Json(payload): Json<CreateCommunityRequest>,
) -> Result<Json<ApiResponse<CommunityResponse>>, StatusCode> {
    // Parse community ID
    let community_uuid = Uuid::parse_str(&community_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Parse user ID
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if community exists and user is owner
    let community_owner: Option<Uuid> = sqlx::query_scalar(
        "SELECT created_by FROM communities WHERE id = $1"
    )
    .bind(community_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch community: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let owner_id = match community_owner {
        Some(id) => id,
        None => {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Community not found".to_string()),
            }));
        }
    };

    // Check authorization
    if owner_id != user_uuid {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("You don't have permission to update this community".to_string()),
        }));
    }

    // Validate input
    let trimmed_name = payload.name.trim();
    if trimmed_name.is_empty() || trimmed_name.len() < 3 || trimmed_name.len() > 255 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Community name must be between 3 and 255 characters".to_string()),
        }));
    }

    if let Some(ref description) = payload.description {
        if description.len() > 2000 {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Description must not exceed 2000 characters".to_string()),
            }));
        }
    }

    // Update community
    let result = sqlx::query(
        "UPDATE communities 
         SET name = $1, description = $2, is_public = $3, requires_approval = $4, updated_at = NOW()
         WHERE id = $5
         RETURNING id, name, description, created_at"
    )
    .bind(trimmed_name)
    .bind(&payload.description.as_ref().map(|d| d.trim()).filter(|s| !s.is_empty()))
    .bind(payload.is_public.unwrap_or(true))
    .bind(payload.requires_approval.unwrap_or(false))
    .bind(community_uuid)
    .fetch_one(&state.db.pool)
    .await;

    match result {
        Ok(row) => {
            tracing::info!("Community {} updated successfully by user {}", community_uuid, user.user_id);
            
            let community = CommunityResponse {
                id: row.get::<Uuid, _>("id").to_string(),
                name: row.get::<String, _>("name"),
                description: row.get::<Option<String>, _>("description"),
                created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                    .format("%Y-%m-%d %H:%M").to_string(),
            };
            
            Ok(Json(ApiResponse {
                success: true,
                data: Some(community),
                message: Some("Community updated successfully".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to update community: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to update community".to_string()),
            }))
        }
    }
}

/// Delete a community (PROTECTED - owner only)
pub async fn delete_community(
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), StatusCode> {
    // Parse community ID
    let community_uuid = Uuid::parse_str(&community_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Parse user ID
    let user_uuid = Uuid::parse_str(&user.user_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if community exists and user is owner
    let community_owner: Option<Uuid> = sqlx::query_scalar(
        "SELECT created_by FROM communities WHERE id = $1"
    )
    .bind(community_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch community: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let owner_id = match community_owner {
        Some(id) => id,
        None => {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Community not found".to_string()),
                }),
            ));
        }
    };

    // Check authorization
    if owner_id != user_uuid {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("You don't have permission to delete this community".to_string()),
            }),
        ));
    }

    // Delete community (CASCADE will handle related records)
    let result = sqlx::query("DELETE FROM communities WHERE id = $1")
        .bind(community_uuid)
        .execute(&state.db.pool)
        .await;

    match result {
        Ok(_) => {
            tracing::info!("Community {} deleted successfully by user {}", community_uuid, user.user_id);
            
            Ok((
                StatusCode::NO_CONTENT,
                Json(ApiResponse {
                    success: true,
                    data: None,
                    message: Some("Community deleted successfully".to_string()),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to delete community: {}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to delete community".to_string()),
                }),
            ))
        }
    }
}
