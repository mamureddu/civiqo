use super::rate_limiter::RateLimiter;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use std::sync::Arc;

#[tokio::test]
async fn test_rate_limiter_creation() {
    let rate_limiter = RateLimiter::new(30, 60);
    assert_eq!(rate_limiter.max_messages_per_window(), 30);
    assert_eq!(rate_limiter.max_typing_per_window(), 60);
    assert_eq!(rate_limiter.window_duration(), Duration::from_secs(60));
}

#[tokio::test]
async fn test_message_rate_limiting_basic() {
    let rate_limiter = RateLimiter::new(3, 10); // Very low limit for testing
    let user_id = Uuid::new_v4();

    // First 3 messages should pass
    for i in 1..=3 {
        let allowed = rate_limiter.check_message_limit(user_id).await.unwrap();
        assert!(allowed, "Message {} should be allowed", i);
        let (msg_count, _) = rate_limiter.get_user_status(user_id).await;
        assert_eq!(msg_count, i, "Message count should be {}", i);
    }

    // 4th message should be rejected
    let allowed = rate_limiter.check_message_limit(user_id).await.unwrap();
    assert!(!allowed, "4th message should be rate limited");

    // Count should remain at 3 (no increment for rejected message)
    let (msg_count, _) = rate_limiter.get_user_status(user_id).await;
    assert_eq!(msg_count, 3, "Message count should remain at 3 after rejection");
}

#[tokio::test]
async fn test_typing_rate_limiting_basic() {
    let rate_limiter = RateLimiter::new(10, 2); // Very low typing limit for testing
    let user_id = Uuid::new_v4();

    // First 2 typing notifications should pass
    for i in 1..=2 {
        let allowed = rate_limiter.check_typing_limit(user_id).await.unwrap();
        assert!(allowed, "Typing notification {} should be allowed", i);
        let (_, typing_count) = rate_limiter.get_user_status(user_id).await;
        assert_eq!(typing_count, i, "Typing count should be {}", i);
    }

    // 3rd typing notification should be rejected
    let allowed = rate_limiter.check_typing_limit(user_id).await.unwrap();
    assert!(!allowed, "3rd typing notification should be rate limited");

    // Count should remain at 2 (no increment for rejected notification)
    let (_, typing_count) = rate_limiter.get_user_status(user_id).await;
    assert_eq!(typing_count, 2, "Typing count should remain at 2 after rejection");
}

#[tokio::test]
async fn test_different_users_independent_limits() {
    let rate_limiter = RateLimiter::new(1, 1); // Very strict limits
    let user1 = Uuid::new_v4();
    let user2 = Uuid::new_v4();

    // Each user should get their own allowance for messages
    assert!(rate_limiter.check_message_limit(user1).await.unwrap());
    assert!(rate_limiter.check_message_limit(user2).await.unwrap());

    // Both should be rate limited on second attempt
    assert!(!rate_limiter.check_message_limit(user1).await.unwrap());
    assert!(!rate_limiter.check_message_limit(user2).await.unwrap());

    // Same for typing notifications
    assert!(rate_limiter.check_typing_limit(user1).await.unwrap());
    assert!(rate_limiter.check_typing_limit(user2).await.unwrap());

    // Both should be rate limited on second attempt
    assert!(!rate_limiter.check_typing_limit(user1).await.unwrap());
    assert!(!rate_limiter.check_typing_limit(user2).await.unwrap());
}

#[tokio::test]
async fn test_user_status_tracking() {
    let rate_limiter = RateLimiter::new(10, 20);
    let user_id = Uuid::new_v4();

    // Initial status should be zero
    let (msg_count, typing_count) = rate_limiter.get_user_status(user_id).await;
    assert_eq!(msg_count, 0);
    assert_eq!(typing_count, 0);

    // Send multiple messages and typing notifications
    for _ in 0..3 {
        rate_limiter.check_message_limit(user_id).await.unwrap();
    }
    for _ in 0..5 {
        rate_limiter.check_typing_limit(user_id).await.unwrap();
    }

    let (msg_count, typing_count) = rate_limiter.get_user_status(user_id).await;
    assert_eq!(msg_count, 3);
    assert_eq!(typing_count, 5);
}

#[tokio::test]
async fn test_mixed_user_actions() {
    let rate_limiter = RateLimiter::new(5, 8);
    let user1 = Uuid::new_v4();
    let user2 = Uuid::new_v4();

    // User1: 3 messages, 2 typing
    for _ in 0..3 {
        assert!(rate_limiter.check_message_limit(user1).await.unwrap());
    }
    for _ in 0..2 {
        assert!(rate_limiter.check_typing_limit(user1).await.unwrap());
    }

    // User2: 1 message, 5 typing
    assert!(rate_limiter.check_message_limit(user2).await.unwrap());
    for _ in 0..5 {
        assert!(rate_limiter.check_typing_limit(user2).await.unwrap());
    }

    // Verify independent tracking
    let (user1_msg, user1_typing) = rate_limiter.get_user_status(user1).await;
    let (user2_msg, user2_typing) = rate_limiter.get_user_status(user2).await;

    assert_eq!(user1_msg, 3);
    assert_eq!(user1_typing, 2);
    assert_eq!(user2_msg, 1);
    assert_eq!(user2_typing, 5);

    // Both users still have quota left for messages
    assert!(rate_limiter.check_message_limit(user1).await.unwrap()); // 4th message
    assert!(rate_limiter.check_message_limit(user2).await.unwrap()); // 2nd message

    // User2 should still have typing quota, user1 should too
    assert!(rate_limiter.check_typing_limit(user1).await.unwrap()); // 3rd typing
    assert!(rate_limiter.check_typing_limit(user2).await.unwrap()); // 6th typing
}

#[tokio::test]
async fn test_rate_limit_exactly_at_boundary() {
    let rate_limiter = RateLimiter::new(3, 2);
    let user_id = Uuid::new_v4();

    // Use exact limit for messages
    for i in 1..=3 {
        let allowed = rate_limiter.check_message_limit(user_id).await.unwrap();
        assert!(allowed, "Message {} at boundary should be allowed", i);
    }

    // One more should fail
    let allowed = rate_limiter.check_message_limit(user_id).await.unwrap();
    assert!(!allowed, "Message beyond limit should be rejected");

    // Use exact limit for typing
    for i in 1..=2 {
        let allowed = rate_limiter.check_typing_limit(user_id).await.unwrap();
        assert!(allowed, "Typing {} at boundary should be allowed", i);
    }

    // One more should fail
    let allowed = rate_limiter.check_typing_limit(user_id).await.unwrap();
    assert!(!allowed, "Typing beyond limit should be rejected");
}

#[tokio::test]
async fn test_zero_rate_limit() {
    let rate_limiter = RateLimiter::new(0, 0); // No quota
    let user_id = Uuid::new_v4();

    // First attempt should fail with zero quota
    let allowed = rate_limiter.check_message_limit(user_id).await.unwrap();
    assert!(!allowed, "First message with zero quota should be rejected");

    let allowed = rate_limiter.check_typing_limit(user_id).await.unwrap();
    assert!(!allowed, "First typing with zero quota should be rejected");
}

#[tokio::test]
async fn test_high_rate_limit() {
    let rate_limiter = RateLimiter::new(1000, 2000); // Very high limits
    let user_id = Uuid::new_v4();

    // Should allow many requests
    for _ in 0..100 {
        assert!(rate_limiter.check_message_limit(user_id).await.unwrap());
    }
    for _ in 0..200 {
        assert!(rate_limiter.check_typing_limit(user_id).await.unwrap());
    }

    // Check status
    let (msg_count, typing_count) = rate_limiter.get_user_status(user_id).await;
    assert_eq!(msg_count, 100);
    assert_eq!(typing_count, 200);
}

#[tokio::test]
async fn test_concurrent_user_access() {
    use std::sync::Arc;

    let rate_limiter = Arc::new(RateLimiter::new(5, 10));
    let user_id = Uuid::new_v4();

    let mut handles = vec![];

    // Spawn multiple tasks trying to use rate limits concurrently
    for _ in 0..10 {
        let rl = Arc::clone(&rate_limiter);
        let handle = tokio::spawn(async move {
            rl.check_message_limit(user_id).await.unwrap()
        });
        handles.push(handle);
    }

    // Collect results
    let mut successful_requests = 0;
    for handle in handles {
        if handle.await.unwrap() {
            successful_requests += 1;
        }
    }

    // Should only allow 5 successful requests due to rate limit
    assert_eq!(successful_requests, 5);
}

#[tokio::test]
async fn test_multiple_concurrent_users() {
    use std::sync::Arc;

    let rate_limiter = Arc::new(RateLimiter::new(2, 3));
    let mut handles = vec![];

    // Create multiple users and spawn concurrent requests
    for _ in 0..5 {
        let user_id = Uuid::new_v4();
        let rl = Arc::clone(&rate_limiter);

        let handle = tokio::spawn(async move {
            let mut successful_msg = 0;
            let mut successful_typing = 0;

            // Each user tries 5 messages and 6 typing notifications
            for _ in 0..5 {
                if rl.check_message_limit(user_id).await.unwrap() {
                    successful_msg += 1;
                }
            }
            for _ in 0..6 {
                if rl.check_typing_limit(user_id).await.unwrap() {
                    successful_typing += 1;
                }
            }

            (successful_msg, successful_typing)
        });
        handles.push(handle);
    }

    // Each user should get exactly their quota (2 messages, 3 typing)
    for handle in handles {
        let (msg_count, typing_count) = handle.await.unwrap();
        assert_eq!(msg_count, 2);
        assert_eq!(typing_count, 3);
    }
}

#[tokio::test]
async fn test_error_conditions() {
    let rate_limiter = RateLimiter::new(10, 20);

    // All operations should return Ok(bool) for valid UUIDs
    let user_id = Uuid::new_v4();
    assert!(rate_limiter.check_message_limit(user_id).await.is_ok());
    assert!(rate_limiter.check_typing_limit(user_id).await.is_ok());

    // get_user_status should always work
    let status = rate_limiter.get_user_status(user_id).await;
    assert!(status.0 <= rate_limiter.max_messages_per_window());
    assert!(status.1 <= rate_limiter.max_typing_per_window());
}

// Note: Testing window expiration would require either:
// 1. Mocking time (complex)
// 2. Using very short windows (unreliable in CI)
// 3. Waiting 60+ seconds (too slow for unit tests)
//
// The window expiration logic is tested indirectly through the cleanup task
// and can be validated in integration tests if needed.