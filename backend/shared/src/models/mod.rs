use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod community;
pub mod user;
pub mod business;
pub mod governance;
pub mod chat;

// Re-export all model types
pub use community::*;
pub use user::*;
pub use business::*;
pub use governance::*;
pub use chat::*;

// Common response types
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

// Common query parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    20
}

impl PaginationParams {
    pub fn offset(&self) -> i32 {
        (self.page - 1) * self.per_page
    }

    pub fn limit(&self) -> i32 {
        self.per_page.min(100) // Cap at 100
    }
}

// Database audit fields
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuditFields {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Geographic point (for PostGIS compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

// Geographic polygon (for community boundaries)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polygon {
    pub coordinates: Vec<Vec<Point>>,
}