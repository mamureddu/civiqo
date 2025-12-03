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
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
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

/// User communities as select options (PROTECTED - for proposal form)
#[allow(dead_code)]
pub async fn user_communities_options(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return Html(r#"<option value="">Errore: ID utente non valido</option>"#.to_string()),
    };
    
    // Fetch communities where user is a member (can create proposals)
    let communities = sqlx::query(
        r#"SELECT c.id, c.name
           FROM communities c
           JOIN community_members cm ON c.id = cm.community_id
           WHERE cm.user_id = $1 AND cm.status = 'active'
           ORDER BY c.name ASC"#
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if communities.is_empty() {
        return Html(r#"<option value="">Nessuna community disponibile</option>"#.to_string());
    }
    
    let mut html = String::from(r#"<option value="">Seleziona una community...</option>"#);
    for row in communities {
        let id: uuid::Uuid = row.get("id");
        let name: String = row.get("name");
        html.push_str(&format!(r#"<option value="{}">{}</option>"#, id, name));
    }
    
    Html(html)
}

/// Dashboard active proposals fragment (PROTECTED - requires authentication)
pub async fn dashboard_active_proposals(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
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
        let _id: uuid::Uuid = row.get("id");
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
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
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
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid community ID")))?;
    
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
        <div class="text-center py-8">
            <svg class="mx-auto h-12 w-12 text-civiqo-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                      d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z" />
            </svg>
            <h3 class="mt-4 text-lg font-medium text-civiqo-gray-900">Nessun post</h3>
            <p class="mt-2 text-civiqo-gray-600">Iscriviti alla community per pubblicare il primo post!</p>
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

/// Businesses list fragment - fetches from database
pub async fn businesses_list(State(state): State<Arc<AppState>>) -> Html<String> {
    let businesses = sqlx::query(
        r#"SELECT b.id, b.name, b.description, b.category, b.address, 
                  b.rating_avg, b.review_count, b.cover_url, b.is_verified
           FROM businesses b
           WHERE b.is_active = true
           ORDER BY b.rating_avg DESC NULLS LAST, b.created_at DESC
           LIMIT 12"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    if businesses.is_empty() {
        return Html(r#"
        <div class="col-span-full text-center py-12">
            <svg class="w-16 h-16 mx-auto text-civiqo-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
            </svg>
            <p class="text-civiqo-gray-600 font-medium">Nessuna attività trovata</p>
            <p class="text-civiqo-gray-500 text-sm mt-1">Sii il primo ad aggiungere un'attività!</p>
        </div>
        "#.to_string());
    }

    let mut html = String::new();
    for row in businesses {
        let id: uuid::Uuid = row.get("id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let category: Option<String> = row.get("category");
        let address: Option<String> = row.get("address");
        let rating_avg: Option<f64> = row.get("rating_avg");
        let review_count: i32 = row.get::<Option<i32>, _>("review_count").unwrap_or(0);
        let is_verified: bool = row.get::<Option<bool>, _>("is_verified").unwrap_or(false);

        let rating = rating_avg.unwrap_or(0.0);
        let stars = render_stars(rating);
        let verified_badge = if is_verified {
            r#"<span class="inline-flex items-center px-2 py-0.5 bg-civiqo-eco-green/10 text-civiqo-eco-green text-xs rounded-full ml-2">
                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/></svg>
                Verificato
            </span>"#
        } else { "" };

        html.push_str(&format!(r#"
        <a href="/businesses/{}" class="bg-white rounded-xl shadow-sm overflow-hidden border border-civiqo-gray-200 hover:shadow-md transition-shadow group">
            <div class="h-40 bg-gradient-to-br from-civiqo-blue/10 to-civiqo-eco-green/10 flex items-center justify-center">
                <svg class="w-16 h-16 text-civiqo-gray-300 group-hover:text-civiqo-blue transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                </svg>
            </div>
            <div class="p-4">
                <div class="flex items-center">
                    <h3 class="text-lg font-brand font-semibold text-civiqo-gray-900 group-hover:text-civiqo-blue transition-colors">{}</h3>
                    {}
                </div>
                <p class="text-civiqo-gray-500 text-sm mt-1">{}</p>
                <p class="text-civiqo-gray-600 text-sm mt-2 line-clamp-2">{}</p>
                <div class="mt-3 flex items-center justify-between">
                    <div class="flex items-center text-sm">
                        {}
                        <span class="ml-1 text-civiqo-gray-500">({} recensioni)</span>
                    </div>
                    {}
                </div>
            </div>
        </a>
        "#, 
            id,
            name,
            verified_badge,
            category.unwrap_or_else(|| "Attività Locale".to_string()),
            description.unwrap_or_default(),
            stars,
            review_count,
            address.map(|a| format!(r#"<span class="text-civiqo-gray-500 text-xs truncate max-w-[120px]">{}</span>"#, a)).unwrap_or_default()
        ));
    }

    Html(html)
}

/// Helper function to render star rating
fn render_stars(rating: f64) -> String {
    let full_stars = rating.floor() as i32;
    let mut stars = String::new();
    for i in 0..5 {
        if i < full_stars {
            stars.push_str(r#"<svg class="w-4 h-4 text-civiqo-yellow" fill="currentColor" viewBox="0 0 20 20"><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/></svg>"#);
        } else {
            stars.push_str(r#"<svg class="w-4 h-4 text-civiqo-gray-300" fill="currentColor" viewBox="0 0 20 20"><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/></svg>"#);
        }
    }
    format!(r#"<div class="flex">{}</div>"#, stars)
}

/// Businesses search fragment
#[derive(Deserialize)]
pub struct BusinessSearchQuery {
    #[serde(default)]
    pub q: String,
    #[serde(default)]
    pub category: String,
}

pub async fn businesses_search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<BusinessSearchQuery>,
) -> Html<String> {
    let search_term = query.q.trim();
    let category = query.category.trim();
    
    let businesses = if search_term.is_empty() && category.is_empty() {
        sqlx::query(
            r#"SELECT b.id, b.name, b.description, b.category, b.address, 
                      b.rating_avg, b.review_count, b.cover_url, b.is_verified
               FROM businesses b
               WHERE b.is_active = true
               ORDER BY b.rating_avg DESC NULLS LAST
               LIMIT 12"#
        )
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
    } else {
        let mut conditions = vec!["b.is_active = true"];
        let mut query_str = String::from(
            r#"SELECT b.id, b.name, b.description, b.category, b.address, 
                      b.rating_avg, b.review_count, b.cover_url, b.is_verified
               FROM businesses b WHERE "#
        );
        
        if !search_term.is_empty() {
            conditions.push("(b.name ILIKE $1 OR b.description ILIKE $1)");
        }
        if !category.is_empty() {
            conditions.push("b.category = $2");
        }
        
        query_str.push_str(&conditions.join(" AND "));
        query_str.push_str(" ORDER BY b.rating_avg DESC NULLS LAST LIMIT 12");
        
        let search_pattern = format!("%{}%", search_term);
        sqlx::query(&query_str)
            .bind(&search_pattern)
            .bind(&category)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default()
    };

    if businesses.is_empty() {
        return Html(format!(r#"
        <div class="col-span-full text-center py-12">
            <svg class="w-16 h-16 mx-auto text-civiqo-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            <p class="text-civiqo-gray-600 font-medium">Nessun risultato per "{}"</p>
            <p class="text-civiqo-gray-500 text-sm mt-1">Prova con termini di ricerca diversi</p>
        </div>
        "#, search_term));
    }

    // Reuse the same rendering logic
    let mut html = String::new();
    for row in businesses {
        let id: uuid::Uuid = row.get("id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let category: Option<String> = row.get("category");
        let _address: Option<String> = row.get("address");
        let rating_avg: Option<f64> = row.get("rating_avg");
        let review_count: i32 = row.get::<Option<i32>, _>("review_count").unwrap_or(0);
        let is_verified: bool = row.get::<Option<bool>, _>("is_verified").unwrap_or(false);

        let rating = rating_avg.unwrap_or(0.0);
        let stars = render_stars(rating);
        let verified_badge = if is_verified {
            r#"<span class="inline-flex items-center px-2 py-0.5 bg-civiqo-eco-green/10 text-civiqo-eco-green text-xs rounded-full ml-2">✓</span>"#
        } else { "" };

        html.push_str(&format!(r#"
        <a href="/businesses/{}" class="bg-white rounded-xl shadow-sm overflow-hidden border border-civiqo-gray-200 hover:shadow-md transition-shadow group">
            <div class="h-40 bg-gradient-to-br from-civiqo-blue/10 to-civiqo-eco-green/10 flex items-center justify-center">
                <svg class="w-16 h-16 text-civiqo-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                </svg>
            </div>
            <div class="p-4">
                <div class="flex items-center">
                    <h3 class="text-lg font-brand font-semibold text-civiqo-gray-900">{}</h3>
                    {}
                </div>
                <p class="text-civiqo-gray-500 text-sm mt-1">{}</p>
                <p class="text-civiqo-gray-600 text-sm mt-2 line-clamp-2">{}</p>
                <div class="mt-3 flex items-center text-sm">
                    {}
                    <span class="ml-1 text-civiqo-gray-500">({} recensioni)</span>
                </div>
            </div>
        </a>
        "#, 
            id, name, verified_badge,
            category.unwrap_or_else(|| "Attività Locale".to_string()),
            description.unwrap_or_default(),
            stars, review_count
        ));
    }

    Html(html)
}

/// Business posts fragment
pub async fn business_posts(
    axum::extract::Path(_business_id): axum::extract::Path<String>,
) -> Html<String> {
    Html(r#"
    <div class="text-center py-8 text-civiqo-gray-500">
        <svg class="w-12 h-12 mx-auto text-civiqo-gray-300 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z"/>
        </svg>
        <p class="font-medium">Nessun aggiornamento</p>
        <p class="text-sm mt-1">Questa attività non ha ancora pubblicato aggiornamenti</p>
    </div>
    "#.to_string())
}

/// Business reviews fragment
pub async fn business_reviews(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(business_id): axum::extract::Path<String>,
) -> Html<String> {
    let business_uuid = match uuid::Uuid::parse_str(&business_id) {
        Ok(id) => id,
        Err(_) => return Html(r#"<div class="text-civiqo-coral">ID attività non valido</div>"#.to_string()),
    };

    let reviews = sqlx::query(
        r#"SELECT r.id, r.rating, r.title, r.content, r.created_at,
                  u.display_name, u.avatar_url
           FROM business_reviews r
           JOIN users u ON r.user_id = u.id
           WHERE r.business_id = $1
           ORDER BY r.created_at DESC
           LIMIT 10"#
    )
    .bind(business_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    if reviews.is_empty() {
        return Html(r#"
        <div class="text-center py-8 text-civiqo-gray-500">
            <svg class="w-12 h-12 mx-auto text-civiqo-gray-300 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"/>
            </svg>
            <p class="font-medium">Nessuna recensione</p>
            <p class="text-sm mt-1">Sii il primo a lasciare una recensione!</p>
        </div>
        "#.to_string());
    }

    let mut html = String::new();
    for row in reviews {
        let rating: i32 = row.get("rating");
        let title: Option<String> = row.get("title");
        let content: Option<String> = row.get("content");
        let display_name: String = row.get("display_name");
        let avatar_url: Option<String> = row.get("avatar_url");
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

        let stars = render_stars(rating as f64);
        let avatar = avatar_url.map(|url| format!(r#"<img src="{}" alt="{}" class="w-10 h-10 rounded-full object-cover"/>"#, url, display_name))
            .unwrap_or_else(|| format!(r#"<div class="w-10 h-10 rounded-full bg-civiqo-blue text-white flex items-center justify-center font-medium">{}</div>"#, 
                display_name.chars().next().unwrap_or('U').to_uppercase()));

        html.push_str(&format!(r#"
        <div class="border-b border-civiqo-gray-200 pb-4 mb-4 last:border-0 last:pb-0 last:mb-0">
            <div class="flex items-start gap-3">
                {}
                <div class="flex-1">
                    <div class="flex items-center justify-between">
                        <span class="font-medium text-civiqo-gray-900">{}</span>
                        <span class="text-xs text-civiqo-gray-500">{}</span>
                    </div>
                    <div class="mt-1">{}</div>
                    {}
                    {}
                </div>
            </div>
        </div>
        "#,
            avatar,
            display_name,
            created_at.format("%d/%m/%Y"),
            stars,
            title.map(|t| format!(r#"<h4 class="font-medium text-civiqo-gray-900 mt-2">{}</h4>"#, t)).unwrap_or_default(),
            content.map(|c| format!(r#"<p class="text-civiqo-gray-600 text-sm mt-1">{}</p>"#, c)).unwrap_or_default()
        ));
    }

    Html(html)
}

// =============================================================================
// GOVERNANCE FRAGMENTS
// =============================================================================

#[derive(Debug, serde::Deserialize)]
pub struct GovernanceProposalsQuery {
    pub status: Option<String>,
}

/// Governance proposals fragment with status filter
pub async fn governance_proposals(
    user: crate::auth::OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Query(params): Query<GovernanceProposalsQuery>,
) -> Html<String> {
    use sqlx::Row;
    
    let filter = params.status.unwrap_or_else(|| "active".to_string());
    
    // Build query based on filter
    let proposals = match filter.as_str() {
        "completed" => {
            sqlx::query(
                r#"SELECT p.id, p.title, p.description, p.status, p.voting_ends_at,
                          c.name as community_name,
                          (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
                   FROM proposals p
                   JOIN communities c ON p.community_id = c.id
                   WHERE p.status IN ('passed', 'rejected', 'closed')
                   ORDER BY p.created_at DESC
                   LIMIT 20"#
            )
            .fetch_all(&state.db.pool)
            .await
        }
        "mine" => {
            // Get user's proposals if logged in
            if let crate::auth::OptionalAuthUser(Some(ref u)) = user {
                if let Ok(user_uuid) = uuid::Uuid::parse_str(&u.user_id) {
                    sqlx::query(
                        r#"SELECT p.id, p.title, p.description, p.status, p.voting_ends_at,
                                  c.name as community_name,
                                  (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
                           FROM proposals p
                           JOIN communities c ON p.community_id = c.id
                           WHERE p.created_by = $1
                           ORDER BY p.created_at DESC
                           LIMIT 20"#
                    )
                    .bind(user_uuid)
                    .fetch_all(&state.db.pool)
                    .await
                } else {
                    Ok(vec![])
                }
            } else {
                Ok(vec![])
            }
        }
        _ => {
            // Default: active proposals
            sqlx::query(
                r#"SELECT p.id, p.title, p.description, p.status, p.voting_ends_at,
                          c.name as community_name,
                          (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
                   FROM proposals p
                   JOIN communities c ON p.community_id = c.id
                   WHERE p.status = 'active'
                   ORDER BY p.created_at DESC
                   LIMIT 20"#
            )
            .fetch_all(&state.db.pool)
            .await
        }
    };
    
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
                
                let (status_class, status_label) = match status.as_str() {
                    "active" => ("bg-civiqo-eco-green/10 text-civiqo-eco-green", "Attiva"),
                    "draft" => ("bg-civiqo-yellow/10 text-civiqo-yellow", "Bozza"),
                    "passed" => ("bg-civiqo-blue/10 text-civiqo-blue", "Approvata"),
                    "rejected" => ("bg-civiqo-coral/10 text-civiqo-coral", "Respinta"),
                    "closed" => ("bg-civiqo-gray-200 text-civiqo-gray-600", "Chiusa"),
                    _ => ("bg-civiqo-gray-200 text-civiqo-gray-600", "Sconosciuto"),
                };
                
                let ends_text = voting_ends
                    .map(|dt| {
                        let now = chrono::Utc::now();
                        let diff = dt.signed_duration_since(now);
                        if diff.num_seconds() < 0 {
                            "Votazione terminata".to_string()
                        } else if diff.num_days() > 0 {
                            format!("Termina tra {} giorni", diff.num_days())
                        } else if diff.num_hours() > 0 {
                            format!("Termina tra {} ore", diff.num_hours())
                        } else {
                            "In scadenza".to_string()
                        }
                    })
                    .unwrap_or_else(|| "Nessuna scadenza".to_string());
                
                // Build action buttons based on status
                let action_buttons = if status == "draft" {
                    format!(r#"
                        <div class="flex items-center space-x-2">
                            <button hx-post="/api/proposals/{}/activate"
                                    hx-target="closest div.bg-white"
                                    hx-swap="outerHTML"
                                    class="px-3 py-1 bg-civiqo-eco-green text-white text-sm rounded-lg hover:bg-civiqo-eco-green/90 transition">
                                Attiva Votazione
                            </button>
                        </div>
                    "#, id)
                } else if status == "active" {
                    format!(r#"
                        <div class="flex items-center space-x-2">
                            <a href="/governance/{}" 
                               class="px-3 py-1 bg-civiqo-blue text-white text-sm rounded-lg hover:bg-civiqo-blue-dark transition">
                                Vota
                            </a>
                        </div>
                    "#, id)
                } else {
                    format!(r#"
                        <a href="/governance/{}" 
                           class="text-civiqo-blue hover:underline font-medium">Dettagli →</a>
                    "#, id)
                };

                html.push_str(&format!(r#"
                <div class="bg-white rounded-xl shadow-sm p-6 hover:shadow-md transition border border-civiqo-gray-200">
                    <div class="flex items-start justify-between mb-4">
                        <div>
                            <h3 class="text-lg font-semibold text-civiqo-gray-900">{title}</h3>
                            <p class="text-civiqo-gray-600 text-sm mt-1">{community}</p>
                        </div>
                        <span class="px-3 py-1 {status_class} text-sm rounded-full font-medium">{status_label}</span>
                    </div>
                    <p class="text-civiqo-gray-700 mb-4 line-clamp-2">{description}</p>
                    <div class="flex items-center justify-between text-sm text-civiqo-gray-600">
                        <div class="flex items-center space-x-4">
                            <span>{ends}</span>
                            <span>•</span>
                            <span>{votes} voti</span>
                        </div>
                        {action_buttons}
                    </div>
                </div>
                "#,
                    title = title,
                    community = community_name,
                    status_class = status_class,
                    status_label = status_label,
                    description = description.unwrap_or_default(),
                    ends = ends_text,
                    votes = vote_count,
                    action_buttons = action_buttons
                ));
            }
            Html(html)
        }
        _ => {
            let (icon, title, message) = match filter.as_str() {
                "completed" => (
                    r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>"#,
                    "Nessuna proposta completata",
                    "Le proposte completate appariranno qui."
                ),
                "mine" => (
                    r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>"#,
                    "Nessuna proposta creata",
                    "Non hai ancora creato proposte. Inizia ora!"
                ),
                _ => (
                    r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>"#,
                    "Nessuna proposta attiva",
                    "Sii il primo a creare una proposta per la tua community!"
                ),
            };
            
            Html(format!(r#"
            <div class="bg-white rounded-xl shadow-sm p-8 text-center border border-civiqo-gray-200">
                <svg class="mx-auto h-12 w-12 text-civiqo-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    {icon}
                </svg>
                <h3 class="mt-4 text-lg font-medium text-civiqo-gray-900">{title}</h3>
                <p class="mt-2 text-civiqo-gray-600">{message}</p>
            </div>
            "#, icon = icon, title = title, message = message))
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
                <p class="mt-2 text-civiqo-gray-600">Iscriviti alla community per creare la prima proposta!</p>
            </div>
            "#.to_string())
    }
}

/// Create proposal via HTMX form
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
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
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;
    
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
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    
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
#[allow(dead_code)]
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

#[allow(dead_code)]
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
    
    // Build query based on filter
    let notifications = match filter.as_str() {
        "unread" => {
            sqlx::query(
                r#"SELECT n.id, n.type, n.message, n.is_read, n.created_at, n.target_type, n.target_id,
                          COALESCE(p.name, u.email) as actor_name, p.avatar_url as actor_avatar
                   FROM notifications n
                   LEFT JOIN users u ON n.actor_id = u.id
                   LEFT JOIN user_profiles p ON u.id = p.user_id
                   WHERE n.user_id = $1 AND n.is_read = false
                   ORDER BY n.created_at DESC
                   LIMIT $2 OFFSET $3"#
            )
            .bind(user_uuid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        "mentions" => {
            sqlx::query(
                r#"SELECT n.id, n.type, n.message, n.is_read, n.created_at, n.target_type, n.target_id,
                          COALESCE(p.name, u.email) as actor_name, p.avatar_url as actor_avatar
                   FROM notifications n
                   LEFT JOIN users u ON n.actor_id = u.id
                   LEFT JOIN user_profiles p ON u.id = p.user_id
                   WHERE n.user_id = $1 AND n.type = 'mention'
                   ORDER BY n.created_at DESC
                   LIMIT $2 OFFSET $3"#
            )
            .bind(user_uuid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        "votes" => {
            sqlx::query(
                r#"SELECT n.id, n.type, n.message, n.is_read, n.created_at, n.target_type, n.target_id,
                          COALESCE(p.name, u.email) as actor_name, p.avatar_url as actor_avatar
                   FROM notifications n
                   LEFT JOIN users u ON n.actor_id = u.id
                   LEFT JOIN user_profiles p ON u.id = p.user_id
                   WHERE n.user_id = $1 AND n.type IN ('vote', 'proposal')
                   ORDER BY n.created_at DESC
                   LIMIT $2 OFFSET $3"#
            )
            .bind(user_uuid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        _ => {
            sqlx::query(
                r#"SELECT n.id, n.type, n.message, n.is_read, n.created_at, n.target_type, n.target_id,
                          COALESCE(p.name, u.email) as actor_name, p.avatar_url as actor_avatar
                   FROM notifications n
                   LEFT JOIN users u ON n.actor_id = u.id
                   LEFT JOIN user_profiles p ON u.id = p.user_id
                   WHERE n.user_id = $1
                   ORDER BY n.created_at DESC
                   LIMIT $2 OFFSET $3"#
            )
            .bind(user_uuid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
    };
    
    match notifications {
        Ok(rows) if !rows.is_empty() => {
            Html(render_notifications_list(rows, page, &filter))
        }
        _ => Html(render_empty_notifications(&filter))
    }
}

#[allow(dead_code)]
fn render_notifications_list(notifications: Vec<sqlx::postgres::PgRow>, _page: i32, _filter: &str) -> String {
    let mut html = String::new();
    
    for notification in notifications {
        let id: uuid::Uuid = notification.get("id");
        let notif_type: String = notification.get("type");
        let message: Option<String> = notification.get("message");
        let is_read: bool = notification.get("is_read");
        let created_at: chrono::DateTime<chrono::Utc> = notification.get("created_at");
        let target_type: Option<String> = notification.get("target_type");
        let target_id: Option<String> = notification.get("target_id");
        let actor_name: Option<String> = notification.get("actor_name");
        let _actor_avatar: Option<String> = notification.get("actor_avatar");
        
        // Calculate time ago
        let now = chrono::Utc::now();
        let diff = now.signed_duration_since(created_at);
        let time_ago = if diff.num_days() > 0 {
            format!("{} giorni fa", diff.num_days())
        } else if diff.num_hours() > 0 {
            format!("{} ore fa", diff.num_hours())
        } else if diff.num_minutes() > 0 {
            format!("{} minuti fa", diff.num_minutes())
        } else {
            "Adesso".to_string()
        };
        
        // Build link based on target
        let link = match (target_type.as_deref(), target_id.as_ref()) {
            (Some("post"), Some(id)) => format!("/posts/{}", id),
            (Some("community"), Some(id)) => format!("/communities/{}", id),
            (Some("user"), Some(id)) => format!("/users/{}", id),
            (Some("proposal"), Some(id)) => format!("/governance?proposal={}", id),
            _ => "#".to_string(),
        };
        
        // Icon and color based on type
        let (icon_class, icon_svg) = match notif_type.as_str() {
            "follow" => ("bg-civiqo-eco-green/10 text-civiqo-eco-green", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>"#),
            "comment" => ("bg-civiqo-blue/10 text-civiqo-blue", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>"#),
            "mention" => ("bg-civiqo-lilac/10 text-civiqo-lilac", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/>"#),
            "vote" | "proposal" => ("bg-civiqo-coral/10 text-civiqo-coral", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"/>"#),
            "reaction" => ("bg-civiqo-yellow/10 text-civiqo-yellow", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>"#),
            _ => ("bg-civiqo-gray-200 text-civiqo-gray-600", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>"#),
        };
        
        let unread_class = if !is_read { "border-l-4 border-l-civiqo-blue" } else { "" };
        let font_class = if !is_read { "font-medium" } else { "" };
        
        let display_message = message.unwrap_or_else(|| {
            match notif_type.as_str() {
                "follow" => format!("{} ha iniziato a seguirti", actor_name.as_deref().unwrap_or("Qualcuno")),
                "comment" => format!("{} ha commentato il tuo post", actor_name.as_deref().unwrap_or("Qualcuno")),
                "mention" => format!("{} ti ha menzionato", actor_name.as_deref().unwrap_or("Qualcuno")),
                "reaction" => format!("{} ha reagito al tuo post", actor_name.as_deref().unwrap_or("Qualcuno")),
                "vote" => "Nuova votazione disponibile".to_string(),
                "proposal" => "Nuova proposta nella tua community".to_string(),
                _ => "Nuova notifica".to_string(),
            }
        });
        
        html.push_str(&format!(r#"
            <div class="bg-white rounded-lg p-4 border border-civiqo-gray-200 hover:border-civiqo-blue/30 transition-colors {unread_class}">
                <a href="{link}" 
                   hx-post="/htmx/notifications/{id}/read"
                   hx-swap="none"
                   class="flex gap-4 items-start">
                    <div class="flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center {icon_class}">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            {icon_svg}
                        </svg>
                    </div>
                    <div class="flex-1 min-w-0">
                        <p class="text-civiqo-gray-900 {font_class}">{display_message}</p>
                        <p class="text-sm text-civiqo-gray-600 mt-1">{time_ago}</p>
                    </div>
                    {unread_dot}
                </a>
            </div>
        "#,
            unread_class = unread_class,
            link = link,
            id = id,
            icon_class = icon_class,
            icon_svg = icon_svg,
            font_class = font_class,
            display_message = display_message,
            time_ago = time_ago,
            unread_dot = if !is_read { r#"<div class="flex-shrink-0"><div class="w-2 h-2 bg-civiqo-blue rounded-full"></div></div>"# } else { "" }
        ));
    }
    
    html
}

#[allow(dead_code)]
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
#[allow(dead_code)]
pub async fn mark_all_notifications_read(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return Html(String::new()),
    };
    
    let _ = sqlx::query(
        "UPDATE notifications SET is_read = true WHERE user_id = $1 AND is_read = false"
    )
    .bind(user_uuid)
    .execute(&state.db.pool)
    .await;
    
    Html(String::new())
}

/// Mark single notification as read
#[allow(dead_code)]
pub async fn mark_notification_read(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path(notification_id): axum::extract::Path<String>,
) -> Html<String> {
    let user_uuid = match uuid::Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return Html(String::new()),
    };
    
    let notification_uuid = match uuid::Uuid::parse_str(&notification_id) {
        Ok(id) => id,
        Err(_) => return Html(String::new()),
    };
    
    let _ = sqlx::query(
        "UPDATE notifications SET is_read = true WHERE id = $1 AND user_id = $2"
    )
    .bind(notification_uuid)
    .bind(user_uuid)
    .execute(&state.db.pool)
    .await;
    
    Html(String::new())
}

// ============================================================================
// COMMUNITY-SPECIFIC HTMX ENDPOINTS
// ============================================================================

/// Community businesses fragment - shows businesses belonging to a community
pub async fn community_businesses(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<String>,
) -> Html<String> {
    let community_uuid = match uuid::Uuid::parse_str(&community_id) {
        Ok(id) => id,
        Err(_) => return Html(render_empty_businesses()),
    };
    
    // Verify community exists
    let community_exists = sqlx::query(
        "SELECT id FROM communities WHERE id = $1"
    )
    .bind(community_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();
    
    if community_exists.is_none() {
        return Html(render_empty_businesses());
    }
    
    // Fetch businesses for this community
    let businesses = sqlx::query(
        r#"SELECT b.id, b.name, b.description, b.category, b.address, 
                  b.rating_avg, b.review_count, b.cover_url, b.is_verified
           FROM businesses b
           WHERE b.community_id = $1
           ORDER BY b.is_verified DESC, b.rating_avg DESC NULLS LAST, b.name ASC
           LIMIT 20"#
    )
    .bind(community_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if businesses.is_empty() {
        return Html(render_empty_businesses());
    }
    
    let mut html = String::from(r#"<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">"#);
    
    for row in businesses {
        let id: uuid::Uuid = row.get("id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let category: Option<String> = row.get("category");
        let address: Option<String> = row.get("address");
        let rating_avg: Option<f64> = row.get("rating_avg");
        let review_count: Option<i32> = row.get("review_count");
        let is_verified: bool = row.get::<Option<bool>, _>("is_verified").unwrap_or(false);
        
        let desc = description.unwrap_or_default();
        let cat = category.unwrap_or_else(|| "Attività".to_string());
        let addr = address.unwrap_or_default();
        let rating = rating_avg.unwrap_or(0.0);
        let reviews = review_count.unwrap_or(0);
        
        let verified_badge = if is_verified {
            r#"<span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-civiqo-eco-green/10 text-civiqo-eco-green">
                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                </svg>
                Verificato
            </span>"#
        } else {
            ""
        };
        
        html.push_str(&format!(r#"
        <a href="/businesses/{}" class="block bg-white rounded-xl shadow-sm border border-civiqo-gray-200 overflow-hidden hover:shadow-md hover:border-civiqo-blue/30 transition-all group">
            <div class="p-5">
                <div class="flex items-start justify-between mb-3">
                    <div>
                        <h3 class="font-semibold text-civiqo-gray-900 group-hover:text-civiqo-blue transition-colors">{}</h3>
                        <span class="text-sm text-civiqo-gray-500">{}</span>
                    </div>
                    {}
                </div>
                <p class="text-sm text-civiqo-gray-600 line-clamp-2 mb-3">{}</p>
                <div class="flex items-center justify-between text-sm">
                    <div class="flex items-center text-civiqo-gray-500">
                        <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                        </svg>
                        {}
                    </div>
                    <div class="flex items-center">
                        <svg class="w-4 h-4 text-civiqo-yellow mr-1" fill="currentColor" viewBox="0 0 20 20">
                            <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                        </svg>
                        <span class="text-civiqo-gray-900 font-medium">{:.1}</span>
                        <span class="text-civiqo-gray-400 ml-1">({})</span>
                    </div>
                </div>
            </div>
        </a>
        "#, id, name, cat, verified_badge, desc, addr, rating, reviews));
    }
    
    html.push_str("</div>");
    Html(html)
}

fn render_empty_businesses() -> String {
    r#"
    <div class="text-center py-12 bg-white rounded-xl border border-civiqo-gray-200">
        <svg class="w-16 h-16 mx-auto text-civiqo-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
        </svg>
        <h3 class="text-lg font-semibold text-civiqo-gray-900 mb-2">Nessuna attività locale</h3>
        <p class="text-civiqo-gray-600 mb-4">Questa community non ha ancora attività registrate.</p>
        <p class="text-sm text-civiqo-gray-500">Sei un'attività locale? Registrati per essere visibile ai membri della community!</p>
    </div>
    "#.to_string()
}

/// Community chat fragment - shows chat interface for a community
pub async fn community_chat(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<String>,
) -> Html<String> {
    let community_uuid = match uuid::Uuid::parse_str(&community_id) {
        Ok(id) => id,
        Err(_) => return Html(render_chat_error()),
    };
    
    // Verify community exists and get its name
    let community = sqlx::query(
        "SELECT id, name FROM communities WHERE id = $1"
    )
    .bind(community_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();
    
    let community_name = match community {
        Some(row) => row.get::<String, _>("name"),
        None => return Html(render_chat_error()),
    };
    
    // Return chat interface HTML
    Html(format!(r#"
    <div class="bg-white rounded-xl border border-civiqo-gray-200 overflow-hidden">
        <!-- Chat Header -->
        <div class="px-6 py-4 border-b border-civiqo-gray-200 bg-civiqo-gray-50">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                    <div class="w-10 h-10 rounded-full bg-civiqo-blue/10 flex items-center justify-center">
                        <svg class="w-5 h-5 text-civiqo-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                        </svg>
                    </div>
                    <div>
                        <h3 class="font-semibold text-civiqo-gray-900">Chat di {}</h3>
                        <p class="text-sm text-civiqo-gray-500">Messaggi della community</p>
                    </div>
                </div>
                <a href="/chat/{}" class="text-sm text-civiqo-blue hover:text-civiqo-blue-dark font-medium">
                    Apri chat completa →
                </a>
            </div>
        </div>
        
        <!-- Chat Messages Preview -->
        <div class="p-6">
            <div class="text-center py-8 text-civiqo-gray-500">
                <svg class="w-12 h-12 mx-auto text-civiqo-gray-300 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                </svg>
                <p class="mb-2">Unisciti alla conversazione!</p>
                <p class="text-sm">Accedi per partecipare alla chat della community.</p>
            </div>
        </div>
        
        <!-- Chat Input (disabled for non-members) -->
        <div class="px-6 py-4 border-t border-civiqo-gray-200 bg-civiqo-gray-50">
            <div class="flex items-center space-x-3">
                <input type="text" 
                       placeholder="Accedi per scrivere un messaggio..." 
                       disabled
                       class="flex-1 px-4 py-2 border border-civiqo-gray-200 rounded-lg bg-civiqo-gray-100 text-civiqo-gray-400 cursor-not-allowed">
                <button disabled class="px-4 py-2 bg-civiqo-gray-200 text-civiqo-gray-400 rounded-lg cursor-not-allowed">
                    Invia
                </button>
            </div>
        </div>
    </div>
    "#, community_name, community_uuid))
}

fn render_chat_error() -> String {
    r#"
    <div class="text-center py-12 bg-white rounded-xl border border-civiqo-gray-200">
        <svg class="w-16 h-16 mx-auto text-civiqo-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
        <h3 class="text-lg font-semibold text-civiqo-gray-900 mb-2">Chat non disponibile</h3>
        <p class="text-civiqo-gray-600">Impossibile caricare la chat per questa community.</p>
    </div>
    "#.to_string()
}

// =============================================================================
// MEMBERSHIP HTMX HANDLERS
// =============================================================================

/// Join community via HTMX - returns HTML fragment
pub async fn join_community_htmx(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Html<String>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Check community exists and is public
    let community: Option<(bool,)> = sqlx::query_as(
        "SELECT is_public FROM communities WHERE id = $1"
    )
    .bind(community_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    
    let (is_public,) = match community {
        Some(c) => c,
        None => {
            return Ok(Html(r#"
                <div class="text-civiqo-coral text-sm">Community non trovata</div>
            "#.to_string()));
        }
    };
    
    if !is_public {
        return Ok(Html(r#"
            <button disabled class="inline-flex items-center px-4 py-2 bg-civiqo-gray-200 text-civiqo-gray-500 rounded-lg font-medium cursor-not-allowed">
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                </svg>
                Community privata
            </button>
        "#.to_string()));
    }
    
    // Check if already member
    let existing: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2)"
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(false);
    
    if existing {
        return Ok(Html(r#"
            <span class="inline-flex items-center px-3 py-1.5 bg-civiqo-eco-green/20 text-white rounded-full text-sm font-medium">
                <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                </svg>
                Membro
            </span>
        "#.to_string()));
    }
    
    // Insert membership
    let result = sqlx::query(
        "INSERT INTO community_members (user_id, community_id, role, status, joined_at)
         VALUES ($1, $2, 'member', 'active', NOW())"
    )
    .bind(user_uuid)
    .bind(community_id)
    .execute(&state.db.pool)
    .await;
    
    match result {
        Ok(_) => {
            tracing::info!("User {} joined community {} via HTMX", user.user_id, community_id);
            Ok(Html(r#"
                <span class="inline-flex items-center px-3 py-1.5 bg-civiqo-eco-green/20 text-white rounded-full text-sm font-medium animate-pulse">
                    <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                    </svg>
                    Iscritto!
                </span>
            "#.to_string()))
        }
        Err(e) => {
            tracing::error!("Failed to join community: {}", e);
            Ok(Html(r#"
                <button disabled class="inline-flex items-center px-4 py-2 bg-civiqo-coral/20 text-civiqo-coral rounded-lg font-medium">
                    <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    Errore
                </button>
            "#.to_string()))
        }
    }
}

/// Request to join a private community via HTMX
pub async fn request_join_htmx(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Html<String>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Check community exists
    let community: Option<(bool, bool)> = sqlx::query_as(
        "SELECT is_public, requires_approval FROM communities WHERE id = $1"
    )
    .bind(community_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    
    let (is_public, requires_approval) = match community {
        Some(c) => c,
        None => {
            return Ok(Html(r#"<div class="text-civiqo-coral text-sm">Community non trovata</div>"#.to_string()));
        }
    };
    
    // If public, redirect to join
    if is_public && !requires_approval {
        return Ok(Html(format!(r#"
            <button hx-post="/htmx/communities/{}/join" hx-swap="outerHTML"
                    class="inline-flex items-center px-4 py-2 bg-white text-civiqo-blue rounded-lg font-medium hover:bg-civiqo-gray-50 transition-colors shadow-sm">
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                </svg>
                Iscriviti
            </button>
        "#, community_id)));
    }
    
    // Check if already member or pending
    let existing: Option<String> = sqlx::query_scalar(
        "SELECT status FROM community_members WHERE community_id = $1 AND user_id = $2"
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);
    
    if let Some(status) = existing {
        if status == "active" {
            return Ok(Html(r#"
                <span class="inline-flex items-center px-3 py-1.5 bg-civiqo-eco-green/20 text-white rounded-full text-sm font-medium">
                    <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                    </svg>
                    Membro
                </span>
            "#.to_string()));
        } else if status == "pending" {
            return Ok(Html(r#"
                <span class="inline-flex items-center px-3 py-1.5 bg-civiqo-amber/20 text-civiqo-amber rounded-full text-sm font-medium">
                    <svg class="w-4 h-4 mr-1.5 animate-pulse" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    Richiesta in attesa
                </span>
            "#.to_string()));
        }
    }
    
    // Insert pending membership request
    let result = sqlx::query(
        "INSERT INTO community_members (user_id, community_id, role, status, joined_at)
         VALUES ($1, $2, 'member', 'pending', NOW())"
    )
    .bind(user_uuid)
    .bind(community_id)
    .execute(&state.db.pool)
    .await;
    
    match result {
        Ok(_) => {
            tracing::info!("User {} requested to join community {} via HTMX", user.user_id, community_id);
            Ok(Html(r#"
                <span class="inline-flex items-center px-3 py-1.5 bg-civiqo-amber/20 text-civiqo-amber rounded-full text-sm font-medium">
                    <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                    </svg>
                    Richiesta inviata!
                </span>
            "#.to_string()))
        }
        Err(e) => {
            tracing::error!("Failed to request join: {}", e);
            Ok(Html(r#"
                <button disabled class="inline-flex items-center px-4 py-2 bg-civiqo-coral/20 text-civiqo-coral rounded-lg font-medium">
                    Errore nell'invio
                </button>
            "#.to_string()))
        }
    }
}

/// Get membership button based on current state
pub async fn membership_button_htmx(
    crate::auth::OptionalAuthUser(user): crate::auth::OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Html<String>, AppError> {
    // Check community type
    let community: Option<(bool, bool)> = sqlx::query_as(
        "SELECT is_public, requires_approval FROM communities WHERE id = $1"
    )
    .bind(community_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
    
    let (is_public, requires_approval) = match community {
        Some(c) => c,
        None => return Ok(Html(String::new())),
    };
    
    // Not logged in
    let Some(user) = user else {
        return Ok(Html(format!(r#"
            <a href="/auth/login" 
               class="inline-flex items-center px-4 py-2 bg-white text-civiqo-blue rounded-lg font-medium hover:bg-civiqo-gray-50 transition-colors shadow-sm">
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"/>
                </svg>
                Accedi per iscriverti
            </a>
        "#)));
    };
    
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Check membership status
    let membership: Option<(String, String)> = sqlx::query_as(
        "SELECT status, role::text FROM community_members WHERE community_id = $1 AND user_id = $2"
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);
    
    match membership {
        Some((status, role)) if status == "active" => {
            let role_badge = match role.as_str() {
                "owner" => "Proprietario",
                "admin" => "Admin",
                "moderator" => "Moderatore",
                _ => "Membro",
            };
            Ok(Html(format!(r#"
                <span class="inline-flex items-center px-3 py-1.5 bg-civiqo-eco-green/20 text-white rounded-full text-sm font-medium">
                    <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                    </svg>
                    {}
                </span>
            "#, role_badge)))
        }
        Some((status, _)) if status == "pending" => {
            Ok(Html(r#"
                <span class="inline-flex items-center px-3 py-1.5 bg-civiqo-amber/20 text-civiqo-amber rounded-full text-sm font-medium">
                    <svg class="w-4 h-4 mr-1.5 animate-pulse" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    Richiesta in attesa
                </span>
            "#.to_string()))
        }
        _ => {
            // Not a member - show appropriate button
            if is_public && !requires_approval {
                Ok(Html(format!(r#"
                    <button hx-post="/htmx/communities/{}/join" hx-swap="outerHTML"
                            class="inline-flex items-center px-4 py-2 bg-white text-civiqo-blue rounded-lg font-medium hover:bg-civiqo-gray-50 transition-colors shadow-sm">
                        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                        </svg>
                        Iscriviti
                    </button>
                "#, community_id)))
            } else {
                Ok(Html(format!(r#"
                    <button hx-post="/htmx/communities/{}/request" hx-swap="outerHTML"
                            class="inline-flex items-center px-4 py-2 bg-white/90 text-civiqo-gray-700 rounded-lg font-medium hover:bg-white transition-colors shadow-sm">
                        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                        </svg>
                        Richiedi accesso
                    </button>
                "#, community_id)))
            }
        }
    }
}

/// List pending membership requests (admin only)
pub async fn membership_requests_htmx(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path(community_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Html<String>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Check if user is admin/owner
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND role IN ('owner', 'admin') AND status = 'active')"
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(false);
    
    if !is_admin {
        return Ok(Html(r#"<div class="text-civiqo-coral">Accesso non autorizzato</div>"#.to_string()));
    }
    
    // Get pending requests
    let requests: Vec<(uuid::Uuid, String, Option<String>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT cm.user_id, u.email, u.display_name, cm.joined_at 
         FROM community_members cm
         JOIN users u ON cm.user_id = u.id
         WHERE cm.community_id = $1 AND cm.status = 'pending'
         ORDER BY cm.joined_at DESC"
    )
    .bind(community_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    if requests.is_empty() {
        return Ok(Html(r#"
            <div class="text-center py-8 text-civiqo-gray-500">
                <svg class="w-12 h-12 mx-auto mb-3 text-civiqo-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
                <p>Nessuna richiesta in attesa</p>
            </div>
        "#.to_string()));
    }
    
    let mut html = String::from(r#"<div class="space-y-3">"#);
    
    for (req_user_id, email, display_name, requested_at) in requests {
        let name = display_name.unwrap_or_else(|| email.clone());
        let time_ago = requested_at.format("%d/%m/%Y %H:%M").to_string();
        
        html.push_str(&format!(r#"
            <div class="flex items-center justify-between p-4 bg-white rounded-lg border border-civiqo-gray-200">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 rounded-full bg-civiqo-blue/10 flex items-center justify-center text-civiqo-blue font-medium">
                        {}
                    </div>
                    <div>
                        <p class="font-medium text-civiqo-gray-900">{}</p>
                        <p class="text-sm text-civiqo-gray-500">{}</p>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <button hx-post="/htmx/communities/{}/requests/{}/approve" hx-swap="outerHTML" hx-target="closest div.flex"
                            class="px-3 py-1.5 bg-civiqo-eco-green text-white rounded-lg text-sm font-medium hover:bg-civiqo-eco-green/90 transition">
                        Approva
                    </button>
                    <button hx-post="/htmx/communities/{}/requests/{}/reject" hx-swap="outerHTML" hx-target="closest div.flex"
                            class="px-3 py-1.5 bg-civiqo-coral text-white rounded-lg text-sm font-medium hover:bg-civiqo-coral/90 transition">
                        Rifiuta
                    </button>
                </div>
            </div>
        "#, 
            name.chars().next().unwrap_or('?').to_uppercase(),
            name,
            time_ago,
            community_id,
            req_user_id,
            community_id,
            req_user_id
        ));
    }
    
    html.push_str("</div>");
    Ok(Html(html))
}

/// Approve membership request (admin only) - HTMX
pub async fn approve_request_htmx(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path((community_id, request_user_id)): axum::extract::Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<Html<String>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Check if user is admin/owner
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND role IN ('owner', 'admin') AND status = 'active')"
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(false);
    
    if !is_admin {
        return Ok(Html(r#"<div class="text-civiqo-coral p-2">Non autorizzato</div>"#.to_string()));
    }
    
    // Update status to active
    let result = sqlx::query(
        "UPDATE community_members SET status = 'active', updated_at = NOW() WHERE community_id = $1 AND user_id = $2 AND status = 'pending'"
    )
    .bind(community_id)
    .bind(request_user_id)
    .execute(&state.db.pool)
    .await;
    
    match result {
        Ok(r) if r.rows_affected() > 0 => {
            tracing::info!("Admin {} approved membership for {} in community {}", user.user_id, request_user_id, community_id);
            Ok(Html(r#"
                <div class="flex items-center gap-2 p-4 bg-civiqo-eco-green/10 rounded-lg text-civiqo-eco-green">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                    </svg>
                    Richiesta approvata
                </div>
            "#.to_string()))
        }
        _ => {
            Ok(Html(r#"<div class="text-civiqo-coral p-2">Richiesta non trovata</div>"#.to_string()))
        }
    }
}

/// Reject membership request (admin only) - HTMX
pub async fn reject_request_htmx(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Path((community_id, request_user_id)): axum::extract::Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<Html<String>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Check if user is admin/owner
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM community_members WHERE community_id = $1 AND user_id = $2 AND role IN ('owner', 'admin') AND status = 'active')"
    )
    .bind(community_id)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(false);
    
    if !is_admin {
        return Ok(Html(r#"<div class="text-civiqo-coral p-2">Non autorizzato</div>"#.to_string()));
    }
    
    // Delete the pending request
    let result = sqlx::query(
        "DELETE FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'pending'"
    )
    .bind(community_id)
    .bind(request_user_id)
    .execute(&state.db.pool)
    .await;
    
    match result {
        Ok(r) if r.rows_affected() > 0 => {
            tracing::info!("Admin {} rejected membership for {} in community {}", user.user_id, request_user_id, community_id);
            Ok(Html(r#"
                <div class="flex items-center gap-2 p-4 bg-civiqo-gray-100 rounded-lg text-civiqo-gray-600">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                    </svg>
                    Richiesta rifiutata
                </div>
            "#.to_string()))
        }
        _ => {
            Ok(Html(r#"<div class="text-civiqo-coral p-2">Richiesta non trovata</div>"#.to_string()))
        }
    }
}
