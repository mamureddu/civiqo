use chrono::{DateTime, Utc};
use uuid::Uuid;
use regex::Regex;
use crate::error::{AppError, Result};

// Generate a URL-friendly slug from a string
pub fn generate_slug(input: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9\s-]").unwrap();
    let cleaned = re.replace_all(input, "");

    cleaned
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

// Validate email format
pub fn validate_email(email: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}

// Validate URL format
pub fn validate_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

// Generate a unique ID
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

// Get current UTC timestamp
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

// Validate geographic coordinates
pub fn validate_coordinates(lat: f64, lon: f64) -> Result<()> {
    if lat < -90.0 || lat > 90.0 {
        return Err(AppError::Validation(
            "Latitude must be between -90 and 90".to_string()
        ));
    }

    if lon < -180.0 || lon > 180.0 {
        return Err(AppError::Validation(
            "Longitude must be between -180 and 180".to_string()
        ));
    }

    Ok(())
}

// Calculate distance between two points in kilometers
pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // Earth's radius in kilometers

    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2) +
        lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}

// Sanitize text input
pub fn sanitize_text(input: &str) -> String {
    input.trim().to_string()
}

// Validate text length
pub fn validate_text_length(text: &str, min: usize, max: usize) -> Result<()> {
    let length = text.len();
    if length < min {
        return Err(AppError::Validation(
            format!("Text must be at least {} characters", min)
        ));
    }
    if length > max {
        return Err(AppError::Validation(
            format!("Text must be no more than {} characters", max)
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hello World!"), "hello-world");
        assert_eq!(generate_slug("Test @#$ 123"), "test-123");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(validate_email("user+tag@domain.org"));
        assert!(!validate_email("invalid-email"));
        assert!(!validate_email("@example.com"));
    }

    #[test]
    fn test_validate_coordinates() {
        assert!(validate_coordinates(0.0, 0.0).is_ok());
        assert!(validate_coordinates(90.0, 180.0).is_ok());
        assert!(validate_coordinates(-90.0, -180.0).is_ok());
        assert!(validate_coordinates(91.0, 0.0).is_err());
        assert!(validate_coordinates(0.0, 181.0).is_err());
    }

    #[test]
    fn test_calculate_distance() {
        // Distance from NYC to LA (approx 3935 km)
        let distance = calculate_distance(40.7128, -74.0060, 34.0522, -118.2437);
        assert!((distance - 3935.0).abs() < 50.0); // Allow 50km margin
    }
}