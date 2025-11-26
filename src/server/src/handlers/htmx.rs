use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;
use std::sync::Arc;
use sqlx::Row;

use super::pages::{AppState, AppError};
use crate::auth::AuthUser;

/// Navigation fragment
pub async fn nav_fragment(State(_state): State<Arc<AppState>>) -> Html<String> {
    Html(r#"
    <div class="container mx-auto px-4 py-4">
        <div class="flex items-center justify-between">
            <a href="/" class="text-xl font-bold text-gray-800 hover:text-blue-600">
                Community Manager
            </a>
            <div class="flex gap-6">
                <a href="/communities" class="text-gray-600 hover:text-blue-600">Communities</a>
                <a href="/businesses" class="text-gray-600 hover:text-blue-600">Businesses</a>
                <a href="/governance" class="text-gray-600 hover:text-blue-600">Governance</a>
                <a href="/auth/login" class="text-blue-600 hover:text-blue-700 font-medium">Sign In</a>
            </div>
        </div>
    </div>
    "#.to_string())
}

/// Recent communities fragment
pub async fn recent_communities(State(_state): State<Arc<AppState>>) -> Html<String> {
    // TODO: Fetch from database
    Html(r#"
    <div class="community-card fade-in">
        <h3 class="text-xl font-bold mb-2">Tech Community Milano</h3>
        <p class="text-gray-600 mb-4">A community for tech enthusiasts in Milan</p>
        <div class="flex items-center justify-between text-sm text-gray-500">
            <span>👥 245 members</span>
            <a href="/communities/tech-milano" class="text-blue-600 hover:text-blue-700">View →</a>
        </div>
    </div>
    <div class="community-card fade-in" style="animation-delay: 100ms;">
        <h3 class="text-xl font-bold mb-2">Green Living Roma</h3>
        <p class="text-gray-600 mb-4">Sustainable living and eco-friendly practices</p>
        <div class="flex items-center justify-between text-sm text-gray-500">
            <span>👥 189 members</span>
            <a href="/communities/green-roma" class="text-blue-600 hover:text-blue-700">View →</a>
        </div>
    </div>
    <div class="community-card fade-in" style="animation-delay: 200ms;">
        <h3 class="text-xl font-bold mb-2">Startup Torino</h3>
        <p class="text-gray-600 mb-4">Connect with entrepreneurs and innovators</p>
        <div class="flex items-center justify-between text-sm text-gray-500">
            <span>👥 312 members</span>
            <a href="/communities/startup-torino" class="text-blue-600 hover:text-blue-700">View →</a>
        </div>
    </div>
    "#.to_string())
}

#[derive(Deserialize)]
pub struct CommunitiesQuery {
    #[serde(default)]
    q: String,
    // ==========================================================
    // COMMENTED FIELDS - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // #[serde(default)]
    // filter: String,                    // Filter by category, type, or status
    // #[serde(default = "default_page")]
    // page: u32,                         // Pagination for large communities lists
}

// ==========================================================
// COMMENTED HELPER - KEPT FOR FUTURE REFERENCE
// ==========================================================
// /// Default page number for pagination
// /// USAGE: When implementing paginated communities list
// /// PURPOSE: Standard pagination starting point
// fn default_page() -> u32 {
//     1
// }

/// Communities list fragment
pub async fn communities_list(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<CommunitiesQuery>,
) -> Html<String> {
    // TODO: Fetch from database with filters
    let filter_text = if !query.q.is_empty() {
        format!(" matching '{}'", query.q)
    } else {
        String::new()
    };
    
    Html(format!(r#"
    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div class="community-card">
            <h3 class="text-xl font-bold mb-2">Tech Community Milano</h3>
            <p class="text-gray-600 mb-4">A community for tech enthusiasts in Milan{}</p>
            <div class="flex items-center justify-between text-sm text-gray-500">
                <span>👥 245 members</span>
                <a href="/communities/tech-milano" class="text-blue-600 hover:text-blue-700">View →</a>
            </div>
        </div>
        <div class="community-card">
            <h3 class="text-xl font-bold mb-2">Green Living Roma</h3>
            <p class="text-gray-600 mb-4">Sustainable living and eco-friendly practices{}</p>
            <div class="flex items-center justify-between text-sm text-gray-500">
                <span>👥 189 members</span>
                <a href="/communities/green-roma" class="text-blue-600 hover:text-blue-700">View →</a>
            </div>
        </div>
        <div class="community-card">
            <h3 class="text-xl font-bold mb-2">Startup Torino</h3>
            <p class="text-gray-600 mb-4">Connect with entrepreneurs and innovators{}</p>
            <div class="flex items-center justify-between text-sm text-gray-500">
                <span>👥 312 members</span>
                <a href="/communities/startup-torino" class="text-blue-600 hover:text-blue-700">View →</a>
            </div>
        </div>
    </div>
    "#, filter_text, filter_text, filter_text))
}

/// Communities search fragment (same as list but with search query)
pub async fn communities_search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CommunitiesQuery>,
) -> Html<String> {
    communities_list(State(state), Query(query)).await
}

/// Chat room header fragment
pub async fn chat_header(State(_state): State<Arc<AppState>>) -> Html<String> {
    Html(r#"
    <div class="flex items-center justify-between">
        <div>
            <h2 class="text-xl font-bold">Tech Community Chat</h2>
            <p class="text-sm text-gray-500">👥 12 members online</p>
        </div>
        <div class="flex gap-2">
            <button class="px-3 py-1 text-sm text-gray-600 hover:text-gray-900">
                ℹ️ Info
            </button>
            <button class="px-3 py-1 text-sm text-gray-600 hover:text-gray-900">
                ⚙️ Settings
            </button>
        </div>
    </div>
    "#.to_string())
}

/// User communities fragment (PROTECTED - requires authentication)
pub async fn user_communities(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    // Parse user_id as UUID
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Fetch user's communities from database
    let communities = sqlx::query(
        "SELECT c.id, c.name, c.description, COUNT(DISTINCT m.user_id) as member_count
         FROM communities c
         LEFT JOIN community_members m ON c.id = m.community_id
         WHERE c.created_by = $1
         GROUP BY c.id, c.name, c.description
         ORDER BY c.created_at DESC
         LIMIT 10"
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if communities.is_empty() {
        return Ok(Html(r#"
        <div class="text-center py-8 text-gray-500">
            <p>No communities yet. <a href="/communities" class="text-indigo-600 hover:text-indigo-700">Browse communities</a> or create one!</p>
        </div>
        "#.to_string()));
    }
    
    let mut html = String::new();
    for row in communities {
        let id = row.get::<uuid::Uuid, _>("id").to_string();
        let name = row.get::<String, _>("name");
        let description = row.get::<Option<String>, _>("description").unwrap_or_default();
        let member_count = row.get::<i64, _>("member_count");
        
        html.push_str(&format!(
            r#"<div class="flex items-center justify-between p-4 border-b hover:bg-gray-50 transition">
                <div class="flex-1">
                    <h3 class="font-semibold text-gray-900">{}</h3>
                    <p class="text-sm text-gray-600">{}</p>
                    <p class="text-xs text-gray-500 mt-1">👥 {} members</p>
                </div>
                <a href="/communities/{}" class="text-indigo-600 hover:text-indigo-700 font-medium">View →</a>
            </div>"#,
            name, description, member_count, id
        ));
    }
    
    Ok(Html(html))
}

/// User activity fragment (PROTECTED - requires authentication)
pub async fn user_activity(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    // Parse user_id as UUID
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Fetch recent posts from user's communities
    let posts = sqlx::query(
        "SELECT p.id, p.title, p.community_id, c.name as community_name, p.created_at
         FROM posts p
         JOIN communities c ON p.community_id = c.id
         WHERE c.created_by = $1
         ORDER BY p.created_at DESC
         LIMIT 5"
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if posts.is_empty() {
        return Ok(Html(r#"
        <div class="text-center py-8 text-gray-500">
            <p>No recent activity in your communities.</p>
        </div>
        "#.to_string()));
    }
    
    let mut html = String::new();
    for row in posts {
        let title = row.get::<String, _>("title");
        let community_name = row.get::<String, _>("community_name");
        let created_at = row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
            .format("%Y-%m-%d %H:%M")
            .to_string();
        
        html.push_str(&format!(
            r#"<div class="flex items-start justify-between p-4 border-b hover:bg-gray-50 transition">
                <div class="flex-1">
                    <p class="font-semibold text-gray-900">{}</p>
                    <p class="text-sm text-gray-600">in <span class="font-medium">{}</span></p>
                    <p class="text-xs text-gray-500 mt-1">{}</p>
                </div>
            </div>"#,
            title, community_name, created_at
        ));
    }
    
    Ok(Html(html))
}
