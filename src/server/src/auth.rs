use axum::{
    extract::{Request, FromRequestParts, State},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Redirect},
    Json,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::info;
use std::sync::Arc;
use uuid::Uuid;

/// Request to sync user from Auth0
#[derive(Debug, Deserialize, Serialize)]
pub struct SyncUserRequest {
    pub auth0_id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
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

/// User info from Auth0 token response
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    // ==========================================================
    // COMMENTED FIELDS - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // pub id_token: Option<String>,      // May be needed for JWT validation or user info extraction
    // pub token_type: String,            // Typically "Bearer" - may be needed for token validation
    // pub expires_in: Option<u64>,       // Token expiration in seconds - may be needed for refresh logic
}

/// Auth0 user info response
#[derive(Debug, Deserialize)]
pub struct Auth0UserInfo {
    pub sub: String,           // User ID
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    // ==========================================================
    // COMMENTED FIELDS - KEPT FOR FUTURE REFERENCE
    // ==========================================================
    // pub email_verified: Option<bool>,  // Critical for security - may prevent unverified email access
    // pub updated_at: Option<String>,    // Last profile update - may be needed for sync logic
    // pub created_at: Option<String>,    // Account creation date - may be needed for user analytics
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

/// Callback from Auth0 - Complete OAuth2 flow with code exchange and user sync
pub async fn callback(
    State(state): State<Arc<crate::handlers::pages::AppState>>,
    req: Request,
) -> impl IntoResponse {
    // Parse query params manually
    let query = req.uri().query().unwrap_or("");
    let code = match query.split('&')
        .find(|p| p.starts_with("code="))
        .and_then(|p| p.strip_prefix("code="))
    {
        Some(c) => c,
        None => {
            tracing::error!("No code in callback");
            return Redirect::to("/?error=no_code").into_response();
        }
    };
    
    info!("Auth0 callback received, exchanging code for token");

    // Get session from request extensions
    let session = match req.extensions().get::<Session>() {
        Some(s) => s.clone(),
        None => {
            tracing::error!("No session found in request");
            return Redirect::to("/?error=no_session").into_response();
        }
    };

    // Get Auth0 config
    let auth0_config = match Auth0Config::from_env() {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Auth0 config error: {}", e);
            return Redirect::to("/?error=config").into_response();
        }
    };

    // 1. Exchange code for tokens
    let client = reqwest::Client::new();
    let token_response = match client
        .post(format!("https://{}/oauth/token", auth0_config.domain))
        .json(&serde_json::json!({
            "grant_type": "authorization_code",
            "client_id": auth0_config.client_id,
            "client_secret": auth0_config.client_secret,
            "code": code,
            "redirect_uri": auth0_config.callback_url,
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            tracing::error!("Failed to exchange code: {}", e);
            return Redirect::to("/?error=token_exchange").into_response();
        }
    };

    let tokens: TokenResponse = match token_response.json().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to parse token response: {}", e);
            return Redirect::to("/?error=token_parse").into_response();
        }
    };

    info!("Successfully exchanged code for access token");

    // 2. Get user info from Auth0
    let user_info: Auth0UserInfo = match client
        .get(format!("https://{}/userinfo", auth0_config.domain))
        .bearer_auth(&tokens.access_token)
        .send()
        .await
    {
        Ok(resp) => match resp.json().await {
            Ok(info) => info,
            Err(e) => {
                tracing::error!("Failed to parse user info: {}", e);
                return Redirect::to("/?error=userinfo_parse").into_response();
            }
        },
        Err(e) => {
            tracing::error!("Failed to get user info: {}", e);
            return Redirect::to("/?error=userinfo").into_response();
        }
    };

    info!("Got user info for: {}", user_info.email);

    // 3. Sync user to database
    let local_user_id = match sync_user_to_database(&state.db, &user_info).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to sync user to database: {}", e);
            return Redirect::to("/?error=db_sync").into_response();
        }
    };

    info!("User synced to database with ID: {}", local_user_id);

    // 4. Create session
    let session_data = SessionData {
        user_id: local_user_id.to_string(),
        email: user_info.email,
        name: user_info.name,
        picture: user_info.picture,
    };

    if let Err(e) = session.insert("user", session_data).await {
        tracing::error!("Failed to create session: {}", e);
        return Redirect::to("/?error=session_failed").into_response();
    }

    info!("Session created successfully for user: {}", local_user_id);
    
    // Redirect to home after successful login
    Redirect::to("/").into_response()
}

/// Sync Auth0 user to local database
async fn sync_user_to_database(
    db: &shared::database::Database,
    user_info: &Auth0UserInfo,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    // 1. Insert or update user in users table
    let user_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (id, auth0_id, email, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (auth0_id) DO UPDATE SET
            email = EXCLUDED.email,
            updated_at = NOW()
         RETURNING id"
    )
    .bind(Uuid::new_v4())
    .bind(&user_info.sub)
    .bind(&user_info.email)
    .fetch_one(&db.pool)
    .await?;

    // 2. Insert or update user profile (user_id is PK, no separate id column)
    sqlx::query(
        "INSERT INTO user_profiles (user_id, name, picture, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         ON CONFLICT (user_id) DO UPDATE SET
            name = EXCLUDED.name,
            picture = EXCLUDED.picture,
            updated_at = NOW()"
    )
    .bind(user_id)
    .bind(&user_info.name)
    .bind(&user_info.picture)
    .execute(&db.pool)
    .await?;

    Ok(user_id)
}

// ==========================================================
// COMMENTED ENDPOINT - KEPT FOR FUTURE REFERENCE
// ==========================================================
// /// Get current user from session - Frontend API endpoint
// /// USAGE: Add to router as GET /auth/me for JavaScript frontend calls
// /// PURPOSE: Provides user authentication status and profile data to frontend
// pub async fn get_current_user(session: Session) -> Json<serde_json::Value> {
//     match session.get::<SessionData>("user").await {
//         Ok(Some(user)) => Json(serde_json::json!({
//             "authenticated": true,
//             "user_id": user.user_id,
//             "email": user.email,
//             "name": user.name,
//             "picture": user.picture,
//         })),
//         Ok(None) => Json(serde_json::json!({
//             "authenticated": false,
//             "error": "Not logged in"
//         })),
//         Err(e) => {
//             tracing::error!("Session error: {}", e);
//             Json(serde_json::json!({
//                 "authenticated": false,
//                 "error": "Session error"
//             }))
//         }
//     }
// }

/// Logout endpoint
pub async fn logout(req: Request) -> impl IntoResponse {
    // Get session from request extensions
    let session = match req.extensions().get::<Session>() {
        Some(s) => s.clone(),
        None => {
            tracing::error!("No session found in logout request");
            return Json(serde_json::json!({
                "success": false,
                "error": "No session found"
            })).into_response();
        }
    };

    // Delete session
    if let Err(e) = session.delete().await {
        tracing::error!("Failed to delete session: {}", e);
        return Json(serde_json::json!({
            "success": false,
            "error": "Failed to delete session"
        })).into_response();
    }

    info!("User logged out");
    Json(serde_json::json!({
        "success": true,
        "message": "Logged out",
        "redirect_url": "/"
    })).into_response()
}

/// Extractor for authenticated user - use this in route handlers
/// Example: `async fn protected_route(AuthUser(user): AuthUser) -> impl IntoResponse`
pub struct AuthUser(pub SessionData);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
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
/// This extractor never fails - it returns None if no session or no user
pub struct OptionalAuthUser(pub Option<SessionData>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to get session, return None if not found
        let session = match parts.extensions.get::<Session>() {
            Some(s) => s,
            None => return Ok(OptionalAuthUser(None)),
        };

        // Try to get user from session, return None on any error
        match session.get::<SessionData>("user").await {
            Ok(user) => Ok(OptionalAuthUser(user)),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}
