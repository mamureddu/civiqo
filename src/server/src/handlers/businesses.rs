//! Business Handlers - Phase 4 Implementation
//!
//! Handles all business-related API endpoints including:
//! - Business CRUD operations
//! - Product management
//! - Reviews
//! - Orders

#![allow(dead_code)]

use crate::auth::AuthUser;
use crate::handlers::pages::{AppError, AppState};
use axum::{
    extract::{Form, Path, Query, State},
    response::{Html, IntoResponse, Json, Response},
};
use sqlx::Row;
use std::sync::Arc;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, serde::Deserialize)]
pub struct BusinessListQuery {
    pub q: Option<String>,
    pub category: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, serde::Serialize)]
pub struct BusinessListResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub rating_avg: f64,
    pub review_count: i32,
    pub is_verified: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateBusinessRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateBusinessRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateProductRequest {
    pub product_name: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateReviewRequest {
    pub rating: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateOrderRequest {
    pub items: Vec<OrderItemRequest>,
    pub notes: Option<String>,
    pub delivery_type: Option<String>,
    pub delivery_address: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrderItemRequest {
    pub product_id: uuid::Uuid,
    pub quantity: i32,
    pub notes: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}

// ============================================================================
// Business Handlers
// ============================================================================

/// List all businesses with optional filtering
pub async fn list_businesses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<BusinessListQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20).min(50);
    let offset = (page - 1) * limit;

    let businesses = if let Some(ref q) = params.q {
        let search = format!("%{}%", q);
        sqlx::query(
            r#"SELECT id, name, description, category, address, phone,
                      COALESCE(rating_avg, 0) as rating_avg,
                      COALESCE(review_count, 0) as review_count,
                      COALESCE(is_verified, false) as is_verified
               FROM businesses
               WHERE is_active = true AND (name ILIKE $1 OR description ILIKE $1)
               ORDER BY rating_avg DESC, name ASC
               LIMIT $2 OFFSET $3"#,
        )
        .bind(&search)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    } else if let Some(ref cat) = params.category {
        sqlx::query(
            r#"SELECT id, name, description, category, address, phone,
                      COALESCE(rating_avg, 0) as rating_avg,
                      COALESCE(review_count, 0) as review_count,
                      COALESCE(is_verified, false) as is_verified
               FROM businesses
               WHERE is_active = true AND category = $1
               ORDER BY rating_avg DESC, name ASC
               LIMIT $2 OFFSET $3"#,
        )
        .bind(cat)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    } else {
        sqlx::query(
            r#"SELECT id, name, description, category, address, phone,
                      COALESCE(rating_avg, 0) as rating_avg,
                      COALESCE(review_count, 0) as review_count,
                      COALESCE(is_verified, false) as is_verified
               FROM businesses
               WHERE is_active = true
               ORDER BY rating_avg DESC, name ASC
               LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    };

    match businesses {
        Ok(rows) => {
            let data: Vec<serde_json::Value> = rows
                .iter()
                .map(|row| {
                    serde_json::json!({
                        "id": row.get::<uuid::Uuid, _>("id").to_string(),
                        "name": row.get::<String, _>("name"),
                        "description": row.get::<Option<String>, _>("description"),
                        "category": row.get::<Option<String>, _>("category"),
                        "address": row.get::<Option<String>, _>("address"),
                        "phone": row.get::<Option<String>, _>("phone"),
                        "rating_avg": row.get::<rust_decimal::Decimal, _>("rating_avg"),
                        "review_count": row.get::<i32, _>("review_count"),
                        "is_verified": row.get::<bool, _>("is_verified")
                    })
                })
                .collect();

            Ok(Json(serde_json::json!({
                "success": true,
                "data": data,
                "page": page,
                "limit": limit
            })))
        }
        Err(e) => {
            tracing::error!("Failed to list businesses: {}", e);
            Err(AppError::Internal(anyhow::anyhow!(
                "Failed to list businesses"
            )))
        }
    }
}

/// Get business details by ID
pub async fn get_business(
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let business = sqlx::query(
        r#"SELECT b.*, 
                  COALESCE(u.email, '') as owner_email,
                  COALESCE(p.name, u.email) as owner_name
           FROM businesses b
           LEFT JOIN users u ON b.owner_id = u.id
           LEFT JOIN user_profiles p ON u.id = p.user_id
           WHERE b.id = $1"#,
    )
    .bind(business_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    match business {
        Some(row) => {
            // Fetch products
            let products = sqlx::query(
                r#"SELECT id, product_name, description, price, currency, is_available
                   FROM business_products
                   WHERE business_id = $1
                   ORDER BY product_name"#,
            )
            .bind(business_id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

            // Fetch hours
            let hours = sqlx::query(
                r#"SELECT day_of_week, open_time, close_time, is_closed
                   FROM business_hours
                   WHERE business_id = $1
                   ORDER BY day_of_week"#,
            )
            .bind(business_id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

            let products_json: Vec<serde_json::Value> = products
                .iter()
                .map(|p| {
                    serde_json::json!({
                        "id": p.get::<uuid::Uuid, _>("id").to_string(),
                        "name": p.get::<String, _>("product_name"),
                        "description": p.get::<Option<String>, _>("description"),
                        "price": p.get::<Option<rust_decimal::Decimal>, _>("price"),
                        "currency": p.get::<Option<String>, _>("currency"),
                        "is_available": p.get::<Option<bool>, _>("is_available").unwrap_or(true)
                    })
                })
                .collect();

            let hours_json: Vec<serde_json::Value> = hours.iter().map(|h| {
                serde_json::json!({
                    "day_of_week": h.get::<Option<String>, _>("day_of_week"),
                    "open_time": h.get::<Option<chrono::NaiveTime>, _>("open_time").map(|t| t.to_string()),
                    "close_time": h.get::<Option<chrono::NaiveTime>, _>("close_time").map(|t| t.to_string()),
                    "is_closed": h.get::<Option<bool>, _>("is_closed").unwrap_or(false)
                })
            }).collect();

            Ok(Json(serde_json::json!({
                "success": true,
                "data": {
                    "id": row.get::<uuid::Uuid, _>("id").to_string(),
                    "name": row.get::<String, _>("name"),
                    "description": row.get::<Option<String>, _>("description"),
                    "category": row.get::<Option<String>, _>("category"),
                    "address": row.get::<Option<String>, _>("address"),
                    "phone": row.get::<Option<String>, _>("phone"),
                    "email": row.get::<Option<String>, _>("email"),
                    "website": row.get::<Option<String>, _>("website"),
                    "is_active": row.get::<bool, _>("is_active"),
                    "is_verified": row.get::<Option<bool>, _>("is_verified").unwrap_or(false),
                    "rating_avg": row.get::<Option<rust_decimal::Decimal>, _>("rating_avg"),
                    "review_count": row.get::<Option<i32>, _>("review_count").unwrap_or(0),
                    "owner_name": row.get::<String, _>("owner_name"),
                    "products": products_json,
                    "hours": hours_json
                }
            })))
        }
        None => Err(AppError::Internal(anyhow::anyhow!("Business not found"))),
    }
}

/// Create a new business (JSON API)
pub async fn create_business(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateBusinessRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;

    // Get user's first community (or require community_id in request)
    let community = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT community_id FROM community_members WHERE user_id = $1 LIMIT 1",
    )
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    let community_id = community
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("User must be member of a community")))?;

    let business_id = uuid::Uuid::now_v7();

    sqlx::query(
        r#"INSERT INTO businesses (id, community_id, owner_id, name, description, category, address, phone, email, website, is_active)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, true)"#
    )
    .bind(business_id)
    .bind(community_id)
    .bind(user_uuid)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.category)
    .bind(&req.address)
    .bind(&req.phone)
    .bind(&req.email)
    .bind(&req.website)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create business: {}", e)))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "id": business_id.to_string(),
            "name": req.name
        },
        "message": "Attività creata con successo"
    })))
}

/// Create a new business (HTMX form handler)
pub async fn create_business_htmx(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Form(req): Form<CreateBusinessRequest>,
) -> Response {
    let user_uuid =
        match uuid::Uuid::parse_str(&user.user_id) {
            Ok(u) => u,
            Err(_) => return Html(
                "<div class=\"p-4 bg-red-50 text-red-700 rounded-lg\">ID utente non valido</div>",
            )
            .into_response(),
        };

    let name = req.name.trim();
    if name.is_empty() {
        return Html("<div class=\"p-4 bg-red-50 text-red-700 rounded-lg\">Il nome dell'attività è obbligatorio</div>").into_response();
    }

    // Get user's first community
    let community = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT community_id FROM community_members WHERE user_id = $1 LIMIT 1",
    )
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await;

    let community_id = match community {
        Ok(Some(id)) => id,
        Ok(None) => return Html("<div class=\"p-4 bg-red-50 text-red-700 rounded-lg\">Devi essere membro di una community per creare un'attività</div>").into_response(),
        Err(e) => {
            tracing::error!("DB error looking up community membership: {}", e);
            return Html("<div class=\"p-4 bg-red-50 text-red-700 rounded-lg\">Errore del database</div>").into_response();
        }
    };

    let business_id = uuid::Uuid::now_v7();

    let result = sqlx::query(
        r#"INSERT INTO businesses (id, community_id, owner_id, name, description, category, address, phone, email, website, is_active)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, true)"#
    )
    .bind(business_id)
    .bind(community_id)
    .bind(user_uuid)
    .bind(name)
    .bind(&req.description)
    .bind(&req.category)
    .bind(&req.address)
    .bind(&req.phone)
    .bind(&req.email)
    .bind(&req.website)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(_) => {
            // Return HX-Redirect header to navigate to the new business page
            let redirect_url = format!("/businesses/{}", business_id);
            Response::builder()
                .status(200)
                .header("HX-Redirect", &redirect_url)
                .body(axum::body::Body::empty())
                .unwrap()
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create business: {}", e);
            Html(format!("<div class=\"p-4 bg-red-50 text-red-700 rounded-lg\">Errore nella creazione dell'attività: {}</div>", e)).into_response()
        }
    }
}

/// Update business (owner only)
pub async fn update_business(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateBusinessRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;

    // Verify ownership
    let owner_id: Option<uuid::Uuid> =
        sqlx::query_scalar("SELECT owner_id FROM businesses WHERE id = $1")
            .bind(business_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    match owner_id {
        Some(id) if id == user_uuid => {}
        Some(_) => {
            return Err(AppError::Internal(anyhow::anyhow!(
                "Non sei il proprietario di questa attività"
            )))
        }
        None => return Err(AppError::Internal(anyhow::anyhow!("Attività non trovata"))),
    }

    // Build dynamic update query
    let mut updates = vec!["updated_at = NOW()".to_string()];
    let mut param_count = 1;

    if req.name.is_some() {
        updates.push(format!("name = ${}", {
            param_count += 1;
            param_count
        }));
    }
    if req.description.is_some() {
        updates.push(format!("description = ${}", {
            param_count += 1;
            param_count
        }));
    }
    if req.category.is_some() {
        updates.push(format!("category = ${}", {
            param_count += 1;
            param_count
        }));
    }
    if req.address.is_some() {
        updates.push(format!("address = ${}", {
            param_count += 1;
            param_count
        }));
    }
    if req.phone.is_some() {
        updates.push(format!("phone = ${}", {
            param_count += 1;
            param_count
        }));
    }
    if req.email.is_some() {
        updates.push(format!("email = ${}", {
            param_count += 1;
            param_count
        }));
    }
    if req.website.is_some() {
        updates.push(format!("website = ${}", {
            param_count += 1;
            param_count
        }));
    }
    if req.is_active.is_some() {
        updates.push(format!("is_active = ${}", {
            param_count += 1;
            param_count
        }));
    }

    let query = format!("UPDATE businesses SET {} WHERE id = $1", updates.join(", "));

    let mut q = sqlx::query(&query).bind(business_id);
    if let Some(ref v) = req.name {
        q = q.bind(v);
    }
    if let Some(ref v) = req.description {
        q = q.bind(v);
    }
    if let Some(ref v) = req.category {
        q = q.bind(v);
    }
    if let Some(ref v) = req.address {
        q = q.bind(v);
    }
    if let Some(ref v) = req.phone {
        q = q.bind(v);
    }
    if let Some(ref v) = req.email {
        q = q.bind(v);
    }
    if let Some(ref v) = req.website {
        q = q.bind(v);
    }
    if let Some(v) = req.is_active {
        q = q.bind(v);
    }

    q.execute(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to update business: {}", e)))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Attività aggiornata con successo"
    })))
}

/// Delete business (owner only)
pub async fn delete_business(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;

    let result = sqlx::query("DELETE FROM businesses WHERE id = $1 AND owner_id = $2")
        .bind(business_id)
        .bind(user_uuid)
        .execute(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Attività non trovata o non autorizzato"
        )));
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Attività eliminata con successo"
    })))
}

// ============================================================================
// Product Handlers
// ============================================================================

/// List products for a business
pub async fn list_products(
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let products = sqlx::query(
        r#"SELECT id, product_name, description, price, currency, is_available
           FROM business_products
           WHERE business_id = $1
           ORDER BY product_name"#,
    )
    .bind(business_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    let data: Vec<serde_json::Value> = products
        .iter()
        .map(|p| {
            serde_json::json!({
                "id": p.get::<uuid::Uuid, _>("id").to_string(),
                "name": p.get::<String, _>("product_name"),
                "description": p.get::<Option<String>, _>("description"),
                "price": p.get::<Option<rust_decimal::Decimal>, _>("price"),
                "currency": p.get::<Option<String>, _>("currency"),
                "is_available": p.get::<Option<bool>, _>("is_available").unwrap_or(true)
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "success": true,
        "data": data
    })))
}

/// Create product (business owner only)
pub async fn create_product(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<uuid::Uuid>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;

    // Verify ownership
    let owner_id: Option<uuid::Uuid> =
        sqlx::query_scalar("SELECT owner_id FROM businesses WHERE id = $1")
            .bind(business_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    match owner_id {
        Some(id) if id == user_uuid => {}
        Some(_) => return Err(AppError::Internal(anyhow::anyhow!("Non autorizzato"))),
        None => return Err(AppError::Internal(anyhow::anyhow!("Attività non trovata"))),
    }

    let product_id = uuid::Uuid::now_v7();

    sqlx::query(
        r#"INSERT INTO business_products (id, business_id, product_name, description, price, currency, is_available)
           VALUES ($1, $2, $3, $4, $5, $6, true)"#
    )
    .bind(product_id)
    .bind(business_id)
    .bind(&req.product_name)
    .bind(&req.description)
    .bind(req.price.map(rust_decimal::Decimal::from_f64_retain).flatten())
    .bind(req.currency.as_deref().unwrap_or("EUR"))
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create product: {}", e)))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "id": product_id.to_string(),
            "name": req.product_name
        },
        "message": "Prodotto creato con successo"
    })))
}

// ============================================================================
// Review Handlers
// ============================================================================

/// List reviews for a business
pub async fn list_reviews(
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let reviews = sqlx::query(
        r#"SELECT r.id, r.rating, r.title, r.content, r.helpful_count, r.created_at,
                  COALESCE(p.name, u.email) as user_name,
                  p.avatar_url as user_avatar
           FROM business_reviews r
           JOIN users u ON r.user_id = u.id
           LEFT JOIN user_profiles p ON u.id = p.user_id
           WHERE r.business_id = $1
           ORDER BY r.created_at DESC
           LIMIT 50"#,
    )
    .bind(business_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    let data: Vec<serde_json::Value> = reviews
        .iter()
        .map(|r| {
            serde_json::json!({
                "id": r.get::<uuid::Uuid, _>("id").to_string(),
                "rating": r.get::<i32, _>("rating"),
                "title": r.get::<Option<String>, _>("title"),
                "content": r.get::<Option<String>, _>("content"),
                "helpful_count": r.get::<i32, _>("helpful_count"),
                "user_name": r.get::<String, _>("user_name"),
                "user_avatar": r.get::<Option<String>, _>("user_avatar"),
                "created_at": r.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "success": true,
        "data": data
    })))
}

/// Create review (authenticated users only, one per business)
pub async fn create_review(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<uuid::Uuid>,
    Json(req): Json<CreateReviewRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;

    // Validate rating
    if req.rating < 1 || req.rating > 5 {
        return Err(AppError::Internal(anyhow::anyhow!(
            "La valutazione deve essere tra 1 e 5"
        )));
    }

    // Check if user already reviewed
    let existing: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT id FROM business_reviews WHERE business_id = $1 AND user_id = $2",
    )
    .bind(business_id)
    .bind(user_uuid)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Hai già recensito questa attività"
        )));
    }

    // Create review
    let review_id = uuid::Uuid::now_v7();

    sqlx::query(
        r#"INSERT INTO business_reviews (id, business_id, user_id, rating, title, content)
           VALUES ($1, $2, $3, $4, $5, $6)"#,
    )
    .bind(review_id)
    .bind(business_id)
    .bind(user_uuid)
    .bind(req.rating)
    .bind(&req.title)
    .bind(&req.content)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create review: {}", e)))?;

    // Update business rating cache
    let _ = sqlx::query(
        r#"UPDATE businesses SET
           rating_avg = (SELECT AVG(rating)::NUMERIC(3,2) FROM business_reviews WHERE business_id = $1),
           review_count = (SELECT COUNT(*) FROM business_reviews WHERE business_id = $1)
           WHERE id = $1"#
    )
    .bind(business_id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": { "id": review_id.to_string() },
        "message": "Recensione pubblicata con successo"
    })))
}

// ============================================================================
// Order Handlers
// ============================================================================

/// List user's orders
pub async fn list_user_orders(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;

    let orders = sqlx::query(
        r#"SELECT o.id, o.status, o.total_amount, o.currency, o.delivery_type, o.created_at,
                  b.name as business_name
           FROM orders o
           JOIN businesses b ON o.business_id = b.id
           WHERE o.user_id = $1
           ORDER BY o.created_at DESC
           LIMIT 50"#,
    )
    .bind(user_uuid)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    let data: Vec<serde_json::Value> = orders
        .iter()
        .map(|o| {
            serde_json::json!({
                "id": o.get::<uuid::Uuid, _>("id").to_string(),
                "status": o.get::<String, _>("status"),
                "total_amount": o.get::<rust_decimal::Decimal, _>("total_amount"),
                "currency": o.get::<String, _>("currency"),
                "delivery_type": o.get::<String, _>("delivery_type"),
                "business_name": o.get::<String, _>("business_name"),
                "created_at": o.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "success": true,
        "data": data
    })))
}

/// Create order
pub async fn create_order(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(business_id): Path<uuid::Uuid>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;

    if req.items.is_empty() {
        return Err(AppError::Internal(anyhow::anyhow!(
            "L'ordine deve contenere almeno un prodotto"
        )));
    }

    // Calculate total and validate products
    let mut total = rust_decimal::Decimal::ZERO;
    let mut order_items = Vec::new();

    for item in &req.items {
        let product = sqlx::query(
            "SELECT product_name, price FROM business_products WHERE id = $1 AND business_id = $2 AND is_available = true"
        )
        .bind(item.product_id)
        .bind(business_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        match product {
            Some(p) => {
                let price: rust_decimal::Decimal = p
                    .get::<Option<rust_decimal::Decimal>, _>("price")
                    .unwrap_or(rust_decimal::Decimal::ZERO);
                let name: String = p.get("product_name");
                let item_total = price * rust_decimal::Decimal::from(item.quantity);
                total += item_total;
                order_items.push((
                    item.product_id,
                    name,
                    item.quantity,
                    price,
                    item_total,
                    item.notes.clone(),
                ));
            }
            None => {
                return Err(AppError::Internal(anyhow::anyhow!(
                    "Prodotto non disponibile: {}",
                    item.product_id
                )))
            }
        }
    }

    // Create order
    let order_id = uuid::Uuid::now_v7();

    sqlx::query(
        r#"INSERT INTO orders (id, business_id, user_id, status, total_amount, currency, notes, delivery_type, delivery_address)
           VALUES ($1, $2, $3, 'pending', $4, 'EUR', $5, $6, $7)"#
    )
    .bind(order_id)
    .bind(business_id)
    .bind(user_uuid)
    .bind(total)
    .bind(&req.notes)
    .bind(req.delivery_type.as_deref().unwrap_or("pickup"))
    .bind(&req.delivery_address)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create order: {}", e)))?;

    // Create order items
    for (product_id, name, qty, unit_price, total_price, notes) in order_items {
        let item_id = uuid::Uuid::now_v7();
        let _ = sqlx::query(
            r#"INSERT INTO order_items (id, order_id, product_id, product_name, quantity, unit_price, total_price, notes)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#
        )
        .bind(item_id)
        .bind(order_id)
        .bind(product_id)
        .bind(name)
        .bind(qty)
        .bind(unit_price)
        .bind(total_price)
        .bind(notes)
        .execute(&state.db.pool)
        .await;
    }

    // Update business order count
    let _ = sqlx::query(
        "UPDATE businesses SET order_count = COALESCE(order_count, 0) + 1 WHERE id = $1",
    )
    .bind(business_id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "id": order_id.to_string(),
            "total": total
        },
        "message": "Ordine creato con successo"
    })))
}

/// Update order status (business owner only)
pub async fn update_order_status(
    AuthUser(user): AuthUser,
    State(state): State<Arc<AppState>>,
    Path(order_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateOrderStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_uuid = uuid::Uuid::parse_str(&user.user_id)
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid user ID")))?;

    // Validate status
    let valid_statuses = [
        "pending",
        "confirmed",
        "preparing",
        "ready",
        "delivered",
        "cancelled",
    ];
    if !valid_statuses.contains(&req.status.as_str()) {
        return Err(AppError::Internal(anyhow::anyhow!("Stato non valido")));
    }

    // Verify ownership
    let owner_check = sqlx::query_scalar::<_, uuid::Uuid>(
        r#"SELECT b.owner_id FROM orders o
           JOIN businesses b ON o.business_id = b.id
           WHERE o.id = $1"#,
    )
    .bind(order_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

    match owner_check {
        Some(id) if id == user_uuid => {}
        Some(_) => return Err(AppError::Internal(anyhow::anyhow!("Non autorizzato"))),
        None => return Err(AppError::Internal(anyhow::anyhow!("Ordine non trovato"))),
    }

    let completed_at = if req.status == "delivered" || req.status == "cancelled" {
        Some(chrono::Utc::now())
    } else {
        None
    };

    sqlx::query(
        "UPDATE orders SET status = $1, completed_at = $2, updated_at = NOW() WHERE id = $3",
    )
    .bind(&req.status)
    .bind(completed_at)
    .bind(order_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to update order: {}", e)))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Stato ordine aggiornato"
    })))
}
