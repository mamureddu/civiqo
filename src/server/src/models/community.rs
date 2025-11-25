use serde::{Deserialize, Serialize};
use validator::Validate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SLUG_REGEX: Regex = Regex::new(r"^[a-z0-9-]+$").unwrap();
}

/// Request to create a new community
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateCommunityRequest {
    /// Community name (3-100 characters)
    #[validate(length(min = 3, max = 100))]
    pub name: String,

    /// Community description (optional, max 1000 characters)
    #[validate(length(max = 1000))]
    pub description: Option<String>,

    /// URL-friendly slug (3-50 characters, lowercase, alphanumeric + hyphens)
    #[validate(length(min = 3, max = 50), regex = "SLUG_REGEX")]
    pub slug: String,

    /// Whether community is public (optional, defaults to true)
    pub is_public: Option<bool>,
}

/// Request to update an existing community
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateCommunityRequest {
    /// Community name (3-100 characters, optional)
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,

    /// Community description (optional, max 1000 characters)
    #[validate(length(max = 1000))]
    pub description: Option<String>,

    /// Whether community is public (optional)
    pub is_public: Option<bool>,
}

/// Response containing community data
#[derive(Debug, Clone, Serialize)]
pub struct CommunityResponse {
    /// Community ID (BIGINT)
    pub id: i64,

    /// Community name
    pub name: String,

    /// Community description
    pub description: Option<String>,

    /// URL-friendly slug
    pub slug: String,

    /// Whether community is public
    pub is_public: bool,

    /// User ID of community creator (UUID)
    pub created_by: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_community_valid() {
        let req = CreateCommunityRequest {
            name: "Test Community".to_string(),
            description: Some("A test community".to_string()),
            slug: "test-community".to_string(),
            is_public: Some(true),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_create_community_name_too_short() {
        let req = CreateCommunityRequest {
            name: "ab".to_string(),
            description: None,
            slug: "test".to_string(),
            is_public: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_create_community_slug_invalid_chars() {
        let req = CreateCommunityRequest {
            name: "Test Community".to_string(),
            description: None,
            slug: "Test_Community".to_string(), // uppercase and underscore
            is_public: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_update_community_valid() {
        let req = UpdateCommunityRequest {
            name: Some("Updated Name".to_string()),
            description: None,
            is_public: None,
        };
        assert!(req.validate().is_ok());
    }
}
