pub mod builder;
pub mod retry;
#[cfg(feature = "rate-limiting")]
pub mod rate_limiter;

use builder::MistralClientBuilder;
use retry::RetryStrategy;
#[cfg(feature = "rate-limiting")]
use rate_limiter::RateLimiter;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;

/// HTTP client for Mistral AI API
#[derive(Debug)]
pub struct MistralClient {
    /// API key for authentication
    pub api_key: String,
    
    /// Base URL for the API
    pub base_url: String,
    
    /// Retry strategy configuration
    pub retry_strategy: RetryStrategy,
    
    /// Inner HTTP client
    client: Client,
    
    /// Optional rate limiter
    #[cfg(feature = "rate-limiting")]
    pub rate_limiter: Option<RateLimiter>,
}

impl MistralClient {
    /// Create a new client with default configuration
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.mistral.ai".to_string(),
            retry_strategy: RetryStrategy::default(),
            client: Client::new(),
            #[cfg(feature = "rate-limiting")]
            rate_limiter: None,
        }
    }
    
    /// Create a client builder for custom configuration
    pub fn builder() -> MistralClientBuilder {
        MistralClientBuilder::new()
    }
    
    /// Perform a GET request with optional query parameters
    pub async fn get(&self, path: &str, query_params: Option<&[(&str, &str)]>) -> Result<String, crate::error::MistralError> {
        let url = self.build_url(path, query_params);
        let request_builder = self.client.get(&url);
        let request_builder = self.add_authentication(request_builder);
        
        self.execute_with_retry(request_builder).await
    }
    
    /// Perform a POST request with JSON body
    pub async fn post<T: serde::Serialize>(&self, path: &str, body: &T) -> Result<String, crate::error::MistralError> {
        let url = self.build_url(path, None);
        let request_builder = self.client.post(&url).json(body);
        let request_builder = self.add_authentication(request_builder);
        
        self.execute_with_retry(request_builder).await
    }
    
    /// Perform a DELETE request
    pub async fn delete(&self, path: &str) -> Result<String, crate::error::MistralError> {
        let url = self.build_url(path, None);
        let request_builder = self.client.delete(&url);
        let request_builder = self.add_authentication(request_builder);
        
        self.execute_with_retry(request_builder).await
    }
    
    /// Build full URL from path and query parameters
    fn build_url(&self, path: &str, query_params: Option<&[(&str, &str)]>) -> String {
        let mut url = format!("{}{}", self.base_url, path);
        
        if let Some(params) = query_params {
            if !params.is_empty() {
                let query_string = params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                url.push_str(&format!("?{}", query_string));
            }
        }
        
        url
    }
    
    /// Add authentication header to request
    fn add_authentication(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let auth_value = format!("Bearer {}", self.api_key);
        request_builder.header(AUTHORIZATION, auth_value)
    }
    
    /// Execute request with retry logic and rate limiting
    async fn execute_with_retry(&self, request_builder: reqwest::RequestBuilder) -> Result<String, crate::error::MistralError> {
        let mut last_error = None;
        
        for attempt in 0..=self.retry_strategy.max_retries {
            // Apply rate limiting if enabled
            #[cfg(feature = "rate-limiting")]
            if let Some(limiter) = &self.rate_limiter {
                // Wait for available permit
                let _ = limiter.acquire().await;
            }
            
            match self.execute_request(request_builder.try_clone().unwrap()).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    last_error = Some(err);
                    
                    if !self.should_retry(&last_error) || attempt == self.retry_strategy.max_retries {
                        break;
                    }
                    
                    tokio::time::sleep(self.retry_strategy.delay).await;
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| crate::error::MistralError::NetworkError(
            std::io::Error::new(std::io::ErrorKind::Other, "Unknown error")
        )))
    }
    
    /// Execute single request attempt
    async fn execute_request(&self, request_builder: reqwest::RequestBuilder) -> Result<String, crate::error::MistralError> {
        let response = request_builder.send().await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        if !status.is_success() {
            return Err(crate::error::MistralError::from_status(status, &body));
        }
        
        Ok(body)
    }
    
    /// Determine if request should be retried
    fn should_retry(&self, error: &Option<crate::error::MistralError>) -> bool {
        error.as_ref().map_or(false, |err| err.is_retryable())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    #[test]
    fn test_client_creation() {
        let client = MistralClient::new("test-api-key".to_string());
        assert_eq!(client.api_key, "test-api-key");
        assert_eq!(client.base_url, "https://api.mistral.ai");
        assert_eq!(client.retry_strategy.max_retries, 3);
    }

    #[test]
    fn test_client_with_custom_config() {
        let client = MistralClient::builder()
            .api_key("custom-key")
            .base_url("https://custom.url")
            .max_retries(5)
            .retry_delay(Duration::from_millis(200))
            .build()
            .unwrap();
            
        assert_eq!(client.api_key, "custom-key");
        assert_eq!(client.base_url, "https://custom.url");
        assert_eq!(client.retry_strategy.max_retries, 5);
        assert_eq!(client.retry_strategy.delay, Duration::from_millis(200));
    }

    #[test]
    fn test_url_building_with_query_params() {
        let client = MistralClient::new("test-key".to_string());
        
        let params = [("page", "1"), ("limit", "10")];
        let url = client.build_url("/v1/models", Some(&params));
        
        assert_eq!(url, "https://api.mistral.ai/v1/models?page=1&limit=10");
    }

    #[test]
    fn test_url_building_without_query_params() {
        let client = MistralClient::new("test-key".to_string());
        
        let url = client.build_url("/v1/models", None);
        assert_eq!(url, "https://api.mistral.ai/v1/models");
    }


}