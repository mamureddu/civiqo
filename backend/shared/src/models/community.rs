use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{Polygon, Point};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub is_public: bool,
    pub requires_approval: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunitySettings {
    pub id: Uuid,
    pub community_id: Uuid,
    pub max_members: Option<i32>,
    pub allow_business_listings: bool,
    pub governance_rules: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunityBoundary {
    pub id: Uuid,
    pub community_id: Uuid,
    pub name: String,
    pub boundary_data: serde_json::Value, // GeoJSON polygon
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunityMember {
    pub id: Uuid,
    pub user_id: Uuid,
    pub community_id: Uuid,
    pub role_id: Uuid,
    pub status: MembershipStatus,
    pub joined_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "membership_status", rename_all = "snake_case")]
pub enum MembershipStatus {
    Pending,
    Active,
    Suspended,
    Banned,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: serde_json::Value, // JSON array of permission strings
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommunityRequest {
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub is_public: bool,
    pub requires_approval: bool,
    pub boundary: Option<Polygon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommunityRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub requires_approval: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinCommunityRequest {
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityWithStats {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub is_public: bool,
    pub requires_approval: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub member_count: i64,
    pub business_count: i64,
    pub is_member: bool,
    pub user_role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberWithProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub community_id: Uuid,
    pub role_id: Uuid,
    pub status: MembershipStatus,
    pub joined_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: Option<String>,
    pub user_avatar: Option<String>,
    pub role_name: String,
}

// Search and discovery
#[derive(Debug, Deserialize)]
pub struct CommunitySearchQuery {
    pub q: Option<String>,
    pub location: Option<Point>,
    pub radius_km: Option<f64>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CommunitySearchResult {
    #[serde(flatten)]
    pub community: Community,
    pub distance_km: Option<f64>,
    pub member_count: i64,
}