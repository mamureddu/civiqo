use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use shared::{
    models::{
        ApiResponse, PaginationParams, CreateCommunityRequest, UpdateCommunityRequest,
        JoinCommunityRequest, CommunityWithStats, MemberWithProfile, Community
    },
    error::{AppError, Result},
    utils::{generate_slug, validate_text_length},
};
use crate::{AppState, middleware::auth::{extract_user, check_community_permission}};

/// List/search communities with optional filtering
pub async fn list_communities(
    State(state): State<AppState>,
    Query(params): Query<CommunitySearchParams>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<CommunityWithStats>>>> {
    // Validate search parameters
    params.validate()
        .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT
            c.id, c.name, c.description, c.slug, c.is_public,
            c.requires_approval, c.created_by, c.created_at, c.updated_at,
            COUNT(DISTINCT cm.id) as member_count,
            COUNT(DISTINCT b.id) as business_count
        FROM communities c
        LEFT JOIN community_members cm ON c.id = cm.community_id AND cm.status = 'active'
        LEFT JOIN businesses b ON c.id = b.community_id AND b.is_active = true
        WHERE 1=1
        "#
    );

    // Add search filters
    if let Some(query) = &params.q {
        query_builder.push(" AND (c.name ILIKE ");
        query_builder.push_bind(format!("%{}%", query));
        query_builder.push(" OR c.description ILIKE ");
        query_builder.push_bind(format!("%{}%", query));
        query_builder.push(")");
    }

    if let Some(is_public) = params.is_public {
        query_builder.push(" AND c.is_public = ");
        query_builder.push_bind(is_public);
    }

    query_builder.push(
        r#"
        GROUP BY c.id, c.name, c.description, c.slug, c.is_public,
                 c.requires_approval, c.created_by, c.created_at, c.updated_at
        ORDER BY c.created_at DESC
        LIMIT "#
    );
    query_builder.push_bind(pagination.limit() as i64);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(pagination.offset() as i64);

    let communities = query_builder
        .build_query_as::<CommunityWithStats>()
        .fetch_all(&state.db.pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(communities),
        message: None,
        error: None,
    }))
}

/// Create a new community
pub async fn create_community(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateCommunityRequest>,
) -> Result<Json<ApiResponse<Community>>> {
    let user = extract_user(&state, &headers).await?;

    // Validate input
    validate_text_length(&request.name, 3, 100)?;
    if let Some(ref desc) = request.description {
        validate_text_length(desc, 10, 1000)?;
    }

    // Generate unique slug
    let mut slug = generate_slug(&request.name);
    let mut counter = 1;

    while sqlx::query!("SELECT 1 as exists FROM communities WHERE slug = $1", slug)
        .fetch_optional(&state.db.pool)
        .await?
        .is_some()
    {
        slug = format!("{}-{}", generate_slug(&request.name), counter);
        counter += 1;
    }

    // Start transaction for atomic community creation
    let mut tx = state.db.pool.begin().await?;

    // Create community
    let community = sqlx::query_as!(
        Community,
        r#"
        INSERT INTO communities (name, description, slug, is_public, requires_approval, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id as "id!",
            name as "name!",
            description,
            slug as "slug!",
            is_public as "is_public!",
            requires_approval as "requires_approval!",
            created_by as "created_by!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        "#,
        request.name,
        request.description,
        slug,
        request.is_public,
        request.requires_approval,
        user.user_id
    )
    .fetch_one(&mut *tx)
    .await?;

    // Create default community settings
    sqlx::query!(
        "INSERT INTO community_settings (community_id) VALUES ($1)",
        community.id
    )
    .execute(&mut *tx)
    .await?;

    // Add creator as owner
    let owner_role = sqlx::query!(
        "SELECT id FROM roles WHERE name = 'owner'"
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
        VALUES ($1, $2, $3, 'active', NOW())
        "#,
        user.user_id,
        community.id,
        owner_role.id
    )
    .execute(&mut *tx)
    .await?;

    // Store boundary if provided
    if let Some(boundary) = request.boundary {
        let boundary_json = serde_json::to_value(boundary)?;
        sqlx::query!(
            "INSERT INTO community_boundaries (community_id, name, boundary_data) VALUES ($1, $2, $3)",
            community.id,
            format!("{} Boundary", community.name),
            boundary_json
        )
        .execute(&mut *tx)
        .await?;
    }

    // Commit transaction
    tx.commit().await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(community),
        message: Some("Community created successfully".to_string()),
        error: None,
    }))
}

/// Get community details
pub async fn get_community(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<CommunityWithStats>>> {
    // Try to get user (optional for public communities)
    let user = extract_user(&state, &headers).await.ok();

    let community = sqlx::query_as!(
        CommunityWithStats,
        r#"
        SELECT
            c.id as "id!",
            c.name as "name!",
            c.description,
            c.slug as "slug!",
            c.is_public as "is_public!",
            c.requires_approval as "requires_approval!",
            c.created_by as "created_by!",
            c.created_at as "created_at!",
            c.updated_at as "updated_at!",
            COUNT(DISTINCT cm.id) as "member_count!",
            COUNT(DISTINCT b.id) as "business_count!",
            CASE WHEN ucm.id IS NOT NULL THEN true ELSE false END as "is_member!",
            r.name as "user_role?"
        FROM communities c
        LEFT JOIN community_members cm ON c.id = cm.community_id AND cm.status = 'active'
        LEFT JOIN businesses b ON c.id = b.community_id AND b.is_active = true
        LEFT JOIN community_members ucm ON c.id = ucm.community_id AND ucm.user_id = $2 AND ucm.status = 'active'
        LEFT JOIN roles r ON ucm.role_id = r.id
        WHERE c.id = $1
        GROUP BY c.id, c.name, c.description, c.slug, c.is_public,
                 c.requires_approval, c.created_by, c.created_at, c.updated_at,
                 ucm.id, r.name
        "#,
        id,
        user.as_ref().map(|u| u.user_id)
    )
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;

    // Check if user can view this community
    if !community.is_public {
        let user = user.ok_or_else(|| AppError::Auth("Authentication required".to_string()))?;
        if !community.is_member {
            return Err(AppError::Authorization("Access denied".to_string()));
        }
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(community),
        message: None,
        error: None,
    }))
}

/// Update community
pub async fn update_community(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<UpdateCommunityRequest>,
) -> Result<Json<ApiResponse<Community>>> {
    let user = extract_user(&state, &headers).await?;

    // Check permissions
    if !check_community_permission(&state, user.user_id, id, "manage_community").await? {
        return Err(AppError::Authorization("Insufficient permissions".to_string()));
    }

    // Validate input
    if let Some(ref name) = request.name {
        validate_text_length(name, 3, 100)?;
    }
    if let Some(ref desc) = request.description.as_ref() {
        validate_text_length(desc, 10, 1000)?;
    }

    let community = sqlx::query_as!(
        Community,
        r#"
        UPDATE communities
        SET
            name = CASE WHEN $1::text IS NOT NULL THEN $1 ELSE name END,
            description = CASE WHEN $2::text IS NOT NULL THEN $2 ELSE description END,
            is_public = CASE WHEN $3::boolean IS NOT NULL THEN $3 ELSE is_public END,
            requires_approval = CASE WHEN $4::boolean IS NOT NULL THEN $4 ELSE requires_approval END,
            updated_at = NOW()
        WHERE id = $5
        RETURNING
            id as "id!",
            name as "name!",
            description,
            slug as "slug!",
            is_public as "is_public!",
            requires_approval as "requires_approval!",
            created_by as "created_by!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        "#,
        request.name,
        request.description,
        request.is_public,
        request.requires_approval,
        id
    )
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(community),
        message: Some("Community updated successfully".to_string()),
        error: None,
    }))
}

/// Join a community
pub async fn join_community(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<JoinCommunityRequest>,
) -> Result<Json<ApiResponse<String>>> {
    let user = extract_user(&state, &headers).await?;

    // Get community details
    let community = sqlx::query!(
        "SELECT requires_approval, is_public FROM communities WHERE id = $1",
        id
    )
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;

    // Check if already a member
    if sqlx::query!(
        "SELECT 1 as exists FROM community_members WHERE user_id = $1 AND community_id = $2",
        user.user_id,
        id
    )
    .fetch_optional(&state.db.pool)
    .await?
    .is_some()
    {
        return Err(AppError::Conflict("Already a member of this community".to_string()));
    }

    // Get default member role
    let member_role = sqlx::query!(
        "SELECT id FROM roles WHERE name = 'member'"
    )
    .fetch_one(&state.db.pool)
    .await?;

    let status = if community.requires_approval.unwrap_or(false) {
        shared::models::MembershipStatus::Pending
    } else {
        shared::models::MembershipStatus::Active
    };
    let joined_at = if matches!(status, shared::models::MembershipStatus::Active) {
        Some(chrono::Utc::now())
    } else {
        None
    };

    sqlx::query!(
        r#"
        INSERT INTO community_members (user_id, community_id, role_id, status, joined_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        user.user_id,
        id,
        member_role.id,
        status.clone() as shared::models::MembershipStatus,
        joined_at
    )
    .execute(&state.db.pool)
    .await?;

    let message = if community.requires_approval.unwrap_or(false) {
        "Join request submitted for approval"
    } else {
        "Successfully joined the community"
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(status.to_string()),
        message: Some(message.to_string()),
        error: None,
    }))
}

/// List community members
pub async fn list_members(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<MemberWithProfile>>>> {
    let user = extract_user(&state, &headers).await?;

    // Check if user can view members
    if !check_community_permission(&state, user.user_id, id, "view_community").await? {
        return Err(AppError::Authorization("Access denied".to_string()));
    }

    let members = sqlx::query_as!(
        MemberWithProfile,
        r#"
        SELECT
            cm.id as "id!",
            cm.user_id as "user_id!",
            cm.community_id as "community_id!",
            cm.role_id as "role_id!",
            cm.status as "status!: shared::models::MembershipStatus",
            cm.joined_at,
            cm.created_at as "created_at!",
            cm.updated_at as "updated_at!",
            COALESCE(p.name, u.email) as "user_name?",
            p.avatar_url as "user_avatar?",
            r.name as "role_name!"
        FROM community_members cm
        JOIN users u ON cm.user_id = u.id
        LEFT JOIN user_profiles p ON u.id = p.user_id
        JOIN roles r ON cm.role_id = r.id
        WHERE cm.community_id = $1
        ORDER BY cm.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        id,
        pagination.limit() as i64,
        pagination.offset() as i64
    )
    .fetch_all(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(members),
        message: None,
        error: None,
    }))
}

/// Update member role
pub async fn update_member_role(
    State(state): State<AppState>,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
    headers: HeaderMap,
    Json(request): Json<UpdateMemberRoleRequest>,
) -> Result<Json<ApiResponse<String>>> {
    // Validate input
    request.validate()
        .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

    let current_user = extract_user(&state, &headers).await?;

    // Check permissions
    if !check_community_permission(&state, current_user.user_id, id, "manage_members").await? {
        return Err(AppError::Authorization("Insufficient permissions".to_string()));
    }

    // Get the new role
    let role = sqlx::query!(
        "SELECT id FROM roles WHERE name = $1",
        request.role_name
    )
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    sqlx::query!(
        "UPDATE community_members SET role_id = $1, updated_at = NOW() WHERE user_id = $2 AND community_id = $3",
        role.id,
        user_id,
        id
    )
    .execute(&state.db.pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some("Member role updated successfully".to_string()),
        message: None,
        error: None,
    }))
}

#[derive(Deserialize, Validate)]
pub struct CommunitySearchParams {
    #[validate(length(min = 1, max = 100, message = "Search query must be 1-100 characters"))]
    pub q: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateMemberRoleRequest {
    #[validate(length(min = 1, max = 50, message = "Role name must be 1-50 characters"))]
    #[validate(regex(path = "VALID_ROLE_NAME", message = "Invalid role name format"))]
    pub role_name: String,
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VALID_ROLE_NAME: Regex = Regex::new(r"^[a-z_]+$").unwrap();
}