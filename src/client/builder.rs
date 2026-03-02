use crate::client::{MistralClient, RetryStrategy};
use std::time::Duration;

/// Builder for MistralClient with configurable options
#[derive(Debug, Default)]
pub struct MistralClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    retry_strategy: RetryStrategy,
}

impl MistralClientBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the API key for authentication
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    
    /// Set the base URL for the API
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }
    
    /// Configure retry strategy
    pub fn retry_strategy(mut self, retry_strategy: RetryStrategy) -> Self {
        self.retry_strategy = retry_strategy;
        self
    }
    
    /// Set maximum number of retries
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.retry_strategy.max_retries = max_retries;
        self
    }
    
    /// Set delay between retries
    pub fn retry_delay(mut self, delay: Duration) -> Self {
        self.retry_strategy.delay = delay;
        self
    }
    
    /// Build the MistralClient
    pub fn build(self) -> Result<MistralClient, crate::error::MistralError> {
        let api_key = self.api_key.ok_or_else(|| {
            crate::error::MistralError::InvalidConfiguration("API key is required".to_string())
        })?;
        
        Ok(MistralClient {
            api_key,
            base_url: self.base_url.unwrap_or_else(|| "https://api.mistral.ai".to_string()),
            retry_strategy: self.retry_strategy,
            client: reqwest::Client::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_builder_defaults() {
        let builder = MistralClientBuilder::new();
        assert!(builder.api_key.is_none());
        assert!(builder.base_url.is_none());
        assert_eq!(builder.retry_strategy.max_retries, 3);
        assert_eq!(builder.retry_strategy.delay, Duration::from_millis(100));
    }

    #[test]
    fn test_builder_configuration() {
        let builder = MistralClientBuilder::new()
            .api_key("test-key")
            .base_url("https://custom.api")
            .max_retries(5)
            .retry_delay(Duration::from_millis(200));
            
        assert_eq!(builder.api_key.unwrap(), "test-key");
        assert_eq!(builder.base_url.unwrap(), "https://custom.api");
        assert_eq!(builder.retry_strategy.max_retries, 5);
        assert_eq!(builder.retry_strategy.delay, Duration::from_millis(200));
    }

    #[test]
    fn test_build_without_api_key() {
        let builder = MistralClientBuilder::new();
        let result = builder.build();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::MistralError::InvalidConfiguration(msg) => {
                assert_eq!(msg, "API key is required");
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_build_success() {
        let builder = MistralClientBuilder::new()
            .api_key("test-key")
            .base_url("https://custom.api");
        
        let result = builder.build();
        assert!(result.is_ok());
        
        let client = result.unwrap();
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, "https://custom.api");
    }
}