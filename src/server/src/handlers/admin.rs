//! Admin Handlers - Phase 7 Implementation
//! 
//! Handles admin-related API endpoints including:
//! - Analytics dashboard
//! - Moderation queue
//! - Audit logs
//! - Admin settings

use axum::{
    extract::{Path, Query, State},
    response::{Html, Json},
};
use sqlx::Row;
use std::sync::Arc;
use crate::handlers::pages::{AppState, AppError};
use crate::auth::AuthUser;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, serde::Deserialize)]
pub struct AnalyticsQuery {
    pub event_type: Option<String>,
    pub community_id: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, serde::Serialize)]
pub struct AnalyticsSummary {
    pub total_users: i64,
    pub active_users_today: i64,
    pub total_communities: i64,
    pub total_posts: i64,
    pub total_proposals: i64,
    pub pending_moderation: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct ModerationQuery {
    pub status: Option<String>,
    pub content_type: Option<String>,
    pub priority: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateModerationRequest {
    pub status: String,
    pub resolution: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AuditLogQuery {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub target_type: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ReportContentRequest {
    pub content_type: String,
    pub content_id: String,
    pub reason: String,
    pub details: Option<String>,
}

// ============================================================================
// Analytics Handlers
// ============================================================================

/// Get analytics summary for admin dashboard
pub async fn get_analytics_summary(
    AuthUser(_user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<AnalyticsSummary>, AppError> {
    // TODO: Add admin role check
    
    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let active_users_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM analytics_events WHERE created_at > NOW() - INTERVAL '24 hours'"
    )
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or(0);
    
    let total_communities: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM communities")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let total_posts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let total_proposals: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM proposals")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let pending_moderation: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM moderation_queue WHERE status = 'pending'"
    )
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or(0);
    
    Ok(Json(AnalyticsSummary {
        total_users,
        active_users_today,
        total_communities,
        total_posts,
        total_proposals,
        pending_moderation,
    }))
}

/// List analytics events
pub async fn list_analytics_events(
    AuthUser(_user): AuthUser,
    State(state): State<Arc<AppState>>,
    Query(params): Query<AnalyticsQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;
    
    let events = sqlx::query(
        r#"SELECT id, event_type, user_id, community_id, session_id, metadata, created_at
           FROM analytics_events
           ORDER BY created_at DESC
           LIMIT $1 OFFSET $2"#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let result: Vec<serde_json::Value> = events.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<i64, _>("id"),
            "event_type": row.get::<String, _>("event_type"),
            "user_id": row.get::<Option<uuid::Uuid>, _>("user_id"),
            "community_id": row.get::<Option<uuid::Uuid>, _>("community_id"),
            "session_id": row.get::<Option<String>, _>("session_id"),
            "metadata": row.get::<serde_json::Value, _>("metadata"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339()
        })
    }).collect();
    
    Ok(Json(serde_json::json!({
        "events": result,
        "page": page,
        "limit": limit
    })))
}

/// Track an analytics event
pub async fn track_event(
    State(state): State<Arc<AppState>>,
    Json(event): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let event_type = event.get("event_type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    let user_id = event.get("user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| uuid::Uuid::parse_str(s).ok());
    
    let community_id = event.get("community_id")
        .and_then(|v| v.as_str())
        .and_then(|s| uuid::Uuid::parse_str(s).ok());
    
    let metadata = event.get("metadata")
        .cloned()
        .unwrap_or(serde_json::json!({}));
    
    sqlx::query(
        "INSERT INTO analytics_events (event_type, user_id, community_id, metadata) VALUES ($1, $2, $3, $4)"
    )
    .bind(event_type)
    .bind(user_id)
    .bind(community_id)
    .bind(metadata)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError(anyhow::anyhow!("Failed to track event: {}", e)))?;
    
    Ok(Json(serde_json::json!({"success": true})))
}

// ============================================================================
// Moderation Handlers
// ============================================================================

/// List moderation queue items
pub async fn list_moderation_queue(
    AuthUser(_user): AuthUser,
    State(state): State<Arc<AppState>>,
    Query(params): Query<ModerationQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20).min(50);
    let offset = (page - 1) * limit;
    let status = params.status.unwrap_or_else(|| "pending".to_string());
    
    let items = sqlx::query(
        r#"SELECT m.id, m.content_type, m.content_id, m.reason, m.details, 
                  m.status, m.priority, m.created_at, m.resolved_at,
                  COALESCE(up.name, u.email) as reporter_name
           FROM moderation_queue m
           LEFT JOIN users u ON m.reported_by = u.id
           LEFT JOIN user_profiles up ON u.id = up.user_id
           WHERE m.status = $1
           ORDER BY 
               CASE m.priority 
                   WHEN 'urgent' THEN 1 
                   WHEN 'high' THEN 2 
                   WHEN 'normal' THEN 3 
                   ELSE 4 
               END,
               m.created_at ASC
           LIMIT $2 OFFSET $3"#
    )
    .bind(&status)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let result: Vec<serde_json::Value> = items.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<uuid::Uuid, _>("id").to_string(),
            "content_type": row.get::<String, _>("content_type"),
            "content_id": row.get::<String, _>("content_id"),
            "reason": row.get::<String, _>("reason"),
            "details": row.get::<Option<String>, _>("details"),
            "status": row.get::<String, _>("status"),
            "priority": row.get::<String, _>("priority"),
            "reporter_name": row.get::<Option<String>, _>("reporter_name"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339()
        })
    }).collect();
    
    Ok(Json(serde_json::json!({
        "items": result,
        "page": page,
        "limit": limit,
        "status": status
    })))
}

/// Update moderation item status
pub async fn update_moderation_item(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(item_id): Path<uuid::Uuid>,
    Json(request): Json<UpdateModerationRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid user ID")))?;
    
    let resolved_at = if request.status == "approved" || request.status == "rejected" {
        Some(chrono::Utc::now())
    } else {
        None
    };
    
    sqlx::query(
        r#"UPDATE moderation_queue 
           SET status = $1, resolution = $2, moderator_id = $3, 
               resolved_at = $4, updated_at = NOW()
           WHERE id = $5"#
    )
    .bind(&request.status)
    .bind(&request.resolution)
    .bind(user_uuid)
    .bind(resolved_at)
    .bind(item_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError(anyhow::anyhow!("Failed to update moderation item: {}", e)))?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "id": item_id.to_string(),
        "status": request.status
    })))
}

/// Report content for moderation
pub async fn report_content(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Json(request): Json<ReportContentRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid user ID")))?;
    
    let id = uuid::Uuid::now_v7();
    
    sqlx::query(
        r#"INSERT INTO moderation_queue (id, content_type, content_id, reported_by, reason, details)
           VALUES ($1, $2, $3, $4, $5, $6)"#
    )
    .bind(id)
    .bind(&request.content_type)
    .bind(&request.content_id)
    .bind(user_uuid)
    .bind(&request.reason)
    .bind(&request.details)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError(anyhow::anyhow!("Failed to report content: {}", e)))?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "id": id.to_string(),
        "message": "Segnalazione inviata. Grazie per il tuo contributo."
    })))
}

// ============================================================================
// Audit Log Handlers
// ============================================================================

/// List audit logs
pub async fn list_audit_logs(
    AuthUser(_user): AuthUser,
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuditLogQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;
    
    let logs = sqlx::query(
        r#"SELECT a.id, a.user_id, a.action, a.target_type, a.target_id, 
                  a.old_value, a.new_value, a.ip_address, a.created_at,
                  COALESCE(up.name, u.email) as user_name
           FROM audit_logs a
           LEFT JOIN users u ON a.user_id = u.id
           LEFT JOIN user_profiles up ON u.id = up.user_id
           ORDER BY a.created_at DESC
           LIMIT $1 OFFSET $2"#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let result: Vec<serde_json::Value> = logs.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<i64, _>("id"),
            "user_id": row.get::<Option<uuid::Uuid>, _>("user_id"),
            "user_name": row.get::<Option<String>, _>("user_name"),
            "action": row.get::<String, _>("action"),
            "target_type": row.get::<Option<String>, _>("target_type"),
            "target_id": row.get::<Option<String>, _>("target_id"),
            "ip_address": row.get::<Option<String>, _>("ip_address"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339()
        })
    }).collect();
    
    Ok(Json(serde_json::json!({
        "logs": result,
        "page": page,
        "limit": limit
    })))
}

// ============================================================================
// Admin Dashboard HTMX Fragment
// ============================================================================

/// Admin dashboard summary fragment
pub async fn admin_dashboard_fragment(
    AuthUser(_user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let total_communities: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM communities")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let total_posts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let pending_moderation: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM moderation_queue WHERE status = 'pending'"
    )
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or(0);
    
    let html = format!(r#"
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div class="bg-white rounded-xl shadow-sm p-6 border border-civiqo-gray-200">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-civiqo-gray-600 text-sm">Utenti Totali</p>
                    <p class="text-3xl font-bold text-civiqo-gray-900">{}</p>
                </div>
                <div class="w-12 h-12 bg-civiqo-blue/10 rounded-full flex items-center justify-center">
                    <svg class="w-6 h-6 text-civiqo-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"/>
                    </svg>
                </div>
            </div>
        </div>
        
        <div class="bg-white rounded-xl shadow-sm p-6 border border-civiqo-gray-200">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-civiqo-gray-600 text-sm">Community</p>
                    <p class="text-3xl font-bold text-civiqo-gray-900">{}</p>
                </div>
                <div class="w-12 h-12 bg-civiqo-eco-green/10 rounded-full flex items-center justify-center">
                    <svg class="w-6 h-6 text-civiqo-eco-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                    </svg>
                </div>
            </div>
        </div>
        
        <div class="bg-white rounded-xl shadow-sm p-6 border border-civiqo-gray-200">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-civiqo-gray-600 text-sm">Post Totali</p>
                    <p class="text-3xl font-bold text-civiqo-gray-900">{}</p>
                </div>
                <div class="w-12 h-12 bg-purple-100 rounded-full flex items-center justify-center">
                    <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z"/>
                    </svg>
                </div>
            </div>
        </div>
        
        <div class="bg-white rounded-xl shadow-sm p-6 border border-civiqo-gray-200">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-civiqo-gray-600 text-sm">In Moderazione</p>
                    <p class="text-3xl font-bold {}">{}</p>
                </div>
                <div class="w-12 h-12 {} rounded-full flex items-center justify-center">
                    <svg class="w-6 h-6 {}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                    </svg>
                </div>
            </div>
        </div>
    </div>
    "#,
        total_users,
        total_communities,
        total_posts,
        if pending_moderation > 0 { "text-red-600" } else { "text-civiqo-gray-900" },
        pending_moderation,
        if pending_moderation > 0 { "bg-red-100" } else { "bg-civiqo-gray-100" },
        if pending_moderation > 0 { "text-red-600" } else { "text-civiqo-gray-600" }
    );
    
    Ok(Html(html))
}
