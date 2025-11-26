// lib.rs - Library interface for testing
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_sessions::{SessionManagerLayer, MemoryStore};
use tera::Tera;
use shared::{database::Database, auth::Auth0Config};

// Re-export modules
pub mod config;
pub mod handlers;
pub mod auth;
pub mod models;
// pub mod middleware; // Disabled - using tower-sessions instead

// Re-export main types from main.rs
pub use config::Config;
pub use handlers::pages::AppState as PageAppState;

pub type AppState = Arc<ApiState>;

#[derive(Clone)]
pub struct ApiState {
    pub db: Database,
    pub config: Config,
    pub auth_config: Auth0Config,
}

/// Create the full application router for testing
/// This mirrors the router in main.rs
pub async fn create_test_app() -> Result<Router, Box<dyn std::error::Error + Send + Sync>> {
    use handlers::{pages, htmx, api, posts, comments, reactions, stubs::health_check};
    use auth::{login, callback, logout};
    
    // Load environment
    dotenvy::dotenv().ok();
    
    // Connect to database
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set")?;
    
    let db = Database::connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    // Initialize Tera templates
    // Try multiple paths to find templates
    let current_dir = std::env::current_dir()?;
    let possible_paths = [
        "server/templates/**/*",
        "src/server/templates/**/*",
        "../server/templates/**/*",
    ];
    
    let mut tera = None;
    for path in &possible_paths {
        if let Ok(t) = Tera::new(path) {
            let count = t.get_template_names().count();
            if count > 0 {
                tracing::info!("Loaded {} templates from {}", count, path);
                tera = Some(t);
                break;
            }
        }
    }
    
    let mut tera = tera.ok_or_else(|| {
        tracing::error!("Failed to load templates. Current dir: {:?}", current_dir);
        format!("Failed to load templates from any path. Current dir: {:?}", current_dir)
    })?;
    tera.autoescape_on(vec![]);
    
    // Create page state
    let page_state = Arc::new(handlers::pages::AppState { 
        tera,
        db: db.clone(),
    });

    // Setup session store
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(tower_sessions::cookie::SameSite::Lax);

    // Build the router (same as main.rs)
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Auth routes
        .route("/auth/login", get(login))
        .route("/auth/callback", get(callback))
        .route("/auth/logout", post(logout))
        
        // HTMX Pages
        .route("/", get(pages::index))
        .route("/dashboard", get(pages::dashboard))
        .route("/communities", get(pages::communities))
        .route("/communities/create", get(pages::create_community))
        .route("/communities/:id", get(pages::community_detail))
        .route("/communities/:id/posts", get(pages::community_posts))
        .route("/communities/:id/posts/new", get(pages::create_post_page))
        .route("/posts/:id", get(pages::post_detail))
        .route("/businesses", get(pages::businesses))
        .route("/businesses/:id", get(pages::business_detail))
        .route("/chat", get(pages::chat_room))
        .route("/chat/:room_id", get(pages::chat_room))
        .route("/governance", get(pages::governance))
        .route("/poi", get(pages::poi))
        .route("/test-db", get(pages::test_db))
        
        // HTMX Fragments (return HTML fragments for dynamic updates)
        .route("/htmx/nav", get(htmx::nav_fragment))
        .route("/htmx/communities/recent", get(htmx::recent_communities))
        .route("/htmx/communities/list", get(htmx::communities_list))
        .route("/htmx/communities/search", get(htmx::communities_search))
        .route("/htmx/communities/:id/feed", get(htmx::community_feed))
        .route("/htmx/chat/:room_id/header", get(htmx::chat_header))
        .route("/htmx/user/communities", get(htmx::user_communities))
        .route("/htmx/user/activity", get(htmx::user_activity))
        // Business HTMX fragments
        .route("/htmx/businesses/list", get(htmx::businesses_list))
        .route("/htmx/businesses/search", get(htmx::businesses_search))
        .route("/htmx/businesses/:id/posts", get(htmx::business_posts))
        .route("/htmx/businesses/:id/reviews", get(htmx::business_reviews))
        // Governance HTMX fragments
        .route("/htmx/governance/proposals", get(htmx::governance_proposals))
        // POI HTMX fragments
        .route("/htmx/poi/nearby", get(htmx::poi_nearby))
        // Comment HTMX fragments
        .route("/htmx/comments/:id/reply-form", get(htmx::comment_reply_form))
        .route("/htmx/comments/:id/edit-form", get(htmx::comment_edit_form))
        .route("/htmx/empty", get(htmx::empty_fragment))
        
        // REST API Endpoints
        .route("/api/users", get(api::get_users))
        .route("/api/communities", post(api::create_community))
        .route("/api/communities", get(api::get_communities))
        .route("/api/communities/my", get(api::get_my_communities))
        .route("/api/communities/trending", get(api::get_trending_communities))
        .route("/api/communities/:id", get(api::get_community_detail))
        .route("/api/communities/:id", put(api::update_community))
        .route("/api/communities/:id", delete(api::delete_community))
        
        // Membership endpoints
        .route("/api/communities/:id/join", post(api::join_community))
        .route("/api/communities/:id/leave", post(api::leave_community))
        .route("/api/communities/:id/members", get(api::list_members))
        .route("/api/communities/:id/members/:user_id/role", put(api::update_member_role))
        .route("/api/communities/:id/members/:user_id", delete(api::remove_member))
        
        // Join request endpoints
        .route("/api/communities/:id/request-join", post(api::request_join_community))
        .route("/api/communities/:id/requests/:user_id/approve", post(api::approve_join_request))
        .route("/api/communities/:id/requests/:user_id/reject", post(api::reject_join_request))
        
        // Owner/Admin management endpoints
        .route("/api/communities/:id/transfer-ownership/:user_id", post(api::transfer_ownership))
        .route("/api/communities/:id/promote/:user_id", post(api::promote_to_admin))
        .route("/api/communities/:id/demote/:user_id", post(api::demote_to_member))
        
        // Posts endpoints
        .route("/api/communities/:id/posts", post(posts::create_post))
        .route("/api/communities/:id/posts", get(posts::list_posts))
        .route("/api/posts/:id", get(posts::get_post))
        .route("/api/posts/:id", put(posts::update_post))
        .route("/api/posts/:id", delete(posts::delete_post))
        
        // Comments endpoints
        .route("/api/posts/:id/comments", post(comments::create_comment))
        .route("/api/posts/:id/comments", get(comments::list_comments))
        .route("/api/comments/:id", put(comments::update_comment))
        .route("/api/comments/:id", delete(comments::delete_comment))
        
        // Reactions endpoints
        .route("/api/posts/:id/reactions", post(reactions::add_reaction))
        .route("/api/posts/:id/reactions", delete(reactions::remove_reaction))
        .route("/api/posts/:id/reactions", get(reactions::list_reactions))
        
        .with_state(page_state.clone())
        .layer(session_layer);

    Ok(app)
}

/// Create a simple application router for basic testing
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .with_state(state)
}