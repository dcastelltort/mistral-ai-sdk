pub mod builder;
pub mod retry;

use builder::MistralClientBuilder;

/// Main client struct
#[derive(Debug, Clone)]
pub struct MistralClient {
    pub api_key: String,
    pub base_url: String,
}

impl MistralClient {
    /// Create a new client with default configuration
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.mistral.ai".to_string(),
        }
    }
    
    /// Create a client builder for custom configuration
    pub fn builder() -> MistralClientBuilder {
        MistralClientBuilder::new()
    }
}