use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use tera::{Context, Tera};
use std::sync::Arc;
use shared::database::Database;

/// Application state for page handlers
pub struct AppState {
    pub tera: Tera,
    pub db: Database,
}

/// Home page
pub async fn index(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    tracing::info!("Rendering index page");
    let html = state.tera.render("index.html", &Context::new())?;
    tracing::info!("Index page rendered successfully");
    Ok(Html(html).into_response())
}

/// Communities list page
pub async fn communities(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    let html = state.tera.render("communities.html", &Context::new())?;
    Ok(Html(html).into_response())
}

/// Chat room page
pub async fn chat_room(
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<String>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    ctx.insert("room_id", &room_id);
    ctx.insert("room_name", &format!("Room {}", &room_id[..8])); // Placeholder
    ctx.insert("user_id", "user-123"); // TODO: Get from session
    
    let html = state.tera.render("chat.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// User dashboard page (after login)
pub async fn dashboard(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    tracing::info!("Rendering dashboard page");
    let html = state.tera.render("dashboard.html", &Context::new())?;
    tracing::info!("Dashboard page rendered successfully");
    Ok(Html(html).into_response())
}

/// Community detail page
pub async fn community_detail(
    State(state): State<Arc<AppState>>,
    Path(community_id): Path<String>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();
    ctx.insert("community_id", &community_id);
    ctx.insert("community_name", &format!("Community {}", &community_id[..8.min(community_id.len())]));
    ctx.insert("community_description", "A vibrant community space");
    
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
