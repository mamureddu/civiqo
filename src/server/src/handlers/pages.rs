use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use tera::{Context, Tera};
use std::sync::Arc;
use shared::database::Database;
use crate::auth::{AuthUser, OptionalAuthUser};

/// Application state for page handlers
pub struct AppState {
    pub tera: Tera,
    pub db: Database,
}

/// Home page
pub async fn index(
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    tracing::info!("Rendering index page");
    
    let mut ctx = Context::new();
    
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
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    
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

/// Chat room page
pub async fn chat_room(
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<String>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
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
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    tracing::info!("Rendering create community page for user: {}", user.user_id);
    
    let mut ctx = Context::new();
    
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
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    tracing::info!("Rendering dashboard page for user: {}", user.user_id);
    
    let mut ctx = Context::new();
    
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
    crate::auth::OptionalAuthUser(user): crate::auth::OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    
    // Add auth info to context
    if let Some(ref u) = user {
        ctx.insert("logged_in", &true);
        ctx.insert("username", &u.name.clone().unwrap_or(u.email.clone()));
        ctx.insert("picture", &u.picture);
        ctx.insert("user_id", &u.user_id);
    } else {
        ctx.insert("logged_in", &false);
    }
    
    // Parse UUID
    let uuid = uuid::Uuid::parse_str(&community_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid community ID")))?;
    
    // Fetch community details (join with user_profiles for creator name)
    let community = sqlx::query(
        "SELECT c.id, c.name, c.description, c.created_at, 
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
        ctx.insert("community_id", &row.get::<uuid::Uuid, _>("id").to_string());
        ctx.insert("community_name", &row.get::<String, _>("name"));
        ctx.insert("community_description", &row.get::<Option<String>, _>("description").unwrap_or_default());
        ctx.insert("creator_name", &row.get::<Option<String>, _>("creator_name").unwrap_or_else(|| "Unknown".to_string()));
        ctx.insert("created_at", &row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").format("%Y-%m-%d").to_string());
        
        // Fetch posts for this community (join with user_profiles for author name)
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
        // Return 404 Not Found for missing communities
        return Ok((
            StatusCode::NOT_FOUND,
            Html("<h1>Community Not Found</h1><p>The requested community does not exist.</p>"),
        ).into_response());
    }
    
    let html = state.tera.render("community_detail.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Businesses list page
pub async fn businesses(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    let html = state.tera.render("businesses.html", &Context::new())?;
    Ok(Html(html).into_response())
}

/// Business detail page
pub async fn business_detail(
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<String>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    ctx.insert("business_id", &business_id);
    ctx.insert("business_name", &format!("Business {}", &business_id[..8.min(business_id.len())]));
    ctx.insert("business_category", "Local Business");
    
    let html = state.tera.render("business_detail.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// Governance page
pub async fn governance(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    let html = state.tera.render("governance.html", &Context::new())?;
    Ok(Html(html).into_response())
}

/// Points of Interest / Map page
pub async fn poi(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    let html = state.tera.render("poi.html", &Context::new())?;
    Ok(Html(html).into_response())
}

/// Community posts list page
pub async fn community_posts(
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
    axum::extract::Query(params): axum::extract::Query<PostsQueryParams>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    
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
    OptionalAuthUser(user): OptionalAuthUser,
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    
    // Parse UUID
    let uuid = uuid::Uuid::parse_str(&post_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid post ID")))?;
    
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
    
    // Post data
    let post_data = serde_json::json!({
        "id": post.get::<uuid::Uuid, _>("id").to_string(),
        "title": post.get::<String, _>("title"),
        "content": post.get::<String, _>("content"),
        "content_type": post.get::<Option<String>, _>("content_type").unwrap_or_else(|| "text".to_string()),
        "media_url": post.get::<Option<String>, _>("media_url"),
        "is_pinned": post.get::<bool, _>("is_pinned"),
        "is_locked": post.get::<bool, _>("is_locked"),
        "view_count": post.get::<i64, _>("view_count"),
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
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    
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
pub async fn test_db(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    
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
