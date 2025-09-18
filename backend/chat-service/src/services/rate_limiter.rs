use shared::error::{AppError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, warn};
use uuid::Uuid;

/// Rate limiter entry tracking usage for a specific user and action
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

/// Rate limiter for different types of user actions
#[derive(Debug)]
pub struct RateLimiter {
    /// Message sending rate limits (user_id -> entry)
    message_limits: Arc<RwLock<HashMap<Uuid, RateLimitEntry>>>,
    /// Typing notification rate limits (user_id -> entry)
    typing_limits: Arc<RwLock<HashMap<Uuid, RateLimitEntry>>>,
    /// Rate limit window duration (1 minute)
    window_duration: Duration,
    /// Maximum messages per window
    max_messages_per_window: u32,
    /// Maximum typing notifications per window
    max_typing_per_window: u32,
}

impl RateLimiter {
    /// Create a new rate limiter with the specified limits
    pub fn new(max_messages_per_minute: u32, max_typing_per_minute: u32) -> Self {
        let rate_limiter = Self {
            message_limits: Arc::new(RwLock::new(HashMap::new())),
            typing_limits: Arc::new(RwLock::new(HashMap::new())),
            window_duration: Duration::from_secs(60), // 1 minute
            max_messages_per_window: max_messages_per_minute,
            max_typing_per_window: max_typing_per_minute,
        };

        // Start cleanup task
        rate_limiter.start_cleanup_task();
        rate_limiter
    }

    /// Check if user can send a message (returns true if allowed)
    pub async fn check_message_limit(&self, user_id: Uuid) -> Result<bool> {
        self.check_rate_limit(
            user_id,
            &self.message_limits,
            self.max_messages_per_window,
            "message",
        ).await
    }

    /// Check if user can send typing notification (returns true if allowed)
    pub async fn check_typing_limit(&self, user_id: Uuid) -> Result<bool> {
        self.check_rate_limit(
            user_id,
            &self.typing_limits,
            self.max_typing_per_window,
            "typing",
        ).await
    }

    /// Generic rate limit check
    async fn check_rate_limit(
        &self,
        user_id: Uuid,
        limits: &Arc<RwLock<HashMap<Uuid, RateLimitEntry>>>,
        max_count: u32,
        action_type: &str,
    ) -> Result<bool> {
        let now = Instant::now();
        let mut limits_map = limits.write().await;

        let entry = limits_map.entry(user_id).or_insert(RateLimitEntry {
            count: 0,
            window_start: now,
        });

        // Reset window if expired
        if now.duration_since(entry.window_start) >= self.window_duration {
            entry.count = 0;
            entry.window_start = now;
            debug!("Rate limit window reset for user {} action {}", user_id, action_type);
        }

        // Check if limit exceeded
        if entry.count >= max_count {
            warn!(
                "Rate limit exceeded for user {} action {}: {}/{}",
                user_id, action_type, entry.count, max_count
            );
            return Ok(false);
        }

        // Increment counter
        entry.count += 1;
        debug!(
            "Rate limit check passed for user {} action {}: {}/{}",
            user_id, action_type, entry.count, max_count
        );

        Ok(true)
    }

    /// Start background cleanup task to remove expired entries
    fn start_cleanup_task(&self) {
        let message_limits = Arc::clone(&self.message_limits);
        let typing_limits = Arc::clone(&self.typing_limits);
        let window_duration = self.window_duration;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // Cleanup every 5 minutes

            loop {
                interval.tick().await;

                let now = Instant::now();

                // Cleanup message limits
                {
                    let mut limits = message_limits.write().await;
                    limits.retain(|user_id, entry| {
                        let retain = now.duration_since(entry.window_start) < window_duration * 2;
                        if !retain {
                            debug!("Cleaned up expired rate limit entry for user {}", user_id);
                        }
                        retain
                    });
                }

                // Cleanup typing limits
                {
                    let mut limits = typing_limits.write().await;
                    limits.retain(|user_id, entry| {
                        let retain = now.duration_since(entry.window_start) < window_duration * 2;
                        if !retain {
                            debug!("Cleaned up expired typing rate limit entry for user {}", user_id);
                        }
                        retain
                    });
                }
            }
        });
    }

    /// Get current rate limit status for a user (for testing/debugging)
    pub async fn get_user_status(&self, user_id: Uuid) -> (u32, u32) {
        let message_limits = self.message_limits.read().await;
        let typing_limits = self.typing_limits.read().await;

        let message_count = message_limits.get(&user_id).map(|e| e.count).unwrap_or(0);
        let typing_count = typing_limits.get(&user_id).map(|e| e.count).unwrap_or(0);

        (message_count, typing_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let rate_limiter = RateLimiter::new(30, 60);
        assert_eq!(rate_limiter.max_messages_per_window, 30);
        assert_eq!(rate_limiter.max_typing_per_window, 60);
    }

    #[tokio::test]
    async fn test_message_rate_limiting() {
        let rate_limiter = RateLimiter::new(3, 10); // Very low limit for testing
        let user_id = Uuid::new_v4();

        // First 3 messages should pass
        for i in 1..=3 {
            let allowed = rate_limiter.check_message_limit(user_id).await.unwrap();
            assert!(allowed, "Message {} should be allowed", i);
        }

        // 4th message should be rejected
        let allowed = rate_limiter.check_message_limit(user_id).await.unwrap();
        assert!(!allowed, "4th message should be rate limited");
    }

    #[tokio::test]
    async fn test_typing_rate_limiting() {
        let rate_limiter = RateLimiter::new(10, 2); // Very low typing limit for testing
        let user_id = Uuid::new_v4();

        // First 2 typing notifications should pass
        for i in 1..=2 {
            let allowed = rate_limiter.check_typing_limit(user_id).await.unwrap();
            assert!(allowed, "Typing notification {} should be allowed", i);
        }

        // 3rd typing notification should be rejected
        let allowed = rate_limiter.check_typing_limit(user_id).await.unwrap();
        assert!(!allowed, "3rd typing notification should be rate limited");
    }

    #[tokio::test]
    async fn test_different_users_independent_limits() {
        let rate_limiter = RateLimiter::new(1, 1); // Very strict limits
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();

        // Each user should get their own allowance
        assert!(rate_limiter.check_message_limit(user1).await.unwrap());
        assert!(rate_limiter.check_message_limit(user2).await.unwrap());

        // Both should be rate limited on second attempt
        assert!(!rate_limiter.check_message_limit(user1).await.unwrap());
        assert!(!rate_limiter.check_message_limit(user2).await.unwrap());
    }

    #[tokio::test]
    async fn test_user_status_tracking() {
        let rate_limiter = RateLimiter::new(10, 20);
        let user_id = Uuid::new_v4();

        let (msg_count, typing_count) = rate_limiter.get_user_status(user_id).await;
        assert_eq!(msg_count, 0);
        assert_eq!(typing_count, 0);

        // Send a message and typing notification
        rate_limiter.check_message_limit(user_id).await.unwrap();
        rate_limiter.check_typing_limit(user_id).await.unwrap();

        let (msg_count, typing_count) = rate_limiter.get_user_status(user_id).await;
        assert_eq!(msg_count, 1);
        assert_eq!(typing_count, 1);
    }

    // Note: Window expiration test would require mocking time or waiting 60+ seconds
    // For now, we trust the logic is correct based on the duration checks
}