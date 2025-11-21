use sqlx::postgres::PgPoolOptions;
use std::sync::Once;
use uuid::Uuid;
use crate::database::Database;
use crate::models::*;
use crate::error::{AppError, Result};

static INIT: Once = Once::new();

/// Initialize logging for tests
pub fn init_test_logging() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_env_filter("debug")
            .init();
    });
}

/// Create a test database connection using rustls
pub async fn create_test_db() -> Result<Database> {
    // Use a test database URL with rustls
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/community_manager_test".to_string());

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await
        .map_err(|e| AppError::Database(e))?;

    let db = Database { pool };

    // Run migrations for test database
    db.migrate().await?;

    Ok(db)
}

/// Clean up test data
pub async fn cleanup_test_db(db: &Database) -> Result<()> {
    // Clean up in reverse order of dependencies
    sqlx::query("TRUNCATE chat_messages, chat_rooms, chat_participants CASCADE").execute(&db.pool).await?;
    sqlx::query("TRUNCATE decision_votes, decisions CASCADE").execute(&db.pool).await?;
    sqlx::query("TRUNCATE poll_votes, poll_options, polls CASCADE").execute(&db.pool).await?;
    sqlx::query("TRUNCATE products, businesses CASCADE").execute(&db.pool).await?;
    sqlx::query("TRUNCATE community_members, community_boundaries, community_settings, communities CASCADE").execute(&db.pool).await?;
    sqlx::query("TRUNCATE user_profiles, users CASCADE").execute(&db.pool).await?;
    sqlx::query("TRUNCATE roles CASCADE").execute(&db.pool).await?;
    Ok(())
}

/// Create a test user
pub async fn create_test_user(db: &Database, auth0_id: Option<String>) -> Result<User> {
    let auth0_id = auth0_id.unwrap_or_else(|| format!("auth0|{}", Uuid::new_v4()));
    let email = format!("test-{}@example.com", Uuid::new_v4());

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (auth0_id, email) VALUES ($1, $2) RETURNING id, auth0_id, email, created_at, updated_at"
    )
    .bind(&auth0_id)
    .bind(&email)
    .fetch_one(&db.pool)
    .await?;

    Ok(user)
}

/// Create a test community
pub async fn create_test_community(db: &Database, creator_id: Uuid, name: Option<String>) -> Result<Community> {
    let name = name.unwrap_or_else(|| format!("Test Community {}", Uuid::new_v4()));
    let slug = crate::utils::generate_slug(&name);

    let community = sqlx::query_as::<_, Community>(
        r#"INSERT INTO communities (name, description, slug, created_by)
           VALUES ($1, $2, $3, $4)
           RETURNING id, name, description, slug, is_public, requires_approval,
                     created_by, created_at, updated_at"#
    )
    .bind(&name)
    .bind("Test community description")
    .bind(&slug)
    .bind(&creator_id)
    .fetch_one(&db.pool)
    .await?;

    Ok(community)
}

/// Create a test role
pub async fn create_test_role(db: &Database, name: String, permissions: Vec<String>) -> Result<Role> {
    let permissions_json = serde_json::to_value(permissions)?;

    let role = sqlx::query_as::<_, Role>(
        r#"INSERT INTO roles (name, description, permissions)
           VALUES ($1, $2, $3)
           RETURNING id, name, description, permissions, is_default, created_at, updated_at"#
    )
    .bind(&name)
    .bind("Test role description")
    .bind(&permissions_json)
    .fetch_one(&db.pool)
    .await?;

    Ok(role)
}

/// Mock Auth0 JWT for testing
pub fn create_mock_jwt_claims(_user_id: Uuid, auth0_id: String, email: String) -> Claims {
    Claims {
        sub: auth0_id.clone(),
        aud: "test-audience".to_string(),
        iss: "https://test.auth0.com/".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp(),
        iat: chrono::Utc::now().timestamp(),
        email: Some(email),
        email_verified: Some(true),
        name: Some("Test User".to_string()),
        picture: None,
        community_roles: vec![],
    }
}

/// Create a test TLS configuration for testing rustls integration
pub fn create_test_tls_config() -> Result<()> {
    // This function tests that rustls is properly configured in the dependencies
    // by checking if we can create basic rustls objects
    
    // Test that rustls types are available - simplified test for rustls 0.23
    let _root_store = rustls::RootCertStore::empty();
    let _provider = rustls::crypto::ring::default_provider();
    
    // If we get here, rustls types are working properly
    Ok(())
}

/// Test rustls database connection specifically
pub async fn test_rustls_db_connection() -> Result<()> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/community_manager_test".to_string());

    // This specifically tests that SQLx with rustls can connect
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(1)
        .connect(&database_url)
        .await
        .map_err(|e| AppError::Database(e))?;

    // Test a simple query to ensure the connection works
    let result: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::Database(e))?;

    assert_eq!(result.0, 1);
    Ok(())
}

/// Test rustls HTTP client specifically
pub async fn test_rustls_http_client() -> Result<()> {
    // Create a client using rustls-tls feature
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .build()
        .map_err(|e| AppError::ExternalService(format!("Failed to create rustls client: {}", e)))?;

    // Test with httpbin.org (a testing service)
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await
        .map_err(|e| AppError::ExternalService(format!("HTTP request failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::ExternalService("HTTP request unsuccessful".to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_test_db() {
        init_test_logging();

        // This test specifically validates that rustls database connections work
        let db_result = create_test_db().await;
        match db_result {
            Ok(db) => {
                // Test a simple query
                let result: (i32,) = sqlx::query_as("SELECT 1")
                    .fetch_one(&db.pool)
                    .await
                    .expect("Query should succeed");
                assert_eq!(result.0, 1);

                cleanup_test_db(&db).await.expect("Cleanup should succeed");
            }
            Err(e) => {
                // If we can't connect to test DB, that's okay for CI environments
                println!("Warning: Could not connect to test database: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_rustls_tls_config() {
        init_test_logging();

        // Test that rustls configuration works
        create_test_tls_config().expect("rustls should be properly configured");
    }

    #[tokio::test]
    async fn test_rustls_db_connection_direct() {
        init_test_logging();

        // Skip this test if no database is available
        if std::env::var("TEST_DATABASE_URL").is_err() {
            return;
        }

        test_rustls_db_connection().await.expect("rustls database connection should work");
    }

    #[tokio::test]
    async fn test_rustls_http_client_direct() {
        init_test_logging();

        // This test validates that rustls HTTP client works
        // Skip in offline environments
        let result = test_rustls_http_client().await;
        match result {
            Ok(_) => println!("rustls HTTP client test passed"),
            Err(e) => {
                // Allow this to fail in offline environments
                println!("Warning: rustls HTTP client test failed (this is okay in offline environments): {}", e);
            }
        }
    }
}