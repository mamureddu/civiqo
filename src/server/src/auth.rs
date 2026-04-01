use axum::{
    extract::{Request, FromRequestParts, State, Form, Query},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Redirect, Html},
    Json,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::info;
use std::sync::Arc;
use uuid::Uuid;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use shared::auth::JwtService;

/// Session data stored in tower-sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

/// Login form data
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

/// Registration form data
#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

/// API token response
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: u64,
}

/// Query params for error display on auth pages
#[derive(Debug, Deserialize)]
pub struct AuthErrorParams {
    pub error: Option<String>,
}

fn error_message(code: &str) -> &str {
    match code {
        "invalid_credentials" => "Email o password non corretti.",
        "email_taken" => "Questa email è già registrata.",
        "password_too_short" => "La password deve essere di almeno 8 caratteri.",
        "invalid_form" => "Dati del form non validi.",
        "session" => "Errore di sessione. Riprova.",
        "server" => "Errore del server. Riprova più tardi.",
        _ => "Si è verificato un errore.",
    }
}

// ── Page Handlers (render HTML) ──────────────────────────────

/// GET /login — render login page
pub async fn login_page(
    crate::i18n_tera::LocaleExtractor(locale): crate::i18n_tera::LocaleExtractor,
    Query(params): Query<AuthErrorParams>,
    State(state): State<Arc<crate::handlers::pages::AppState>>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    crate::i18n_tera::add_i18n_context(&mut ctx, &locale);
    ctx.insert("logged_in", &false);
    let error_msg = params.error.as_deref().map(error_message);
    ctx.insert("error", &error_msg);
    match state.tera.render("login.html", &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
        }
    }
}

/// GET /register — render registration page
pub async fn register_page(
    crate::i18n_tera::LocaleExtractor(locale): crate::i18n_tera::LocaleExtractor,
    Query(params): Query<AuthErrorParams>,
    State(state): State<Arc<crate::handlers::pages::AppState>>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    crate::i18n_tera::add_i18n_context(&mut ctx, &locale);
    ctx.insert("logged_in", &false);
    let error_msg = params.error.as_deref().map(error_message);
    ctx.insert("error", &error_msg);
    match state.tera.render("register.html", &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
        }
    }
}

// ── Form Handlers (HTMX / browser forms) ─────────────────────

/// POST /login — process login form
pub async fn login_handler(
    State(state): State<Arc<crate::handlers::pages::AppState>>,
    req: Request,
) -> impl IntoResponse {
    // Extract session
    let session = match req.extensions().get::<Session>() {
        Some(s) => s.clone(),
        None => return Redirect::to("/login?error=session").into_response(),
    };

    // Parse form body
    let body = match axum::body::to_bytes(req.into_body(), 1024 * 16).await {
        Ok(b) => b,
        Err(_) => return Redirect::to("/login?error=invalid_form").into_response(),
    };
    let form: LoginForm = match serde_urlencoded::from_bytes(&body) {
        Ok(f) => f,
        Err(_) => return Redirect::to("/login?error=invalid_form").into_response(),
    };

    // Look up user by email
    let user = match sqlx::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
        "SELECT u.id, u.email, u.password_hash, p.name
         FROM users u
         LEFT JOIN user_profiles p ON p.user_id = u.id
         WHERE u.email = $1 AND u.provider = 'local'"
    )
    .bind(&form.email)
    .fetch_optional(&state.db.pool)
    .await {
        Ok(Some(u)) => u,
        Ok(None) => return Redirect::to("/login?error=invalid_credentials").into_response(),
        Err(e) => {
            tracing::error!("DB error during login: {}", e);
            return Redirect::to("/login?error=server").into_response();
        }
    };

    let (user_id, email, password_hash, name) = user;

    // Verify password
    let password_hash = match password_hash {
        Some(h) => h,
        None => return Redirect::to("/login?error=invalid_credentials").into_response(),
    };

    let parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(h) => h,
        Err(_) => return Redirect::to("/login?error=server").into_response(),
    };

    if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_err() {
        return Redirect::to("/login?error=invalid_credentials").into_response();
    }

    // Create session
    let session_data = SessionData {
        user_id: user_id.to_string(),
        email,
        name,
        picture: None,
    };

    if let Err(e) = session.insert("user", session_data).await {
        tracing::error!("Failed to create session: {}", e);
        return Redirect::to("/login?error=session").into_response();
    }

    info!("User logged in: {}", user_id);
    Redirect::to("/").into_response()
}

/// POST /register — process registration form
pub async fn register_handler(
    State(state): State<Arc<crate::handlers::pages::AppState>>,
    req: Request,
) -> impl IntoResponse {
    // Extract session
    let session = match req.extensions().get::<Session>() {
        Some(s) => s.clone(),
        None => return Redirect::to("/register?error=session").into_response(),
    };

    // Parse form body
    let body = match axum::body::to_bytes(req.into_body(), 1024 * 16).await {
        Ok(b) => b,
        Err(_) => return Redirect::to("/register?error=invalid_form").into_response(),
    };
    let form: RegisterForm = match serde_urlencoded::from_bytes(&body) {
        Ok(f) => f,
        Err(_) => return Redirect::to("/register?error=invalid_form").into_response(),
    };

    // Validate
    if form.password.len() < 8 {
        return Redirect::to("/register?error=password_too_short").into_response();
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match Argon2::default().hash_password(form.password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(e) => {
            tracing::error!("Password hash error: {}", e);
            return Redirect::to("/register?error=server").into_response();
        }
    };

    // Insert user + profile
    let user_id = Uuid::new_v4();
    let result = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (id, email, password_hash, provider, email_verified, created_at, updated_at)
         VALUES ($1, $2, $3, 'local', false, NOW(), NOW())
         RETURNING id"
    )
    .bind(user_id)
    .bind(&form.email)
    .bind(&password_hash)
    .fetch_one(&state.db.pool)
    .await;

    let user_id = match result {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Registration DB error: {}", e);
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                return Redirect::to("/register?error=email_taken").into_response();
            }
            return Redirect::to("/register?error=server").into_response();
        }
    };

    // Create profile
    let _ = sqlx::query(
        "INSERT INTO user_profiles (user_id, name, created_at, updated_at)
         VALUES ($1, $2, NOW(), NOW())"
    )
    .bind(user_id)
    .bind(&form.name)
    .execute(&state.db.pool)
    .await;

    // Create session
    let session_data = SessionData {
        user_id: user_id.to_string(),
        email: form.email,
        name: form.name,
        picture: None,
    };

    if let Err(e) = session.insert("user", session_data).await {
        tracing::error!("Failed to create session: {}", e);
        return Redirect::to("/login").into_response();
    }

    info!("New user registered: {}", user_id);
    Redirect::to("/").into_response()
}

// ── API Handlers (return JSON + JWT) ─────────────────────────

/// POST /api/auth/login — returns JWT token
pub async fn api_login(
    State(state): State<Arc<crate::handlers::pages::AppState>>,
    Json(form): Json<LoginForm>,
) -> impl IntoResponse {
    // Look up user
    let user = match sqlx::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
        "SELECT u.id, u.email, u.password_hash, p.name
         FROM users u
         LEFT JOIN user_profiles p ON p.user_id = u.id
         WHERE u.email = $1 AND u.provider = 'local'"
    )
    .bind(&form.email)
    .fetch_optional(&state.db.pool)
    .await {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid credentials"}))).into_response(),
        Err(e) => {
            tracing::error!("DB error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Server error"}))).into_response();
        }
    };

    let (user_id, email, password_hash, name) = user;

    // Verify password
    let password_hash = match password_hash {
        Some(h) => h,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid credentials"}))).into_response(),
    };

    let parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Server error"}))).into_response(),
    };

    if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_err() {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid credentials"}))).into_response();
    }

    // Issue JWT
    let jwt_config = match shared::auth::JwtConfig::from_env() {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Server config error"}))).into_response(),
    };
    let jwt_service = JwtService::new(jwt_config);

    let token = match jwt_service.issue_token(user_id, &email, name.as_deref(), vec![]) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Token generation failed"}))).into_response(),
    };

    Json(TokenResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: jwt_service.expiry_seconds(),
    }).into_response()
}

/// POST /api/auth/register — returns JWT token
pub async fn api_register(
    State(state): State<Arc<crate::handlers::pages::AppState>>,
    Json(form): Json<RegisterForm>,
) -> impl IntoResponse {
    if form.password.len() < 8 {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Password must be at least 8 characters"}))).into_response();
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match Argon2::default().hash_password(form.password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Server error"}))).into_response(),
    };

    // Insert user
    let user_id = Uuid::new_v4();
    if let Err(e) = sqlx::query(
        "INSERT INTO users (id, email, password_hash, provider, email_verified, created_at, updated_at)
         VALUES ($1, $2, $3, 'local', false, NOW(), NOW())"
    )
    .bind(user_id)
    .bind(&form.email)
    .bind(&password_hash)
    .execute(&state.db.pool)
    .await {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            return (StatusCode::CONFLICT, Json(serde_json::json!({"error": "Email already registered"}))).into_response();
        }
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Server error"}))).into_response();
    }

    // Create profile
    let _ = sqlx::query(
        "INSERT INTO user_profiles (user_id, name, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())"
    )
    .bind(user_id)
    .bind(&form.name)
    .execute(&state.db.pool)
    .await;

    // Issue JWT
    let jwt_config = match shared::auth::JwtConfig::from_env() {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Server config error"}))).into_response(),
    };
    let jwt_service = JwtService::new(jwt_config);

    let token = match jwt_service.issue_token(user_id, &form.email, form.name.as_deref(), vec![]) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Token generation failed"}))).into_response(),
    };

    (StatusCode::CREATED, Json(TokenResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: jwt_service.expiry_seconds(),
    })).into_response()
}

/// POST /api/auth/refresh — refresh JWT token (requires valid token via session)
pub async fn refresh_token(
    AuthUser(user): AuthUser,
) -> impl IntoResponse {
    let jwt_config = match shared::auth::JwtConfig::from_env() {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Server config error"}))).into_response(),
    };
    let jwt_service = JwtService::new(jwt_config);

    let user_id = match Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Invalid user ID"}))).into_response(),
    };

    let token = match jwt_service.issue_token(user_id, &user.email, user.name.as_deref(), vec![]) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Token generation failed"}))).into_response(),
    };

    Json(TokenResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: jwt_service.expiry_seconds(),
    }).into_response()
}

// ── Logout ───────────────────────────────────────────────────

/// POST /auth/logout — delete session
pub async fn logout(req: Request) -> impl IntoResponse {
    let session = match req.extensions().get::<Session>() {
        Some(s) => s.clone(),
        None => {
            return Json(serde_json::json!({
                "success": false,
                "error": "No session found"
            })).into_response();
        }
    };

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

// ── Extractors (unchanged interface) ─────────────────────────

/// Extractor for authenticated user — use in route handlers
pub struct AuthUser(pub SessionData);

impl<S> FromRequestParts<S> for AuthUser
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
            Ok(Some(user)) => Ok(AuthUser(user)),
            Ok(None) => Err((StatusCode::UNAUTHORIZED, "Not authenticated")),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error")),
        }
    }
}

/// Optional auth — returns None if not authenticated instead of error
pub struct OptionalAuthUser(pub Option<SessionData>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session = match parts.extensions.get::<Session>() {
            Some(s) => s,
            None => return Ok(OptionalAuthUser(None)),
        };

        match session.get::<SessionData>("user").await {
            Ok(user) => Ok(OptionalAuthUser(user)),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}
