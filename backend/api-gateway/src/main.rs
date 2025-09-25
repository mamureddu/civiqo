use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::info;

mod config;
mod handlers;
mod middleware;
mod services;

use config::Config;
use handlers::*;
use shared::{database::Database, auth::Auth0Config};

pub type AppState = Arc<ApiState>;

pub struct ApiState {
    pub db: Database,
    pub config: Config,
    pub auth_config: Auth0Config,
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

    // Check if we're running in AWS Lambda environment
    if std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {
        #[cfg(feature = "lambda")]
        {
            info!("Running in AWS Lambda environment");
            let app = app.into_make_service();
            return Ok(lambda_http::run(app).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?);
        }
        #[cfg(not(feature = "lambda"))]
        {
            return Err("Lambda runtime detected but lambda feature not enabled".into());
        }
    } else {
        // Running locally
        info!("Running in local development mode");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:9001").await?;
        info!("API Gateway listening on http://0.0.0.0:9001");
        axum::serve(listener, app).await?;
    }

    Ok(())
}

async fn create_app() -> Result<Router, Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::from_env()?;

    // Initialize database
    let db = Database::connect(&config.database_url).await?;

    // Run migrations in development
    if config.development_mode {
        info!("Running database migrations...");
        db.migrate().await?;
    }

    // Initialize Auth0 config
    let auth_config = Auth0Config::from_env()?;

    // Create shared application state
    let state = Arc::new(ApiState {
        db,
        config: config.clone(),
        auth_config,
    });

    // Create CORS layer with security-conscious configuration
    let cors = CorsLayer::new()
        .allow_origin(config.cors_origins.parse::<axum::http::HeaderValue>()?)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600));

    // Build the router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        .route("/", get(root))

        // Authentication routes (temporarily disabled for initial testing)
        // .route("/auth/me", get(auth::get_current_user))
        // .route("/auth/sync", post(auth::sync_user_from_auth0)
        //     .layer(axum::middleware::from_fn_with_state(state.clone(), crate::middleware::rate_limit::rate_limit_middleware)))
        // .route("/auth/profile", put(auth::update_user_profile))

        // Community routes (temporarily disabled - need handler signature fixes)
        // .route("/communities", get(communities::list_communities))
        // .route("/communities", post(communities::create_community))
        // .route("/communities/:id", get(communities::get_community))
        // .route("/communities/:id", put(communities::update_community))
        // .route("/communities/:id/join", post(communities::join_community))
        // .route("/communities/:id/members", get(communities::list_members))
        // .route("/communities/:id/members/:user_id", put(communities::update_member_role))

        // Business routes (temporarily disabled - need handler signature fixes)
        // .route("/communities/:id/businesses", get(businesses::list_businesses))
        // .route("/communities/:id/businesses", post(businesses::create_business))
        // .route("/businesses/:id", get(businesses::get_business))
        // .route("/businesses/:id", put(businesses::update_business))
        // .route("/businesses/:id/products", get(businesses::list_products))
        // .route("/businesses/:id/products", post(businesses::create_product))

        // Governance routes (temporarily disabled - need handler signature fixes)
        // .route("/communities/:id/polls", get(governance::list_polls))
        // .route("/communities/:id/polls", post(governance::create_poll))
        // .route("/polls/:id", get(governance::get_poll))
        // .route("/polls/:id/vote", post(governance::cast_vote))
        // .route("/polls/:id/results", get(governance::get_poll_results))
        // .route("/communities/:id/decisions", get(governance::list_decisions))
        // .route("/communities/:id/decisions", post(governance::create_decision))

        // File upload routes (temporarily disabled - need handler signature fixes)
        // .route("/upload/presigned-url", post(uploads::get_presigned_url))

        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    Ok(app)
}

// Health check and root endpoints are now in handlers/stubs.rs