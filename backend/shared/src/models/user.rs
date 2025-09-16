use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub auth0_id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub auth0_id: String,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserProfileRequest {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithProfile {
    pub id: Uuid,
    pub auth0_id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Profile fields (optional)
    pub profile_id: Option<Uuid>,
    pub profile_name: Option<String>,
    pub profile_avatar_url: Option<String>,
    pub profile_bio: Option<String>,
    pub profile_location: Option<String>,
    pub profile_website: Option<String>,
    pub profile_created_at: Option<DateTime<Utc>>,
    pub profile_updated_at: Option<DateTime<Utc>>,
}

// Auth0 JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub aud: String,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    // Custom claims for community roles
    #[serde(default)]
    pub community_roles: Vec<CommunityRole>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityRole {
    pub community_id: Uuid,
    pub role: String,
    pub permissions: Vec<String>,
}