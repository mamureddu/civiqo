use axum::{
    routing::get,
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

use handlers::{pages, htmx, stubs::health_check};
use auth::{login, callback, logout, get_current_user}; // Auth handlers

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
        // .route("/auth/logout", get(logout))      // TODO: Session extractor issue
        // .route("/auth/me", get(get_current_user)) // TODO: Session extractor issue
        
        // HTMX Pages
        .route("/", get(pages::index))
        .route("/dashboard", get(pages::dashboard))
        .route("/communities", get(pages::communities))
        .route("/communities/:id", get(pages::community_detail))
        .route("/businesses", get(pages::businesses))
        .route("/businesses/:id", get(pages::business_detail))
        .route("/chat", get(pages::chat_room))
        .route("/chat/:room_id", get(pages::chat_room))
        .route("/governance", get(pages::governance))
        .route("/poi", get(pages::poi))
        
        // HTMX API Fragments
        .route("/api/nav", get(htmx::nav_fragment))
        .route("/api/communities/recent", get(htmx::recent_communities))
        .route("/api/communities/list", get(htmx::communities_list))
        .route("/api/communities/search", get(htmx::communities_list))
        .route("/api/chat/:room_id/header", get(htmx::chat_header))
        
        // Static files
        .nest_service("/static", ServeDir::new(static_path))
        
        .with_state(page_state.clone())
        .layer(session_layer)
        .layer(TraceLayer::new_for_http());

    Ok(app)
}

// Health check and root endpoints are now in handlers/stubs.rs