use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use tera::{Context, Tera};
use std::sync::Arc;
use shared::database::Database;
use crate::auth::{AuthUser, OptionalAuthUser};
use crate::i18n_tera::{LocaleExtractor, add_i18n_context};
use sqlx::Row;

/// Application state for page handlers
pub struct AppState {
    pub tera: Tera,
    pub db: Database,
}

/// Home page
pub async fn index(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    tracing::info!("Rendering index page");
    
    let mut ctx = Context::new();
    
    // Add i18n context
    add_i18n_context(&mut ctx, &locale);
    
    // Add auth info to context
    if let Some(user) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &user.name.unwrap_or(user.email.clone()));
        ctx.insert("picture", &user.picture);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    let html = state.tera.render("index.html", &ctx)?;
    tracing::info!("Index page rendered successfully");
    Ok(Html(html).into_response())
}

/// Communities list page
pub async fn communities(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Add auth info to context
    if let Some(user) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &user.name.unwrap_or(user.email.clone()));
        ctx.insert("picture", &user.picture);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    // Fetch all communities from database
    let communities = sqlx::query(
        "SELECT c.id, c.name, c.description, c.created_at, 
                COALESCE(p.name, u.email) as creator_name 
         FROM communities c 
         LEFT JOIN users u ON c.created_by = u.id 
         LEFT JOIN user_profiles p ON u.id = p.user_id
         ORDER BY c.created_at DESC"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let communities_data: Vec<serde_json::Value> = communities.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<uuid::Uuid, _>("id").to_string(),
            "name": row.get::<String, _>("name"),
            "description": row.get::<Option<String>, _>("description").unwrap_or_default(),
            "creator_name": row.get::<Option<String>, _>("creator_name").unwrap_or_else(|| "Unknown".to_string()),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d").to_string(),
        })
    }).collect();
    
    ctx.insert("communities", &communities_data);
    
    let html = state.tera.render("communities.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Chat rooms list page
pub async fn chat_list(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Add auth info to context
    if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    // Fetch chat rooms from communities (each community has a chat room)
    let rooms = sqlx::query(
        r#"SELECT c.id, c.name, c.description, COUNT(DISTINCT cm.user_id) as member_count
           FROM communities c
           LEFT JOIN community_members cm ON c.id = cm.community_id AND cm.status = 'active'
           WHERE c.is_public = true
           GROUP BY c.id, c.name, c.description
           ORDER BY c.created_at DESC
           LIMIT 20"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let rooms_data: Vec<serde_json::Value> = rooms.iter().map(|r| {
        serde_json::json!({
            "id": r.get::<uuid::Uuid, _>("id").to_string(),
            "name": r.get::<String, _>("name"),
            "description": r.get::<Option<String>, _>("description"),
            "member_count": r.get::<i64, _>("member_count"),
        })
    }).collect();
    
    ctx.insert("rooms", &rooms_data);
    
    let html = state.tera.render("chat_list.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Chat room page
pub async fn chat_room(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<String>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    ctx.insert("room_id", &room_id);
    ctx.insert("room_name", &format!("Room {}", &room_id[..8])); // Placeholder
    
    // Add auth info to context
    if let Some(user) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &user.name.unwrap_or(user.email.clone()));
        ctx.insert("picture", &user.picture);
        ctx.insert("user_id", &user.user_id);
    } else {
        ctx.insert("logged_in", &false);
        ctx.insert("user_id", "guest");
    }
    
    let html = state.tera.render("chat.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Create community page (PROTECTED - requires authentication)
pub async fn create_community(
    LocaleExtractor(locale): LocaleExtractor,
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    tracing::info!("Rendering create community page for user: {}", user.user_id);
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Auth info (always logged in for create community)
    ctx.insert("logged_in", &true);
    ctx.insert("user_id", &user.user_id);
    ctx.insert("email", &user.email);
    ctx.insert("username", &user.name.clone().unwrap_or_else(|| "User".to_string()));
    ctx.insert("picture", &user.picture);
    
    let html = state.tera.render("create_community.html", &ctx)?;
    tracing::info!("Create community page rendered successfully");
    Ok(Html(html).into_response())
}

/// User dashboard page (PROTECTED - requires authentication)
pub async fn dashboard(
    LocaleExtractor(locale): LocaleExtractor,
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    tracing::info!("Rendering dashboard page for user: {}", user.user_id);
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Auth info (always logged in for dashboard)
    ctx.insert("logged_in", &true);
    ctx.insert("user_id", &user.user_id);
    ctx.insert("email", &user.email);
    ctx.insert("username", &user.name.clone().unwrap_or_else(|| "User".to_string()));
    ctx.insert("picture", &user.picture);
    
    // Parse user_id as UUID
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|e| AppError(anyhow::anyhow!("Invalid user ID: {}", e)))?;
    
    // Fetch user's communities from database
    let communities = sqlx::query(
        "SELECT c.id, c.name, c.description, c.created_at, COUNT(DISTINCT m.user_id) as member_count
         FROM communities c
         LEFT JOIN community_members m ON c.id = m.community_id
         WHERE c.created_by = $1
         GROUP BY c.id, c.name, c.description, c.created_at
         ORDER BY c.created_at DESC
         LIMIT 10"
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let communities_data: Vec<serde_json::Value> = communities.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<uuid::Uuid, _>("id").to_string(),
            "name": row.get::<String, _>("name"),
            "description": row.get::<Option<String>, _>("description").unwrap_or_default(),
            "member_count": row.get::<i64, _>("member_count"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d").to_string(),
        })
    }).collect();
    
    ctx.insert("communities", &communities_data);
    ctx.insert("communities_count", &communities_data.len());
    
    let html = state.tera.render("dashboard.html", &ctx)?;
    tracing::info!("Dashboard page rendered successfully");
    Ok(Html(html).into_response())
}

/// Community detail page
pub async fn community_detail(
    LocaleExtractor(locale): LocaleExtractor,
    crate::auth::OptionalAuthUser(user): crate::auth::OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Parse user UUID if logged in
    let user_uuid = if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
        uuid::Uuid::parse_str(&u.user_id).ok()
    } else {
        ctx.insert("logged_in", &false);
        None
    };
    
    // Parse community UUID
    let uuid = uuid::Uuid::parse_str(&community_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid community ID")))?;
    
    // Fetch community details with all needed fields
    let community = sqlx::query(
        "SELECT c.id, c.name, c.slug, c.description, c.is_public, c.requires_approval,
                c.created_by, c.created_at, 
                COALESCE(p.name, u.email) as creator_name 
         FROM communities c 
         LEFT JOIN users u ON c.created_by = u.id 
         LEFT JOIN user_profiles p ON u.id = p.user_id
         WHERE c.id = $1"
    )
    .bind(uuid)
    .fetch_optional(&state.db.pool)
    .await?;
    
    if let Some(row) = community {
        let community_uuid = row.get::<uuid::Uuid, _>("id");
        let created_by = row.get::<uuid::Uuid, _>("created_by");
        let is_public = row.get::<Option<bool>, _>("is_public").unwrap_or(true);
        let requires_approval = row.get::<Option<bool>, _>("requires_approval").unwrap_or(false);
        
        ctx.insert("community_id", &community_uuid.to_string());
        ctx.insert("community_name", &row.get::<String, _>("name"));
        ctx.insert("community_slug", &row.get::<String, _>("slug"));
        ctx.insert("community_description", &row.get::<Option<String>, _>("description").unwrap_or_default());
        ctx.insert("creator_name", &row.get::<Option<String>, _>("creator_name").unwrap_or_else(|| "Unknown".to_string()));
        ctx.insert("created_at", &row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%d %B %Y").to_string());
        ctx.insert("is_public", &is_public);
        ctx.insert("requires_approval", &requires_approval);
        
        // Check membership status
        let (is_member, is_owner) = if let Some(uid) = user_uuid {
            let is_owner = uid == created_by;
            let membership = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active'"
            )
            .bind(community_uuid)
            .bind(uid)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or(0);
            (membership > 0 || is_owner, is_owner)
        } else {
            (false, false)
        };
        ctx.insert("is_member", &is_member);
        ctx.insert("is_owner", &is_owner);
        
        // Fetch community stats
        let member_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND status = 'active'"
        )
        .bind(uuid)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
        
        let post_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM posts WHERE community_id = $1"
        )
        .bind(uuid)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
        
        ctx.insert("member_count", &member_count);
        ctx.insert("post_count", &post_count);
        ctx.insert("event_count", &0i64);
        ctx.insert("active_today", &0i64);
        
        // Fetch posts for this community
        let posts = sqlx::query(
            "SELECT p.id, p.title, p.content, p.created_at, 
                    COALESCE(pr.name, u.email) as author_name 
             FROM posts p 
             LEFT JOIN users u ON p.author_id = u.id 
             LEFT JOIN user_profiles pr ON u.id = pr.user_id
             WHERE p.community_id = $1 
             ORDER BY p.created_at DESC 
             LIMIT 10"
        )
        .bind(uuid)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();
        
        let posts_data: Vec<serde_json::Value> = posts.iter().map(|row| {
            serde_json::json!({
                "id": row.get::<uuid::Uuid, _>("id").to_string(),
                "title": row.get::<String, _>("title"),
                "content": row.get::<String, _>("content"),
                "author_name": row.get::<Option<String>, _>("author_name").unwrap_or_else(|| "Anonymous".to_string()),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
            })
        }).collect();
        
        ctx.insert("posts", &posts_data);
    } else {
        return Ok((
            StatusCode::NOT_FOUND,
            Html("<h1>Community Not Found</h1><p>The requested community does not exist.</p>"),
        ).into_response());
    }
    
    let html = state.tera.render("community_detail.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Businesses list page
pub async fn businesses(
    LocaleExtractor(locale): LocaleExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    let html = state.tera.render("businesses.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Business detail page
pub async fn business_detail(
    LocaleExtractor(locale): LocaleExtractor,
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<String>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    ctx.insert("business_id", &business_id);
    ctx.insert("business_name", &format!("Business {}", &business_id[..8.min(business_id.len())]));
    ctx.insert("business_category", "Local Business");
    
    let html = state.tera.render("business_detail.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Governance page
pub async fn governance(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Add auth info to context
    if let Some(ref user) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &user.name.clone().unwrap_or(user.email.clone()));
        ctx.insert("picture", &user.picture);
        ctx.insert("user_id", &user.user_id);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    // Fetch governance stats from database
    let active_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM proposals WHERE status = 'active'"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    let passed_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM proposals WHERE status = 'passed'"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    let participants_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM votes"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    let ending_soon_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM proposals WHERE status = 'active' AND voting_ends_at <= NOW() + INTERVAL '24 hours'"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    ctx.insert("active_proposals", &active_count);
    ctx.insert("passed_proposals", &passed_count);
    ctx.insert("participants", &participants_count);
    ctx.insert("ending_soon", &ending_soon_count);
    
    let html = state.tera.render("governance.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Proposal detail page
pub async fn proposal_detail(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<uuid::Uuid>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Fetch proposal details
    let proposal = sqlx::query(
        r#"SELECT p.id, p.title, p.description, p.proposal_type, p.status,
                  p.voting_starts_at, p.voting_ends_at, p.created_at, p.quorum_required,
                  p.created_by, p.community_id,
                  c.name as community_name, c.slug as community_slug,
                  COALESCE(up.name, u.email) as author_name,
                  (SELECT COUNT(*) FROM votes v WHERE v.proposal_id = p.id) as vote_count
           FROM proposals p
           JOIN communities c ON p.community_id = c.id
           JOIN users u ON p.created_by = u.id
           LEFT JOIN user_profiles up ON u.id = up.user_id
           WHERE p.id = $1"#
    )
    .bind(proposal_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError(anyhow::anyhow!("Database error: {}", e)))?;
    
    let proposal = match proposal {
        Some(p) => p,
        None => return Ok(axum::response::Redirect::to("/governance").into_response()),
    };
    
    // Add auth info to context
    let mut is_author = false;
    let mut user_vote: Option<String> = None;
    let mut is_member = false;
    
    if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
        
        let created_by: uuid::Uuid = proposal.get("created_by");
        if let Ok(user_uuid) = uuid::Uuid::parse_str(&u.user_id) {
            is_author = created_by == user_uuid;
            
            // Check if user is community member
            let community_id: uuid::Uuid = proposal.get("community_id");
            is_member = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND user_id = $2 AND status = 'active'"
            )
            .bind(community_id)
            .bind(user_uuid)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or(0) > 0;
            
            // Check if user already voted
            let vote = sqlx::query_scalar::<_, String>(
                "SELECT vote_value FROM votes WHERE proposal_id = $1 AND user_id = $2"
            )
            .bind(proposal_id)
            .bind(user_uuid)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten();
            
            user_vote = vote;
        }
    } else {
        ctx.insert("logged_in", &false);
    }
    
    // Extract proposal data
    let id: uuid::Uuid = proposal.get("id");
    let title: String = proposal.get("title");
    let description: Option<String> = proposal.get("description");
    let proposal_type: String = proposal.get("proposal_type");
    let status: String = proposal.get("status");
    let community_name: String = proposal.get("community_name");
    let community_id: uuid::Uuid = proposal.get("community_id");
    let author_name: String = proposal.get("author_name");
    let vote_count: i64 = proposal.get("vote_count");
    let quorum_required: Option<i64> = proposal.get("quorum_required");
    let voting_ends: Option<chrono::DateTime<chrono::Utc>> = proposal.get("voting_ends_at");
    let created_at: chrono::DateTime<chrono::Utc> = proposal.get("created_at");
    
    // Get vote results
    let votes = sqlx::query(
        "SELECT vote_value, COUNT(*) as count FROM votes WHERE proposal_id = $1 GROUP BY vote_value"
    )
    .bind(proposal_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let mut yes_votes: i64 = 0;
    let mut no_votes: i64 = 0;
    let mut abstain_votes: i64 = 0;
    
    for vote in &votes {
        let value: String = vote.get("vote_value");
        let count: i64 = vote.get("count");
        match value.as_str() {
            "yes" => yes_votes = count,
            "no" => no_votes = count,
            "abstain" => abstain_votes = count,
            _ => {}
        }
    }
    
    let total_votes = yes_votes + no_votes + abstain_votes;
    let yes_percent = if total_votes > 0 { (yes_votes as f64 / total_votes as f64) * 100.0 } else { 0.0 };
    let no_percent = if total_votes > 0 { (no_votes as f64 / total_votes as f64) * 100.0 } else { 0.0 };
    let abstain_percent = if total_votes > 0 { (abstain_votes as f64 / total_votes as f64) * 100.0 } else { 0.0 };
    
    ctx.insert("proposal_id", &id.to_string());
    ctx.insert("title", &title);
    ctx.insert("description", &description.clone().unwrap_or_default());
    ctx.insert("proposal_type", &proposal_type);
    ctx.insert("status", &status);
    ctx.insert("community_name", &community_name);
    ctx.insert("community_id", &community_id.to_string());
    ctx.insert("author_name", &author_name);
    ctx.insert("vote_count", &vote_count);
    ctx.insert("quorum_required", &quorum_required.unwrap_or(0));
    ctx.insert("is_author", &is_author);
    ctx.insert("is_member", &is_member);
    ctx.insert("user_vote", &user_vote);
    ctx.insert("yes_votes", &yes_votes);
    ctx.insert("no_votes", &no_votes);
    ctx.insert("abstain_votes", &abstain_votes);
    ctx.insert("total_votes", &total_votes);
    ctx.insert("yes_percent", &yes_percent);
    ctx.insert("no_percent", &no_percent);
    ctx.insert("abstain_percent", &abstain_percent);
    ctx.insert("created_at", &created_at.format("%d/%m/%Y %H:%M").to_string());
    
    if let Some(ends) = voting_ends {
        ctx.insert("voting_ends_at", &ends.format("%d/%m/%Y %H:%M").to_string());
        let now = chrono::Utc::now();
        ctx.insert("voting_ended", &(ends < now));
    } else {
        ctx.insert("voting_ends_at", &"");
        ctx.insert("voting_ended", &false);
    }
    
    // Render inline template since we don't have a dedicated template file
    let html = format!(r#"
<!DOCTYPE html>
<html lang="it">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Governance - Civiqo</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <script src="https://unpkg.com/alpinejs@3.x.x/dist/cdn.min.js" defer></script>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        body {{ font-family: 'Inter', sans-serif; }}
    </style>
    <script>
        tailwind.config = {{
            theme: {{
                extend: {{
                    colors: {{
                        'civiqo-blue': '#4F46E5',
                        'civiqo-green': '#57C98A',
                        'civiqo-eco-green': '#10B981',
                        'civiqo-gray-50': '#F9FAFB',
                        'civiqo-gray-100': '#F3F4F6',
                        'civiqo-gray-200': '#E5E7EB',
                        'civiqo-gray-600': '#4B5563',
                        'civiqo-gray-700': '#374151',
                        'civiqo-gray-900': '#111827',
                    }}
                }}
            }}
        }}
    </script>
</head>
<body class="bg-civiqo-gray-50 min-h-screen">
    <!-- Header -->
    <header class="bg-white shadow-sm border-b border-civiqo-gray-200">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
            <div class="flex items-center justify-between">
                <a href="/" class="text-2xl font-bold text-civiqo-blue">Civiqo</a>
                <nav class="flex items-center space-x-6">
                    <a href="/dashboard" class="text-civiqo-gray-600 hover:text-civiqo-gray-900">Dashboard</a>
                    <a href="/communities" class="text-civiqo-gray-600 hover:text-civiqo-gray-900">Community</a>
                    <a href="/governance" class="text-civiqo-blue font-medium">Governance</a>
                </nav>
            </div>
        </div>
    </header>

    <main class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <!-- Breadcrumb -->
        <nav class="mb-6">
            <ol class="flex items-center space-x-2 text-sm text-civiqo-gray-600">
                <li><a href="/governance" class="hover:text-civiqo-blue">Governance</a></li>
                <li><span class="mx-2">/</span></li>
                <li class="text-civiqo-gray-900 font-medium">{title}</li>
            </ol>
        </nav>

        <!-- Proposal Card -->
        <div class="bg-white rounded-xl shadow-sm border border-civiqo-gray-200 overflow-hidden">
            <!-- Header -->
            <div class="p-6 border-b border-civiqo-gray-200">
                <div class="flex items-start justify-between">
                    <div>
                        <h1 class="text-2xl font-bold text-civiqo-gray-900">{title}</h1>
                        <p class="mt-1 text-civiqo-gray-600">
                            Proposta da <span class="font-medium">{author_name}</span> in 
                            <a href="/communities/{community_id}" class="text-civiqo-blue hover:underline">{community_name}</a>
                        </p>
                    </div>
                    <span class="px-3 py-1 {status_class} text-sm rounded-full font-medium">{status_label}</span>
                </div>
            </div>

            <!-- Description -->
            <div class="p-6 border-b border-civiqo-gray-200">
                <h2 class="text-lg font-semibold text-civiqo-gray-900 mb-3">Descrizione</h2>
                <p class="text-civiqo-gray-700 whitespace-pre-wrap">{description}</p>
            </div>

            <!-- Voting Section -->
            <div class="p-6" id="voting-section">
                <h2 class="text-lg font-semibold text-civiqo-gray-900 mb-4">Risultati Votazione</h2>
                
                <!-- Results -->
                <div class="space-y-4 mb-6">
                    <div>
                        <div class="flex items-center justify-between mb-1">
                            <span class="text-sm font-medium text-civiqo-gray-900">Sì</span>
                            <span class="text-sm text-civiqo-gray-600">{yes_percent:.1}% ({yes_votes})</span>
                        </div>
                        <div class="w-full bg-civiqo-gray-200 rounded-full h-3">
                            <div class="h-3 rounded-full bg-civiqo-eco-green transition-all" style="width: {yes_percent}%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex items-center justify-between mb-1">
                            <span class="text-sm font-medium text-civiqo-gray-900">No</span>
                            <span class="text-sm text-civiqo-gray-600">{no_percent:.1}% ({no_votes})</span>
                        </div>
                        <div class="w-full bg-civiqo-gray-200 rounded-full h-3">
                            <div class="h-3 rounded-full bg-red-500 transition-all" style="width: {no_percent}%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex items-center justify-between mb-1">
                            <span class="text-sm font-medium text-civiqo-gray-900">Astenuto</span>
                            <span class="text-sm text-civiqo-gray-600">{abstain_percent:.1}% ({abstain_votes})</span>
                        </div>
                        <div class="w-full bg-civiqo-gray-200 rounded-full h-3">
                            <div class="h-3 rounded-full bg-civiqo-gray-400 transition-all" style="width: {abstain_percent}%"></div>
                        </div>
                    </div>
                </div>

                <p class="text-sm text-civiqo-gray-600 mb-6">Totale voti: {total_votes}</p>

                {vote_buttons}
            </div>
        </div>

        <!-- Back link -->
        <div class="mt-6">
            <a href="/governance" class="text-civiqo-blue hover:underline">← Torna alla lista proposte</a>
        </div>
    </main>
</body>
</html>
"#,
        title = title,
        author_name = author_name,
        community_id = community_id,
        community_name = community_name,
        status_class = match status.as_str() {
            "active" => "bg-civiqo-eco-green/10 text-civiqo-eco-green",
            "draft" => "bg-yellow-100 text-yellow-800",
            "passed" => "bg-blue-100 text-blue-800",
            "rejected" => "bg-red-100 text-red-800",
            _ => "bg-gray-100 text-gray-800",
        },
        status_label = match status.as_str() {
            "active" => "Attiva",
            "draft" => "Bozza",
            "passed" => "Approvata",
            "rejected" => "Respinta",
            "closed" => "Chiusa",
            _ => "Sconosciuto",
        },
        description = description.unwrap_or_default(),
        yes_percent = yes_percent,
        yes_votes = yes_votes,
        no_percent = no_percent,
        no_votes = no_votes,
        abstain_percent = abstain_percent,
        abstain_votes = abstain_votes,
        total_votes = total_votes,
        vote_buttons = if status == "active" && is_member {
            let current_vote = user_vote.as_deref().unwrap_or("");
            let yes_class = if current_vote == "yes" { "bg-civiqo-eco-green text-white" } else { "bg-civiqo-gray-100 hover:bg-civiqo-eco-green/20 text-civiqo-gray-700" };
            let no_class = if current_vote == "no" { "bg-red-500 text-white" } else { "bg-civiqo-gray-100 hover:bg-red-100 text-civiqo-gray-700" };
            let abstain_class = if current_vote == "abstain" { "bg-civiqo-gray-500 text-white" } else { "bg-civiqo-gray-100 hover:bg-civiqo-gray-200 text-civiqo-gray-700" };
            format!(r##"
                <div class="flex items-center space-x-3">
                    <span class="text-sm text-civiqo-gray-600 mr-2">Il tuo voto:</span>
                    <button hx-post="/api/proposals/{id}/vote" 
                            hx-vals='{{"vote_value": "yes"}}'
                            hx-target="#voting-section"
                            hx-swap="innerHTML"
                            class="px-4 py-2 rounded-lg font-medium transition {yes_class}">
                        👍 Sì
                    </button>
                    <button hx-post="/api/proposals/{id}/vote" 
                            hx-vals='{{"vote_value": "no"}}'
                            hx-target="#voting-section"
                            hx-swap="innerHTML"
                            class="px-4 py-2 rounded-lg font-medium transition {no_class}">
                        👎 No
                    </button>
                    <button hx-post="/api/proposals/{id}/vote" 
                            hx-vals='{{"vote_value": "abstain"}}'
                            hx-target="#voting-section"
                            hx-swap="innerHTML"
                            class="px-4 py-2 rounded-lg font-medium transition {abstain_class}">
                        🤷 Astenuto
                    </button>
                </div>
            "##, 
                id = proposal_id,
                yes_class = yes_class,
                no_class = no_class,
                abstain_class = abstain_class
            )
        } else if status == "active" && !is_member {
            r#"<p class="text-sm text-civiqo-gray-600 italic">Devi essere membro della community per votare.</p>"#.to_string()
        } else if status == "draft" && is_author {
            format!(r##"
                <button hx-post="/api/proposals/{id}/activate" 
                        hx-target="#voting-section"
                        hx-swap="innerHTML"
                        class="px-4 py-2 bg-civiqo-eco-green text-white rounded-lg font-medium hover:bg-civiqo-eco-green/90 transition">
                    Attiva Votazione
                </button>
            "##, id = proposal_id)
        } else {
            String::new()
        }
    );
    
    Ok(Html(html).into_response())
}

/// Points of Interest / Map page
pub async fn poi(
    LocaleExtractor(locale): LocaleExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    let html = state.tera.render("poi.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Community posts list page
pub async fn community_posts(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
    axum::extract::Query(params): axum::extract::Query<PostsQueryParams>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Parse UUID
    let uuid = uuid::Uuid::parse_str(&community_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid community ID")))?;
    
    // Add auth info
    let user_uuid = if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
        uuid::Uuid::parse_str(&u.user_id).ok()
    } else {
        ctx.insert("logged_in", &false);
        None
    };
    
    // Fetch community
    let community = sqlx::query(
        "SELECT id, name, description, is_public FROM communities WHERE id = $1"
    )
    .bind(uuid)
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError(anyhow::anyhow!("Community not found")))?;
    
    let community_data = serde_json::json!({
        "id": community.get::<uuid::Uuid, _>("id").to_string(),
        "name": community.get::<String, _>("name"),
        "description": community.get::<Option<String>, _>("description").unwrap_or_default(),
    });
    ctx.insert("community", &community_data);
    
    // Check membership
    let is_member = if let Some(user_id) = user_uuid {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND user_id = $2"
        )
        .bind(uuid)
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0) > 0
    } else {
        false
    };
    ctx.insert("is_member", &is_member);
    
    // Pagination
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).min(50);
    let offset = (page - 1) * limit;
    let sort = params.sort.as_deref().unwrap_or("newest");
    
    ctx.insert("page", &page);
    ctx.insert("limit", &limit);
    ctx.insert("sort", &sort);
    
    // Order by clause
    let order_by = match sort {
        "popular" => "p.view_count DESC, p.created_at DESC",
        "discussed" => "comment_count DESC, p.created_at DESC",
        _ => "p.created_at DESC",
    };
    
    // Fetch posts with counts using JOINs (more efficient than subqueries)
    let query = format!(
        "SELECT p.id, p.title, p.content, p.media_url, p.is_pinned, p.view_count, p.created_at,
                COALESCE(pr.name, u.email) as author_name, u.email as author_email,
                COALESCE(cc.comment_count, 0) as comment_count,
                COALESCE(rc.reaction_count, 0) as reaction_count
         FROM posts p
         LEFT JOIN users u ON p.author_id = u.id
         LEFT JOIN user_profiles pr ON u.id = pr.user_id
         LEFT JOIN (
             SELECT post_id, COUNT(*) as comment_count 
             FROM comments 
             GROUP BY post_id
         ) cc ON cc.post_id = p.id
         LEFT JOIN (
             SELECT post_id, COUNT(*) as reaction_count 
             FROM reactions 
             GROUP BY post_id
         ) rc ON rc.post_id = p.id
         WHERE p.community_id = $1
         ORDER BY {}
         LIMIT $2 OFFSET $3",
        order_by
    );
    
    let posts = sqlx::query(&query)
        .bind(uuid)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();
    
    let posts_data: Vec<serde_json::Value> = posts.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<uuid::Uuid, _>("id").to_string(),
            "title": row.get::<String, _>("title"),
            "content": row.get::<String, _>("content"),
            "media_url": row.get::<Option<String>, _>("media_url"),
            "is_pinned": row.get::<bool, _>("is_pinned"),
            "view_count": row.get::<i64, _>("view_count"),
            "author_name": row.get::<Option<String>, _>("author_name"),
            "author_email": row.get::<Option<String>, _>("author_email").unwrap_or_default(),
            "comment_count": row.get::<i64, _>("comment_count"),
            "reaction_count": row.get::<i64, _>("reaction_count"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        })
    }).collect();
    
    ctx.insert("posts", &posts_data);
    
    // Total count
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE community_id = $1")
        .bind(uuid)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    ctx.insert("total", &total);
    
    let html = state.tera.render("community_posts.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Post detail page
pub async fn post_detail(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Parse UUID
    let uuid = uuid::Uuid::parse_str(&post_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid post ID")))?;
    
    // Add auth info - always insert user_id (even as null) for template
    let user_uuid = if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
        uuid::Uuid::parse_str(&u.user_id).ok()
    } else {
        ctx.insert("logged_in", &false);
        ctx.insert("user_id", &Option::<String>::None);
        None
    };
    
    // Fetch post with author
    let post = sqlx::query(
        "SELECT p.*, COALESCE(pr.name, u.email) as author_name, u.email as author_email
         FROM posts p
         LEFT JOIN users u ON p.author_id = u.id
         LEFT JOIN user_profiles pr ON u.id = pr.user_id
         WHERE p.id = $1"
    )
    .bind(uuid)
    .fetch_optional(&state.db.pool)
    .await?
    .ok_or_else(|| AppError(anyhow::anyhow!("Post not found")))?;
    
    let community_id = post.get::<uuid::Uuid, _>("community_id");
    let author_id = post.get::<uuid::Uuid, _>("author_id");
    
    // Fetch community
    let community = sqlx::query("SELECT id, name FROM communities WHERE id = $1")
        .bind(community_id)
        .fetch_one(&state.db.pool)
        .await?;
    
    let community_data = serde_json::json!({
        "id": community.get::<uuid::Uuid, _>("id").to_string(),
        "name": community.get::<String, _>("name"),
    });
    ctx.insert("community", &community_data);
    
    // Check permissions
    let is_author = user_uuid.map(|u| u == author_id).unwrap_or(false);
    ctx.insert("is_author", &is_author);
    
    // Check membership
    let is_member = if let Some(user_id) = user_uuid {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND user_id = $2"
        )
        .bind(community_id)
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0) > 0
    } else {
        false
    };
    ctx.insert("is_member", &is_member);
    
    // Check if admin
    let is_admin = if let Some(user_id) = user_uuid {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM community_members cm
             JOIN roles r ON cm.role_id = r.id
             WHERE cm.community_id = $1 AND cm.user_id = $2 AND r.name IN ('admin', 'owner')"
        )
        .bind(community_id)
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0) > 0
    } else {
        false
    };
    ctx.insert("is_admin", &is_admin);
    
    // Post data - handle nullable fields with defaults
    let post_data = serde_json::json!({
        "id": post.get::<uuid::Uuid, _>("id").to_string(),
        "title": post.get::<String, _>("title"),
        "content": post.get::<String, _>("content"),
        "content_type": post.get::<Option<String>, _>("content_type").unwrap_or_else(|| "text".to_string()),
        "media_url": post.get::<Option<String>, _>("media_url"),
        "is_pinned": post.get::<Option<bool>, _>("is_pinned").unwrap_or(false),
        "is_locked": post.get::<Option<bool>, _>("is_locked").unwrap_or(false),
        "view_count": post.get::<Option<i64>, _>("view_count").unwrap_or(0),
        "author_id": author_id.to_string(),
        "author_name": post.get::<Option<String>, _>("author_name"),
        "author_email": post.get::<Option<String>, _>("author_email").unwrap_or_default(),
        "created_at": post.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        "updated_at": post.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").format("%Y-%m-%d %H:%M").to_string(),
    });
    ctx.insert("post", &post_data);
    ctx.insert("post_id", &uuid.to_string());
    
    // Fetch reactions
    let reactions = sqlx::query(
        "SELECT reaction_type, COUNT(*) as count FROM reactions WHERE post_id = $1 GROUP BY reaction_type"
    )
    .bind(uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let mut reactions_map = serde_json::Map::new();
    for row in &reactions {
        let reaction_type = row.get::<String, _>("reaction_type");
        let count = row.get::<i64, _>("count");
        reactions_map.insert(reaction_type, serde_json::json!(count));
    }
    ctx.insert("reactions", &serde_json::Value::Object(reactions_map));
    
    // User's reaction
    let user_reaction = if let Some(user_id) = user_uuid {
        sqlx::query_scalar::<_, String>(
            "SELECT reaction_type FROM reactions WHERE post_id = $1 AND user_id = $2"
        )
        .bind(uuid)
        .bind(user_id)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten()
    } else {
        None
    };
    ctx.insert("user_reaction", &user_reaction);
    
    // Fetch comments (top-level only, replies loaded separately)
    let comments = sqlx::query(
        "SELECT c.*, COALESCE(pr.name, u.email) as author_name, u.email as author_email
         FROM comments c
         LEFT JOIN users u ON c.author_id = u.id
         LEFT JOIN user_profiles pr ON u.id = pr.user_id
         WHERE c.post_id = $1 AND c.parent_id IS NULL
         ORDER BY c.created_at ASC"
    )
    .bind(uuid)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();
    
    let comments_data: Vec<serde_json::Value> = comments.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<uuid::Uuid, _>("id").to_string(),
            "post_id": row.get::<uuid::Uuid, _>("post_id").to_string(),
            "content": row.get::<String, _>("content"),
            "author_id": row.get::<uuid::Uuid, _>("author_id").to_string(),
            "author_name": row.get::<Option<String>, _>("author_name"),
            "author_email": row.get::<Option<String>, _>("author_email").unwrap_or_default(),
            "is_edited": row.get::<bool, _>("is_edited"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        })
    }).collect();
    ctx.insert("comments", &comments_data);
    
    // Increment view count (fire and forget)
    let _ = sqlx::query("UPDATE posts SET view_count = view_count + 1 WHERE id = $1")
        .bind(uuid)
        .execute(&state.db.pool)
        .await;
    
    let html = state.tera.render("post_detail.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Create post page (PROTECTED)
pub async fn create_post_page(
    LocaleExtractor(locale): LocaleExtractor,
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Parse UUID
    let uuid = uuid::Uuid::parse_str(&community_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid community ID")))?;
    
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid user ID")))?;
    
    // Auth info
    ctx.insert("logged_in", &true);
    ctx.insert("username", &user.name.clone().unwrap_or(user.email.clone()));
    ctx.insert("picture", &user.picture);
    ctx.insert("user_id", &user.user_id);
    
    // Fetch community
    let community = sqlx::query("SELECT id, name FROM communities WHERE id = $1")
        .bind(uuid)
        .fetch_optional(&state.db.pool)
        .await?
        .ok_or_else(|| AppError(anyhow::anyhow!("Community not found")))?;
    
    // Check membership
    let is_member = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM community_members WHERE community_id = $1 AND user_id = $2"
    )
    .bind(uuid)
    .bind(user_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0) > 0;
    
    if !is_member {
        return Err(AppError(anyhow::anyhow!("You must be a member to create posts")));
    }
    
    let community_data = serde_json::json!({
        "id": community.get::<uuid::Uuid, _>("id").to_string(),
        "name": community.get::<String, _>("name"),
    });
    ctx.insert("community", &community_data);
    
    let html = state.tera.render("create_post.html", &ctx)?;
    Ok(Html(html).into_response())
}

#[derive(Debug, serde::Deserialize)]
pub struct PostsQueryParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
}

/// Database test page - shows real data from DB
pub async fn test_db(
    LocaleExtractor(locale): LocaleExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Get counts
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let community_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM communities")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    let post_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);
    
    ctx.insert("user_count", &user_count);
    ctx.insert("community_count", &community_count);
    ctx.insert("post_count", &post_count);
    
    // Get recent users (join with user_profiles for name)
    let users = sqlx::query(
        "SELECT u.id, u.email, u.created_at, p.name 
         FROM users u 
         LEFT JOIN user_profiles p ON u.id = p.user_id 
         ORDER BY u.created_at DESC LIMIT 5"
    )
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();
    
    let users_data: Vec<serde_json::Value> = users.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<uuid::Uuid, _>("id").to_string(),
            "name": row.get::<Option<String>, _>("name").unwrap_or_default(),
            "email": row.get::<String, _>("email"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        })
    }).collect();
    
    ctx.insert("users", &users_data);
    
    // Get recent communities
    let communities = sqlx::query("SELECT id, name, description, created_at FROM communities ORDER BY created_at DESC LIMIT 5")
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();
    
    let communities_data: Vec<serde_json::Value> = communities.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<uuid::Uuid, _>("id").to_string(),
            "name": row.get::<String, _>("name"),
            "description": row.get::<Option<String>, _>("description").unwrap_or_default(),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d %H:%M").to_string(),
        })
    }).collect();
    
    ctx.insert("communities", &communities_data);
    
    let html = state.tera.render("test_db.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Error type for page handlers
#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Page render error: {:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("<h1>Internal Server Error</h1>"),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// =============================================================================
// USER PROFILE PAGES
// =============================================================================

/// User profile page (PUBLIC)
pub async fn user_profile(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(current_user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    // Parse target user UUID
    let target_uuid = uuid::Uuid::parse_str(&user_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid user ID")))?;
    
    // Add auth info to context
    let is_own_profile = if let Some(ref u) = current_user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
        u.user_id == user_id
    } else {
        ctx.insert("logged_in", &false);
        false
    };
    ctx.insert("is_own_profile", &is_own_profile);
    
    // Fetch user profile
    let profile = sqlx::query(
        r#"SELECT u.id, u.email, u.created_at,
                  p.name, p.picture, p.bio, p.location, p.website, 
                  p.cover_image, p.avatar_url, p.is_public,
                  COALESCE(p.follower_count, 0) as follower_count,
                  COALESCE(p.following_count, 0) as following_count
           FROM users u
           LEFT JOIN user_profiles p ON u.id = p.user_id
           WHERE u.id = $1"#
    )
    .bind(target_uuid)
    .fetch_optional(&state.db.pool)
    .await?;
    
    let Some(row) = profile else {
        return Ok((
            StatusCode::NOT_FOUND,
            Html("<h1>Utente non trovato</h1>"),
        ).into_response());
    };
    
    // Check privacy
    let is_public: bool = row.get::<Option<bool>, _>("is_public").unwrap_or(true);
    if !is_public && !is_own_profile {
        return Ok((
            StatusCode::FORBIDDEN,
            Html("<h1>Profilo privato</h1><p>Questo profilo non è pubblico.</p>"),
        ).into_response());
    }
    
    let email: String = row.get("email");
    let profile_name = row.get::<Option<String>, _>("name").unwrap_or_else(|| email.clone());
    let avatar_url = row.get::<Option<String>, _>("avatar_url")
        .or_else(|| row.get::<Option<String>, _>("picture"));
    
    ctx.insert("profile_user_id", &user_id);
    ctx.insert("email", &email);
    ctx.insert("profile_name", &profile_name);
    ctx.insert("avatar_url", &avatar_url);
    ctx.insert("cover_image", &row.get::<Option<String>, _>("cover_image"));
    ctx.insert("bio", &row.get::<Option<String>, _>("bio"));
    ctx.insert("location", &row.get::<Option<String>, _>("location"));
    ctx.insert("website", &row.get::<Option<String>, _>("website"));
    ctx.insert("follower_count", &row.get::<i64, _>("follower_count"));
    ctx.insert("following_count", &row.get::<i64, _>("following_count"));
    
    let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
    ctx.insert("joined_at", &created_at.format("%B %Y").to_string());
    
    // Count communities and posts
    let community_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM community_members WHERE user_id = $1 AND status = 'active'"
    )
    .bind(target_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    let post_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM posts WHERE author_id = $1"
    )
    .bind(target_uuid)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);
    
    ctx.insert("community_count", &community_count);
    ctx.insert("post_count", &post_count);
    
    let html = state.tera.render("profile.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Edit profile page (PROTECTED)
pub async fn edit_profile_page(
    LocaleExtractor(locale): LocaleExtractor,
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    // Only allow editing own profile
    if user.user_id != user_id {
        return Ok((
            StatusCode::FORBIDDEN,
            Html("<h1>Accesso negato</h1><p>Puoi modificare solo il tuo profilo.</p>"),
        ).into_response());
    }
    
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    ctx.insert("logged_in", &true);
    ctx.insert("username", &user.name.clone().unwrap_or(user.email.clone()));
    ctx.insert("picture", &user.picture);
    ctx.insert("user_id", &user.user_id);
    
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid user ID")))?;
    
    // Fetch current profile
    let profile = sqlx::query(
        r#"SELECT p.name, p.picture, p.bio, p.location, p.website, 
                  p.cover_image, p.avatar_url, p.is_public
           FROM user_profiles p
           WHERE p.user_id = $1"#
    )
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await?;
    
    if let Some(row) = profile {
        ctx.insert("profile_name", &row.get::<Option<String>, _>("name").unwrap_or(user.email.clone()));
        ctx.insert("avatar_url", &row.get::<Option<String>, _>("avatar_url").or_else(|| row.get::<Option<String>, _>("picture")));
        ctx.insert("cover_image", &row.get::<Option<String>, _>("cover_image"));
        ctx.insert("bio", &row.get::<Option<String>, _>("bio"));
        ctx.insert("location", &row.get::<Option<String>, _>("location"));
        ctx.insert("website", &row.get::<Option<String>, _>("website"));
        ctx.insert("is_public", &row.get::<Option<bool>, _>("is_public").unwrap_or(true));
    } else {
        ctx.insert("profile_name", &user.email);
        ctx.insert("is_public", &true);
    }
    
    let html = state.tera.render("profile_edit.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// 404 Not Found page
#[allow(dead_code)]
pub async fn not_found(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Response {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    match state.tera.render("404.html", &ctx) {
        Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Html("<h1>404 - Pagina non trovata</h1>")).into_response(),
    }
}

/// 500 Internal Server Error page
#[allow(dead_code)]
pub async fn internal_error(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Response {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    // Generate error ID for tracking
    let error_id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    ctx.insert("error_id", &error_id);
    
    match state.tera.render("500.html", &ctx) {
        Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Html("<h1>500 - Errore interno</h1>")).into_response(),
    }
}

/// Notifications page
#[allow(dead_code)]
pub async fn notifications(
    LocaleExtractor(locale): LocaleExtractor,
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    ctx.insert("logged_in", &true);
    ctx.insert("username", &user.name.clone().unwrap_or(user.email.clone()));
    ctx.insert("picture", &user.picture);
    ctx.insert("user_id", &user.user_id);
    
    let html = state.tera.render("notifications.html", &ctx)?;
    Ok(Html(html).into_response())
}

#[derive(serde::Deserialize)]
pub struct SearchPageQuery {
    pub q: Option<String>,
    pub filter: Option<String>,
}

/// Search results page
#[allow(dead_code)]
pub async fn search_page(
    LocaleExtractor(locale): LocaleExtractor,
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    axum::extract::Query(params): axum::extract::Query<SearchPageQuery>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    add_i18n_context(&mut ctx, &locale);
    
    if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    let query = params.q.unwrap_or_default();
    let filter = params.filter.unwrap_or_else(|| "all".to_string());
    
    ctx.insert("query", &query);
    ctx.insert("filter", &filter);
    
    if query.len() >= 2 {
        let search_pattern = format!("%{}%", query.to_lowercase());
        
        // Search users (if filter allows)
        let users: Vec<serde_json::Value> = if filter == "all" || filter == "users" {
            sqlx::query(
                r#"SELECT u.id, u.email, p.name, p.avatar_url
                   FROM users u
                   LEFT JOIN user_profiles p ON u.id = p.user_id
                   WHERE LOWER(u.email) LIKE $1 OR LOWER(p.name) LIKE $1
                   LIMIT 20"#
            )
            .bind(&search_pattern)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default()
            .iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<uuid::Uuid, _>("id").to_string(),
                    "email": row.get::<String, _>("email"),
                    "name": row.get::<Option<String>, _>("name"),
                    "avatar_url": row.get::<Option<String>, _>("avatar_url"),
                })
            })
            .collect()
        } else {
            vec![]
        };
        
        // Search communities (if filter allows)
        let communities: Vec<serde_json::Value> = if filter == "all" || filter == "communities" {
            sqlx::query(
                r#"SELECT c.id, c.name, c.description, COUNT(cm.user_id) as member_count
                   FROM communities c
                   LEFT JOIN community_members cm ON c.id = cm.community_id AND cm.status = 'active'
                   WHERE LOWER(c.name) LIKE $1 OR LOWER(c.description) LIKE $1
                   GROUP BY c.id, c.name, c.description
                   LIMIT 20"#
            )
            .bind(&search_pattern)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default()
            .iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<uuid::Uuid, _>("id").to_string(),
                    "name": row.get::<String, _>("name"),
                    "description": row.get::<Option<String>, _>("description"),
                    "member_count": row.get::<i64, _>("member_count"),
                })
            })
            .collect()
        } else {
            vec![]
        };
        
        // Search posts (if filter allows)
        let posts: Vec<serde_json::Value> = if filter == "all" || filter == "posts" {
            sqlx::query(
                r#"SELECT p.id, p.title, c.name as community_name, p.created_at
                   FROM posts p
                   JOIN communities c ON p.community_id = c.id
                   WHERE LOWER(p.title) LIKE $1
                   ORDER BY p.created_at DESC
                   LIMIT 20"#
            )
            .bind(&search_pattern)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default()
            .iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<uuid::Uuid, _>("id").to_string(),
                    "title": row.get::<String, _>("title"),
                    "community_name": row.get::<String, _>("community_name"),
                    "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%d/%m/%Y").to_string(),
                })
            })
            .collect()
        } else {
            vec![]
        };
        
        let total_results = users.len() + communities.len() + posts.len();
        
        ctx.insert("users", &users);
        ctx.insert("communities", &communities);
        ctx.insert("posts", &posts);
        ctx.insert("total_results", &total_results);
    } else {
        ctx.insert("users", &Vec::<serde_json::Value>::new());
        ctx.insert("communities", &Vec::<serde_json::Value>::new());
        ctx.insert("posts", &Vec::<serde_json::Value>::new());
        ctx.insert("total_results", &0);
    }
    
    let html = state.tera.render("search.html", &ctx)?;
    Ok(Html(html).into_response())
}
