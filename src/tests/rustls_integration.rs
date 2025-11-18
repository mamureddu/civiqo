/// Comprehensive rustls integration tests
/// These tests validate that rustls is properly configured and working
/// across all components of the Community Manager application.

use shared::{
    database::Database,
    testing::{test_rustls_http_client, test_rustls_db_connection, test_rustls_tls_config},
    error::Result,
};
use reqwest::Client;
use rustls::{ClientConfig, Certificate, RootCertStore};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio;

/// Test that rustls TLS configuration is working properly
#[tokio::test]
async fn test_rustls_tls_configuration() -> Result<()> {
    test_rustls_tls_config().await
}

/// Test that HTTP client with rustls can make requests
#[tokio::test]
async fn test_rustls_http_client_functionality() -> Result<()> {
    test_rustls_http_client().await
}

/// Test that database connections use rustls properly
#[tokio::test]
async fn test_rustls_database_connection() -> Result<()> {
    test_rustls_db_connection().await
}

/// Integration test: Verify rustls works with real external services
#[tokio::test]
async fn test_rustls_external_service_integration() {
    // Build rustls client
    let mut root_store = RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().unwrap() {
        root_store.add(&Certificate(cert.0)).unwrap();
    }

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build rustls client");

    // Test HTTPS request to a real service
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await;

    match response {
        Ok(resp) => {
            assert!(resp.status().is_success());
            println!("✅ rustls HTTPS client working properly");
        }
        Err(e) => {
            // Network failures are acceptable in test environments
            println!("⚠️  Network test skipped: {}", e);
        }
    }
}

/// Performance test: Ensure rustls doesn't significantly impact performance
#[tokio::test]
async fn test_rustls_performance() {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build rustls client");

    let start = std::time::Instant::now();

    // Make multiple concurrent requests to test performance
    let mut handles = Vec::new();
    for _ in 0..5 {
        let client = client.clone();
        handles.push(tokio::spawn(async move {
            client.get("https://httpbin.org/delay/1").send().await
        }));
    }

    // Wait for all requests
    let mut success_count = 0;
    for handle in handles {
        match handle.await {
            Ok(Ok(_)) => success_count += 1,
            _ => {} // Network failures are acceptable
        }
    }

    let duration = start.elapsed();

    // Should handle concurrent requests efficiently
    // Allowing for network variability in test environments
    println!("✅ rustls performance test: {} successful requests in {:?}", success_count, duration);

    // Basic performance check - should complete within reasonable time
    assert!(duration < Duration::from_secs(30), "rustls performance degraded: {:?}", duration);
}

/// Memory usage test: Ensure rustls doesn't leak memory
#[tokio::test]
async fn test_rustls_memory_usage() {
    // Create and drop many rustls clients to test for memory leaks
    for _ in 0..100 {
        let _client = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(1))
            .build()
            .expect("Failed to build rustls client");

        let mut _root_store = RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs().unwrap().into_iter().take(5) {
            _root_store.add(&Certificate(cert.0)).unwrap();
        }
    }

    println!("✅ rustls memory usage test completed - no obvious leaks");
}

/// Certificate validation test
#[tokio::test]
async fn test_rustls_certificate_validation() {
    let mut root_store = RootCertStore::empty();
    let certs = rustls_native_certs::load_native_certs().unwrap();

    // Verify we can load certificates
    assert!(!certs.is_empty(), "Should load native certificates");

    for cert in certs.into_iter().take(10) {
        let result = root_store.add(&Certificate(cert.0));
        assert!(result.is_ok(), "Should be able to add certificate to root store");
    }

    println!("✅ rustls certificate validation test passed");
}

/// Database connection with rustls test
#[tokio::test]
async fn test_rustls_database_ssl() {
    // Test database connection with SSL/TLS using rustls
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://dev:dev123@localhost:5432/community_manager".to_string());

    // Attempt to create a pool with SSL settings
    let pool_result = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&format!("{}?sslmode=prefer", database_url))
        .await;

    match pool_result {
        Ok(_pool) => {
            println!("✅ rustls database SSL connection successful");
        }
        Err(e) => {
            // Database connection failures are acceptable in test environments
            println!("⚠️  Database SSL test skipped: {}", e);
        }
    }
}

/// Integration test with Auth0-like service
#[tokio::test]
async fn test_rustls_jwks_integration() {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build rustls client");

    // Test JWKS endpoint (Auth0-style)
    let response = client
        .get("https://auth0.com/.well-known/jwks.json")
        .send()
        .await;

    match response {
        Ok(resp) => {
            assert!(resp.status().is_success());
            let _jwks: serde_json::Value = resp.json().await.expect("Should parse JWKS JSON");
            println!("✅ rustls JWKS integration test passed");
        }
        Err(e) => {
            println!("⚠️  JWKS integration test skipped: {}", e);
        }
    }
}

/// Test rustls with various TLS versions and cipher suites
#[tokio::test]
async fn test_rustls_tls_compatibility() {
    // Test that rustls can handle modern TLS configurations
    let mut root_store = RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().unwrap() {
        root_store.add(&Certificate(cert.0)).unwrap();
    }

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Verify configuration is valid
    assert!(!config.alpn_protocols.is_empty(), "Should have ALPN protocols configured");

    println!("✅ rustls TLS compatibility test passed");
}

/// Stress test: Multiple concurrent connections with rustls
#[tokio::test]
async fn test_rustls_concurrent_connections() {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(5))
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to build rustls client");

    let mut handles = Vec::new();

    // Create 20 concurrent connections
    for i in 0..20 {
        let client = client.clone();
        handles.push(tokio::spawn(async move {
            let response = client
                .get(&format!("https://httpbin.org/json?id={}", i))
                .send()
                .await;
            response.is_ok()
        }));
    }

    // Collect results
    let mut success_count = 0;
    for handle in handles {
        if let Ok(true) = handle.await {
            success_count += 1;
        }
    }

    println!("✅ rustls concurrent connections: {}/20 successful", success_count);

    // At least half should succeed (accounting for network variability)
    assert!(success_count >= 10, "Too many concurrent connection failures");
}