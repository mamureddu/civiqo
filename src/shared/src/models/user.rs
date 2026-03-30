use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
    pub provider: String,              // "local", "google", "github", etc.
    pub provider_id: Option<String>,   // NULL for local users, provider-specific ID for SSO
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserProfileRequest {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserWithProfile {
    pub id: Uuid,
    pub email: String,
    pub provider: String,
    pub provider_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Profile fields (optional)
    pub profile_id: Option<Uuid>,
    pub profile_name: Option<String>,
    pub profile_picture: Option<String>,
    pub profile_bio: Option<String>,
    pub profile_location: Option<String>,
    pub profile_website: Option<String>,
    pub profile_created_at: Option<DateTime<Utc>>,
    pub profile_updated_at: Option<DateTime<Utc>>,
}

// JWT Claims (self-issued, SSO-ready)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // User UUID
    pub aud: String,    // "civiqo-api"
    pub iss: String,    // "civiqo" (or configured issuer)
    pub exp: i64,
    pub iat: i64,
    pub email: Option<String>,
    pub name: Option<String>,
    #[serde(default)]
    pub community_roles: Vec<CommunityRole>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityRole {
    pub community_id: Uuid,
    pub role: String,
    pub permissions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use test_case::test_case;
    use rstest::*;

    #[fixture]
    fn sample_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: Some("$argon2id$v=19$m=19456,t=2,p=1$test".to_string()),
            provider: "local".to_string(),
            provider_id: None,
            email_verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[fixture]
    fn sample_user_profile() -> UserProfile {
        UserProfile {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: Some("John Doe".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            bio: Some("Software engineer and community builder".to_string()),
            location: Some("San Francisco, CA".to_string()),
            website: Some("https://johndoe.dev".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[fixture]
    fn sample_claims() -> Claims {
        Claims {
            sub: Uuid::new_v4().to_string(),
            aud: "civiqo-api".to_string(),
            iss: "civiqo".to_string(),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            email: Some("test@example.com".to_string()),
            name: Some("Test User".to_string()),
            community_roles: vec![
                CommunityRole {
                    community_id: Uuid::new_v4(),
                    role: "admin".to_string(),
                    permissions: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
                },
                CommunityRole {
                    community_id: Uuid::new_v4(),
                    role: "member".to_string(),
                    permissions: vec!["read".to_string()],
                }
            ],
        }
    }

    // User model tests
    #[rstest]
    fn test_user_serialization(sample_user: User) {
        let json = serde_json::to_string(&sample_user).expect("Should serialize");
        let deserialized: User = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, sample_user.id);
        assert_eq!(deserialized.email, sample_user.email);
        assert_eq!(deserialized.provider, sample_user.provider);
    }

    #[test]
    fn test_user_with_profile_serialization() {
        let now = Utc::now();
        let user_with_profile = UserWithProfile {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            provider: "local".to_string(),
            provider_id: None,
            created_at: now,
            updated_at: now,
            profile_id: Some(Uuid::new_v4()),
            profile_name: Some("John Doe".to_string()),
            profile_picture: Some("https://example.com/avatar.jpg".to_string()),
            profile_bio: Some("Bio here".to_string()),
            profile_location: Some("NYC".to_string()),
            profile_website: Some("https://example.com".to_string()),
            profile_created_at: Some(now),
            profile_updated_at: Some(now),
        };

        let json = serde_json::to_string(&user_with_profile).expect("Should serialize");
        let deserialized: UserWithProfile = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, user_with_profile.id);
        assert_eq!(deserialized.profile_name, user_with_profile.profile_name);
        assert_eq!(deserialized.profile_website, user_with_profile.profile_website);
    }

    #[test]
    fn test_user_with_profile_no_profile() {
        let now = Utc::now();
        let user_without_profile = UserWithProfile {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            provider: "local".to_string(),
            provider_id: None,
            created_at: now,
            updated_at: now,
            profile_id: None,
            profile_name: None,
            profile_picture: None,
            profile_bio: None,
            profile_location: None,
            profile_website: None,
            profile_created_at: None,
            profile_updated_at: None,
        };

        let json = serde_json::to_string(&user_without_profile).expect("Should serialize");
        let deserialized: UserWithProfile = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, user_without_profile.id);
        assert!(deserialized.profile_id.is_none());
        assert!(deserialized.profile_name.is_none());
    }

    // UserProfile model tests
    #[rstest]
    fn test_user_profile_serialization(sample_user_profile: UserProfile) {
        let json = serde_json::to_string(&sample_user_profile).expect("Should serialize");
        let deserialized: UserProfile = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, sample_user_profile.id);
        assert_eq!(deserialized.user_id, sample_user_profile.user_id);
        assert_eq!(deserialized.name, sample_user_profile.name);
        assert_eq!(deserialized.bio, sample_user_profile.bio);
    }

    #[test]
    fn test_user_profile_minimal() {
        let minimal_profile = UserProfile {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: None,
            avatar_url: None,
            bio: None,
            location: None,
            website: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&minimal_profile).expect("Should serialize");
        let deserialized: UserProfile = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.id, minimal_profile.id);
        assert!(deserialized.name.is_none());
        assert!(deserialized.avatar_url.is_none());
        assert!(deserialized.bio.is_none());
    }

    // Request models tests
    #[test]
    fn test_create_user_request() {
        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "securepassword123".to_string(),
            name: Some("Test User".to_string()),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: CreateUserRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.email, request.email);
        assert_eq!(deserialized.name, request.name);
    }

    #[test]
    fn test_create_user_request_no_name() {
        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "securepassword123".to_string(),
            name: None,
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: CreateUserRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.email, request.email);
        assert!(deserialized.name.is_none());
    }

    #[test]
    fn test_update_user_profile_request() {
        let request = UpdateUserProfileRequest {
            name: Some("Updated Name".to_string()),
            bio: Some("Updated bio".to_string()),
            location: Some("New York".to_string()),
            website: Some("https://newsite.com".to_string()),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: UpdateUserProfileRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, request.name);
        assert_eq!(deserialized.bio, request.bio);
        assert_eq!(deserialized.location, request.location);
        assert_eq!(deserialized.website, request.website);
    }

    #[test]
    fn test_update_user_profile_request_partial() {
        let request = UpdateUserProfileRequest {
            name: Some("Only name updated".to_string()),
            bio: None,
            location: None,
            website: None,
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: UpdateUserProfileRequest = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, request.name);
        assert!(deserialized.bio.is_none());
        assert!(deserialized.location.is_none());
        assert!(deserialized.website.is_none());
    }

    // Claims and JWT tests
    #[rstest]
    fn test_claims_serialization(sample_claims: Claims) {
        let json = serde_json::to_string(&sample_claims).expect("Should serialize");
        let deserialized: Claims = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.sub, sample_claims.sub);
        assert_eq!(deserialized.aud, sample_claims.aud);
        assert_eq!(deserialized.iss, sample_claims.iss);
        assert_eq!(deserialized.email, sample_claims.email);
        assert_eq!(deserialized.community_roles.len(), sample_claims.community_roles.len());
    }

    #[test]
    fn test_claims_with_minimal_fields() {
        let minimal_claims = Claims {
            sub: Uuid::new_v4().to_string(),
            aud: "civiqo-api".to_string(),
            iss: "civiqo".to_string(),
            exp: Utc::now().timestamp(),
            iat: Utc::now().timestamp(),
            email: None,
            name: None,
            community_roles: vec![],
        };

        let json = serde_json::to_string(&minimal_claims).expect("Should serialize");
        let deserialized: Claims = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.sub, minimal_claims.sub);
        assert!(deserialized.email.is_none());
        assert!(deserialized.name.is_none());
        assert!(deserialized.community_roles.is_empty());
    }

    #[test]
    fn test_claims_timestamps() {
        let now = Utc::now();
        let claims = Claims {
            sub: Uuid::new_v4().to_string(),
            aud: "civiqo-api".to_string(),
            iss: "civiqo".to_string(),
            exp: (now + chrono::Duration::hours(1)).timestamp(),
            iat: now.timestamp(),
            email: Some("test@example.com".to_string()),
            name: None,
            community_roles: vec![],
        };

        // Test that exp is in the future
        assert!(claims.exp > claims.iat);
        assert!(claims.exp > now.timestamp());

        // Test that iat is reasonable (within last minute)
        let current_timestamp = Utc::now().timestamp();
        assert!((current_timestamp - claims.iat).abs() < 60);
    }

    // CommunityRole tests
    #[test]
    fn test_community_role_serialization() {
        let role = CommunityRole {
            community_id: Uuid::new_v4(),
            role: "moderator".to_string(),
            permissions: vec![
                "read".to_string(),
                "write".to_string(),
                "moderate".to_string(),
                "ban_users".to_string(),
            ],
        };

        let json = serde_json::to_string(&role).expect("Should serialize");
        let deserialized: CommunityRole = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.community_id, role.community_id);
        assert_eq!(deserialized.role, role.role);
        assert_eq!(deserialized.permissions, role.permissions);
    }

    #[test]
    fn test_community_role_empty_permissions() {
        let role = CommunityRole {
            community_id: Uuid::new_v4(),
            role: "guest".to_string(),
            permissions: vec![],
        };

        let json = serde_json::to_string(&role).expect("Should serialize");
        let deserialized: CommunityRole = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.role, "guest");
        assert!(deserialized.permissions.is_empty());
    }

    #[test_case("admin", vec!["all"]; "admin role")]
    #[test_case("moderator", vec!["read", "write", "moderate"]; "moderator role")]
    #[test_case("member", vec!["read", "write"]; "member role")]
    #[test_case("guest", vec!["read"]; "guest role")]
    #[test_case("banned", vec![]; "banned role")]
    fn test_community_role_permission_patterns(role_name: &str, permissions: Vec<&str>) {
        let role = CommunityRole {
            community_id: Uuid::new_v4(),
            role: role_name.to_string(),
            permissions: permissions.iter().map(|s| s.to_string()).collect(),
        };

        let json = serde_json::to_string(&role).expect("Should serialize");
        let deserialized: CommunityRole = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.role, role_name);
        assert_eq!(deserialized.permissions.len(), permissions.len());

        for permission in permissions {
            assert!(deserialized.permissions.contains(&permission.to_string()));
        }
    }

    // Edge cases and validation tests
    #[test]
    fn test_user_id_uniqueness() {
        let user1 = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: Some("hash1".to_string()),
            provider: "local".to_string(),
            provider_id: None,
            email_verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let user2 = User {
            id: Uuid::new_v4(),
            email: "different@example.com".to_string(),
            password_hash: Some("hash2".to_string()),
            provider: "local".to_string(),
            provider_id: None,
            email_verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_ne!(user1.id, user2.id);
        assert_ne!(user1.email, user2.email);
    }

    #[test]
    fn test_claims_deserialization_from_jwt_payload() {
        let jwt_payload = r#"{
            "sub": "550e8400-e29b-41d4-a716-446655440000",
            "aud": "civiqo-api",
            "iss": "civiqo",
            "exp": 1234567890,
            "iat": 1234564290,
            "email": "user@example.com",
            "name": "John Doe",
            "community_roles": [
                {
                    "community_id": "550e8400-e29b-41d4-a716-446655440000",
                    "role": "admin",
                    "permissions": ["read", "write", "admin"]
                }
            ]
        }"#;

        let claims: Claims = serde_json::from_str(jwt_payload).expect("Should parse JWT payload");

        assert_eq!(claims.sub, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(claims.aud, "civiqo-api");
        assert_eq!(claims.iss, "civiqo");
        assert_eq!(claims.community_roles.len(), 1);
        assert_eq!(claims.community_roles[0].role, "admin");
    }

    #[test]
    fn test_user_profile_url_validation_patterns() {
        let valid_urls = vec![
            "https://example.com",
            "http://subdomain.example.org/path",
            "https://github.com/username",
            "https://linkedin.com/in/profile",
        ];

        let _invalid_urls = vec![
            "not-a-url",
            "ftp://example.com", // might be valid URL but not for profiles
            "",
            "javascript:alert('xss')",
        ];

        for url in valid_urls {
            let profile = UserProfile {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                name: Some("Test".to_string()),
                avatar_url: Some(url.to_string()),
                bio: None,
                location: None,
                website: Some(url.to_string()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            // Should serialize/deserialize without issues
            let json = serde_json::to_string(&profile).expect("Should serialize valid URLs");
            let _: UserProfile = serde_json::from_str(&json).expect("Should deserialize valid URLs");
        }

        // Note: URL validation should be done at the application layer, not model layer
        // These are just testing that the model can handle various URL formats
    }

    // Performance and edge case tests
    #[test]
    fn test_large_community_roles_list() {
        let mut large_roles = Vec::new();
        for i in 0..100 {
            large_roles.push(CommunityRole {
                community_id: Uuid::new_v4(),
                role: format!("role_{}", i),
                permissions: vec!["read".to_string(), "write".to_string()],
            });
        }

        let claims = Claims {
            sub: Uuid::new_v4().to_string(),
            aud: "civiqo-api".to_string(),
            iss: "civiqo".to_string(),
            exp: Utc::now().timestamp(),
            iat: Utc::now().timestamp(),
            email: Some("test@example.com".to_string()),
            name: Some("Test User".to_string()),
            community_roles: large_roles,
        };

        let start = std::time::Instant::now();
        let json = serde_json::to_string(&claims).expect("Should serialize large roles");
        let _: Claims = serde_json::from_str(&json).expect("Should deserialize large roles");
        let duration = start.elapsed();

        // Should handle large lists efficiently
        assert!(duration.as_millis() < 100, "Large serialization too slow: {:?}", duration);
        assert_eq!(claims.community_roles.len(), 100);
    }

    #[test]
    fn test_unicode_and_special_characters() {
        let profile = UserProfile {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: Some("José María García-López 🚀".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            bio: Some("Software engineer from España 🇪🇸. Loves Rust 🦀 and Go ⚡".to_string()),
            location: Some("São Paulo, Brasil 🇧🇷".to_string()),
            website: Some("https://josé-maría.dev".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&profile).expect("Should handle Unicode");
        let deserialized: UserProfile = serde_json::from_str(&json).expect("Should deserialize Unicode");

        assert_eq!(deserialized.name, profile.name);
        assert_eq!(deserialized.bio, profile.bio);
        assert_eq!(deserialized.location, profile.location);
    }
}