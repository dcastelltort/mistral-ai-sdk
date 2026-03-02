use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limiter implementation using token bucket algorithm
#[derive(Debug)]
pub struct RateLimiter {
    tokens: Arc<Mutex<usize>>,
    capacity: usize,
    refill_rate: f64,
    last_refill: Arc<Mutex<Instant>>,
    min_refill_interval: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of tokens (requests) allowed
    /// * `refill_rate` - Tokens per second to refill
    pub fn new(capacity: usize, refill_rate: f64) -> Self {
        Self {
            tokens: Arc::new(Mutex::new(capacity)),
            capacity,
            refill_rate,
            last_refill: Arc::new(Mutex::new(Instant::now())),
            min_refill_interval: Duration::from_millis(100), // Minimum 100ms between refills
        }
    }
    
    /// Try to acquire a permit for a request
    ///
    /// Returns `true` if the request is allowed, `false` if rate limited
    pub async fn try_acquire(&self) -> bool {
        self.refill_tokens().await;
        let mut tokens = self.tokens.lock().await;
        if *tokens > 0 {
            *tokens -= 1;
            true
        } else {
            false
        }
    }
    
    /// Acquire a permit, waiting if necessary
    ///
    /// Returns when a permit is available
    pub async fn acquire(&self) {
        while !self.try_acquire().await {
            self.refill_tokens().await;
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
    
    /// Refill tokens based on time elapsed
    async fn refill_tokens(&self) {
        let now = Instant::now();
        let mut last_refill = self.last_refill.lock().await;
        
        let elapsed = now.duration_since(*last_refill);
        
        // Only refill if minimum interval has passed
        if elapsed >= self.min_refill_interval {
            let elapsed_secs = elapsed.as_secs_f64();
            let tokens_to_add = (elapsed_secs * self.refill_rate) as usize;
            
            if tokens_to_add > 0 {
                *last_refill = now;
                let mut tokens = self.tokens.lock().await;
                *tokens = (*tokens + tokens_to_add).min(self.capacity);
            }
        }
    }
    
    /// Get current available permits
    pub async fn available_permits(&self) -> usize {
        self.refill_tokens().await;
        let tokens = self.tokens.lock().await;
        *tokens
    }
    
    /// Get capacity (for testing)
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get refill rate (for testing)
    pub fn refill_rate(&self) -> f64 {
        self.refill_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let limiter = RateLimiter::new(10, 5.0);
        assert_eq!(limiter.capacity, 10);
        assert_eq!(limiter.refill_rate, 5.0);
    }

    #[tokio::test]
    async fn test_rate_limiter_initial_permits() {
        let limiter = RateLimiter::new(10, 5.0);
        assert_eq!(limiter.available_permits().await, 10);
    }

    #[tokio::test]
    async fn test_rate_limiter_acquire() {
        let limiter = RateLimiter::new(2, 1.0);
        
        // Should be able to acquire 2 permits immediately
        assert!(limiter.try_acquire().await);
        assert!(limiter.try_acquire().await);
        
        // Third request should be rate limited
        assert!(!limiter.try_acquire().await);
    }

    #[tokio::test]
    async fn test_rate_limiter_refill() {
        let limiter = RateLimiter::new(2, 2.0); // 2 tokens per second
        
        // Acquire all tokens
        assert!(limiter.try_acquire().await);
        assert!(limiter.try_acquire().await);
        assert!(!limiter.try_acquire().await);
        
        // Wait for refill (500ms should give us 1 token)
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // Should have 1 token available now
        assert!(limiter.try_acquire().await);
        assert!(!limiter.try_acquire().await);
    }

    #[tokio::test]
    async fn test_rate_limiter_burst() {
        let limiter = RateLimiter::new(5, 1.0);
        
        // Should allow burst up to capacity
        for i in 0..5 {
            assert!(limiter.try_acquire().await, "Failed on request {}", i);
        }
        
        // Next request should be limited
        assert!(!limiter.try_acquire().await);
    }

    #[tokio::test]
    async fn test_rate_limiter_acquire_with_wait() {
        let limiter = RateLimiter::new(1, 1.0); // 1 token per second
        
        // Acquire the only token
        assert!(limiter.try_acquire().await);
        
        // Try to acquire immediately - should fail
        assert!(!limiter.try_acquire().await);
        
        // Wait for refill and try again
        tokio::time::sleep(Duration::from_millis(1100)).await;
        assert!(limiter.try_acquire().await);
    }
}