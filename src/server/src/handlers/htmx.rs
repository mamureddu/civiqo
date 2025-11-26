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

/// Recent communities fragment - fetches from database
pub async fn recent_communities(State(state): State<Arc<AppState>>) -> Html<String> {
    use sqlx::Row;
    
    // Fetch recent communities from database
    let communities = sqlx::query(
        "SELECT c.id, c.name, c.slug, c.description, 
                COUNT(DISTINCT m.user_id) as member_count
         FROM communities c
         LEFT JOIN community_members m ON c.id = m.community_id
         WHERE c.is_public = true
         GROUP BY c.id, c.name, c.slug, c.description
         ORDER BY c.created_at DESC
         LIMIT 6"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if communities.is_empty() {
        return Html(r#"<div class="col-span-full text-center py-8 text-gray-500">No communities yet. Be the first to create one!</div>"#.to_string());
    }
    
    let mut html = String::new();
    for (i, row) in communities.iter().enumerate() {
        let id: uuid::Uuid = row.get("id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let member_count: i64 = row.get("member_count");
        let desc = description.unwrap_or_else(|| "A community on Civiqo".to_string());
        
        html.push_str(&format!(r#"
        <div class="community-card fade-in bg-white rounded-lg shadow-sm p-6 border border-gray-200 hover:shadow-md transition-shadow" style="animation-delay: {}ms;">
            <h3 class="text-xl font-bold mb-2 text-gray-900">{}</h3>
            <p class="text-gray-600 mb-4 line-clamp-2">{}</p>
            <div class="flex items-center justify-between text-sm text-gray-500">
                <span>👥 {} members</span>
                <a href="/communities/{}" class="text-blue-600 hover:text-blue-700">View →</a>
            </div>
        </div>
        "#, i * 100, name, desc, member_count, id));
    }
    
    Html(html)
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

/// Communities list fragment - fetches from database with optional search
pub async fn communities_list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CommunitiesQuery>,
) -> Html<String> {
    use sqlx::Row;
    
    // Build query based on search term
    let communities = if query.q.is_empty() {
        sqlx::query(
            "SELECT c.id, c.name, c.slug, c.description, 
                    COUNT(DISTINCT m.user_id) as member_count
             FROM communities c
             LEFT JOIN community_members m ON c.id = m.community_id
             WHERE c.is_public = true
             GROUP BY c.id, c.name, c.slug, c.description
             ORDER BY c.created_at DESC
             LIMIT 20"
        )
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
    } else {
        let search_pattern = format!("%{}%", query.q);
        sqlx::query(
            "SELECT c.id, c.name, c.slug, c.description, 
                    COUNT(DISTINCT m.user_id) as member_count
             FROM communities c
             LEFT JOIN community_members m ON c.id = m.community_id
             WHERE c.is_public = true AND (c.name ILIKE $1 OR c.description ILIKE $1)
             GROUP BY c.id, c.name, c.slug, c.description
             ORDER BY c.created_at DESC
             LIMIT 20"
        )
        .bind(&search_pattern)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
    };
    
    if communities.is_empty() {
        let msg = if query.q.is_empty() {
            "No communities yet. Be the first to create one!"
        } else {
            "No communities found matching your search."
        };
        return Html(format!(r#"<div class="col-span-full text-center py-8 text-gray-500">{}</div>"#, msg));
    }
    
    let mut html = String::from(r#"<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">"#);
    for row in &communities {
        let id: uuid::Uuid = row.get("id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let member_count: i64 = row.get("member_count");
        let desc = description.unwrap_or_else(|| "A community on Civiqo".to_string());
        
        html.push_str(&format!(r#"
        <div class="community-card bg-white rounded-lg shadow-sm p-6 border border-gray-200 hover:shadow-md transition-shadow">
            <h3 class="text-xl font-bold mb-2 text-gray-900">{}</h3>
            <p class="text-gray-600 mb-4 line-clamp-2">{}</p>
            <div class="flex items-center justify-between text-sm text-gray-500">
                <span>👥 {} members</span>
                <a href="/communities/{}" class="text-blue-600 hover:text-blue-700">View →</a>
            </div>
        </div>
        "#, name, desc, member_count, id));
    }
    html.push_str("</div>");
    
    Html(html)
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

// =============================================================================
// COMMUNITY FEED FRAGMENT
// =============================================================================

/// Community feed fragment - shows posts for a specific community
pub async fn community_feed(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<String>,
) -> Result<Html<String>, AppError> {
    let uuid = uuid::Uuid::parse_str(&community_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid community ID")))?;
    
    let posts = sqlx::query(
        "SELECT p.id, p.title, p.content, p.created_at,
                COALESCE(pr.name, u.email) as author_name
         FROM posts p
         JOIN users u ON p.author_id = u.id
         LEFT JOIN user_profiles pr ON u.id = pr.user_id
         WHERE p.community_id = $1
         ORDER BY p.created_at DESC
         LIMIT 20"
    )
    .bind(uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if posts.is_empty() {
        return Ok(Html(r#"
        <div class="text-center py-12 text-gray-500">
            <p class="text-lg">No posts yet in this community.</p>
            <p class="text-sm mt-2">Be the first to share something!</p>
        </div>
        "#.to_string()));
    }
    
    let mut html = String::new();
    for row in posts {
        let title = row.get::<String, _>("title");
        let content = row.get::<String, _>("content");
        let author = row.get::<Option<String>, _>("author_name").unwrap_or_else(|| "Anonymous".to_string());
        let created_at = row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
            .format("%Y-%m-%d %H:%M")
            .to_string();
        
        html.push_str(&format!(
            r#"<article class="bg-white rounded-lg shadow-sm p-6 border border-gray-200">
                <h3 class="text-lg font-semibold text-gray-900 mb-2">{}</h3>
                <p class="text-gray-600 mb-4">{}</p>
                <div class="flex items-center text-sm text-gray-500">
                    <span>By {}</span>
                    <span class="mx-2">•</span>
                    <span>{}</span>
                </div>
            </article>"#,
            title, content, author, created_at
        ));
    }
    
    Ok(Html(html))
}

// =============================================================================
// BUSINESSES FRAGMENTS
// =============================================================================

/// Businesses list fragment
pub async fn businesses_list(State(_state): State<Arc<AppState>>) -> Html<String> {
    // TODO: Fetch from database when businesses table exists
    Html(r#"
    <div class="bg-white rounded-lg shadow-sm overflow-hidden">
        <div class="h-48 bg-gradient-to-r from-blue-400 to-blue-600"></div>
        <div class="p-6">
            <h3 class="text-lg font-semibold text-gray-900">Sample Business</h3>
            <p class="text-gray-600 text-sm mt-1">A sample business listing</p>
            <div class="mt-4 flex items-center text-sm text-gray-500">
                <span>⭐ 4.5</span>
                <span class="mx-2">•</span>
                <span>Open Now</span>
            </div>
        </div>
    </div>
    <div class="bg-white rounded-lg shadow-sm overflow-hidden">
        <div class="h-48 bg-gradient-to-r from-green-400 to-green-600"></div>
        <div class="p-6">
            <h3 class="text-lg font-semibold text-gray-900">Another Business</h3>
            <p class="text-gray-600 text-sm mt-1">Another sample business</p>
            <div class="mt-4 flex items-center text-sm text-gray-500">
                <span>⭐ 4.8</span>
                <span class="mx-2">•</span>
                <span>Closed</span>
            </div>
        </div>
    </div>
    "#.to_string())
}

/// Businesses search fragment
#[derive(Deserialize)]
pub struct BusinessSearchQuery {
    #[serde(default)]
    pub q: String,
}

pub async fn businesses_search(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<BusinessSearchQuery>,
) -> Html<String> {
    // TODO: Implement actual search when businesses table exists
    let search_term = query.q;
    Html(format!(r#"
    <div class="col-span-full text-center py-8 text-gray-500">
        <p>Search results for "{}"</p>
        <p class="text-sm mt-2">No businesses found matching your search.</p>
    </div>
    "#, search_term))
}

/// Business posts fragment
pub async fn business_posts(
    axum::extract::Path(_business_id): axum::extract::Path<String>,
) -> Html<String> {
    Html(r#"
    <div class="text-center py-4 text-gray-500">
        <p>No updates from this business yet.</p>
    </div>
    "#.to_string())
}

/// Business reviews fragment
pub async fn business_reviews(
    axum::extract::Path(_business_id): axum::extract::Path<String>,
) -> Html<String> {
    Html(r#"
    <div class="text-center py-4 text-gray-500">
        <p>No reviews yet. Be the first to leave a review!</p>
    </div>
    "#.to_string())
}

// =============================================================================
// GOVERNANCE FRAGMENTS
// =============================================================================

/// Governance proposals fragment
pub async fn governance_proposals(State(_state): State<Arc<AppState>>) -> Html<String> {
    // TODO: Fetch from database when governance tables exist
    Html(r#"
    <div class="bg-white rounded-lg shadow-sm p-6">
        <div class="flex items-start justify-between mb-4">
            <div>
                <h3 class="text-lg font-semibold text-gray-900">Sample Proposal</h3>
                <p class="text-gray-600 text-sm mt-1">This is a sample governance proposal</p>
            </div>
            <span class="px-3 py-1 bg-green-100 text-green-800 text-sm rounded-full">Active</span>
        </div>
        <div class="flex items-center text-sm text-gray-500">
            <span>Ends in 5 days</span>
            <span class="mx-2">•</span>
            <span>42 votes</span>
        </div>
    </div>
    "#.to_string())
}

// =============================================================================
// POI FRAGMENTS
// =============================================================================

/// POI nearby places fragment
pub async fn poi_nearby(State(_state): State<Arc<AppState>>) -> Html<String> {
    // TODO: Fetch from database/external API when POI feature is implemented
    Html(r#"
    <div class="bg-gray-50 rounded-lg p-3 hover:bg-gray-100 cursor-pointer transition">
        <h4 class="font-medium text-gray-900">Central Park</h4>
        <p class="text-sm text-gray-600">0.5 km away</p>
    </div>
    <div class="bg-gray-50 rounded-lg p-3 hover:bg-gray-100 cursor-pointer transition">
        <h4 class="font-medium text-gray-900">City Library</h4>
        <p class="text-sm text-gray-600">0.8 km away</p>
    </div>
    <div class="bg-gray-50 rounded-lg p-3 hover:bg-gray-100 cursor-pointer transition">
        <h4 class="font-medium text-gray-900">Community Center</h4>
        <p class="text-sm text-gray-600">1.2 km away</p>
    </div>
    "#.to_string())
}

// =============================================================================
// COMMENT FRAGMENTS
// =============================================================================

/// Comment reply form fragment
pub async fn comment_reply_form(
    axum::extract::Path(comment_id): axum::extract::Path<String>,
) -> Html<String> {
    Html(format!(r#"
    <form hx-post="/api/comments/{}/replies" hx-swap="outerHTML" class="mt-2">
        <textarea name="content" rows="2" 
                  class="w-full px-3 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-[#57C98A] text-sm"
                  placeholder="Write a reply..."></textarea>
        <div class="flex justify-end mt-2 space-x-2">
            <button type="button" hx-get="/htmx/empty" hx-target="closest form" hx-swap="outerHTML"
                    class="px-3 py-1 text-gray-500 text-sm">Cancel</button>
            <button type="submit" class="px-3 py-1 bg-[#57C98A] text-white text-sm rounded">Reply</button>
        </div>
    </form>
    "#, comment_id))
}

/// Comment edit form fragment
pub async fn comment_edit_form(
    axum::extract::Path(comment_id): axum::extract::Path<String>,
) -> Html<String> {
    // TODO: Fetch actual comment content from database
    Html(format!(r#"
    <form hx-put="/api/comments/{}" hx-swap="outerHTML" class="mt-2">
        <textarea name="content" rows="3" 
                  class="w-full px-3 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-[#57C98A] text-sm"
                  placeholder="Edit your comment..."></textarea>
        <div class="flex justify-end mt-2 space-x-2">
            <button type="button" hx-get="/htmx/empty" hx-target="closest form" hx-swap="outerHTML"
                    class="px-3 py-1 text-gray-500 text-sm">Cancel</button>
            <button type="submit" class="px-3 py-1 bg-[#57C98A] text-white text-sm rounded">Save</button>
        </div>
    </form>
    "#, comment_id))
}

/// Empty fragment - used for clearing content
pub async fn empty_fragment() -> Html<String> {
    Html(String::new())
}

// =============================================================================
// COMMUNITY MEMBERS FRAGMENT
// =============================================================================

/// Community members list fragment - returns HTML for HTMX
pub async fn community_members(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<String>,
) -> Html<String> {
    let uuid = match uuid::Uuid::parse_str(&community_id) {
        Ok(u) => u,
        Err(_) => return Html("<p class=\"text-red-500\">Invalid community ID</p>".to_string()),
    };
    
    // Fetch members from database
    let members = sqlx::query(
        r#"SELECT u.id, u.email, p.name, p.avatar_url, cm.role, cm.joined_at
           FROM community_members cm
           JOIN users u ON cm.user_id = u.id
           LEFT JOIN user_profiles p ON u.id = p.user_id
           WHERE cm.community_id = $1 AND cm.status = 'active'
           ORDER BY cm.joined_at ASC
           LIMIT 50"#
    )
    .bind(uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if members.is_empty() {
        return Html(r#"
        <div class="text-center py-8 text-gray-500">
            <p>Nessun membro ancora.</p>
        </div>
        "#.to_string());
    }
    
    let mut html = String::from(r#"<div class="space-y-3">"#);
    
    for member in members {
        let email: String = member.get("email");
        let name: Option<String> = member.get("name");
        let avatar_url: Option<String> = member.get("avatar_url");
        let role: Option<String> = member.get("role");
        let display_name = name.unwrap_or_else(|| email.clone());
        let initial = display_name.chars().next().unwrap_or('?').to_uppercase().to_string();
        let role_badge = match role.as_deref() {
            Some("admin") => r#"<span class="px-2 py-0.5 bg-[#3B7FBA]/10 text-[#3B7FBA] text-xs rounded-full">Admin</span>"#,
            Some("moderator") => r#"<span class="px-2 py-0.5 bg-[#57C98A]/10 text-[#57C98A] text-xs rounded-full">Mod</span>"#,
            _ => "",
        };
        
        let avatar_html = if let Some(url) = avatar_url {
            format!(r#"<img src="{}" alt="{}" class="w-10 h-10 rounded-full object-cover">"#, url, display_name)
        } else {
            format!(r#"<div class="w-10 h-10 rounded-full bg-[#57C98A]/10 flex items-center justify-center text-[#57C98A] font-medium">{}</div>"#, initial)
        };
        
        html.push_str(&format!(r#"
        <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition">
            <div class="flex items-center space-x-3">
                {}
                <div>
                    <p class="font-medium text-gray-900">{}</p>
                    <p class="text-sm text-gray-500">{}</p>
                </div>
            </div>
            {}
        </div>
        "#, avatar_html, display_name, email, role_badge));
    }
    
    html.push_str("</div>");
    Html(html)
}
