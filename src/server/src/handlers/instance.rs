//! Instance configuration handlers
//! 
//! Handles instance-level settings, setup wizard, and federation config.

use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;

use crate::auth::AuthUser;
use crate::handlers::pages::{AppState, AppError};

// ============================================================================
// TYPES
// ============================================================================

#[derive(Debug, Serialize)]
pub struct InstanceInfo {
    pub name: String,
    pub description: String,
    pub setup_completed: bool,
    pub community: Option<CommunityInfo>,
}

#[derive(Debug, Serialize)]
pub struct CommunityInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub member_count: i64,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetupRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub requires_approval: bool,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub requires_approval: Option<bool>,
    pub logo_url: Option<String>,
    pub cover_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFederationRequest {
    pub enabled: bool,
    pub hub_url: Option<String>,
    pub sync_members: bool,
    pub sync_posts: bool,
    pub sync_proposals: bool,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Check if setup is completed (community exists)
pub async fn is_setup_completed(state: &AppState) -> bool {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM communities")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    count > 0
}

/// Get the single community (if exists)
pub async fn get_community(state: &AppState) -> Option<sqlx::postgres::PgRow> {
    sqlx::query(
        r#"SELECT c.*, 
                  (SELECT COUNT(*) FROM community_members WHERE community_id = c.id) as member_count
           FROM communities c
           LIMIT 1"#
    )
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten()
}

/// Check if user is instance admin
pub async fn is_instance_admin(state: &AppState, user_id: &str) -> bool {
    let user_uuid = match uuid::Uuid::parse_str(user_id) {
        Ok(u) => u,
        Err(_) => return false,
    };
    
    // Check instance_admins table
    let is_admin: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM instance_admins WHERE user_id = $1"
    )
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    if is_admin > 0 {
        return true;
    }
    
    // Also check if user is community owner/admin
    let is_community_admin: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM community_members cm
           JOIN roles r ON cm.role_id = r.id
           WHERE cm.user_id = $1 AND r.name IN ('owner', 'admin')"#
    )
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    is_community_admin > 0
}

// ============================================================================
// PUBLIC API HANDLERS
// ============================================================================

/// Get instance info (PUBLIC)
/// GET /api/instance
pub async fn get_instance_info(
    State(state): State<Arc<AppState>>,
) -> Result<Json<InstanceInfo>, AppError> {
    let setup_completed = is_setup_completed(&state).await;
    
    let community = if let Some(row) = get_community(&state).await {
        Some(CommunityInfo {
            id: row.get::<uuid::Uuid, _>("id").to_string(),
            name: row.get("name"),
            description: row.get("description"),
            is_public: row.get("is_public"),
            member_count: row.get("member_count"),
            logo_url: row.get("logo_url"),
            primary_color: row.get("primary_color"),
            secondary_color: row.get("secondary_color"),
            accent_color: row.get("accent_color"),
        })
    } else {
        None
    };
    
    // Get instance name from settings
    let name: String = sqlx::query_scalar(
        "SELECT value FROM instance_settings WHERE key = 'instance_name'"
    )
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or_else(|| "Civiqo".to_string());
    
    let description: String = sqlx::query_scalar(
        "SELECT value FROM instance_settings WHERE key = 'instance_description'"
    )
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or_else(|| "Piattaforma di partecipazione civica".to_string());
    
    Ok(Json(InstanceInfo {
        name,
        description,
        setup_completed,
        community,
    }))
}

/// Complete setup wizard (creates community)
/// POST /api/setup
pub async fn complete_setup(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SetupRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Check if already set up
    if is_setup_completed(&state).await {
        return Err(AppError::BadRequest("Setup già completato".to_string()));
    }
    
    // Validate
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("Il nome è obbligatorio".to_string()));
    }
    
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;
    
    // Generate slug from name
    let slug = req.name
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-");
    
    let community_id = uuid::Uuid::now_v7();
    
    // Create the community
    sqlx::query(
        r#"INSERT INTO communities 
           (id, name, description, slug, is_public, requires_approval, created_by,
            primary_color, secondary_color, accent_color)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#
    )
    .bind(community_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&slug)
    .bind(req.is_public)
    .bind(req.requires_approval)
    .bind(user_uuid)
    .bind(req.primary_color.as_deref().unwrap_or("#2563EB"))
    .bind(req.secondary_color.as_deref().unwrap_or("#57C98A"))
    .bind(req.accent_color.as_deref().unwrap_or("#FF6B6B"))
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create community: {}", e)))?;
    
    // Get owner role
    let owner_role_id: i64 = sqlx::query_scalar(
        "SELECT id FROM roles WHERE name = 'owner'"
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
    .unwrap_or(1); // Fallback to role 1 if owner doesn't exist
    
    // Add user as owner
    sqlx::query(
        r#"INSERT INTO community_members (community_id, user_id, role_id, status)
           VALUES ($1, $2, $3, 'active')"#
    )
    .bind(community_id)
    .bind(user_uuid)
    .bind(owner_role_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to add owner: {}", e)))?;
    
    // Add user as instance admin
    sqlx::query(
        r#"INSERT INTO instance_admins (user_id, created_by)
           VALUES ($1, $1)
           ON CONFLICT (user_id) DO NOTHING"#
    )
    .bind(user_uuid)
    .execute(&state.db.pool)
    .await
    .ok(); // Ignore errors
    
    // Update instance settings
    sqlx::query(
        "UPDATE instance_settings SET value = 'true', updated_at = NOW() WHERE key = 'setup_completed'"
    )
    .execute(&state.db.pool)
    .await
    .ok();
    
    sqlx::query(
        "UPDATE instance_settings SET value = $1, updated_at = NOW() WHERE key = 'instance_name'"
    )
    .bind(&req.name)
    .execute(&state.db.pool)
    .await
    .ok();
    
    if let Some(desc) = &req.description {
        sqlx::query(
            "UPDATE instance_settings SET value = $1, updated_at = NOW() WHERE key = 'instance_description'"
        )
        .bind(desc)
        .execute(&state.db.pool)
        .await
        .ok();
    }
    
    Ok(Json(serde_json::json!({
        "success": true,
        "community_id": community_id.to_string(),
        "message": "Setup completato con successo!"
    })))
}

/// Get instance settings (ADMIN ONLY)
/// GET /api/instance/settings
pub async fn get_instance_settings(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !is_instance_admin(&state, &user.user_id).await {
        return Err(AppError::BadRequest("Accesso non autorizzato".to_string()));
    }
    
    let community = get_community(&state).await;
    
    let settings = if let Some(row) = community {
        serde_json::json!({
            "community": {
                "id": row.get::<uuid::Uuid, _>("id").to_string(),
                "name": row.get::<String, _>("name"),
                "description": row.get::<Option<String>, _>("description"),
                "slug": row.get::<String, _>("slug"),
                "is_public": row.get::<bool, _>("is_public"),
                "requires_approval": row.get::<bool, _>("requires_approval"),
                "logo_url": row.get::<Option<String>, _>("logo_url"),
                "cover_url": row.get::<Option<String>, _>("cover_url"),
                "primary_color": row.get::<Option<String>, _>("primary_color"),
                "secondary_color": row.get::<Option<String>, _>("secondary_color"),
                "accent_color": row.get::<Option<String>, _>("accent_color"),
            }
        })
    } else {
        serde_json::json!({
            "community": null
        })
    };
    
    Ok(Json(settings))
}

/// Update instance settings (ADMIN ONLY)
/// PUT /api/instance/settings
pub async fn update_instance_settings(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateSettingsRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !is_instance_admin(&state, &user.user_id).await {
        return Err(AppError::BadRequest("Accesso non autorizzato".to_string()));
    }
    
    let community = get_community(&state).await
        .ok_or_else(|| AppError::BadRequest("Nessuna comunità configurata".to_string()))?;
    
    let community_id: uuid::Uuid = community.get("id");
    
    // Build dynamic update
    let mut updates = vec!["updated_at = NOW()".to_string()];
    
    if req.name.is_some() { updates.push("name = $2".to_string()); }
    if req.description.is_some() { updates.push("description = $3".to_string()); }
    if req.is_public.is_some() { updates.push("is_public = $4".to_string()); }
    if req.requires_approval.is_some() { updates.push("requires_approval = $5".to_string()); }
    if req.logo_url.is_some() { updates.push("logo_url = $6".to_string()); }
    if req.cover_url.is_some() { updates.push("cover_url = $7".to_string()); }
    if req.primary_color.is_some() { updates.push("primary_color = $8".to_string()); }
    if req.secondary_color.is_some() { updates.push("secondary_color = $9".to_string()); }
    if req.accent_color.is_some() { updates.push("accent_color = $10".to_string()); }
    
    let query = format!(
        "UPDATE communities SET {} WHERE id = $1",
        updates.join(", ")
    );
    
    sqlx::query(&query)
        .bind(community_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.is_public)
        .bind(req.requires_approval)
        .bind(&req.logo_url)
        .bind(&req.cover_url)
        .bind(&req.primary_color)
        .bind(&req.secondary_color)
        .bind(&req.accent_color)
        .execute(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to update: {}", e)))?;
    
    // Update instance name if changed
    if let Some(name) = &req.name {
        sqlx::query(
            "UPDATE instance_settings SET value = $1, updated_at = NOW() WHERE key = 'instance_name'"
        )
        .bind(name)
        .execute(&state.db.pool)
        .await
        .ok();
    }
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Impostazioni aggiornate"
    })))
}

/// Get federation config (ADMIN ONLY)
/// GET /api/instance/federation
pub async fn get_federation_config(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !is_instance_admin(&state, &user.user_id).await {
        return Err(AppError::BadRequest("Accesso non autorizzato".to_string()));
    }
    
    let config = sqlx::query(
        "SELECT * FROM federation_config LIMIT 1"
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    
    let response = if let Some(row) = config {
        serde_json::json!({
            "enabled": row.get::<bool, _>("enabled"),
            "hub_url": row.get::<Option<String>, _>("hub_url"),
            "instance_id": row.get::<Option<String>, _>("instance_id"),
            "sync_members": row.get::<bool, _>("sync_members"),
            "sync_posts": row.get::<bool, _>("sync_posts"),
            "sync_proposals": row.get::<bool, _>("sync_proposals"),
            "last_sync_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("last_sync_at"),
        })
    } else {
        serde_json::json!({
            "enabled": false,
            "hub_url": null,
            "instance_id": null,
            "sync_members": false,
            "sync_posts": false,
            "sync_proposals": false,
            "last_sync_at": null,
        })
    };
    
    Ok(Json(response))
}

/// Update federation config (ADMIN ONLY)
/// PUT /api/instance/federation
pub async fn update_federation_config(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateFederationRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !is_instance_admin(&state, &user.user_id).await {
        return Err(AppError::BadRequest("Accesso non autorizzato".to_string()));
    }
    
    // Upsert federation config
    sqlx::query(
        r#"INSERT INTO federation_config (id, enabled, hub_url, sync_members, sync_posts, sync_proposals, updated_at)
           VALUES (1, $1, $2, $3, $4, $5, NOW())
           ON CONFLICT (id) DO UPDATE SET
               enabled = $1,
               hub_url = $2,
               sync_members = $3,
               sync_posts = $4,
               sync_proposals = $5,
               updated_at = NOW()"#
    )
    .bind(req.enabled)
    .bind(&req.hub_url)
    .bind(req.sync_members)
    .bind(req.sync_posts)
    .bind(req.sync_proposals)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to update federation: {}", e)))?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Configurazione federazione aggiornata"
    })))
}

