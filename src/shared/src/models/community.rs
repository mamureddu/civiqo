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

impl std::fmt::Display for MembershipStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MembershipStatus::Pending => write!(f, "pending"),
            MembershipStatus::Active => write!(f, "active"),
            MembershipStatus::Suspended => write!(f, "suspended"),
            MembershipStatus::Banned => write!(f, "banned"),
        }
    }
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct CommunitySearchQuery {
    pub q: Option<String>,
    pub location: Option<Point>,
    pub radius_km: Option<f64>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunitySearchResult {
    #[serde(flatten)]
    pub community: Community,
    pub distance_km: Option<f64>,
    pub member_count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use test_case::test_case;
    use rstest::*;

    #[fixture]
    fn sample_community() -> Community {
        Community {
            id: Uuid::new_v4(),
            name: "Tech Community SF".to_string(),
            description: Some("A community for tech enthusiasts in San Francisco".to_string()),
            slug: "tech-community-sf".to_string(),
            is_public: true,
            requires_approval: false,
            created_by: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[fixture]
    fn sample_community_settings() -> CommunitySettings {
        CommunitySettings {
            id: Uuid::new_v4(),
            community_id: Uuid::new_v4(),
            max_members: Some(1000),
            allow_business_listings: true,
            governance_rules: serde_json::json!({
                "voting_threshold": 0.6,
                "proposal_duration_days": 7,
                "require_verification": true
            }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[fixture]
    fn sample_polygon() -> Polygon {
        Polygon {
            coordinates: vec![
                vec![
                    Point { latitude: 37.7749, longitude: -122.4194 }, // SF
                    Point { latitude: 37.7849, longitude: -122.4094 },
                    Point { latitude: 37.7649, longitude: -122.4094 },
                    Point { latitude: 37.7749, longitude: -122.4194 }, // Close the ring
                ],
            ],
        }
    }

    // Community model tests
    #[rstest]
    fn test_community_serialization(sample_community: Community) {
        let json = serde_json::to_string(&sample_community).expect("Should serialize");
        let deserialized: Community = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, sample_community.id);
        assert_eq!(deserialized.name, sample_community.name);
        assert_eq!(deserialized.slug, sample_community.slug);
        assert_eq!(deserialized.is_public, sample_community.is_public);
        assert_eq!(deserialized.requires_approval, sample_community.requires_approval);
    }

    #[test]
    fn test_community_minimal() {
        let minimal_community = Community {
            id: Uuid::new_v4(),
            name: "Minimal Community".to_string(),
            description: None,
            slug: "minimal-community".to_string(),
            is_public: false,
            requires_approval: true,
            created_by: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&minimal_community).expect("Should serialize");
        let deserialized: Community = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, "Minimal Community");
        assert!(deserialized.description.is_none());
        assert!(!deserialized.is_public);
        assert!(deserialized.requires_approval);
    }

    // MembershipStatus enum tests
    #[test_case(MembershipStatus::Pending, "pending"; "pending status")]
    #[test_case(MembershipStatus::Active, "active"; "active status")]
    #[test_case(MembershipStatus::Suspended, "suspended"; "suspended status")]
    #[test_case(MembershipStatus::Banned, "banned"; "banned status")]
    fn test_membership_status_display(status: MembershipStatus, expected: &str) {
        assert_eq!(status.to_string(), expected);
    }

    #[test]
    fn test_membership_status_serialization() {
        let statuses = vec![
            MembershipStatus::Pending,
            MembershipStatus::Active,
            MembershipStatus::Suspended,
            MembershipStatus::Banned,
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).expect("Should serialize status");
            let deserialized: MembershipStatus = serde_json::from_str(&json).expect("Should deserialize status");
            assert_eq!(status.to_string(), deserialized.to_string());
        }
    }

    // CommunitySettings tests
    #[rstest]
    fn test_community_settings_serialization(sample_community_settings: CommunitySettings) {
        let json = serde_json::to_string(&sample_community_settings).expect("Should serialize");
        let deserialized: CommunitySettings = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, sample_community_settings.id);
        assert_eq!(deserialized.community_id, sample_community_settings.community_id);
        assert_eq!(deserialized.max_members, sample_community_settings.max_members);
        assert_eq!(deserialized.allow_business_listings, sample_community_settings.allow_business_listings);
    }

    #[test]
    fn test_community_settings_governance_rules() {
        let settings = CommunitySettings {
            id: Uuid::new_v4(),
            community_id: Uuid::new_v4(),
            max_members: None,
            allow_business_listings: false,
            governance_rules: serde_json::json!({
                "custom_rule": "value",
                "another_rule": {
                    "nested": true,
                    "values": [1, 2, 3]
                }
            }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&settings).expect("Should serialize");
        let deserialized: CommunitySettings = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.governance_rules["custom_rule"], "value");
        assert_eq!(deserialized.governance_rules["another_rule"]["nested"], true);
    }

    // CommunityBoundary tests
    #[test]
    fn test_community_boundary_serialization() {
        let boundary = CommunityBoundary {
            id: Uuid::new_v4(),
            community_id: Uuid::new_v4(),
            name: "Downtown Area".to_string(),
            boundary_data: serde_json::json!({
                "type": "Polygon",
                "coordinates": [[[
                    [-122.4194, 37.7749],
                    [-122.4094, 37.7849],
                    [-122.4094, 37.7649],
                    [-122.4194, 37.7749]
                ]]]
            }),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&boundary).expect("Should serialize");
        let deserialized: CommunityBoundary = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, boundary.id);
        assert_eq!(deserialized.name, boundary.name);
        assert_eq!(deserialized.boundary_data["type"], "Polygon");
    }

    // CommunityMember tests
    #[test]
    fn test_community_member_serialization() {
        let member = CommunityMember {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            community_id: Uuid::new_v4(),
            role_id: Uuid::new_v4(),
            status: MembershipStatus::Active,
            joined_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&member).expect("Should serialize");
        let deserialized: CommunityMember = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, member.id);
        assert_eq!(deserialized.user_id, member.user_id);
        assert_eq!(deserialized.status.to_string(), member.status.to_string());
        assert!(deserialized.joined_at.is_some());
    }

    #[test]
    fn test_community_member_pending() {
        let pending_member = CommunityMember {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            community_id: Uuid::new_v4(),
            role_id: Uuid::new_v4(),
            status: MembershipStatus::Pending,
            joined_at: None, // Pending members haven't joined yet
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&pending_member).expect("Should serialize");
        let deserialized: CommunityMember = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.status.to_string(), "pending");
        assert!(deserialized.joined_at.is_none());
    }

    // Role tests
    #[test]
    fn test_role_serialization() {
        let role = Role {
            id: Uuid::new_v4(),
            name: "Moderator".to_string(),
            description: Some("Can moderate discussions and ban users".to_string()),
            permissions: serde_json::json!(["read", "write", "moderate", "ban_users"]),
            is_default: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&role).expect("Should serialize");
        let deserialized: Role = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, role.id);
        assert_eq!(deserialized.name, role.name);
        assert_eq!(deserialized.permissions, role.permissions);
        assert!(!deserialized.is_default);
    }

    #[test]
    fn test_role_default() {
        let default_role = Role {
            id: Uuid::new_v4(),
            name: "Member".to_string(),
            description: Some("Default member role".to_string()),
            permissions: serde_json::json!(["read", "write"]),
            is_default: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&default_role).expect("Should serialize");
        let deserialized: Role = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, "Member");
        assert!(deserialized.is_default);
        assert_eq!(deserialized.permissions.as_array().unwrap().len(), 2);
    }

    // Request models tests
    #[test]
    fn test_create_community_request() {
        let polygon = Polygon {
            coordinates: vec![
                vec![
                    Point { latitude: 37.7749, longitude: -122.4194 },
                    Point { latitude: 37.7849, longitude: -122.4094 },
                    Point { latitude: 37.7649, longitude: -122.4094 },
                    Point { latitude: 37.7749, longitude: -122.4194 },
                ],
            ],
        };

        let request = CreateCommunityRequest {
            name: "New Tech Community".to_string(),
            description: Some("A community for tech enthusiasts".to_string()),
            slug: "new-tech-community".to_string(),
            is_public: true,
            requires_approval: false,
            boundary: Some(polygon),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: CreateCommunityRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, request.name);
        assert_eq!(deserialized.slug, request.slug);
        assert!(deserialized.boundary.is_some());
        assert!(deserialized.is_public);
    }

    #[test]
    fn test_create_community_request_minimal() {
        let request = CreateCommunityRequest {
            name: "Minimal Community".to_string(),
            description: None,
            slug: "minimal".to_string(),
            is_public: false,
            requires_approval: true,
            boundary: None,
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: CreateCommunityRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, "Minimal Community");
        assert!(deserialized.description.is_none());
        assert!(deserialized.boundary.is_none());
        assert!(!deserialized.is_public);
        assert!(deserialized.requires_approval);
    }

    #[test]
    fn test_update_community_request() {
        let request = UpdateCommunityRequest {
            name: Some("Updated Name".to_string()),
            description: Some("Updated description".to_string()),
            is_public: Some(false),
            requires_approval: Some(true),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: UpdateCommunityRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, Some("Updated Name".to_string()));
        assert_eq!(deserialized.is_public, Some(false));
        assert_eq!(deserialized.requires_approval, Some(true));
    }

    #[test]
    fn test_update_community_request_partial() {
        let request = UpdateCommunityRequest {
            name: Some("Only name updated".to_string()),
            description: None,
            is_public: None,
            requires_approval: None,
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: UpdateCommunityRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, Some("Only name updated".to_string()));
        assert!(deserialized.description.is_none());
        assert!(deserialized.is_public.is_none());
        assert!(deserialized.requires_approval.is_none());
    }

    #[test]
    fn test_join_community_request() {
        let request = JoinCommunityRequest {
            message: Some("I would like to join this community because...".to_string()),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: JoinCommunityRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.message, request.message);
    }

    #[test]
    fn test_join_community_request_no_message() {
        let request = JoinCommunityRequest { message: None };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: JoinCommunityRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert!(deserialized.message.is_none());
    }

    // Complex model tests
    #[test]
    fn test_community_with_stats_serialization() {
        let community_with_stats = CommunityWithStats {
            id: Uuid::new_v4(),
            name: "Tech Community".to_string(),
            description: Some("Description here".to_string()),
            slug: "tech-community".to_string(),
            is_public: true,
            requires_approval: false,
            created_by: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            member_count: 150,
            business_count: 25,
            is_member: true,
            user_role: Some("admin".to_string()),
        };

        let json = serde_json::to_string(&community_with_stats).expect("Should serialize");
        let deserialized: CommunityWithStats = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, "Tech Community");
        assert_eq!(deserialized.member_count, 150);
        assert_eq!(deserialized.business_count, 25);
        assert!(deserialized.is_member);
        assert_eq!(deserialized.user_role, Some("admin".to_string()));
    }

    #[test]
    fn test_member_with_profile_serialization() {
        let member_with_profile = MemberWithProfile {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            community_id: Uuid::new_v4(),
            role_id: Uuid::new_v4(),
            status: MembershipStatus::Active,
            joined_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_name: Some("John Doe".to_string()),
            user_avatar: Some("https://example.com/avatar.jpg".to_string()),
            role_name: "Moderator".to_string(),
        };

        let json = serde_json::to_string(&member_with_profile).expect("Should serialize");
        let deserialized: MemberWithProfile = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.user_name, Some("John Doe".to_string()));
        assert_eq!(deserialized.role_name, "Moderator");
        assert_eq!(deserialized.status.to_string(), "active");
    }

    // Search and discovery tests
    #[test]
    fn test_community_search_query_serialization() {
        let search_query = CommunitySearchQuery {
            q: Some("tech community".to_string()),
            location: Some(Point { latitude: 37.7749, longitude: -122.4194 }),
            radius_km: Some(10.0),
            is_public: Some(true),
        };

        let json = serde_json::to_string(&search_query).expect("Should serialize");
        let deserialized: CommunitySearchQuery = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.q, Some("tech community".to_string()));
        assert!(deserialized.location.is_some());
        assert_eq!(deserialized.radius_km, Some(10.0));
        assert_eq!(deserialized.is_public, Some(true));
    }

    #[test]
    fn test_community_search_query_minimal() {
        let search_query = CommunitySearchQuery {
            q: None,
            location: None,
            radius_km: None,
            is_public: None,
        };

        let json = serde_json::to_string(&search_query).expect("Should serialize");
        let deserialized: CommunitySearchQuery = serde_json::from_str(&json).expect("Should deserialize");

        assert!(deserialized.q.is_none());
        assert!(deserialized.location.is_none());
        assert!(deserialized.radius_km.is_none());
        assert!(deserialized.is_public.is_none());
    }

    #[test]
    fn test_community_search_result_serialization() {
        let community = Community {
            id: Uuid::new_v4(),
            name: "Nearby Tech Community".to_string(),
            description: Some("A local tech community".to_string()),
            slug: "nearby-tech".to_string(),
            is_public: true,
            requires_approval: false,
            created_by: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let search_result = CommunitySearchResult {
            community,
            distance_km: Some(5.2),
            member_count: 89,
        };

        let json = serde_json::to_string(&search_result).expect("Should serialize");
        let deserialized: CommunitySearchResult = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.community.name, "Nearby Tech Community");
        assert_eq!(deserialized.distance_km, Some(5.2));
        assert_eq!(deserialized.member_count, 89);
    }

    // Edge cases and validation tests
    #[test]
    fn test_community_slug_patterns() {
        let test_cases = vec![
            ("tech-community-sf", true),
            ("tech_community_sf", true),
            ("tech123", true),
            ("123tech", true),
            ("tech-community-with-very-long-name", true),
            ("", false), // Should be validated at application layer
            ("UPPERCASE", true), // Valid but should be normalized
        ];

        for (slug, _should_be_valid) in test_cases {
            let community = Community {
                id: Uuid::new_v4(),
                name: "Test Community".to_string(),
                description: None,
                slug: slug.to_string(),
                is_public: true,
                requires_approval: false,
                created_by: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            // Should always serialize/deserialize (validation is at app layer)
            let json = serde_json::to_string(&community).expect("Should serialize");
            let _: Community = serde_json::from_str(&json).expect("Should deserialize");
        }
    }

    #[test]
    fn test_permissions_json_patterns() {
        let permission_patterns = vec![
            serde_json::json!(["read", "write"]),
            serde_json::json!(["all"]),
            serde_json::json!([]),
            serde_json::json!(["read", "write", "moderate", "admin", "ban_users"]),
            serde_json::json!(["custom_permission_1", "custom_permission_2"]),
        ];

        for permissions in permission_patterns {
            let role = Role {
                id: Uuid::new_v4(),
                name: "Test Role".to_string(),
                description: Some("Test role".to_string()),
                permissions: permissions.clone(),
                is_default: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            let json = serde_json::to_string(&role).expect("Should serialize");
            let deserialized: Role = serde_json::from_str(&json).expect("Should deserialize");

            assert_eq!(deserialized.permissions, permissions);
        }
    }

    #[test]
    fn test_community_name_unicode() {
        let unicode_names = vec![
            "Comunidad de Tecnología",
            "コミュニティ",
            "المجتمع التقني",
            "Сообщество разработчиков",
            "🚀 Tech Community 🚀",
            "Community with émojís and açcents",
        ];

        for name in unicode_names {
            let community = Community {
                id: Uuid::new_v4(),
                name: name.to_string(),
                description: Some("Unicode test community".to_string()),
                slug: "unicode-test".to_string(),
                is_public: true,
                requires_approval: false,
                created_by: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            let json = serde_json::to_string(&community).expect("Should serialize Unicode names");
            let deserialized: Community = serde_json::from_str(&json).expect("Should deserialize Unicode names");

            assert_eq!(deserialized.name, name);
        }
    }

    // Performance tests
    #[test]
    fn test_large_governance_rules() {
        let mut large_rules = serde_json::Map::new();
        for i in 0..100 {
            large_rules.insert(
                format!("rule_{}", i),
                serde_json::json!({
                    "value": i,
                    "description": format!("Rule number {}", i),
                    "enabled": i % 2 == 0
                })
            );
        }

        let settings = CommunitySettings {
            id: Uuid::new_v4(),
            community_id: Uuid::new_v4(),
            max_members: Some(5000),
            allow_business_listings: true,
            governance_rules: serde_json::Value::Object(large_rules),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let start = std::time::Instant::now();
        let json = serde_json::to_string(&settings).expect("Should serialize large rules");
        let _: CommunitySettings = serde_json::from_str(&json).expect("Should deserialize large rules");
        let duration = start.elapsed();

        // Should handle large governance rules efficiently
        assert!(duration.as_millis() < 100, "Large rules serialization too slow: {:?}", duration);
    }

    #[test]
    fn test_complex_boundary_data() {
        // Test with a complex GeoJSON polygon with holes
        let complex_boundary = serde_json::json!({
            "type": "Polygon",
            "coordinates": [
                // Exterior ring
                [
                    [-122.4194, 37.7749],
                    [-122.4094, 37.7849],
                    [-122.4094, 37.7649],
                    [-122.4294, 37.7649],
                    [-122.4194, 37.7749]
                ],
                // Interior hole
                [
                    [-122.4144, 37.7699],
                    [-122.4174, 37.7699],
                    [-122.4174, 37.7729],
                    [-122.4144, 37.7729],
                    [-122.4144, 37.7699]
                ]
            ]
        });

        let boundary = CommunityBoundary {
            id: Uuid::new_v4(),
            community_id: Uuid::new_v4(),
            name: "Complex Boundary".to_string(),
            boundary_data: complex_boundary.clone(),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&boundary).expect("Should serialize complex boundary");
        let deserialized: CommunityBoundary = serde_json::from_str(&json).expect("Should deserialize complex boundary");

        assert_eq!(deserialized.boundary_data, complex_boundary);
        assert_eq!(deserialized.name, "Complex Boundary");
    }
}