use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use shared::error::AppError;
use crate::AppState;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    max_requests: u32,
    window: Duration,
}

#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub fn check_rate_limit(&self, identifier: &str) -> Result<bool, AppError> {
        let mut requests = self.requests.lock()
            .map_err(|_| AppError::Internal(anyhow::anyhow!("Failed to acquire rate limit lock")))?;

        let now = Instant::now();

        match requests.get_mut(identifier) {
            Some(entry) => {
                // Check if window has expired
                if now.duration_since(entry.window_start) > self.window {
                    // Reset window
                    entry.count = 1;
                    entry.window_start = now;
                    Ok(true)
                } else if entry.count < self.max_requests {
                    // Still within limit
                    entry.count += 1;
                    Ok(true)
                } else {
                    // Rate limit exceeded
                    Ok(false)
                }
            }
            None => {
                // First request from this identifier
                requests.insert(identifier.to_string(), RateLimitEntry {
                    count: 1,
                    window_start: now,
                });
                Ok(true)
            }
        }
    }

    // Clean up expired entries to prevent memory leaks
    pub fn cleanup_expired(&self) {
        if let Ok(mut requests) = self.requests.lock() {
            let now = Instant::now();
            requests.retain(|_, entry| {
                now.duration_since(entry.window_start) <= self.window
            });
        }
    }
}

pub async fn rate_limit_middleware(
    headers: HeaderMap,
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Create rate limiter for this request - in production, this should be shared
    let rate_limiter = RateLimiter::new(100, Duration::from_secs(60)); // 100 requests per minute

    // Get identifier (IP address or user ID if authenticated)
    let identifier = if let Some(auth_header) = headers.get("authorization") {
        // Try to extract user from token for better identification
        if let Ok(auth_str) = auth_header.to_str() {
            if let Ok(token) = shared::auth::extract_bearer_token(auth_str) {
                // Use a hash of the token as identifier to avoid storing sensitive data
                use std::hash::{Hash, Hasher};
                use std::collections::hash_map::DefaultHasher;
                let mut hasher = DefaultHasher::new();
                token.hash(&mut hasher);
                format!("user_{}", hasher.finish())
            } else {
                // Fallback to IP
                get_client_ip(&headers)
            }
        } else {
            get_client_ip(&headers)
        }
    } else {
        get_client_ip(&headers)
    };

    // Check rate limit
    match rate_limiter.check_rate_limit(&identifier) {
        Ok(allowed) => {
            if allowed {
                Ok(next.run(request).await)
            } else {
                Err(StatusCode::TOO_MANY_REQUESTS)
            }
        }
        Err(_) => {
            // On error, allow the request but log the issue
            tracing::error!("Rate limiting error for identifier: {}", identifier);
            Ok(next.run(request).await)
        }
    }
}

fn get_client_ip(headers: &HeaderMap) -> String {
    // Try various headers in order of preference
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            // Take the first IP in the chain
            return forwarded_str.split(',').next()
                .unwrap_or("unknown")
                .trim()
                .to_string();
        }
    }

    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }

    if let Some(cf_ip) = headers.get("cf-connecting-ip") {
        if let Ok(ip_str) = cf_ip.to_str() {
            return ip_str.to_string();
        }
    }

    "unknown".to_string()
}