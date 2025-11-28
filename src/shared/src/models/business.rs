use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

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
    pub price: Option<Decimal>,
    pub currency: String, // Has default 'USD' in DB, so never null
    pub unit: Option<String>,
    pub is_available: bool, // Has default TRUE in DB, so never null
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

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessSearchResult {
    #[serde(flatten)]
    pub business: Business,
    pub distance_km: Option<f64>,
    pub product_count: i64,
}

// ============================================================================
// Reviews
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BusinessReview {
    pub id: i64,
    pub business_id: i64,
    pub user_id: Uuid,
    pub rating: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_verified_purchase: bool,
    pub helpful_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewWithUser {
    pub id: i64,
    pub business_id: i64,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_avatar: Option<String>,
    pub rating: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_verified_purchase: bool,
    pub helpful_count: i32,
    pub created_at: DateTime<Utc>,
    pub response: Option<ReviewResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub id: i64,
    pub content: String,
    pub responder_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReviewRequest {
    pub rating: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReviewResponseRequest {
    pub content: String,
}

// ============================================================================
// Orders
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Preparing,
    Ready,
    Delivered,
    Cancelled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Confirmed => write!(f, "confirmed"),
            OrderStatus::Preparing => write!(f, "preparing"),
            OrderStatus::Ready => write!(f, "ready"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i64,
    pub business_id: i64,
    pub user_id: Uuid,
    pub status: String,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub delivery_address: Option<String>,
    pub delivery_type: String,
    pub estimated_ready_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderWithItems {
    pub order: Order,
    pub items: Vec<OrderItem>,
    pub business_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub items: Vec<CreateOrderItemRequest>,
    pub notes: Option<String>,
    pub delivery_type: Option<String>,
    pub delivery_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderItemRequest {
    pub product_id: i64,
    pub quantity: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
    pub estimated_ready_at: Option<DateTime<Utc>>,
}