use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::Point;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Business {
    pub id: Uuid,
    pub community_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: BusinessCategory,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "business_category", rename_all = "snake_case")]
pub enum BusinessCategory {
    Food,
    Retail,
    Services,
    Healthcare,
    Education,
    Technology,
    Manufacturing,
    Agriculture,
    Arts,
    Recreation,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BusinessProduct {
    pub id: Uuid,
    pub business_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<sqlx::types::Decimal>,
    pub currency: Option<String>,
    pub unit: Option<String>,
    pub is_available: bool,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BusinessHours {
    pub id: Uuid,
    pub business_id: Uuid,
    pub day_of_week: i32, // 0 = Sunday, 6 = Saturday
    pub open_time: Option<chrono::NaiveTime>,
    pub close_time: Option<chrono::NaiveTime>,
    pub is_closed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBusinessRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: BusinessCategory,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub location: Option<Point>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBusinessRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<BusinessCategory>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub location: Option<Point>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessWithProducts {
    #[serde(flatten)]
    pub business: Business,
    pub products: Vec<BusinessProduct>,
    pub hours: Vec<BusinessHours>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub unit: Option<String>,
    pub is_available: Option<bool>,
}

// Search and discovery
#[derive(Debug, Deserialize)]
pub struct BusinessSearchQuery {
    pub q: Option<String>,
    pub category: Option<BusinessCategory>,
    pub location: Option<Point>,
    pub radius_km: Option<f64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BusinessSearchResult {
    #[serde(flatten)]
    pub business: Business,
    pub distance_km: Option<f64>,
    pub product_count: i64,
}