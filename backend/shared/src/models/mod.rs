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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use test_case::test_case;

    #[test]
    fn test_api_response_serialization() {
        let response = ApiResponse {
            success: true,
            data: Some("test data".to_string()),
            message: Some("Success message".to_string()),
            error: None,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ApiResponse<String> = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.success, true);
        assert_eq!(deserialized.data, Some("test data".to_string()));
        assert_eq!(deserialized.message, Some("Success message".to_string()));
        assert!(deserialized.error.is_none());
    }

    #[test]
    fn test_api_response_with_error() {
        let response: ApiResponse<String> = ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some(ApiError {
                code: "VALIDATION_ERROR".to_string(),
                message: "Invalid input".to_string(),
            }),
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ApiResponse<String> = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.success, false);
        assert!(deserialized.data.is_none());
        assert!(deserialized.message.is_none());
        assert!(deserialized.error.is_some());

        let error = deserialized.error.unwrap();
        assert_eq!(error.code, "VALIDATION_ERROR");
        assert_eq!(error.message, "Invalid input");
    }

    #[test]
    fn test_paginated_response() {
        let items = vec!["item1".to_string(), "item2".to_string(), "item3".to_string()];
        let response = PaginatedResponse {
            items: items.clone(),
            total: 100,
            page: 2,
            per_page: 20,
            total_pages: 5,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: PaginatedResponse<String> = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.items, items);
        assert_eq!(deserialized.total, 100);
        assert_eq!(deserialized.page, 2);
        assert_eq!(deserialized.per_page, 20);
        assert_eq!(deserialized.total_pages, 5);
    }

    #[test]
    fn test_pagination_params_defaults() {
        let params = PaginationParams {
            page: default_page(),
            per_page: default_per_page(),
        };

        assert_eq!(params.page, 1);
        assert_eq!(params.per_page, 20);
    }

    #[test]
    fn test_pagination_params_deserialization() {
        // Test with explicit values
        let json = r#"{"page": 3, "per_page": 10}"#;
        let params: PaginationParams = serde_json::from_str(json).expect("Should deserialize");

        assert_eq!(params.page, 3);
        assert_eq!(params.per_page, 10);

        // Test with missing values (should use defaults)
        let json_minimal = r#"{}"#;
        let params_minimal: PaginationParams = serde_json::from_str(json_minimal).expect("Should deserialize");

        assert_eq!(params_minimal.page, 1);
        assert_eq!(params_minimal.per_page, 20);
    }

    #[test_case(1, 20, 0; "first page")]
    #[test_case(2, 20, 20; "second page")]
    #[test_case(5, 10, 40; "fifth page with 10 per page")]
    #[test_case(1, 50, 0; "first page with 50 per page")]
    fn test_pagination_offset(page: i32, per_page: i32, expected_offset: i32) {
        let params = PaginationParams { page, per_page };
        assert_eq!(params.offset(), expected_offset);
    }

    #[test_case(10, 10; "normal limit")]
    #[test_case(150, 100; "limit capped at 100")]
    #[test_case(50, 50; "limit under cap")]
    fn test_pagination_limit(per_page: i32, expected_limit: i32) {
        let params = PaginationParams { page: 1, per_page };
        assert_eq!(params.limit(), expected_limit);
    }

    #[test]
    fn test_audit_fields_serialization() {
        let now = chrono::Utc::now();
        let audit = AuditFields {
            created_at: now,
            updated_at: now,
        };

        let json = serde_json::to_string(&audit).expect("Should serialize");
        let deserialized: AuditFields = serde_json::from_str(&json).expect("Should deserialize");

        // Due to serialization precision, we check that times are very close
        let diff = (deserialized.created_at - audit.created_at).num_milliseconds().abs();
        assert!(diff < 1000); // Within 1 second
    }

    #[test]
    fn test_point_serialization() {
        let point = Point {
            latitude: 40.7128,
            longitude: -74.0060,
        };

        let json = serde_json::to_string(&point).expect("Should serialize");
        let deserialized: Point = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.latitude, 40.7128);
        assert_eq!(deserialized.longitude, -74.0060);
    }

    #[test]
    fn test_polygon_serialization() {
        let polygon = Polygon {
            coordinates: vec![
                vec![
                    Point { latitude: 0.0, longitude: 0.0 },
                    Point { latitude: 0.0, longitude: 1.0 },
                    Point { latitude: 1.0, longitude: 1.0 },
                    Point { latitude: 1.0, longitude: 0.0 },
                    Point { latitude: 0.0, longitude: 0.0 }, // Close the ring
                ],
            ],
        };

        let json = serde_json::to_string(&polygon).expect("Should serialize");
        let deserialized: Polygon = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.coordinates.len(), 1);
        assert_eq!(deserialized.coordinates[0].len(), 5);
        assert_eq!(deserialized.coordinates[0][0].latitude, 0.0);
        assert_eq!(deserialized.coordinates[0][0].longitude, 0.0);
    }

    #[test]
    fn test_complex_polygon_with_holes() {
        // Test a polygon with exterior ring and interior holes
        let polygon = Polygon {
            coordinates: vec![
                // Exterior ring
                vec![
                    Point { latitude: 0.0, longitude: 0.0 },
                    Point { latitude: 0.0, longitude: 10.0 },
                    Point { latitude: 10.0, longitude: 10.0 },
                    Point { latitude: 10.0, longitude: 0.0 },
                    Point { latitude: 0.0, longitude: 0.0 },
                ],
                // Interior hole
                vec![
                    Point { latitude: 2.0, longitude: 2.0 },
                    Point { latitude: 2.0, longitude: 8.0 },
                    Point { latitude: 8.0, longitude: 8.0 },
                    Point { latitude: 8.0, longitude: 2.0 },
                    Point { latitude: 2.0, longitude: 2.0 },
                ],
            ],
        };

        let json = serde_json::to_string(&polygon).expect("Should serialize");
        let deserialized: Polygon = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.coordinates.len(), 2); // Exterior + 1 hole
        assert_eq!(deserialized.coordinates[0].len(), 5); // Exterior ring
        assert_eq!(deserialized.coordinates[1].len(), 5); // Interior hole
    }

    #[test]
    fn test_api_error_serialization() {
        let error = ApiError {
            code: "VALIDATION_ERROR".to_string(),
            message: "Field 'email' is required".to_string(),
        };

        let json = serde_json::to_string(&error).expect("Should serialize");
        let deserialized: ApiError = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.code, "VALIDATION_ERROR");
        assert_eq!(deserialized.message, "Field 'email' is required");
    }

    #[test]
    fn test_pagination_edge_cases() {
        // Test with zero page (should still work mathematically)
        let params = PaginationParams { page: 0, per_page: 20 };
        assert_eq!(params.offset(), -20); // Mathematically correct but likely invalid in practice

        // Test with negative page
        let params = PaginationParams { page: -1, per_page: 20 };
        assert_eq!(params.offset(), -40);

        // These edge cases show that validation should be done at a higher level
    }

    #[test]
    fn test_json_round_trip_with_special_characters() {
        let response = ApiResponse {
            success: true,
            data: Some("Special chars: 🚀 αβγ 中文 \"quotes\" 'apostrophes' & < >".to_string()),
            message: Some("Message with\nnewlines\tand\ttabs".to_string()),
            error: None,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: ApiResponse<String> = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.data, response.data);
        assert_eq!(deserialized.message, response.message);
    }

    #[test]
    fn test_point_edge_cases() {
        // Test extreme coordinates
        let point = Point {
            latitude: -90.0,   // South pole
            longitude: -180.0, // International date line
        };

        let json = serde_json::to_string(&point).expect("Should serialize");
        let deserialized: Point = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.latitude, -90.0);
        assert_eq!(deserialized.longitude, -180.0);

        // Test north pole and opposite date line
        let point2 = Point {
            latitude: 90.0,
            longitude: 180.0,
        };

        let json2 = serde_json::to_string(&point2).expect("Should serialize");
        let deserialized2: Point = serde_json::from_str(&json2).expect("Should deserialize");

        assert_eq!(deserialized2.latitude, 90.0);
        assert_eq!(deserialized2.longitude, 180.0);
    }
}