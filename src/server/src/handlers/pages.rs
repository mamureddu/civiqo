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
        "SELECT c.id, c.name, c.description, c.created_at, u.username as creator_name 
         FROM communities c 
         LEFT JOIN users u ON c.created_by = u.id 
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

/// User dashboard page (PROTECTED - requires authentication)
pub async fn dashboard(
    AuthUser(user): AuthUser, // Requires authentication
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    tracing::info!("Rendering dashboard page for user: {}", user.user_id);
    
    let mut ctx = Context::new();
    
    // Auth info (always logged in for dashboard)
    ctx.insert("logged_in", &true);
    ctx.insert("user_id", &user.user_id);
    ctx.insert("email", &user.email);
    ctx.insert("username", &user.name.clone().unwrap_or_else(|| "User".to_string()));
    ctx.insert("picture", &user.picture);
    
    let html = state.tera.render("dashboard.html", &ctx)?;
    tracing::info!("Dashboard page rendered successfully");
    Ok(Html(html).into_response())
}

/// Community detail page
pub async fn community_detail(
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
) -> Result<Response, AppError> {
    use sqlx::Row;
    
    let mut ctx = Context::new();
    
    // Parse UUID
    let uuid = uuid::Uuid::parse_str(&community_id)
        .map_err(|_| AppError(anyhow::anyhow!("Invalid community ID")))?;
    
    // Fetch community details
    let community = sqlx::query(
        "SELECT c.id, c.name, c.description, c.created_at, u.username as creator_name 
         FROM communities c 
         LEFT JOIN users u ON c.created_by = u.id 
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
        
        // Fetch posts for this community
        let posts = sqlx::query(
            "SELECT p.id, p.title, p.content, p.created_at, u.username as author_name 
             FROM posts p 
             LEFT JOIN users u ON p.author_id = u.id 
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
        return Err(AppError(anyhow::anyhow!("Community not found")));
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
    
    // Get recent users
    let users = sqlx::query("SELECT id, username, email, created_at FROM users ORDER BY created_at DESC LIMIT 5")
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();
    
    let users_data: Vec<serde_json::Value> = users.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<uuid::Uuid, _>("id").to_string(),
            "username": row.get::<String, _>("username"),
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
pub struct AppError(anyhow::Error);

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
