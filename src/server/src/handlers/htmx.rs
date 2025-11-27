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

/// Dashboard active proposals fragment (PROTECTED - requires authentication)
pub async fn dashboard_active_proposals(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Fetch active proposals from user's communities
    let proposals = sqlx::query(
        r#"SELECT p.id, p.title, p.status, p.voting_ends_at, c.name as community_name, c.id as community_id,
                  (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
           FROM proposals p
           JOIN communities c ON p.community_id = c.id
           JOIN community_members cm ON c.id = cm.community_id AND cm.user_id = $1 AND cm.status = 'active'
           WHERE p.status = 'active'
           ORDER BY p.voting_ends_at ASC NULLS LAST
           LIMIT 5"#
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if proposals.is_empty() {
        return Ok(Html(r#"
        <div class="text-center py-6 text-civiqo-gray-600">
            <svg class="mx-auto h-8 w-8 text-civiqo-gray-400 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"/>
            </svg>
            <p class="text-sm">Nessuna votazione attiva nelle tue community.</p>
        </div>
        "#.to_string()));
    }
    
    let mut html = String::new();
    for row in proposals {
        let id: uuid::Uuid = row.get("id");
        let title: String = row.get("title");
        let community_name: String = row.get("community_name");
        let community_id: uuid::Uuid = row.get("community_id");
        let vote_count: i64 = row.get("vote_count");
        let voting_ends: Option<chrono::DateTime<chrono::Utc>> = row.get("voting_ends_at");
        
        let time_left = if let Some(ends) = voting_ends {
            let now = chrono::Utc::now();
            if ends > now {
                let diff = ends - now;
                if diff.num_days() > 0 {
                    format!("⏱️ {} giorni", diff.num_days())
                } else if diff.num_hours() > 0 {
                    format!("⏱️ {} ore", diff.num_hours())
                } else {
                    "⏱️ A breve".to_string()
                }
            } else {
                "⏱️ Scaduta".to_string()
            }
        } else {
            "⏱️ Nessuna scadenza".to_string()
        };
        
        html.push_str(&format!(r#"
        <a href="/communities/{}?tab=governance" class="block p-3 border border-civiqo-gray-200 rounded-lg hover:bg-civiqo-gray-50 hover:border-civiqo-blue transition">
            <div class="flex items-center justify-between">
                <div class="flex-1 min-w-0">
                    <p class="font-medium text-civiqo-gray-900 truncate">{}</p>
                    <p class="text-xs text-civiqo-gray-600">{}</p>
                </div>
                <div class="ml-4 flex items-center space-x-3 text-xs text-civiqo-gray-600">
                    <span>🗳️ {}</span>
                    <span class="text-civiqo-coral">{}</span>
                </div>
            </div>
        </a>
        "#, community_id, title, community_name, vote_count, time_left));
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
pub async fn governance_proposals(State(state): State<Arc<AppState>>) -> Html<String> {
    use sqlx::Row;
    
    // Fetch proposals from database
    let proposals = sqlx::query(
        r#"SELECT p.id, p.title, p.description, p.status, p.voting_ends_at,
                  c.name as community_name,
                  (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
           FROM proposals p
           JOIN communities c ON p.community_id = c.id
           ORDER BY p.created_at DESC
           LIMIT 10"#
    )
    .fetch_all(&state.db.pool)
    .await;
    
    match proposals {
        Ok(rows) if !rows.is_empty() => {
            let mut html = String::new();
            for row in rows {
                let id: uuid::Uuid = row.get("id");
                let title: String = row.get("title");
                let description: Option<String> = row.get("description");
                let status: String = row.get("status");
                let community_name: String = row.get("community_name");
                let vote_count: i64 = row.get("vote_count");
                let voting_ends: Option<chrono::DateTime<chrono::Utc>> = row.get("voting_ends_at");
                
                let status_class = match status.as_str() {
                    "active" => "bg-green-100 text-green-800",
                    "draft" => "bg-yellow-100 text-yellow-800",
                    "closed" => "bg-gray-100 text-gray-800",
                    _ => "bg-blue-100 text-blue-800",
                };
                
                let ends_text = voting_ends
                    .map(|dt| {
                        let now = chrono::Utc::now();
                        let diff = dt.signed_duration_since(now);
                        if diff.num_days() > 0 {
                            format!("Ends in {} days", diff.num_days())
                        } else if diff.num_hours() > 0 {
                            format!("Ends in {} hours", diff.num_hours())
                        } else {
                            "Ending soon".to_string()
                        }
                    })
                    .unwrap_or_else(|| "No end date".to_string());
                
                html.push_str(&format!(r#"
                <div class="bg-white rounded-lg shadow-sm p-6 hover:shadow-md transition">
                    <div class="flex items-start justify-between mb-4">
                        <div>
                            <h3 class="text-lg font-semibold text-civiqo-gray-900">{}</h3>
                            <p class="text-civiqo-gray-600 text-sm mt-1">{}</p>
                        </div>
                        <span class="px-3 py-1 {} text-sm rounded-full capitalize">{}</span>
                    </div>
                    <p class="text-civiqo-gray-700 mb-4">{}</p>
                    <div class="flex items-center justify-between text-sm text-civiqo-gray-500">
                        <div class="flex items-center space-x-4">
                            <span>{}</span>
                            <span>•</span>
                            <span>{} votes</span>
                        </div>
                        <a href="/governance/{}" 
                           class="text-civiqo-blue hover:underline"
                           hx-boost="true">View Details →</a>
                    </div>
                </div>
                "#,
                    title,
                    community_name,
                    status_class,
                    status,
                    description.unwrap_or_default(),
                    ends_text,
                    vote_count,
                    id
                ));
            }
            Html(html)
        }
        _ => {
            Html(r#"
            <div class="bg-white rounded-lg shadow-sm p-8 text-center">
                <svg class="mx-auto h-12 w-12 text-civiqo-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                </svg>
                <h3 class="mt-4 text-lg font-medium text-civiqo-gray-900">No proposals yet</h3>
                <p class="mt-2 text-civiqo-gray-600">Be the first to create a proposal for your community!</p>
            </div>
            "#.to_string())
        }
    }
}

// =============================================================================
// COMMUNITY PROPOSALS FRAGMENTS
// =============================================================================

/// Community proposals list fragment
pub async fn community_proposals(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<uuid::Uuid>,
) -> Html<String> {
    let proposals = sqlx::query(
        r#"SELECT p.id, p.title, p.description, p.status, p.proposal_type,
                  p.voting_starts_at, p.voting_ends_at,
                  COALESCE(up.name, u.email) as author_name,
                  (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
           FROM proposals p
           JOIN users u ON p.created_by = u.id
           LEFT JOIN user_profiles up ON u.id = up.user_id
           WHERE p.community_id = $1
           ORDER BY 
               CASE p.status 
                   WHEN 'active' THEN 1 
                   WHEN 'draft' THEN 2 
                   ELSE 3 
               END,
               p.created_at DESC
           LIMIT 20"#
    )
    .bind(community_id)
    .fetch_all(&state.db.pool)
    .await;
    
    match proposals {
        Ok(rows) if !rows.is_empty() => {
            let mut html = String::new();
            html.push_str("<div class=\"space-y-4\">");
            
            for row in rows {
                let id: uuid::Uuid = row.get("id");
                let title: String = row.get("title");
                let description: Option<String> = row.get("description");
                let status: String = row.get("status");
                let proposal_type: String = row.get("proposal_type");
                let author_name: String = row.get("author_name");
                let vote_count: i64 = row.get("vote_count");
                let voting_ends: Option<chrono::DateTime<chrono::Utc>> = row.get("voting_ends_at");
                
                let status_badge = match status.as_str() {
                    "active" => r#"<span class="px-2 py-1 text-xs font-medium bg-civiqo-green/10 text-civiqo-green rounded-full">🗳️ Votazione Aperta</span>"#,
                    "draft" => r#"<span class="px-2 py-1 text-xs font-medium bg-civiqo-gray-200 text-civiqo-gray-600 rounded-full">📝 Bozza</span>"#,
                    "closed" => r#"<span class="px-2 py-1 text-xs font-medium bg-civiqo-blue/10 text-civiqo-blue rounded-full">✓ Conclusa</span>"#,
                    _ => r#"<span class="px-2 py-1 text-xs font-medium bg-gray-100 text-gray-600 rounded-full">-</span>"#,
                };
                
                let type_icon = match proposal_type.as_str() {
                    "vote" => "🗳️",
                    "poll" => "📊",
                    _ => "💬",
                };
                
                let time_info = if let Some(ends) = voting_ends {
                    let now = chrono::Utc::now();
                    if ends > now {
                        let diff = ends - now;
                        if diff.num_days() > 0 {
                            format!("Termina tra {} giorni", diff.num_days())
                        } else if diff.num_hours() > 0 {
                            format!("Termina tra {} ore", diff.num_hours())
                        } else {
                            "Termina a breve".to_string()
                        }
                    } else {
                        "Votazione terminata".to_string()
                    }
                } else {
                    "Nessuna scadenza".to_string()
                };
                
                html.push_str(&format!(r#"
                <div class="bg-white border border-civiqo-gray-200 rounded-lg p-4 hover:shadow-md transition">
                    <div class="flex items-start justify-between">
                        <div class="flex-1">
                            <div class="flex items-center space-x-2 mb-2">
                                <span class="text-lg">{}</span>
                                {}
                            </div>
                            <h4 class="font-semibold text-civiqo-gray-900 mb-1">{}</h4>
                            <p class="text-sm text-civiqo-gray-600 line-clamp-2">{}</p>
                            <div class="flex items-center space-x-4 mt-3 text-xs text-civiqo-gray-600">
                                <span>👤 {}</span>
                                <span>🗳️ {} voti</span>
                                <span>⏱️ {}</span>
                            </div>
                        </div>
                        {}
                    </div>
                </div>
                "#,
                    type_icon,
                    status_badge,
                    title,
                    description.unwrap_or_default(),
                    author_name,
                    vote_count,
                    time_info,
                    if status == "active" {
                        format!(r#"
                        <button hx-post="/api/proposals/{}/vote" 
                                hx-target="closest div"
                                hx-swap="outerHTML"
                                class="ml-4 px-4 py-2 bg-civiqo-green text-white rounded-lg hover:bg-civiqo-green/90 transition text-sm font-medium">
                            Vota
                        </button>
                        "#, id)
                    } else {
                        String::new()
                    }
                ));
            }
            
            html.push_str("</div>");
            Html(html)
        }
        _ => Html(r#"
            <div class="text-center py-8">
                <svg class="mx-auto h-12 w-12 text-civiqo-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                </svg>
                <h3 class="mt-4 text-lg font-medium text-civiqo-gray-900">Nessuna proposta</h3>
                <p class="mt-2 text-civiqo-gray-600">Sii il primo a creare una proposta per questa community!</p>
            </div>
            "#.to_string())
    }
}

/// Create proposal via HTMX form
#[derive(Debug, serde::Deserialize)]
pub struct CreateProposalForm {
    pub community_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub proposal_type: Option<String>,
    pub voting_starts_at: Option<String>,
    pub voting_ends_at: Option<String>,
}

pub async fn create_proposal_htmx(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<uuid::Uuid>,
    axum::Form(form): axum::Form<CreateProposalForm>,
) -> Result<Html<String>, AppError> {
    // Validate title
    if form.title.trim().is_empty() {
        return Ok(Html(r#"<div class="p-4 bg-red-100 text-red-700 rounded-lg mb-4">Il titolo è obbligatorio</div>"#.to_string()));
    }
    
    // Parse user_id
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid user ID")))?;
    
    // Verify user is member of community
    let is_member = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active'"
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    if is_member == 0 {
        return Ok(Html(r#"<div class="p-4 bg-red-100 text-red-700 rounded-lg mb-4">Devi essere membro della community per creare proposte</div>"#.to_string()));
    }
    
    let proposal_type = form.proposal_type.unwrap_or_else(|| "text".to_string());
    
    // Parse dates (datetime-local format: 2024-01-15T10:30)
    let voting_starts: Option<chrono::DateTime<chrono::Utc>> = form.voting_starts_at
        .as_ref()
        .filter(|s| !s.is_empty())
        .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M").ok())
        .map(|dt| dt.and_utc());
    
    let voting_ends: Option<chrono::DateTime<chrono::Utc>> = form.voting_ends_at
        .as_ref()
        .filter(|s| !s.is_empty())
        .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M").ok())
        .map(|dt| dt.and_utc());
    
    // Insert proposal
    let proposal_id = uuid::Uuid::now_v7();
    sqlx::query(
        r#"INSERT INTO proposals (id, community_id, created_by, title, description, 
                                   proposal_type, status, voting_starts_at, voting_ends_at, 
                                   quorum_required, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, 'draft', $7, $8, 0, NOW(), NOW())"#
    )
    .bind(proposal_id)
    .bind(community_id)
    .bind(user_uuid)
    .bind(&form.title)
    .bind(&form.description)
    .bind(&proposal_type)
    .bind(voting_starts)
    .bind(voting_ends)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError(anyhow::anyhow!("Database error: {}", e)))?;
    
    // Return updated proposals list
    Ok(community_proposals(State(state), axum::extract::Path(community_id)).await)
}

/// Community proposals count badge
pub async fn community_proposals_count(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<uuid::Uuid>,
) -> Html<String> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM proposals WHERE community_id = $1 AND status = 'active'"
    )
    .bind(community_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    if count > 0 {
        Html(format!(r#"<script>document.getElementById('proposals-badge').classList.remove('hidden');</script>{}"#, count))
    } else {
        Html(String::new())
    }
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
        r#"SELECT u.id, u.email, p.name, p.avatar_url, r.name as role, cm.joined_at
           FROM community_members cm
           JOIN users u ON cm.user_id = u.id
           LEFT JOIN user_profiles p ON u.id = p.user_id
           LEFT JOIN roles r ON cm.role_id = r.id
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

// =============================================================================
// SEARCH FRAGMENTS
// =============================================================================

#[derive(Debug, serde::Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

/// Global search - returns HTML fragment
pub async fn search_results(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Html<String> {
    let query = params.q.trim();
    if query.len() < 2 {
        return Html("<div class=\"p-4 text-center text-gray-500\">Inserisci almeno 2 caratteri</div>".to_string());
    }
    
    let search_pattern = format!("%{}%", query.to_lowercase());
    
    // Search users
    let users = sqlx::query(
        r#"SELECT u.id, u.email, p.name, p.avatar_url
           FROM users u
           LEFT JOIN user_profiles p ON u.id = p.user_id
           WHERE LOWER(u.email) LIKE $1 OR LOWER(p.name) LIKE $1
           LIMIT 5"#
    )
    .bind(&search_pattern)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    // Search communities
    let communities = sqlx::query(
        r#"SELECT c.id, c.name, c.description, COUNT(cm.user_id) as member_count
           FROM communities c
           LEFT JOIN community_members cm ON c.id = cm.community_id AND cm.status = 'active'
           WHERE LOWER(c.name) LIKE $1 OR LOWER(c.description) LIKE $1
           GROUP BY c.id, c.name, c.description
           LIMIT 5"#
    )
    .bind(&search_pattern)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    // Search posts
    let posts = sqlx::query(
        r#"SELECT p.id, p.title, c.name as community_name, p.created_at
           FROM posts p
           JOIN communities c ON p.community_id = c.id
           WHERE LOWER(p.title) LIKE $1
           ORDER BY p.created_at DESC
           LIMIT 5"#
    )
    .bind(&search_pattern)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if users.is_empty() && communities.is_empty() && posts.is_empty() {
        return Html(format!(r#"
            <div class="p-4 text-center text-gray-500">
                <p>Nessun risultato per "{}"</p>
            </div>
        "#, query));
    }
    
    let mut html = String::from("<div class=\"p-2 space-y-4\">");
    
    // Users section
    if !users.is_empty() {
        html.push_str("<div><p class=\"text-xs font-semibold text-gray-400 uppercase px-2 mb-1\">Utenti</p>");
        for user in users {
            let id: uuid::Uuid = user.get("id");
            let email: String = user.get("email");
            let name: Option<String> = user.get("name");
            let avatar_url: Option<String> = user.get("avatar_url");
            let display_name = name.unwrap_or_else(|| email.clone());
            let initial = display_name.chars().next().unwrap_or('?').to_uppercase().to_string();
            
            let avatar = if let Some(url) = avatar_url {
                format!(r#"<img src="{}" class="w-8 h-8 rounded-full object-cover">"#, url)
            } else {
                format!(r#"<div class="w-8 h-8 rounded-full bg-[#57C98A]/10 text-[#57C98A] flex items-center justify-center text-sm font-medium">{}</div>"#, initial)
            };
            
            html.push_str(&format!(r#"
                <a href="/users/{}" class="flex items-center space-x-2 p-2 hover:bg-gray-100 rounded-lg">
                    {}
                    <span class="text-sm text-gray-900">{}</span>
                </a>
            "#, id, avatar, display_name));
        }
        html.push_str("</div>");
    }
    
    // Communities section
    if !communities.is_empty() {
        html.push_str("<div><p class=\"text-xs font-semibold text-gray-400 uppercase px-2 mb-1\">Community</p>");
        for community in communities {
            let id: uuid::Uuid = community.get("id");
            let name: String = community.get("name");
            let member_count: i64 = community.get("member_count");
            
            html.push_str(&format!(r#"
                <a href="/communities/{}" class="block p-2 hover:bg-gray-100 rounded-lg">
                    <p class="text-sm font-medium text-gray-900">{}</p>
                    <p class="text-xs text-gray-500">{} membri</p>
                </a>
            "#, id, name, member_count));
        }
        html.push_str("</div>");
    }
    
    // Posts section
    if !posts.is_empty() {
        html.push_str("<div><p class=\"text-xs font-semibold text-gray-400 uppercase px-2 mb-1\">Post</p>");
        for post in posts {
            let id: uuid::Uuid = post.get("id");
            let title: String = post.get("title");
            let community_name: String = post.get("community_name");
            
            html.push_str(&format!(r#"
                <a href="/posts/{}" class="block p-2 hover:bg-gray-100 rounded-lg">
                    <p class="text-sm font-medium text-gray-900">{}</p>
                    <p class="text-xs text-gray-500">in {}</p>
                </a>
            "#, id, title, community_name));
        }
        html.push_str("</div>");
    }
    
    html.push_str("</div>");
    Html(html)
}

// =============================================================================
// FOLLOW BUTTON FRAGMENT
// =============================================================================

/// Follow button fragment - checks if current user follows target
pub async fn follow_button(
    crate::auth::OptionalAuthUser(current_user): crate::auth::OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path(target_user_id): axum::extract::Path<String>,
) -> Html<String> {
    let Some(user) = current_user else {
        // Not logged in - show login prompt
        return Html(r#"
            <a href="/auth/login" class="px-4 py-2 bg-[#57C98A] hover:bg-[#4ab87a] text-white rounded-lg transition text-sm font-medium">
                Accedi per seguire
            </a>
        "#.to_string());
    };
    
    // Don't show follow button for own profile
    if user.user_id == target_user_id {
        return Html(String::new());
    }
    
    let follower_uuid = match uuid::Uuid::parse_str(&user.user_id) {
        Ok(u) => u,
        Err(_) => return Html("<div class=\"text-red-500\">Errore</div>".to_string()),
    };
    let following_uuid = match uuid::Uuid::parse_str(&target_user_id) {
        Ok(u) => u,
        Err(_) => return Html("<div class=\"text-red-500\">Errore</div>".to_string()),
    };
    
    // Check if already following
    let is_following: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM user_follows WHERE follower_id = $1 AND following_id = $2)"
    )
    .bind(follower_uuid)
    .bind(following_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(false);
    
    if is_following {
        Html(format!(r#"
            <button hx-post="/api/users/{}/unfollow"
                    hx-target="this"
                    hx-swap="outerHTML"
                    class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 hover:border-red-300 hover:text-red-600 transition text-sm font-medium group">
                <span class="group-hover:hidden">Seguendo</span>
                <span class="hidden group-hover:inline">Smetti di seguire</span>
            </button>
        "#, target_user_id))
    } else {
        Html(format!(r#"
            <button hx-post="/api/users/{}/follow"
                    hx-target="this"
                    hx-swap="outerHTML"
                    class="px-4 py-2 bg-[#57C98A] hover:bg-[#4ab87a] text-white rounded-lg transition text-sm font-medium">
                Segui
            </button>
        "#, target_user_id))
    }
}

// =============================================================================
// NOTIFICATIONS FRAGMENT
// =============================================================================

/// Notifications dropdown content
pub async fn notifications_dropdown(
    crate::auth::AuthUser(user): crate::auth::AuthUser,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user.user_id) {
        Ok(u) => u,
        Err(_) => return Html("<div class=\"p-4 text-red-500\">Errore</div>".to_string()),
    };
    
    // Fetch recent notifications
    let notifications = sqlx::query(
        r#"SELECT n.id, n.type, n.message, n.is_read, n.created_at,
                  n.target_type, n.target_id,
                  p.name as actor_name, p.avatar_url as actor_avatar
           FROM notifications n
           LEFT JOIN users u ON n.actor_id = u.id
           LEFT JOIN user_profiles p ON u.id = p.user_id
           WHERE n.user_id = $1
           ORDER BY n.created_at DESC
           LIMIT 10"#
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if notifications.is_empty() {
        return Html(r#"
            <div class="p-4 text-center text-gray-500">
                <p>Nessuna notifica</p>
            </div>
        "#.to_string());
    }
    
    let mut html = String::from("<div class=\"divide-y divide-gray-100\">");
    
    for notif in notifications {
        let message: Option<String> = notif.get("message");
        let is_read: bool = notif.get("is_read");
        let created_at: chrono::DateTime<chrono::Utc> = notif.get("created_at");
        let actor_name: Option<String> = notif.get("actor_name");
        let actor_avatar: Option<String> = notif.get("actor_avatar");
        
        let bg_class = if is_read { "" } else { "bg-blue-50" };
        let display_name = actor_name.unwrap_or_else(|| "Qualcuno".to_string());
        let display_message = message.unwrap_or_else(|| "ha interagito con te".to_string());
        let time_ago = format_time_ago(created_at);
        
        let avatar = if let Some(url) = actor_avatar {
            format!(r#"<img src="{}" class="w-10 h-10 rounded-full object-cover">"#, url)
        } else {
            let initial = display_name.chars().next().unwrap_or('?').to_uppercase().to_string();
            format!(r#"<div class="w-10 h-10 rounded-full bg-[#57C98A]/10 text-[#57C98A] flex items-center justify-center font-medium">{}</div>"#, initial)
        };
        
        html.push_str(&format!(r#"
            <div class="flex items-start space-x-3 p-3 {} hover:bg-gray-50">
                {}
                <div class="flex-1 min-w-0">
                    <p class="text-sm text-gray-900"><span class="font-medium">{}</span> {}</p>
                    <p class="text-xs text-gray-500">{}</p>
                </div>
            </div>
        "#, bg_class, avatar, display_name, display_message, time_ago));
    }
    
    html.push_str("</div>");
    Html(html)
}

fn format_time_ago(dt: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(dt);
    
    if diff.num_minutes() < 1 {
        "ora".to_string()
    } else if diff.num_minutes() < 60 {
        format!("{} min fa", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{} ore fa", diff.num_hours())
    } else if diff.num_days() < 7 {
        format!("{} giorni fa", diff.num_days())
    } else {
        dt.format("%d %b").to_string()
    }
}

// =============================================================================
// USER PROFILE TAB FRAGMENTS
// =============================================================================

/// User posts fragment
pub async fn user_posts(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user_id) {
        Ok(u) => u,
        Err(_) => return Html("<p class=\"text-red-500\">Invalid user ID</p>".to_string()),
    };
    
    let posts = sqlx::query(
        r#"SELECT p.id, p.title, p.created_at, c.name as community_name
           FROM posts p
           JOIN communities c ON p.community_id = c.id
           WHERE p.author_id = $1
           ORDER BY p.created_at DESC
           LIMIT 20"#
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if posts.is_empty() {
        return Html("<div class=\"text-center py-8 text-gray-500\">Nessun post ancora</div>".to_string());
    }
    
    let mut html = String::from("<div class=\"space-y-3\">");
    for post in posts {
        let id: uuid::Uuid = post.get("id");
        let title: String = post.get("title");
        let community_name: String = post.get("community_name");
        let created_at: chrono::DateTime<chrono::Utc> = post.get("created_at");
        
        html.push_str(&format!(r#"
            <a href="/posts/{}" class="block p-4 bg-gray-50 rounded-lg hover:bg-gray-100 transition">
                <h4 class="font-medium text-gray-900">{}</h4>
                <p class="text-sm text-gray-500">in {} • {}</p>
            </a>
        "#, id, title, community_name, created_at.format("%d %b %Y")));
    }
    html.push_str("</div>");
    Html(html)
}

/// User communities fragment
pub async fn user_profile_communities(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user_id) {
        Ok(u) => u,
        Err(_) => return Html("<p class=\"text-red-500\">Invalid user ID</p>".to_string()),
    };
    
    let communities = sqlx::query(
        r#"SELECT c.id, c.name, c.description
           FROM communities c
           JOIN community_members cm ON c.id = cm.community_id
           WHERE cm.user_id = $1 AND cm.status = 'active'
           ORDER BY cm.joined_at DESC
           LIMIT 20"#
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if communities.is_empty() {
        return Html("<div class=\"text-center py-8 text-gray-500\">Nessuna community</div>".to_string());
    }
    
    let mut html = String::from("<div class=\"space-y-3\">");
    for community in communities {
        let id: uuid::Uuid = community.get("id");
        let name: String = community.get("name");
        let description: Option<String> = community.get("description");
        
        html.push_str(&format!(r#"
            <a href="/communities/{}" class="block p-4 bg-gray-50 rounded-lg hover:bg-gray-100 transition">
                <h4 class="font-medium text-gray-900">{}</h4>
                <p class="text-sm text-gray-500">{}</p>
            </a>
        "#, id, name, description.unwrap_or_default()));
    }
    html.push_str("</div>");
    Html(html)
}

/// User followers fragment
pub async fn user_followers(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user_id) {
        Ok(u) => u,
        Err(_) => return Html("<p class=\"text-red-500\">Invalid user ID</p>".to_string()),
    };
    
    let followers = sqlx::query(
        r#"SELECT u.id, u.email, p.name, p.avatar_url
           FROM user_follows f
           JOIN users u ON f.follower_id = u.id
           LEFT JOIN user_profiles p ON u.id = p.user_id
           WHERE f.following_id = $1
           ORDER BY f.created_at DESC
           LIMIT 50"#
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if followers.is_empty() {
        return Html("<div class=\"text-center py-8 text-gray-500\">Nessun follower</div>".to_string());
    }
    
    render_user_list(followers)
}

/// User following fragment
pub async fn user_following(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user_id) {
        Ok(u) => u,
        Err(_) => return Html("<p class=\"text-red-500\">Invalid user ID</p>".to_string()),
    };
    
    let following = sqlx::query(
        r#"SELECT u.id, u.email, p.name, p.avatar_url
           FROM user_follows f
           JOIN users u ON f.following_id = u.id
           LEFT JOIN user_profiles p ON u.id = p.user_id
           WHERE f.follower_id = $1
           ORDER BY f.created_at DESC
           LIMIT 50"#
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if following.is_empty() {
        return Html("<div class=\"text-center py-8 text-gray-500\">Non segue nessuno</div>".to_string());
    }
    
    render_user_list(following)
}

fn render_user_list(users: Vec<sqlx::postgres::PgRow>) -> Html<String> {
    let mut html = String::from("<div class=\"space-y-3\">");
    
    for user in users {
        let id: uuid::Uuid = user.get("id");
        let email: String = user.get("email");
        let name: Option<String> = user.get("name");
        let avatar_url: Option<String> = user.get("avatar_url");
        let display_name = name.unwrap_or_else(|| email.clone());
        let initial = display_name.chars().next().unwrap_or('?').to_uppercase().to_string();
        
        let avatar = if let Some(url) = avatar_url {
            format!(r#"<img src="{}" class="w-12 h-12 rounded-full object-cover">"#, url)
        } else {
            format!(r#"<div class="w-12 h-12 rounded-full bg-[#57C98A]/10 text-[#57C98A] flex items-center justify-center font-medium">{}</div>"#, initial)
        };
        
        html.push_str(&format!(r#"
            <a href="/users/{}" class="flex items-center space-x-3 p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition">
                {}
                <div>
                    <p class="font-medium text-gray-900">{}</p>
                    <p class="text-sm text-gray-500">{}</p>
                </div>
            </a>
        "#, id, avatar, display_name, email));
    }
    
    html.push_str("</div>");
    Html(html)
}

/// Notifications list fragment for the notifications page
#[derive(Deserialize)]
pub struct NotificationsQuery {
    pub filter: Option<String>,
    pub page: Option<i32>,
}

pub async fn notifications_list(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Query(params): Query<NotificationsQuery>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return Html(render_empty_notifications("all")),
    };
    
    let filter = params.filter.unwrap_or_else(|| "all".to_string());
    let page = params.page.unwrap_or(1);
    let limit = 20;
    let offset = (page - 1) * limit;
    
    // For now, return placeholder since notifications table may not exist yet
    // TODO: Implement actual notifications query when table is created
    Html(render_empty_notifications(&filter))
}

fn render_empty_notifications(filter: &str) -> String {
    let message = match filter {
        "unread" => "Hai letto tutte le notifiche!",
        "mentions" => "Nessuna menzione recente.",
        "votes" => "Nessuna votazione attiva.",
        _ => "Le notifiche appariranno qui quando ci saranno novità.",
    };
    
    format!(r#"
        <div class="text-center py-12 bg-white rounded-lg border border-gray-200">
            <svg class="w-16 h-16 mx-auto text-gray-200 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>
            </svg>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">Nessuna notifica</h3>
            <p class="text-gray-600">{}</p>
        </div>
    "#, message)
}

/// Mark all notifications as read
pub async fn mark_all_notifications_read(
    AuthUser(user): AuthUser,
    State(_state): State<Arc<AppState>>,
) -> Html<String> {
    // TODO: Implement when notifications table exists
    // For now, just return success
    Html(String::new())
}
