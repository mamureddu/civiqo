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
    use test_case::test_case;

    // Slug generation tests
    #[test_case("Hello World!", "hello-world"; "basic slug")]
    #[test_case("Test @#$ 123", "test-123"; "with special chars")]
    #[test_case("  Multiple   Spaces  ", "multiple-spaces"; "multiple spaces")]
    #[test_case("", ""; "empty string")]
    #[test_case("UPPERCASE", "uppercase"; "uppercase")]
    #[test_case("Already-A-Slug", "already-a-slug"; "existing slug")]
    #[test_case("Unicode: αβγ 中文 🚀", "unicode"; "unicode characters")]
    #[test_case("Tabs\tand\nNewlines", "tabs-and-newlines"; "whitespace chars")]
    #[test_case("dots.and.underscores_here", "dotsandunderscoreshere"; "dots and underscores")]
    #[test_case("numbers123and456letters", "numbers123and456letters"; "alphanumeric")]
    fn test_generate_slug(input: &str, expected: &str) {
        assert_eq!(generate_slug(input), expected);
    }

    #[test]
    fn test_generate_slug_very_long() {
        let long_input = "a".repeat(1000);
        let slug = generate_slug(&long_input);
        assert_eq!(slug, long_input.to_lowercase());
    }

    // Email validation tests
    #[test_case("test@example.com", true; "basic valid email")]
    #[test_case("user+tag@domain.org", true; "email with plus")]
    #[test_case("user.name@sub.domain.co.uk", true; "subdomain email")]
    #[test_case("123456@numbers.com", true; "numeric local part")]
    #[test_case("user_name@domain-name.com", true; "underscore and dash")]
    #[test_case("very.long.email.address@very-long-domain-name.example.com", true; "long email")]
    #[test_case("invalid-email", false; "no at symbol")]
    #[test_case("@example.com", false; "missing local part")]
    #[test_case("user@", false; "missing domain")]
    #[test_case("user@domain", false; "missing TLD")]
    #[test_case("user..double@domain.com", true; "double dots")]
    #[test_case("user@domain..com", true; "double dots in domain")]
    #[test_case("", false; "empty string")]
    #[test_case("spaces in@email.com", false; "spaces")]
    #[test_case("user@domain.c", false; "single char TLD")]
    fn test_validate_email(email: &str, expected: bool) {
        assert_eq!(validate_email(email), expected);
    }

    // URL validation tests
    #[test_case("https://example.com", true; "basic HTTPS URL")]
    #[test_case("http://example.com", true; "basic HTTP URL")]
    #[test_case("https://sub.domain.example.com/path?query=value#fragment", true; "complex URL")]
    #[test_case("ftp://files.example.com/file.txt", true; "FTP URL")]
    #[test_case("mailto:user@example.com", true; "mailto URL")]
    #[test_case("not-a-url", false; "invalid URL")]
    #[test_case("", false; "empty string")]
    #[test_case("://missing-scheme.com", false; "missing scheme")]
    #[test_case("https://", false; "incomplete URL")]
    fn test_validate_url(url: &str, expected: bool) {
        assert_eq!(validate_url(url), expected);
    }

    // Coordinate validation tests
    #[test_case(0.0, 0.0, true; "equator and prime meridian")]
    #[test_case(90.0, 180.0, true; "north pole, international date line")]
    #[test_case(-90.0, -180.0, true; "south pole, opposite date line")]
    #[test_case(45.5, -122.7, true; "Portland, OR coordinates")]
    #[test_case(91.0, 0.0, false; "latitude too high")]
    #[test_case(-91.0, 0.0, false; "latitude too low")]
    #[test_case(0.0, 181.0, false; "longitude too high")]
    #[test_case(0.0, -181.0, false; "longitude too low")]
    #[test_case(90.1, 0.0, false; "slightly over max latitude")]
    #[test_case(0.0, 180.1, false; "slightly over max longitude")]
    fn test_validate_coordinates(lat: f64, lon: f64, should_pass: bool) {
        let result = validate_coordinates(lat, lon);
        assert_eq!(result.is_ok(), should_pass);

        if !should_pass {
            assert!(result.is_err());
            let error = result.unwrap_err();
            match error {
                AppError::Validation(msg) => {
                    assert!(msg.contains("Latitude") || msg.contains("Longitude"));
                }
                _ => panic!("Expected validation error"),
            }
        }
    }

    // Distance calculation tests
    #[test]
    fn test_calculate_distance_known_cities() {
        // Distance from NYC to LA (approx 3935 km)
        let distance = calculate_distance(40.7128, -74.0060, 34.0522, -118.2437);
        assert!((distance - 3935.0).abs() < 50.0, "NYC to LA distance should be ~3935km, got {}", distance);

        // Distance from London to Paris (approx 344 km)
        let distance = calculate_distance(51.5074, -0.1278, 48.8566, 2.3522);
        assert!((distance - 344.0).abs() < 20.0, "London to Paris distance should be ~344km, got {}", distance);

        // Distance from Tokyo to Sydney (approx 7823 km)
        let distance = calculate_distance(35.6762, 139.6503, -33.8688, 151.2093);
        assert!((distance - 7823.0).abs() < 100.0, "Tokyo to Sydney distance should be ~7823km, got {}", distance);
    }

    #[test]
    fn test_calculate_distance_same_point() {
        let distance = calculate_distance(40.7128, -74.0060, 40.7128, -74.0060);
        assert!(distance < 0.001, "Distance to same point should be ~0, got {}", distance);
    }

    #[test]
    fn test_calculate_distance_edge_cases() {
        // Antipodal points (opposite sides of Earth)
        let distance = calculate_distance(0.0, 0.0, 0.0, 180.0);
        assert!((distance - 20003.9).abs() < 100.0, "Half Earth circumference should be ~20004km, got {}", distance);

        // North to South pole
        let distance = calculate_distance(90.0, 0.0, -90.0, 0.0);
        assert!((distance - 20003.9).abs() < 100.0, "Pole to pole distance should be ~20004km, got {}", distance);
    }

    // Text validation tests
    #[test_case("Valid text", 1, 100, true; "valid length")]
    #[test_case("", 1, 100, false; "too short")]
    #[test_case("a", 1, 100, true; "minimum length")]
    #[test_case("a".repeat(100).as_str(), 1, 100, true; "maximum length")]
    #[test_case("a".repeat(101).as_str(), 1, 100, false; "too long")]
    #[test_case("Short", 10, 100, false; "below minimum")]
    fn test_validate_text_length(text: &str, min: usize, max: usize, should_pass: bool) {
        let result = validate_text_length(text, min, max);
        assert_eq!(result.is_ok(), should_pass);

        if !should_pass {
            assert!(result.is_err());
            let error = result.unwrap_err();
            match error {
                AppError::Validation(msg) => {
                    assert!(msg.contains("characters"));
                }
                _ => panic!("Expected validation error"),
            }
        }
    }

    // Text sanitization tests
    #[test_case("  Normal text  ", "Normal text"; "trim spaces")]
    #[test_case("\t\n  Text with whitespace  \r\n", "Text with whitespace"; "trim all whitespace")]
    #[test_case("", ""; "empty string")]
    #[test_case("No changes needed", "No changes needed"; "no changes")]
    #[test_case("   ", ""; "only whitespace")]
    fn test_sanitize_text(input: &str, expected: &str) {
        assert_eq!(sanitize_text(input), expected);
    }

    // ID and timestamp generation tests
    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();

        // IDs should be different
        assert_ne!(id1, id2);

        // IDs should be valid UUIDs
        assert!(id1.get_version().is_some());
        assert!(id2.get_version().is_some());
    }

    #[test]
    fn test_now() {
        let time1 = now();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let time2 = now();

        // Time should advance
        assert!(time2 > time1);

        // Should be recent (within last minute)
        let diff = time2 - time1;
        assert!(diff.num_seconds() < 60);
    }

    // Performance tests
    #[test]
    fn test_slug_generation_performance() {
        let input = "This is a test string for performance testing with special chars!@#$%^&*()".repeat(10);

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = generate_slug(&input);
        }
        let duration = start.elapsed();

        // Should complete 1000 operations in reasonable time (increased for CI)
        assert!(duration.as_millis() < 10000, "Slug generation too slow: {:?}", duration);
    }

    #[test]
    fn test_distance_calculation_performance() {
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let lat1 = i as f64 * 0.001;
            let lon1 = i as f64 * 0.001;
            let lat2 = (i + 1) as f64 * 0.001;
            let lon2 = (i + 1) as f64 * 0.001;
            let _ = calculate_distance(lat1, lon1, lat2, lon2);
        }
        let duration = start.elapsed();

        // Should complete 1000 calculations in reasonable time
        assert!(duration.as_millis() < 100, "Distance calculation too slow: {:?}", duration);
    }

    // Edge case and robustness tests
    #[test]
    fn test_extreme_coordinates() {
        // Test with very precise coordinates
        let result = validate_coordinates(89.999999, 179.999999);
        assert!(result.is_ok());

        let result = validate_coordinates(-89.999999, -179.999999);
        assert!(result.is_ok());

        // Test with exactly boundary values
        let result = validate_coordinates(90.0, 180.0);
        assert!(result.is_ok());

        let result = validate_coordinates(-90.0, -180.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unicode_handling() {
        // Test slug generation with various Unicode
        let unicode_tests = vec![
            ("Hello 世界", "hello"),
            ("Café résumé", "caf-rsum"),
            ("🚀 Rocket", "rocket"),
            ("αβγδε ΑΒΓΔΕ", ""),
            ("Mix 123 αβγ 456", "mix-123-456"),
        ];

        for (input, expected) in unicode_tests {
            assert_eq!(generate_slug(input), expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_very_long_inputs() {
        // Test with very long strings
        let long_string = "a".repeat(10000);

        // Should not panic or take too long
        let start = std::time::Instant::now();
        let slug = generate_slug(&long_string);
        let duration = start.elapsed();

        assert_eq!(slug, long_string.to_lowercase());
        assert!(duration.as_millis() < 100, "Long string processing too slow");
    }

    #[test]
    fn test_text_validation_edge_cases() {
        // Test with zero length constraints
        let result = validate_text_length("test", 0, 10);
        assert!(result.is_ok());

        // Test with equal min/max
        let result = validate_text_length("exact", 5, 5);
        assert!(result.is_ok());

        let result = validate_text_length("toolong", 5, 5);
        assert!(result.is_err());

        // Test with very large constraints
        let result = validate_text_length("test", 0, usize::MAX);
        assert!(result.is_ok());
    }
}