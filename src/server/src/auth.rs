use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::info;

/// Auth0 configuration
#[derive(Clone, Debug)]
pub struct Auth0Config {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub callback_url: String,
}

impl Auth0Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            domain: std::env::var("AUTH0_DOMAIN")
                .map_err(|_| "AUTH0_DOMAIN not set".to_string())?,
            client_id: std::env::var("AUTH0_CLIENT_ID")
                .map_err(|_| "AUTH0_CLIENT_ID not set".to_string())?,
            client_secret: std::env::var("AUTH0_CLIENT_SECRET")
                .map_err(|_| "AUTH0_CLIENT_SECRET not set".to_string())?,
            callback_url: std::env::var("AUTH0_CALLBACK_URL")
                .map_err(|_| "AUTH0_CALLBACK_URL not set".to_string())?,
        })
    }

    pub fn authorization_url(&self, state: &str) -> String {
        format!(
            "https://{}/authorize?client_id={}&response_type=code&redirect_uri={}&scope=openid%20profile%20email&state={}",
            self.domain, self.client_id, urlencoding::encode(&self.callback_url), state
        )
    }
}

/// User info from Auth0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,           // User ID
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

/// Session data stored in tower-sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

/// Login endpoint - redirect to Auth0
pub async fn login() -> impl IntoResponse {
    let auth0_config = match Auth0Config::from_env() {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Auth0 config error: {}", e);
            return Html("<h1>Auth0 not configured</h1>").into_response();
        }
    };

    let state = uuid::Uuid::new_v4().to_string();
    let auth_url = auth0_config.authorization_url(&state);

    info!("Redirecting to Auth0: {}", auth_url);
    // Use a Box to store the string for the redirect
    (StatusCode::TEMPORARY_REDIRECT, [("Location", auth_url.as_str())]).into_response()
}

/// Callback from Auth0
#[derive(Debug, Deserialize)]
pub struct AuthCallback {
    code: Option<String>,
    state: Option<String>,
}

pub async fn callback(
    session: Session,
    Query(_params): Query<AuthCallback>,
) -> impl IntoResponse {
    info!("Auth0 callback received");

    // TODO: Exchange code for token with Auth0
    // For now, create a test session
    let session_data = SessionData {
        user_id: "test-user-123".to_string(),
        email: "test@example.com".to_string(),
        name: Some("Test User".to_string()),
        picture: None,
    };

    if let Err(e) = session.insert("user", session_data).await {
        tracing::error!("Failed to insert session: {}", e);
        return Html("<h1>Session error</h1>").into_response();
    }

    info!("Session created for user");
    (StatusCode::TEMPORARY_REDIRECT, [("Location", "/")]).into_response()
}

/// Get current user from session
pub async fn get_current_user(session: Session) -> (StatusCode, Json<serde_json::Value>) {
    match session.get::<SessionData>("user").await {
        Ok(Some(user)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "user_id": user.user_id,
                "email": user.email,
                "name": user.name,
                "picture": user.picture,
            })),
        ),
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Not logged in" })),
        ),
        Err(e) => {
            tracing::error!("Session error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Session error" })),
            )
        }
    }
}

/// Logout endpoint
pub async fn logout(session: Session) -> impl IntoResponse {
    if let Err(e) = session.delete().await {
        tracing::error!("Failed to delete session: {}", e);
    }

    info!("User logged out");
    (StatusCode::TEMPORARY_REDIRECT, [("Location", "/")]).into_response()
}

/// Middleware to check if user is authenticated
pub async fn require_auth(session: Session) -> Result<SessionData, (StatusCode, &'static str)> {
    match session.get::<SessionData>("user").await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err((StatusCode::UNAUTHORIZED, "Not authenticated")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error")),
    }
}
