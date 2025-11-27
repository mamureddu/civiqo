use axum::{
    routing::{get, put, delete, post},
    Router,
};
use std::sync::Arc;
use tower_http::{
    trace::TraceLayer,
    services::ServeDir,
};
use tower_sessions::{SessionManagerLayer, MemoryStore};
use tracing::info;
use tera::Tera;
use shared::database::Database;

mod handlers;
mod auth;

use handlers::{pages, htmx, api, posts, comments, reactions, proposals, stubs::health_check};
use auth::{login, callback, logout}; // Auth handlers

pub struct AppState {
    pub tera: Tera,
    pub db: Database,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .init();

    let app = create_app().await?;

    // For now, only local development mode
    // Lambda support will be added later with proper adapter
    info!("Running in local development mode");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9001").await?;
    info!("API Gateway listening on http://0.0.0.0:9001");
    info!("HTMX pages available at http://localhost:9001");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn create_app() -> Result<Router, Box<dyn std::error::Error>> {
    // Connect to database
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    info!("Connecting to database...");
    let db = Database::connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    info!("Running database migrations...");
    db.migrate()
        .await
        .map_err(|e| format!("Failed to run migrations: {}", e))?;
    
    info!("Database connected and migrations complete");

    // Initialize Tera templates
    // Get current directory and build paths
    let current_dir = std::env::current_dir()?;
    info!("Current directory: {:?}", current_dir);
    
    let (template_path, static_path) = if current_dir.ends_with("src") {
        ("server/templates/**/*", "server/static")
    } else {
        ("src/server/templates/**/*", "src/server/static")
    };
    
    info!("Loading templates from: {}", template_path);
    info!("Static files from: {}", static_path);
    
    let mut tera = Tera::new(template_path)
        .map_err(|e| {
            eprintln!("Template loading error: {}", e);
            format!("Failed to load templates from {}: {}", template_path, e)
        })?;
    
    // Disable auto-escape for now (can be enabled later)
    tera.autoescape_on(vec![]);
    info!("Templates loaded successfully: {:?}", tera.get_template_names().collect::<Vec<_>>());
    
    // Create page state
    let page_state = Arc::new(handlers::pages::AppState { 
        tera,
        db: db.clone(),
    });

    // Setup session store
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_same_site(tower_sessions::cookie::SameSite::Lax);

    // Build the router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Auth routes
        .route("/auth/login", get(login))
        .route("/auth/callback", get(callback))
        .route("/auth/logout", axum::routing::post(logout))
        
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
        .route("/chat", get(pages::chat_list))
        .route("/chat/:room_id", get(pages::chat_room))
        .route("/governance", get(pages::governance))
        .route("/poi", get(pages::poi))
        .route("/test-db", get(pages::test_db))
        // User profile pages
        .route("/users/:id", get(pages::user_profile))
        .route("/users/:id/edit", get(pages::edit_profile_page))
        
        // HTMX Fragments (return HTML fragments for dynamic updates)
        .route("/htmx/nav", get(htmx::nav_fragment))
        .route("/htmx/communities/recent", get(htmx::recent_communities))
        .route("/htmx/communities/list", get(htmx::communities_list))
        .route("/htmx/communities/search", get(htmx::communities_search))
        .route("/htmx/communities/:id/feed", get(htmx::community_feed))
        .route("/htmx/communities/:id/members", get(htmx::community_members))
        .route("/htmx/communities/:id/posts", post(posts::create_post_htmx))
        .route("/htmx/chat/:room_id/header", get(htmx::chat_header))
        .route("/htmx/user/communities", get(htmx::user_communities))
        .route("/htmx/user/activity", get(htmx::user_activity))
        .route("/htmx/dashboard/active-proposals", get(htmx::dashboard_active_proposals))
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
        // Search and user profile HTMX fragments
        .route("/htmx/search", get(htmx::search_results))
        .route("/htmx/users/:id/follow-button", get(htmx::follow_button))
        .route("/htmx/users/:id/posts", get(htmx::user_posts))
        .route("/htmx/users/:id/communities", get(htmx::user_profile_communities))
        .route("/htmx/users/:id/followers", get(htmx::user_followers))
        .route("/htmx/users/:id/following", get(htmx::user_following))
        .route("/htmx/notifications", get(htmx::notifications_dropdown))
        // Community proposals HTMX fragments
        .route("/htmx/communities/:id/proposals", get(htmx::community_proposals))
        .route("/htmx/communities/:id/proposals", post(htmx::create_proposal_htmx))
        .route("/htmx/communities/:id/proposals/count", get(htmx::community_proposals_count))
        
        // REST API Endpoints
        // NOTE: POST /api/users removed - users are created via Auth0 OAuth2 flow
        .route("/api/users", get(api::get_users))
        .route("/api/communities", axum::routing::post(api::create_community))
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
        
        // Join request endpoints (for private communities requiring approval)
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
        
        // User profile endpoints
        .route("/api/users/:id", put(api::update_profile))
        .route("/api/users/:id/follow", post(api::follow_user))
        .route("/api/users/:id/unfollow", post(api::unfollow_user))
        
        // Proposals/Governance endpoints
        .route("/api/proposals", get(proposals::list_proposals))
        .route("/api/proposals", post(proposals::create_proposal))
        .route("/api/proposals/:id", get(proposals::get_proposal))
        .route("/api/proposals/:id/vote", post(proposals::cast_vote))
        .route("/api/proposals/:id/results", get(proposals::get_results))
        .route("/api/proposals/:id/activate", post(proposals::activate_proposal))
        .route("/api/proposals/:id/close", post(proposals::close_proposal))
        
        // Static files
        .nest_service("/static", ServeDir::new(static_path))
        
        .with_state(page_state.clone())
        .layer(session_layer)
        .layer(TraceLayer::new_for_http());

    Ok(app)
}

// Health check and root endpoints are now in handlers/stubs.rs