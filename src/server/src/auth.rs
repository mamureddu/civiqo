use axum::{
    async_trait,
    extract::{Query, Request, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Redirect, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::info;

/// Request to sync user from Auth0
#[derive(Debug, Deserialize, Serialize)]
pub struct SyncUserRequest {
    pub auth0_id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

/// Helper to extract session from request
async fn get_session_from_request(req: &mut Request) -> Option<Session> {
    req.extensions().get::<Session>().cloned()
}

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
            return Redirect::to("/").into_response();
        }
    };

    let state = uuid::Uuid::new_v4().to_string();
    let auth_url = auth0_config.authorization_url(&state);

    info!("Redirecting to Auth0: {}", auth_url);
    
    // Redirect direttamente a Auth0
    Redirect::temporary(&auth_url).into_response()
}

/// Callback from Auth0
/// Note: Query + Session doesn't work in Axum 0.7 with tower-sessions
/// So we parse query params manually from the request
pub async fn callback(mut req: Request) -> impl IntoResponse {
    // Parse query params manually
    let query = req.uri().query().unwrap_or("");
    let code = query.split('&')
        .find(|p| p.starts_with("code="))
        .and_then(|p| p.strip_prefix("code="));
    
    info!("Auth0 callback received with code: {:?}", code);

    // Get session from request extensions
    let session = match req.extensions().get::<Session>() {
        Some(s) => s.clone(),
        None => {
            tracing::error!("No session found in request");
            return Redirect::to("/?error=no_session").into_response();
        }
    };

    // TODO: Exchange code for token with Auth0
    // For now, create a test session with mock data
    let session_data = SessionData {
        user_id: "auth0|test-user-123".to_string(),
        email: "user@example.com".to_string(),
        name: Some("Test User".to_string()),
        picture: None,
    };

    if let Err(e) = session.insert("user", session_data).await {
        tracing::error!("Failed to create session: {}", e);
        return Redirect::to("/?error=session_failed").into_response();
    }

    info!("Session created successfully for user");
    // Redirect to dashboard after successful login
    Redirect::to("/dashboard").into_response()
}

/// Get current user from session
pub async fn get_current_user(session: Session) -> Json<serde_json::Value> {
    match session.get::<SessionData>("user").await {
        Ok(Some(user)) => Json(serde_json::json!({
            "authenticated": true,
            "user_id": user.user_id,
            "email": user.email,
            "name": user.name,
            "picture": user.picture,
        })),
        Ok(None) => Json(serde_json::json!({
            "authenticated": false,
            "error": "Not logged in"
        })),
        Err(e) => {
            tracing::error!("Session error: {}", e);
            Json(serde_json::json!({
                "authenticated": false,
                "error": "Session error"
            }))
        }
    }
}

/// Logout endpoint
pub async fn logout(session: Session) -> Json<serde_json::Value> {
    if let Err(e) = session.delete().await {
        tracing::error!("Failed to delete session: {}", e);
    }

    info!("User logged out");
    Json(serde_json::json!({
        "success": true,
        "message": "Logged out",
        "redirect_url": "/"
    }))
}

/// Extractor for authenticated user - use this in route handlers
/// Example: `async fn protected_route(AuthUser(user): AuthUser) -> impl IntoResponse`
pub struct AuthUser(pub SessionData);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract session from request extensions
        let session = parts
            .extensions
            .get::<Session>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Session not found"))?;

        // Get user from session
        match session.get::<SessionData>("user").await {
            Ok(Some(user)) => Ok(AuthUser(user)),
            Ok(None) => Err((StatusCode::UNAUTHORIZED, "Not authenticated")),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error")),
        }
    }
}

/// Optional auth - returns None if not authenticated instead of error
pub struct OptionalAuthUser(pub Option<SessionData>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session = parts
            .extensions
            .get::<Session>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Session not found"))?;

        match session.get::<SessionData>("user").await {
            Ok(user) => Ok(OptionalAuthUser(user)),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error")),
        }
    }
}
