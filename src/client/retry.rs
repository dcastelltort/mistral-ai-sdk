use std::time::Duration;

/// Retry strategy configuration
#[derive(Debug, Clone)]
pub struct RetryStrategy {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    
    /// Delay between retry attempts
    pub delay: Duration,
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            delay: Duration::from_millis(100),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_default_retry_strategy() {
        let strategy = RetryStrategy::default();
        assert_eq!(strategy.max_retries, 3);
        assert_eq!(strategy.delay, Duration::from_millis(100));
    }

    #[test]
    fn test_custom_retry_strategy() {
        let strategy = RetryStrategy {
            max_retries: 5,
            delay: Duration::from_secs(1),
        };
        assert_eq!(strategy.max_retries, 5);
        assert_eq!(strategy.delay, Duration::from_secs(1));
    }
}