use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::{AuthUser, OptionalAuthUser};
use crate::handlers::pages::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateCommunityRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub requires_approval: Option<bool>,
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
    pub name: Option<String>, // From user_profiles, not users table
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
    pub sort: Option<String>, // "created", "name", "members"
}

fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    20
}

// ============================================================================
// User Endpoints
// ============================================================================

/// Get all users
pub async fn get_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    use sqlx::Row;

    let users = sqlx::query(
        "SELECT u.id, u.email, u.created_at, p.name 
         FROM users u 
         LEFT JOIN user_profiles p ON u.id = p.user_id 
         ORDER BY u.created_at DESC",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users_data: Vec<UserResponse> = users
        .iter()
        .map(|row| UserResponse {
            id: row.get::<Uuid, _>("id").to_string(),
            name: row.get::<Option<String>, _>("name"),
            email: row.get::<String, _>("email"),
            created_at: row
                .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                .format("%Y-%m-%d %H:%M")
                .to_string(),
        })
        .collect();

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
) -> Result<(StatusCode, Json<ApiResponse<CommunityResponse>>), StatusCode> {
    // Validate name: 3-100 characters
    let trimmed_name = payload.name.trim();

    if trimmed_name.len() < 3 || trimmed_name.len() > 100 {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Community name must be between 3 and 100 characters".to_string()),
            }),
        ));
    }

    // Validate description: max 1000 characters (per guidelines)
    if let Some(ref description) = payload.description {
        if description.len() > 1000 {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Description must not exceed 1000 characters".to_string()),
                }),
            ));
        }
    }

    // Validate slug: 3-50 chars, lowercase, alphanumeric + hyphens
    let trimmed_slug = payload.slug.trim();
    if trimmed_slug.len() < 3 || trimmed_slug.len() > 50 {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Slug must be between 3 and 50 characters".to_string()),
            }),
        ));
    }

    // Validate slug format: lowercase, alphanumeric + hyphens only
    if !trimmed_slug
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some(
                    "Slug must contain only lowercase letters, numbers, and hyphens".to_string(),
                ),
            }),
        ));
    }

    // Parse authenticated user ID
    let creator_id = Uuid::parse_str(&user.user_id).map_err(|e| {
        tracing::error!("Invalid user ID format: {} - Error: {}", user.user_id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Check slug uniqueness - return 409 Conflict if exists
    let slug_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM communities WHERE slug = $1)")
            .bind(trimmed_slug)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check slug uniqueness: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    if slug_exists {
        return Ok((
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("A community with this slug already exists".to_string()),
            }),
        ));
    }

    // Start transaction for atomic community creation + admin membership
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin database transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Generate UUIDv7 for community ID (time-ordered, globally unique for federation)
    let community_id = Uuid::now_v7();

    // Insert community with UUIDv7 ID
    let community_result = sqlx::query(
        "INSERT INTO communities (id, name, description, slug, is_public, created_by, created_at, updated_at) 
         VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
         RETURNING created_at"
    )
    .bind(community_id)
    .bind(trimmed_name)
    .bind(payload.description.as_ref().map(|d| d.trim()).filter(|s| !s.is_empty()))
    .bind(trimmed_slug)
    .bind(payload.is_public.unwrap_or(true))
    .bind(creator_id)
    .fetch_one(&mut *tx)
    .await;

    let community_row = match community_result {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("Failed to create community '{}': {}", trimmed_name, e);
            tx.rollback().await.map_err(|rollback_err| {
                tracing::error!("Failed to rollback transaction: {}", rollback_err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to create community".to_string()),
                }),
            ));
        }
    };

    // Add creator as admin member (using ENUM directly)
    let membership_result = sqlx::query(
        "INSERT INTO community_members (user_id, community_id, role, status, joined_at) 
         VALUES ($1, $2, 'admin', 'active', NOW())",
    )
    .bind(creator_id)
    .bind(community_id)
    .execute(&mut *tx)
    .await;

    match membership_result {
        Ok(_) => {
            // Commit transaction
            tx.commit().await.map_err(|e| {
                tracing::error!("Failed to commit transaction: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            tracing::info!(
                "Community '{}' (slug: {}) created successfully by user {}",
                trimmed_name,
                trimmed_slug,
                user.user_id
            );

            let community = CommunityResponse {
                id: community_id.to_string(), // Convert i64 to String for JSON response
                name: trimmed_name.to_string(),
                description: payload
                    .description
                    .as_ref()
                    .map(|d| d.trim().to_string())
                    .filter(|s| !s.is_empty()),
                created_at: community_row
                    .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
            };

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse {
                    success: true,
                    data: Some(community),
                    message: Some(format!("Community '{}' created successfully", trimmed_name)),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to add creator as admin member: {}", e);
            tx.rollback().await.map_err(|rollback_err| {
                tracing::error!("Failed to rollback transaction: {}", rollback_err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to assign administrator permissions".to_string()),
                }),
            ))
        }
    }
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

    tracing::info!(
        "📋 get_communities called - user: {:?}, page: {}, limit: {}, search: {:?}",
        user.as_ref().map(|u| &u.user_id),
        params.page,
        params.limit,
        params.search
    );

    let offset = (params.page - 1) * params.limit;
    let search_param = params.search.as_deref().unwrap_or("");

    let sort_clause = match params.sort.as_deref() {
        Some("name") => "ORDER BY c.name ASC",
        Some("members") => "ORDER BY member_count DESC",
        Some("created") => "ORDER BY c.created_at DESC",
        _ => "ORDER BY c.created_at DESC", // Default
    };

    // Build query based on filter type - ALWAYS use parameterized queries
    let (query, count_query) = if let Some(ref _user) = user {
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

        let count_query = "SELECT COUNT(DISTINCT c.id) as total
             FROM communities c
             LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
             LEFT JOIN community_members m_user ON c.id = m_user.community_id AND m_user.user_id = $1
             WHERE (c.is_public = true OR m_user.user_id IS NOT NULL)
             AND ($2 = '' OR c.name ILIKE '%' || $2 || '%' OR c.description ILIKE '%' || $2 || '%')".to_string();

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

        let count_query = "SELECT COUNT(DISTINCT c.id) as total
             FROM communities c
             WHERE c.is_public = true
             AND ($1 = '' OR c.name ILIKE '%' || $1 || '%' OR c.description ILIKE '%' || $1 || '%')".to_string();

        (main_query, count_query)
    };

    tracing::info!(
        "Executing communities query with user_id: {:?}",
        user.as_ref().map(|u| &u.user_id)
    );

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
    let communities_data: Vec<CommunityListResponse> = communities
        .iter()
        .map(|row| CommunityListResponse {
            id: row.get::<uuid::Uuid, _>("id").to_string(),
            name: row.get::<String, _>("name"),
            description: row.get::<Option<String>, _>("description"),
            slug: row.get::<String, _>("slug"),
            is_public: row.get::<bool, _>("is_public"),
            member_count: row.get::<i64, _>("member_count"),
            created_at: row
                .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                .format("%Y-%m-%d %H:%M")
                .to_string(),
            user_role: row.get::<Option<String>, _>("user_role"),
        })
        .collect();

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
        let user_uuid = Uuid::parse_str(&user.user_id).ok();
        if let Ok(uuid) = community_uuid {
            // Bind as UUID
            sqlx::query(&query)
                .bind(user_uuid)
                .bind(uuid)
                .fetch_optional(&state.db.pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to fetch community detail: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?
        } else {
            // Bind as slug string
            sqlx::query(&query)
                .bind(user_uuid)
                .bind(&community_id_or_slug)
                .fetch_optional(&state.db.pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to fetch community detail: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?
        }
    } else {
        // For unauthenticated users, bind NULL for user_id
        if let Ok(uuid) = community_uuid {
            sqlx::query(&query)
                .bind::<Option<Uuid>>(None)
                .bind(uuid)
                .fetch_optional(&state.db.pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to fetch community detail: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?
        } else {
            sqlx::query(&query)
                .bind::<Option<Uuid>>(None)
                .bind(&community_id_or_slug)
                .fetch_optional(&state.db.pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to fetch community detail: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?
        }
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
        created_at: community_row
            .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
            .format("%Y-%m-%d %H:%M")
            .to_string(),
        updated_at: community_row
            .get::<chrono::DateTime<chrono::Utc>, _>("updated_at")
            .format("%Y-%m-%d %H:%M")
            .to_string(),
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
    Path(community_id): Path<Uuid>,
    Json(payload): Json<CreateCommunityRequest>,
) -> Result<Json<ApiResponse<CommunityResponse>>, StatusCode> {
    // Parse user ID (UUID from Auth0)
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if community exists and user is owner
    let community_owner: Option<Uuid> =
        sqlx::query_scalar("SELECT created_by FROM communities WHERE id = $1")
            .bind(community_id)
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
         RETURNING id, name, description, created_at",
    )
    .bind(trimmed_name)
    .bind(
        payload
            .description
            .as_ref()
            .map(|d| d.trim())
            .filter(|s| !s.is_empty()),
    )
    .bind(payload.is_public.unwrap_or(true))
    .bind(payload.requires_approval.unwrap_or(false))
    .bind(community_id)
    .fetch_one(&state.db.pool)
    .await;

    match result {
        Ok(row) => {
            tracing::info!(
                "Community {} updated successfully by user {}",
                community_id,
                user.user_id
            );

            let community = CommunityResponse {
                id: row.get::<Uuid, _>("id").to_string(),
                name: row.get::<String, _>("name"),
                description: row.get::<Option<String>, _>("description"),
                created_at: row
                    .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
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
    Path(community_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), StatusCode> {
    // Parse user ID (UUID from Auth0)
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if community exists and user is owner
    let community_owner: Option<Uuid> =
        sqlx::query_scalar("SELECT created_by FROM communities WHERE id = $1")
            .bind(community_id)
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
        .bind(community_id)
        .execute(&state.db.pool)
        .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "Community {} deleted successfully by user {}",
                community_id,
                user.user_id
            );

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

// ============================================================================
// Membership Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String, // "admin", "moderator", "member"
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub user_id: String,
    pub email: String,
    pub role: String,
    pub joined_at: String,
}

#[derive(Debug, Serialize)]
pub struct MembersListResponse {
    pub members: Vec<MemberResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct MembershipResponse {
    pub community_id: String,
    pub role: String,
    pub joined_at: String,
}

// ============================================================================
// Membership Endpoints
// ============================================================================

/// Join a community (PROTECTED - requires authentication)
pub async fn join_community(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<MembershipResponse>>), StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check community exists and is public
    let community: Option<(bool, bool)> =
        sqlx::query_as("SELECT is_public, requires_approval FROM communities WHERE id = $1")
            .bind(community_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch community: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    let (is_public, _requires_approval) = match community {
        Some(c) => c,
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

    if !is_public {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Cannot join private community".to_string()),
            }),
        ));
    }

    // Check user not already member
    let existing: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM community_members WHERE community_id = $1 AND user_id = $2",
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check membership: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if existing.is_some() {
        return Ok((
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("User already member of this community".to_string()),
            }),
        ));
    }

    // Insert membership (using ENUM directly)
    let result = sqlx::query(
        "INSERT INTO community_members (user_id, community_id, role, status, joined_at)
         VALUES ($1, $2, 'member', 'active', NOW())
         RETURNING joined_at",
    )
    .bind(user_uuid)
    .bind(community_id)
    .fetch_one(&state.db.pool)
    .await;

    match result {
        Ok(row) => {
            tracing::info!("User {} joined community {}", user.user_id, community_id);

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse {
                    success: true,
                    data: Some(MembershipResponse {
                        community_id: community_id.to_string(),
                        role: "member".to_string(),
                        joined_at: row
                            .get::<chrono::DateTime<chrono::Utc>, _>("joined_at")
                            .format("%Y-%m-%d %H:%M")
                            .to_string(),
                    }),
                    message: Some("Successfully joined community".to_string()),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to join community: {}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to join community".to_string()),
                }),
            ))
        }
    }
}

/// Leave a community (PROTECTED - requires authentication)
pub async fn leave_community(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check user is member
    let membership: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM community_members WHERE community_id = $1 AND user_id = $2",
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check membership: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if membership.is_none() {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("User is not a member of this community".to_string()),
            }),
        ));
    }

    // Check if user is the only admin/owner
    let admin_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM community_members
         WHERE community_id = $1 AND role IN ('admin', 'owner')",
    )
    .bind(community_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to count admins: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if admin_count == 1 {
        // Check if this user is the admin/owner
        let is_admin: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM community_members
                WHERE community_id = $1 AND user_id = $2 AND role IN ('admin', 'owner')
            )",
        )
        .bind(community_id)
        .bind(user_uuid)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check admin status: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if is_admin {
            return Ok((
                StatusCode::FORBIDDEN,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some(
                        "Cannot leave: you are the only admin. Transfer ownership first."
                            .to_string(),
                    ),
                }),
            ));
        }
    }

    // Delete membership
    let result =
        sqlx::query("DELETE FROM community_members WHERE community_id = $1 AND user_id = $2")
            .bind(community_id)
            .bind(user_uuid)
            .execute(&state.db.pool)
            .await;

    match result {
        Ok(_) => {
            tracing::info!("User {} left community {}", user.user_id, community_id);

            Ok((
                StatusCode::NO_CONTENT,
                Json(ApiResponse {
                    success: true,
                    data: None,
                    message: Some("Successfully left community".to_string()),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to leave community: {}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to leave community".to_string()),
                }),
            ))
        }
    }
}

/// List community members (PROTECTED for private communities)
pub async fn list_members(
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
) -> Result<Json<ApiResponse<MembersListResponse>>, StatusCode> {
    // Check community exists and access
    let is_public: bool = sqlx::query_scalar("SELECT is_public FROM communities WHERE id = $1")
        .bind(community_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch community: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Check access for private communities
    if !is_public {
        let user_uuid = match user {
            Some(u) => {
                Uuid::parse_str(&u.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            }
            None => {
                return Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some(
                        "Cannot view members of private community without authentication"
                            .to_string(),
                    ),
                }));
            }
        };

        let is_member: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2)"
        )
        .bind(community_id)
        .bind(user_uuid)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check membership: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if !is_member {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("You are not a member of this community".to_string()),
            }));
        }
    }

    // Fetch members
    let rows = sqlx::query(
        "SELECT cm.user_id, u.email, cm.role::text as role, cm.joined_at
         FROM community_members cm
         JOIN users u ON cm.user_id = u.id
         WHERE cm.community_id = $1 AND cm.status = 'active'
         ORDER BY cm.joined_at DESC",
    )
    .bind(community_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch members: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = rows.len() as i64;
    let members: Vec<MemberResponse> = rows
        .iter()
        .map(|row| MemberResponse {
            user_id: row.get::<Uuid, _>("user_id").to_string(),
            email: row.get::<String, _>("email"),
            role: row.get::<String, _>("role"),
            joined_at: row
                .get::<chrono::DateTime<chrono::Utc>, _>("joined_at")
                .format("%Y-%m-%d %H:%M")
                .to_string(),
        })
        .collect();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(MembersListResponse { members, total }),
        message: Some(format!("Found {} members", total)),
    }))
}

/// Update member role (PROTECTED - admin only)
pub async fn update_member_role(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path((community_id, member_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check requester is admin/owner
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM community_members
            WHERE community_id = $1 AND user_id = $2 AND role IN ('admin', 'owner')
        )",
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check admin status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !is_admin {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Only admins can update member roles".to_string()),
        }));
    }

    // Check target user is member
    let member_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2)",
    )
    .bind(community_id)
    .bind(member_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check membership: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !member_exists {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("User is not a member of this community".to_string()),
        }));
    }

    // Validate role is valid ENUM value
    let valid_roles = ["owner", "admin", "moderator", "member"];
    if !valid_roles.contains(&payload.role.as_str()) {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!(
                "Invalid role: {}. Valid roles: owner, admin, moderator, member",
                payload.role
            )),
        }));
    }

    // Update role (using ENUM directly)
    let result = sqlx::query(
        "UPDATE community_members SET role = $1::member_role WHERE community_id = $2 AND user_id = $3"
    )
    .bind(&payload.role)
    .bind(community_id)
    .bind(member_id)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "Updated role for user {} in community {} to {}",
                member_id,
                community_id,
                payload.role
            );

            Ok(Json(ApiResponse {
                success: true,
                data: None,
                message: Some(format!("Member role updated to {}", payload.role)),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to update member role: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to update member role".to_string()),
            }))
        }
    }
}

/// Request to join a private community (PROTECTED - requires authentication)
pub async fn request_join_community(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check community exists and requires approval
    let community: Option<(bool, bool)> =
        sqlx::query_as("SELECT is_public, requires_approval FROM communities WHERE id = $1")
            .bind(community_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch community: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    let (is_public, requires_approval) = match community {
        Some(c) => c,
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

    // Can only request join for private communities that require approval
    if is_public {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some(
                    "Cannot request join for public community. Use /join instead.".to_string(),
                ),
            }),
        ));
    }

    if !requires_approval {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some(
                    "This private community does not require approval. Contact owner for invite."
                        .to_string(),
                ),
            }),
        ));
    }

    // Check user not already member or pending
    let existing: Option<String> = sqlx::query_scalar(
        "SELECT status FROM community_members WHERE community_id = $1 AND user_id = $2",
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check membership: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match existing {
        Some(status) if status == "active" => {
            return Ok((
                StatusCode::CONFLICT,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("User already member of this community".to_string()),
                }),
            ));
        }
        Some(status) if status == "pending" => {
            return Ok((
                StatusCode::CONFLICT,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Join request already pending".to_string()),
                }),
            ));
        }
        _ => {}
    }

    // Get member role ID
    let member_role_id: i64 =
        sqlx::query_scalar("SELECT id FROM roles WHERE name = 'member' LIMIT 1")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get member role: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    // Insert membership with 'pending' status
    let result = sqlx::query(
        "INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
         VALUES ($1, $2, $3, 'pending', NOW())
         RETURNING joined_at",
    )
    .bind(user_uuid)
    .bind(community_id)
    .bind(member_role_id)
    .fetch_one(&state.db.pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "User {} requested to join community {}",
                user.user_id,
                community_id
            );

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse {
                    success: true,
                    data: None,
                    message: Some("Join request submitted. Awaiting admin approval.".to_string()),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to request join: {}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to submit join request".to_string()),
                }),
            ))
        }
    }
}

/// Approve join request (PROTECTED - admin only)
pub async fn approve_join_request(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path((community_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check requester is admin
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM community_members cm
            JOIN roles r ON cm.role_id = r.id
            WHERE cm.community_id = $1 AND cm.user_id = $2 AND r.name = 'admin'
        )",
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check admin status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !is_admin {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Only admins can approve join requests".to_string()),
        }));
    }

    // Check request exists and is pending
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM community_members WHERE community_id = $1 AND user_id = $2",
    )
    .bind(community_id)
    .bind(member_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check request status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match status {
        Some(s) if s == "pending" => {}
        Some(s) if s == "active" => {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("User is already an active member".to_string()),
            }));
        }
        _ => {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("No pending join request found".to_string()),
            }));
        }
    }

    // Update status to active
    let result = sqlx::query(
        "UPDATE community_members SET status = 'active' WHERE community_id = $1 AND user_id = $2",
    )
    .bind(community_id)
    .bind(member_id)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "Admin {} approved join request for user {} in community {}",
                user.user_id,
                member_id,
                community_id
            );

            Ok(Json(ApiResponse {
                success: true,
                data: None,
                message: Some("Join request approved".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to approve join request: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to approve join request".to_string()),
            }))
        }
    }
}

/// Reject join request (PROTECTED - admin only)
pub async fn reject_join_request(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path((community_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check requester is admin
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM community_members cm
            JOIN roles r ON cm.role_id = r.id
            WHERE cm.community_id = $1 AND cm.user_id = $2 AND r.name = 'admin'
        )",
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check admin status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !is_admin {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Only admins can reject join requests".to_string()),
            }),
        ));
    }

    // Check request exists and is pending
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM community_members WHERE community_id = $1 AND user_id = $2",
    )
    .bind(community_id)
    .bind(member_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check request status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match status {
        Some(s) if s == "pending" => {}
        _ => {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("No pending join request found".to_string()),
                }),
            ));
        }
    }

    // Delete the pending request
    let result =
        sqlx::query("DELETE FROM community_members WHERE community_id = $1 AND user_id = $2")
            .bind(community_id)
            .bind(member_id)
            .execute(&state.db.pool)
            .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "Admin {} rejected join request for user {} in community {}",
                user.user_id,
                member_id,
                community_id
            );

            Ok((
                StatusCode::NO_CONTENT,
                Json(ApiResponse {
                    success: true,
                    data: None,
                    message: Some("Join request rejected".to_string()),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to reject join request: {}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to reject join request".to_string()),
                }),
            ))
        }
    }
}

/// Remove member from community (PROTECTED - admin only)
pub async fn remove_member(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path((community_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check requester is admin
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM community_members cm
            JOIN roles r ON cm.role_id = r.id
            WHERE cm.community_id = $1 AND cm.user_id = $2 AND r.name = 'admin'
        )",
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check admin status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !is_admin {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Only admins can remove members".to_string()),
            }),
        ));
    }

    // Delete membership
    let result =
        sqlx::query("DELETE FROM community_members WHERE community_id = $1 AND user_id = $2")
            .bind(community_id)
            .bind(member_id)
            .execute(&state.db.pool)
            .await;

    match result {
        Ok(rows) => {
            if rows.rows_affected() == 0 {
                return Ok((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        message: Some("User is not a member of this community".to_string()),
                    }),
                ));
            }

            tracing::info!(
                "Admin {} removed user {} from community {}",
                user.user_id,
                member_id,
                community_id
            );

            Ok((
                StatusCode::NO_CONTENT,
                Json(ApiResponse {
                    success: true,
                    data: None,
                    message: Some("Member removed successfully".to_string()),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to remove member: {}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to remove member".to_string()),
                }),
            ))
        }
    }
}

// ============================================================================
// Discovery Endpoints
// ============================================================================

/// Get user's communities (PROTECTED - requires authentication)
pub async fn get_my_communities(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Query(params): Query<CommunitiesQueryParams>,
) -> Result<Json<ApiResponse<CommunitiesListResponse>>, StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let offset = ((params.page.saturating_sub(1)) * params.limit) as i64;
    let search_param = params.search.as_deref().unwrap_or("").to_string();

    // Fetch user's communities (both created and member of)
    let communities = sqlx::query(
        "SELECT DISTINCT c.id, c.name, c.description, c.slug, c.is_public, c.created_at,
                COUNT(DISTINCT m.user_id) as member_count,
                CASE WHEN c.created_by = $1 THEN 'owner' 
                     WHEN cm.role_id = (SELECT id FROM roles WHERE name = 'admin') THEN 'admin'
                     ELSE 'member' END as user_role
         FROM communities c
         LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
         LEFT JOIN community_members cm ON c.id = cm.community_id AND cm.user_id = $1 AND cm.status = 'active'
         WHERE (c.created_by = $1 OR cm.user_id = $1)
         AND ($2 = '' OR c.name ILIKE '%' || $2 || '%' OR c.description ILIKE '%' || $2 || '%')
         GROUP BY c.id, c.name, c.description, c.slug, c.is_public, c.created_at, c.created_by, cm.role_id
         ORDER BY c.created_at DESC
         LIMIT $3 OFFSET $4"
    )
    .bind(user_uuid)
    .bind(&search_param)
    .bind(params.limit as i64)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch user communities: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get total count
    let total_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT c.id)
         FROM communities c
         LEFT JOIN community_members cm ON c.id = cm.community_id AND cm.user_id = $1 AND cm.status = 'active'
         WHERE (c.created_by = $1 OR cm.user_id = $1)
         AND ($2 = '' OR c.name ILIKE '%' || $2 || '%' OR c.description ILIKE '%' || $2 || '%')"
    )
    .bind(user_uuid)
    .bind(&search_param)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to count user communities: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let communities_data: Vec<CommunityListResponse> = communities
        .iter()
        .map(|row| CommunityListResponse {
            id: row.get::<uuid::Uuid, _>("id").to_string(),
            name: row.get::<String, _>("name"),
            description: row.get::<Option<String>, _>("description"),
            slug: row.get::<String, _>("slug"),
            is_public: row.get::<bool, _>("is_public"),
            member_count: row.get::<i64, _>("member_count"),
            created_at: row
                .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                .format("%Y-%m-%d %H:%M")
                .to_string(),
            user_role: row.get::<Option<String>, _>("user_role"),
        })
        .collect();

    let has_next = (params.page * params.limit) < total_count as u32;
    let has_prev = params.page > 1;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(CommunitiesListResponse {
            communities: communities_data,
            total_count,
            page: params.page,
            limit: params.limit,
            has_next,
            has_prev,
        }),
        message: Some(format!("Found {} communities", total_count)),
    }))
}

/// Get trending communities (public, sorted by member count)
pub async fn get_trending_communities(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CommunitiesQueryParams>,
) -> Result<Json<ApiResponse<CommunitiesListResponse>>, StatusCode> {
    let offset = ((params.page.saturating_sub(1)) * params.limit) as i64;
    let search_param = params.search.as_deref().unwrap_or("").to_string();

    // Fetch trending communities (public, sorted by member count)
    let communities = sqlx::query(
        "SELECT c.id, c.name, c.description, c.slug, c.is_public, c.created_at,
                COUNT(DISTINCT m.user_id) as member_count,
                NULL as user_role
         FROM communities c
         LEFT JOIN community_members m ON c.id = m.community_id AND m.status = 'active'
         WHERE c.is_public = true
         AND ($1 = '' OR c.name ILIKE '%' || $1 || '%' OR c.description ILIKE '%' || $1 || '%')
         GROUP BY c.id, c.name, c.description, c.slug, c.is_public, c.created_at
         ORDER BY member_count DESC, c.created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(&search_param)
    .bind(params.limit as i64)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch trending communities: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get total count
    let total_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT c.id)
         FROM communities c
         WHERE c.is_public = true
         AND ($1 = '' OR c.name ILIKE '%' || $1 || '%' OR c.description ILIKE '%' || $1 || '%')",
    )
    .bind(&search_param)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to count trending communities: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let communities_data: Vec<CommunityListResponse> = communities
        .iter()
        .map(|row| CommunityListResponse {
            id: row.get::<uuid::Uuid, _>("id").to_string(),
            name: row.get::<String, _>("name"),
            description: row.get::<Option<String>, _>("description"),
            slug: row.get::<String, _>("slug"),
            is_public: row.get::<bool, _>("is_public"),
            member_count: row.get::<i64, _>("member_count"),
            created_at: row
                .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                .format("%Y-%m-%d %H:%M")
                .to_string(),
            user_role: row.get::<Option<String>, _>("user_role"),
        })
        .collect();

    let has_next = (params.page * params.limit) < total_count as u32;
    let has_prev = params.page > 1;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(CommunitiesListResponse {
            communities: communities_data,
            total_count,
            page: params.page,
            limit: params.limit,
            has_next,
            has_prev,
        }),
        message: Some(format!("Found {} trending communities", total_count)),
    }))
}

// ============================================================================
// Owner/Admin Management Endpoints
// ============================================================================

/// Transfer community ownership (PROTECTED - owner only)
pub async fn transfer_ownership(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path((community_id, new_owner_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check requester is owner
    let is_owner: bool =
        sqlx::query_scalar("SELECT created_by = $1 FROM communities WHERE id = $2")
            .bind(user_uuid)
            .bind(community_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check ownership: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .unwrap_or(false);

    if !is_owner {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Only community owner can transfer ownership".to_string()),
        }));
    }

    // Check new owner is member
    let is_member: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active')"
    )
    .bind(community_id)
    .bind(new_owner_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check membership: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !is_member {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("New owner must be an active member of the community".to_string()),
        }));
    }

    // Get admin role ID
    let admin_role_id: i64 =
        sqlx::query_scalar("SELECT id FROM roles WHERE name = 'admin' LIMIT 1")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get admin role: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    // Update new owner to admin role
    sqlx::query(
        "UPDATE community_members SET role_id = $1 WHERE community_id = $2 AND user_id = $3",
    )
    .bind(admin_role_id)
    .bind(community_id)
    .bind(new_owner_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update new owner role: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Transfer ownership in communities table
    let result =
        sqlx::query("UPDATE communities SET created_by = $1, updated_at = NOW() WHERE id = $2")
            .bind(new_owner_id)
            .bind(community_id)
            .execute(&state.db.pool)
            .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "Owner {} transferred community {} to {}",
                user.user_id,
                community_id,
                new_owner_id
            );

            Ok(Json(ApiResponse {
                success: true,
                data: None,
                message: Some("Ownership transferred successfully".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to transfer ownership: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to transfer ownership".to_string()),
            }))
        }
    }
}

/// Promote member to admin (PROTECTED - owner only)
pub async fn promote_to_admin(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path((community_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check requester is owner
    let is_owner: bool =
        sqlx::query_scalar("SELECT created_by = $1 FROM communities WHERE id = $2")
            .bind(user_uuid)
            .bind(community_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check ownership: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .unwrap_or(false);

    if !is_owner {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Only community owner can promote members".to_string()),
        }));
    }

    // Check target is member
    let is_member: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active')"
    )
    .bind(community_id)
    .bind(member_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check membership: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !is_member {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("User is not an active member of this community".to_string()),
        }));
    }

    // Get admin role ID
    let admin_role_id: i64 =
        sqlx::query_scalar("SELECT id FROM roles WHERE name = 'admin' LIMIT 1")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get admin role: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    // Update role to admin
    let result = sqlx::query(
        "UPDATE community_members SET role_id = $1 WHERE community_id = $2 AND user_id = $3",
    )
    .bind(admin_role_id)
    .bind(community_id)
    .bind(member_id)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "Owner {} promoted user {} to admin in community {}",
                user.user_id,
                member_id,
                community_id
            );

            Ok(Json(ApiResponse {
                success: true,
                data: None,
                message: Some("Member promoted to admin".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to promote member: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to promote member".to_string()),
            }))
        }
    }
}

/// Demote admin to member (PROTECTED - owner only)
pub async fn demote_to_member(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path((community_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid =
        Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check requester is owner
    let is_owner: bool =
        sqlx::query_scalar("SELECT created_by = $1 FROM communities WHERE id = $2")
            .bind(user_uuid)
            .bind(community_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check ownership: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .unwrap_or(false);

    if !is_owner {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Only community owner can demote members".to_string()),
        }));
    }

    // Check target is not the owner
    let is_target_owner: bool =
        sqlx::query_scalar("SELECT created_by = $1 FROM communities WHERE id = $2")
            .bind(member_id)
            .bind(community_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check if target is owner: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .unwrap_or(false);

    if is_target_owner {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Cannot demote the community owner".to_string()),
        }));
    }

    // Get member role ID
    let member_role_id: i64 =
        sqlx::query_scalar("SELECT id FROM roles WHERE name = 'member' LIMIT 1")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get member role: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    // Update role to member
    let result = sqlx::query(
        "UPDATE community_members SET role_id = $1 WHERE community_id = $2 AND user_id = $3",
    )
    .bind(member_role_id)
    .bind(community_id)
    .bind(member_id)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "Owner {} demoted user {} to member in community {}",
                user.user_id,
                member_id,
                community_id
            );

            Ok(Json(ApiResponse {
                success: true,
                data: None,
                message: Some("Admin demoted to member".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to demote admin: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to demote admin".to_string()),
            }))
        }
    }
}

// =============================================================================
// USER PROFILE API
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_image: Option<String>,
    pub is_public: Option<bool>,
}

/// Update user profile (PROTECTED - own profile only)
pub async fn update_profile(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Only allow updating own profile
    if user.user_id != user_id {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("You can only update your own profile".to_string()),
        }));
    }

    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Upsert profile
    let result = sqlx::query(
        r#"INSERT INTO user_profiles (user_id, name, bio, location, website, avatar_url, cover_image, is_public, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
           ON CONFLICT (user_id) DO UPDATE SET
               name = COALESCE($2, user_profiles.name),
               bio = COALESCE($3, user_profiles.bio),
               location = COALESCE($4, user_profiles.location),
               website = COALESCE($5, user_profiles.website),
               avatar_url = COALESCE($6, user_profiles.avatar_url),
               cover_image = COALESCE($7, user_profiles.cover_image),
               is_public = COALESCE($8, user_profiles.is_public),
               updated_at = NOW()"#
    )
    .bind(user_uuid)
    .bind(&payload.name)
    .bind(&payload.bio)
    .bind(&payload.location)
    .bind(&payload.website)
    .bind(&payload.avatar_url)
    .bind(&payload.cover_image)
    .bind(payload.is_public)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!("User {} updated their profile", user.user_id);
            Ok(Json(ApiResponse {
                success: true,
                data: None,
                message: Some("Profile updated successfully".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to update profile: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to update profile".to_string()),
            }))
        }
    }
}

// =============================================================================
// FOLLOW SYSTEM API
// =============================================================================

/// Follow a user (PROTECTED)
pub async fn follow_user(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(target_user_id): Path<String>,
) -> Result<axum::response::Html<String>, StatusCode> {
    let follower_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let following_uuid = Uuid::parse_str(&target_user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Can't follow yourself
    if follower_uuid == following_uuid {
        return Ok(axum::response::Html(
            "<div class=\"text-red-500\">Non puoi seguire te stesso</div>".to_string(),
        ));
    }

    // Insert follow relationship
    let result = sqlx::query(
        "INSERT INTO user_follows (follower_id, following_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
    )
    .bind(follower_uuid)
    .bind(following_uuid)
    .execute(&state.db.pool)
    .await;

    if let Err(e) = result {
        tracing::error!("Failed to follow user: {}", e);
        return Ok(axum::response::Html(
            "<div class=\"text-red-500\">Errore</div>".to_string(),
        ));
    }

    // Update follower counts
    let _ = sqlx::query(
        "UPDATE user_profiles SET follower_count = follower_count + 1 WHERE user_id = $1",
    )
    .bind(following_uuid)
    .execute(&state.db.pool)
    .await;
    let _ = sqlx::query(
        "UPDATE user_profiles SET following_count = following_count + 1 WHERE user_id = $1",
    )
    .bind(follower_uuid)
    .execute(&state.db.pool)
    .await;

    tracing::info!("User {} followed user {}", user.user_id, target_user_id);

    // Return updated follow button (now showing "following")
    Ok(axum::response::Html(format!(
        r#"
        <button hx-post="/api/users/{}/unfollow"
                hx-target="this"
                hx-swap="outerHTML"
                class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 hover:border-red-300 hover:text-red-600 transition text-sm font-medium group">
            <span class="group-hover:hidden">Seguendo</span>
            <span class="hidden group-hover:inline">Smetti di seguire</span>
        </button>
    "#,
        target_user_id
    )))
}

/// Unfollow a user (PROTECTED)
pub async fn unfollow_user(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(target_user_id): Path<String>,
) -> Result<axum::response::Html<String>, StatusCode> {
    let follower_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let following_uuid = Uuid::parse_str(&target_user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Delete follow relationship
    let result =
        sqlx::query("DELETE FROM user_follows WHERE follower_id = $1 AND following_id = $2")
            .bind(follower_uuid)
            .bind(following_uuid)
            .execute(&state.db.pool)
            .await;

    if let Err(e) = result {
        tracing::error!("Failed to unfollow user: {}", e);
        return Ok(axum::response::Html(
            "<div class=\"text-red-500\">Errore</div>".to_string(),
        ));
    }

    // Update follower counts
    let _ = sqlx::query("UPDATE user_profiles SET follower_count = GREATEST(follower_count - 1, 0) WHERE user_id = $1")
        .bind(following_uuid)
        .execute(&state.db.pool)
        .await;
    let _ = sqlx::query("UPDATE user_profiles SET following_count = GREATEST(following_count - 1, 0) WHERE user_id = $1")
        .bind(follower_uuid)
        .execute(&state.db.pool)
        .await;

    tracing::info!("User {} unfollowed user {}", user.user_id, target_user_id);

    // Return updated follow button (now showing "follow")
    Ok(axum::response::Html(format!(
        r#"
        <button hx-post="/api/users/{}/follow"
                hx-target="this"
                hx-swap="outerHTML"
                class="px-4 py-2 bg-[#57C98A] hover:bg-[#4ab87a] text-white rounded-lg transition text-sm font-medium">
            Segui
        </button>
    "#,
        target_user_id
    )))
}

/// Dismiss welcome modal (PROTECTED)
#[allow(dead_code)]
pub async fn dismiss_welcome(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Update user profile to mark welcome as dismissed
    let _ = sqlx::query("UPDATE user_profiles SET welcome_dismissed = true WHERE user_id = $1")
        .bind(user_uuid)
        .execute(&state.db.pool)
        .await;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Welcome dismissed".to_string()),
    }))
}

/// Dismiss profile completion banner (PROTECTED)
#[allow(dead_code)]
pub async fn dismiss_profile_banner(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let user_uuid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Update user profile to mark banner as dismissed
    let _ =
        sqlx::query("UPDATE user_profiles SET profile_banner_dismissed = true WHERE user_id = $1")
            .bind(user_uuid)
            .execute(&state.db.pool)
            .await;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Banner dismissed".to_string()),
    }))
}
