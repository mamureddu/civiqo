use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::{
    trace::TraceLayer,
    services::ServeDir,
};
use tracing::info;
use tera::Tera;

mod handlers;

use handlers::{pages, htmx, stubs::health_check};

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
    // TODO: Database and Auth0 will be added later
    // For now, just serve HTMX pages

    // Initialize Tera templates
    // Get current directory and build absolute path
    let current_dir = std::env::current_dir()?;
    info!("Current directory: {:?}", current_dir);
    
    let template_path = if current_dir.ends_with("backend") {
        "api-gateway/templates/**/*"
    } else {
        "backend/api-gateway/templates/**/*"
    };
    
    info!("Loading templates from: {}", template_path);
    let mut tera = Tera::new(template_path)
        .map_err(|e| {
            eprintln!("Template loading error: {}", e);
            format!("Failed to load templates from {}: {}", template_path, e)
        })?;
    
    // Disable auto-escape for now (can be enabled later)
    tera.autoescape_on(vec![]);
    info!("Templates loaded successfully: {:?}", tera.get_template_names().collect::<Vec<_>>());
    
    // Create page state (separate from API state for now)
    let page_state = Arc::new(handlers::pages::AppState { tera });

    // Build the router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // HTMX Pages
        .route("/", get(pages::index))
        .route("/communities", get(pages::communities))
        .route("/chat/:room_id", get(pages::chat_room))
        
        // HTMX API Fragments
        .route("/api/nav", get(htmx::nav_fragment))
        .route("/api/communities/recent", get(htmx::recent_communities))
        .route("/api/communities/list", get(htmx::communities_list))
        .route("/api/communities/search", get(htmx::communities_list))
        .route("/api/chat/:room_id/header", get(htmx::chat_header))
        
        // Static files
        .nest_service("/static", ServeDir::new("backend/api-gateway/static"))
        
        .with_state(page_state.clone())

        // API Routes (will be added later with proper auth)
        // For now, HTMX pages work without backend API
        
        .layer(TraceLayer::new_for_http());

    Ok(app)
}

// Health check and root endpoints are now in handlers/stubs.rs