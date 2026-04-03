use shared::error::Result;
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
        )
        .await
    }

    /// Check if user can send typing notification (returns true if allowed)
    pub async fn check_typing_limit(&self, user_id: Uuid) -> Result<bool> {
        self.check_rate_limit(
            user_id,
            &self.typing_limits,
            self.max_typing_per_window,
            "typing",
        )
        .await
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
            debug!(
                "Rate limit window reset for user {} action {}",
                user_id, action_type
            );
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
                            debug!(
                                "Cleaned up expired typing rate limit entry for user {}",
                                user_id
                            );
                        }
                        retain
                    });
                }
            }
        });
    }
}
