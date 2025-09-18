use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    Router,
};
use axum_test::TestServer;
use serial_test::serial;
use serde_json;
use shared::{
    database::Database,
    testing::{init_test_logging, create_test_db, cleanup_test_db, create_test_user, create_test_community},
    models::{
        ApiResponse, Claims, CreateCommunityRequest,
    },
    auth::{Auth0Config, JwtValidator},
    error::AppError,
};
use uuid::Uuid;
use wiremock::{
    matchers::{method, path, header as header_matcher},
    Mock, MockServer, ResponseTemplate,
};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use chrono::Utc;
use std::{sync::Arc, time::{Duration, Instant}};
use tokio::time::sleep;

// Import the actual API Gateway app
use api_gateway::{AppState, create_app};

/// Test configuration for performance and load tests
struct PerformanceTestContext {
    server: TestServer,
    db: Database,
    mock_auth0: MockServer,
    auth_config: Auth0Config,
}

impl PerformanceTestContext {
    async fn new() -> Self {
        init_test_logging();

        // Create test database
        let db = create_test_db().await.expect("Failed to create test database");

        // Setup mock Auth0 server
        let mock_auth0 = MockServer::start().await;
        let auth_config = Auth0Config {
            domain: mock_auth0.uri().trim_start_matches("http://").to_string(),
            audience: "test-audience".to_string(),
            client_id: "test-client-id".to_string(),
            client_secret: "test-client-secret".to_string(),
        };

        // Create app state
        let app_state = AppState {
            db: db.clone(),
            auth_config: auth_config.clone(),
        };

        // Create the router
        let app = create_app(app_state);
        let server = TestServer::new(app).unwrap();

        Self {
            server,
            db,
            mock_auth0,
            auth_config,
        }
    }

    async fn cleanup(&self) {
        cleanup_test_db(&self.db).await.expect("Failed to cleanup test database");
    }

    /// Create a valid JWT token for testing
    fn create_test_jwt(&self, claims: &Claims) -> String {
        let header = Header::new(Algorithm::HS256);
        let secret = "test-secret";
        encode(&header, claims, &EncodingKey::from_secret(secret.as_ref()))
            .expect("Failed to create test JWT")
    }

    /// Setup JWKS mock for token validation
    async fn setup_jwks_mock(&self) {
        let jwks_response = serde_json::json!({
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "test-key-id",
                    "use": "sig",
                    "n": "test-n-value",
                    "e": "AQAB",
                    "x5c": ["LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t"]
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/.well-known/jwks.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&jwks_response))
            .mount(&self.mock_auth0)
            .await;
    }

    /// Create authenticated user for testing
    async fn create_authenticated_user(&self) -> (shared::models::User, String, Uuid) {
        let user = create_test_user(&self.db, None).await.expect("Failed to create test user");
        let community = create_test_community(&self.db, user.id, Some("Test Community".to_string()))
            .await.expect("Failed to create test community");

        let claims = Claims {
            sub: user.auth0_id.clone(),
            aud: self.auth_config.audience.clone(),
            iss: format!("https://{}/", self.auth_config.domain),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
            iat: Utc::now().timestamp(),
            email: Some(user.email.clone()),
            email_verified: Some(true),
            name: Some("Test User".to_string()),
            picture: None,
            community_roles: vec![],
        };

        let token = self.create_test_jwt(&claims);
        (user, token, community.id)
    }
}

// Response Time Tests
#[tokio::test]
#[serial]
async fn test_health_check_response_time() {
    let ctx = PerformanceTestContext::new().await;

    let mut response_times = Vec::new();

    // Measure response time for multiple health checks
    for _ in 0..50 {
        let start_time = Instant::now();

        let response = ctx.server.get("/health").await;
        response.assert_status_ok();

        let response_time = start_time.elapsed();
        response_times.push(response_time);
    }

    // Calculate statistics
    let total_time: Duration = response_times.iter().sum();
    let avg_time = total_time / response_times.len() as u32;
    let mut sorted_times = response_times.clone();
    sorted_times.sort();
    let median_time = sorted_times[sorted_times.len() / 2];
    let min_time = *sorted_times.first().unwrap();
    let max_time = *sorted_times.last().unwrap();

    println!("Health Check Performance:");
    println!("  Average: {:?}", avg_time);
    println!("  Median:  {:?}", median_time);
    println!("  Min:     {:?}", min_time);
    println!("  Max:     {:?}", max_time);

    // Assert reasonable response times
    assert!(avg_time < Duration::from_millis(100), "Average response time should be under 100ms");
    assert!(max_time < Duration::from_millis(500), "Maximum response time should be under 500ms");
    assert!(median_time < Duration::from_millis(50), "Median response time should be under 50ms");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_authenticated_endpoint_response_time() {
    let ctx = PerformanceTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token, community_id) = ctx.create_authenticated_user().await;
    let mut response_times = Vec::new();

    // Measure response time for authenticated requests
    for _ in 0..30 {
        let start_time = Instant::now();

        let response = ctx.server
            .get("/api/auth/me")
            .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
            .await;

        response.assert_status_ok();
        let response_time = start_time.elapsed();
        response_times.push(response_time);
    }

    // Calculate statistics
    let total_time: Duration = response_times.iter().sum();
    let avg_time = total_time / response_times.len() as u32;
    let mut sorted_times = response_times.clone();
    sorted_times.sort();
    let median_time = sorted_times[sorted_times.len() / 2];
    let max_time = *sorted_times.last().unwrap();

    println!("Authenticated Endpoint Performance:");
    println!("  Average: {:?}", avg_time);
    println!("  Median:  {:?}", median_time);
    println!("  Max:     {:?}", max_time);

    // Authenticated requests should still be reasonably fast
    assert!(avg_time < Duration::from_millis(200), "Average auth response time should be under 200ms");
    assert!(max_time < Duration::from_secs(1), "Maximum auth response time should be under 1s");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_database_query_performance() {
    let ctx = PerformanceTestContext::new().await;

    let mut response_times = Vec::new();

    // Measure response time for database-heavy endpoints
    for _ in 0..25 {
        let start_time = Instant::now();

        let response = ctx.server
            .get("/api/communities")
            .await;

        response.assert_status_ok();
        let response_time = start_time.elapsed();
        response_times.push(response_time);
    }

    // Calculate statistics
    let total_time: Duration = response_times.iter().sum();
    let avg_time = total_time / response_times.len() as u32;
    let mut sorted_times = response_times.clone();
    sorted_times.sort();
    let median_time = sorted_times[sorted_times.len() / 2];
    let max_time = *sorted_times.last().unwrap();

    println!("Database Query Performance:");
    println!("  Average: {:?}", avg_time);
    println!("  Median:  {:?}", median_time);
    println!("  Max:     {:?}", max_time);

    // Database queries should be reasonably fast
    assert!(avg_time < Duration::from_millis(500), "Average DB query time should be under 500ms");
    assert!(max_time < Duration::from_secs(2), "Maximum DB query time should be under 2s");

    ctx.cleanup().await;
}

// Concurrent Request Tests
#[tokio::test]
#[serial]
async fn test_concurrent_health_checks() {
    let ctx = PerformanceTestContext::new().await;

    let start_time = Instant::now();
    let concurrent_requests = 100;

    // Launch concurrent health check requests
    let mut handles = Vec::new();
    for _ in 0..concurrent_requests {
        let server = ctx.server.clone();
        handles.push(tokio::spawn(async move {
            let request_start = Instant::now();
            let response = server.get("/health").await;
            let request_time = request_start.elapsed();
            (response.status_code(), request_time)
        }));
    }

    // Wait for all requests to complete
    let mut successful_requests = 0;
    let mut failed_requests = 0;
    let mut response_times = Vec::new();

    for handle in handles {
        let (status_code, response_time) = handle.await.unwrap();
        response_times.push(response_time);

        if status_code == StatusCode::OK {
            successful_requests += 1;
        } else {
            failed_requests += 1;
        }
    }

    let total_time = start_time.elapsed();

    // Calculate throughput
    let requests_per_second = concurrent_requests as f64 / total_time.as_secs_f64();

    println!("Concurrent Health Check Performance:");
    println!("  Total time: {:?}", total_time);
    println!("  Successful: {}/{}", successful_requests, concurrent_requests);
    println!("  Failed: {}", failed_requests);
    println!("  Throughput: {:.2} req/s", requests_per_second);

    // Assert performance expectations
    assert_eq!(successful_requests, concurrent_requests, "All concurrent requests should succeed");
    assert!(requests_per_second > 50.0, "Should handle at least 50 requests per second");
    assert!(total_time < Duration::from_secs(10), "All requests should complete within 10 seconds");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_concurrent_authenticated_requests() {
    let ctx = PerformanceTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token, community_id) = ctx.create_authenticated_user().await;

    let start_time = Instant::now();
    let concurrent_requests = 50;

    // Launch concurrent authenticated requests
    let mut handles = Vec::new();
    for _ in 0..concurrent_requests {
        let server = ctx.server.clone();
        let token = token.clone();

        handles.push(tokio::spawn(async move {
            let request_start = Instant::now();
            let response = server
                .get("/api/auth/me")
                .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
                .await;
            let request_time = request_start.elapsed();
            (response.status_code(), request_time)
        }));
    }

    // Wait for all requests to complete
    let mut successful_requests = 0;
    let mut response_times = Vec::new();

    for handle in handles {
        let (status_code, response_time) = handle.await.unwrap();
        response_times.push(response_time);

        if status_code == StatusCode::OK {
            successful_requests += 1;
        }
    }

    let total_time = start_time.elapsed();
    let requests_per_second = concurrent_requests as f64 / total_time.as_secs_f64();

    // Calculate response time statistics
    let avg_response_time = response_times.iter().sum::<Duration>() / response_times.len() as u32;
    response_times.sort();
    let median_response_time = response_times[response_times.len() / 2];
    let p95_response_time = response_times[(response_times.len() as f64 * 0.95) as usize];

    println!("Concurrent Authenticated Request Performance:");
    println!("  Total time: {:?}", total_time);
    println!("  Successful: {}/{}", successful_requests, concurrent_requests);
    println!("  Throughput: {:.2} req/s", requests_per_second);
    println!("  Avg response time: {:?}", avg_response_time);
    println!("  Median response time: {:?}", median_response_time);
    println!("  95th percentile: {:?}", p95_response_time);

    // Assert performance expectations
    assert!(successful_requests >= concurrent_requests * 9 / 10,
            "At least 90% of concurrent auth requests should succeed");
    assert!(requests_per_second > 10.0, "Should handle at least 10 auth requests per second");
    assert!(p95_response_time < Duration::from_secs(2), "95th percentile should be under 2 seconds");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_mixed_workload_performance() {
    let ctx = PerformanceTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token, community_id) = ctx.create_authenticated_user().await;

    let start_time = Instant::now();
    let total_requests = 60;

    // Launch mixed workload (different types of requests)
    let mut handles = Vec::new();
    for i in 0..total_requests {
        let server = ctx.server.clone();
        let token = token.clone();
        let community_id = community_id.clone();

        handles.push(tokio::spawn(async move {
            let request_start = Instant::now();

            let (status_code, request_type) = match i % 4 {
                0 => {
                    // Health check
                    let response = server.get("/health").await;
                    (response.status_code(), "health")
                },
                1 => {
                    // Public community list
                    let response = server.get("/api/communities").await;
                    (response.status_code(), "communities")
                },
                2 => {
                    // Authenticated user profile
                    let response = server
                        .get("/api/auth/me")
                        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
                        .await;
                    (response.status_code(), "auth")
                },
                3 => {
                    // Business listing (stub)
                    let response = server
                        .get(&format!("/api/communities/{}/businesses", community_id))
                        .add_header(header::AUTHORIZATION, format!("Bearer {}", token))
                        .await;
                    (response.status_code(), "businesses")
                },
                _ => unreachable!(),
            };

            let request_time = request_start.elapsed();
            (status_code, request_type, request_time)
        }));
    }

    // Wait for all requests to complete and collect results
    let mut results = std::collections::HashMap::new();
    for handle in handles {
        let (status_code, request_type, response_time) = handle.await.unwrap();

        let entry = results.entry(request_type).or_insert_with(|| Vec::new());
        entry.push((status_code, response_time));
    }

    let total_time = start_time.elapsed();
    let overall_throughput = total_requests as f64 / total_time.as_secs_f64();

    println!("Mixed Workload Performance:");
    println!("  Total time: {:?}", total_time);
    println!("  Overall throughput: {:.2} req/s", overall_throughput);

    // Analyze results per request type
    for (request_type, responses) in results {
        let successful = responses.iter().filter(|(status, _)| status.is_success()).count();
        let total = responses.len();
        let success_rate = successful as f64 / total as f64 * 100.0;

        let response_times: Vec<Duration> = responses.iter().map(|(_, time)| *time).collect();
        let avg_time = response_times.iter().sum::<Duration>() / response_times.len() as u32;

        println!("  {}: {}/{} successful ({:.1}%), avg: {:?}",
                request_type, successful, total, success_rate, avg_time);

        // Each request type should have decent success rate
        assert!(success_rate > 80.0, "Request type {} should have >80% success rate", request_type);
    }

    // Overall throughput should be reasonable
    assert!(overall_throughput > 20.0, "Mixed workload should achieve >20 req/s throughput");

    ctx.cleanup().await;
}

// Memory and Resource Usage Tests
#[tokio::test]
#[serial]
async fn test_sustained_load() {
    let ctx = PerformanceTestContext::new().await;

    let duration = Duration::from_secs(30); // 30 second sustained load
    let start_time = Instant::now();
    let mut request_count = 0;
    let mut successful_requests = 0;

    println!("Starting 30-second sustained load test...");

    while start_time.elapsed() < duration {
        let response = ctx.server.get("/health").await;
        request_count += 1;

        if response.status_code() == StatusCode::OK {
            successful_requests += 1;
        }

        // Small delay to avoid overwhelming the system
        sleep(Duration::from_millis(10)).await;
    }

    let actual_duration = start_time.elapsed();
    let success_rate = successful_requests as f64 / request_count as f64 * 100.0;
    let avg_throughput = request_count as f64 / actual_duration.as_secs_f64();

    println!("Sustained Load Test Results:");
    println!("  Duration: {:?}", actual_duration);
    println!("  Total requests: {}", request_count);
    println!("  Successful: {} ({:.1}%)", successful_requests, success_rate);
    println!("  Average throughput: {:.2} req/s", avg_throughput);

    // Assert that the system maintains good performance under sustained load
    assert!(success_rate > 95.0, "Should maintain >95% success rate under sustained load");
    assert!(avg_throughput > 50.0, "Should maintain >50 req/s under sustained load");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_burst_traffic_handling() {
    let ctx = PerformanceTestContext::new().await;

    // Simulate burst traffic pattern
    let burst_sizes = vec![10, 25, 50, 75, 100];

    for burst_size in burst_sizes {
        println!("Testing burst of {} requests...", burst_size);

        let start_time = Instant::now();
        let mut handles = Vec::new();

        // Launch all requests simultaneously (burst)
        for _ in 0..burst_size {
            let server = ctx.server.clone();
            handles.push(tokio::spawn(async move {
                let response = server.get("/health").await;
                response.status_code()
            }));
        }

        // Wait for all requests to complete
        let mut successful = 0;
        for handle in handles {
            let status_code = handle.await.unwrap();
            if status_code == StatusCode::OK {
                successful += 1;
            }
        }

        let burst_time = start_time.elapsed();
        let success_rate = successful as f64 / burst_size as f64 * 100.0;
        let burst_throughput = burst_size as f64 / burst_time.as_secs_f64();

        println!("  Burst size {}: {}/{} successful ({:.1}%), time: {:?}, throughput: {:.2} req/s",
                burst_size, successful, burst_size, success_rate, burst_time, burst_throughput);

        // Even under burst traffic, should maintain reasonable success rates
        assert!(success_rate > 90.0, "Burst {} should maintain >90% success rate", burst_size);
        assert!(burst_time < Duration::from_secs(5), "Burst {} should complete within 5 seconds", burst_size);

        // Small delay between bursts
        sleep(Duration::from_millis(500)).await;
    }

    ctx.cleanup().await;
}

// Error Rate Tests
#[tokio::test]
#[serial]
async fn test_error_rate_under_load() {
    let ctx = PerformanceTestContext::new().await;
    ctx.setup_jwks_mock().await;

    let (user, token, community_id) = ctx.create_authenticated_user().await;

    let total_requests = 200;
    let mut handles = Vec::new();

    // Mix of valid and invalid requests to test error handling performance
    for i in 0..total_requests {
        let server = ctx.server.clone();
        let token = token.clone();
        let community_id = community_id.clone();

        handles.push(tokio::spawn(async move {
            let start_time = Instant::now();

            let (status_code, error_type) = match i % 10 {
                0..=6 => {
                    // Valid requests (70%)
                    let response = server.get("/health").await;
                    (response.status_code(), "none")
                },
                7 => {
                    // Invalid endpoint (10%)
                    let response = server.get("/api/invalid-endpoint").await;
                    (response.status_code(), "not_found")
                },
                8 => {
                    // Unauthorized request (10%)
                    let response = server.get("/api/auth/me").await;
                    (response.status_code(), "unauthorized")
                },
                9 => {
                    // Invalid method (10%)
                    let response = server.delete("/health").await;
                    (response.status_code(), "method_not_allowed")
                },
                _ => unreachable!(),
            };

            let response_time = start_time.elapsed();
            (status_code, error_type, response_time)
        }));
    }

    // Collect results
    let mut results = std::collections::HashMap::new();
    for handle in handles {
        let (status_code, error_type, response_time) = handle.await.unwrap();

        let entry = results.entry(error_type).or_insert_with(|| Vec::new());
        entry.push((status_code, response_time));
    }

    println!("Error Rate Test Results:");

    for (error_type, responses) in results {
        let total = responses.len();
        let response_times: Vec<Duration> = responses.iter().map(|(_, time)| *time).collect();
        let avg_time = response_times.iter().sum::<Duration>() / response_times.len() as u32;
        let max_time = response_times.iter().max().unwrap();

        println!("  {}: {} requests, avg: {:?}, max: {:?}",
                error_type, total, avg_time, max_time);

        // Error responses should still be fast
        assert!(avg_time < Duration::from_millis(200),
                "Error type {} should have fast response times", error_type);
        assert!(*max_time < Duration::from_secs(1),
                "Error type {} should not have extremely slow responses", error_type);
    }

    ctx.cleanup().await;
}

// Memory Efficiency Tests
#[tokio::test]
#[serial]
async fn test_large_response_handling() {
    let ctx = PerformanceTestContext::new().await;

    // Create multiple communities to generate a larger response
    let user = create_test_user(&ctx.db, None).await.expect("Failed to create user");

    for i in 0..20 {
        create_test_community(
            &ctx.db,
            user.id,
            Some(format!("Large Response Test Community {}", i))
        ).await.expect("Failed to create community");
    }

    let start_time = Instant::now();

    // Request large response
    let response = ctx.server
        .get("/api/communities?limit=50")
        .await;

    let response_time = start_time.elapsed();
    response.assert_status_ok();

    let body: ApiResponse<Vec<shared::models::CommunityWithStats>> = response.json();
    let communities = body.data.unwrap();

    println!("Large Response Test:");
    println!("  Communities returned: {}", communities.len());
    println!("  Response time: {:?}", response_time);

    // Large responses should still be handled efficiently
    assert!(communities.len() >= 20, "Should return the created communities");
    assert!(response_time < Duration::from_secs(2), "Large responses should complete within 2 seconds");

    ctx.cleanup().await;
}

#[tokio::test]
#[serial]
async fn test_connection_pool_efficiency() {
    let ctx = PerformanceTestContext::new().await;

    let concurrent_db_requests = 25;
    let mut handles = Vec::new();

    let start_time = Instant::now();

    // Launch concurrent database-heavy requests
    for _ in 0..concurrent_db_requests {
        let server = ctx.server.clone();
        handles.push(tokio::spawn(async move {
            let request_start = Instant::now();
            let response = server.get("/api/communities").await;
            let request_time = request_start.elapsed();
            (response.status_code(), request_time)
        }));
    }

    let mut successful_requests = 0;
    let mut response_times = Vec::new();

    for handle in handles {
        let (status_code, response_time) = handle.await.unwrap();
        response_times.push(response_time);

        if status_code == StatusCode::OK {
            successful_requests += 1;
        }
    }

    let total_time = start_time.elapsed();
    let avg_response_time = response_times.iter().sum::<Duration>() / response_times.len() as u32;

    println!("Connection Pool Efficiency Test:");
    println!("  Successful requests: {}/{}", successful_requests, concurrent_db_requests);
    println!("  Total time: {:?}", total_time);
    println!("  Average response time: {:?}", avg_response_time);

    // Connection pool should handle concurrent DB requests efficiently
    assert_eq!(successful_requests, concurrent_db_requests,
               "All concurrent DB requests should succeed");
    assert!(avg_response_time < Duration::from_millis(500),
               "Average DB response time should be reasonable with connection pooling");

    ctx.cleanup().await;
}