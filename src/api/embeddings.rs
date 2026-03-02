use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};

/// Embedding request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Input text to embed
    pub input: String,
    
    /// Model to use (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    
    /// Encoding format (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
    
    /// User identifier (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Embedding response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Object type
    pub object: String,
    
    /// List of embedding data
    pub data: Vec<EmbeddingData>,
    
    /// Model used
    pub model: String,
    
    /// Usage information
    pub usage: EmbeddingUsage,
}

/// Embedding data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingData {
    /// Object type
    pub object: String,
    
    /// Embedding vector
    pub embedding: Vec<f32>,
    
    /// Index
    pub index: i32,
}

/// Usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingUsage {
    /// Number of prompt tokens
    pub prompt_tokens: i32,
    
    /// Total number of tokens
    pub total_tokens: i32,
}

/// Embeddings API client
#[derive(Debug)]
pub struct EmbeddingsApi {
    client: MistralClient,
}

impl EmbeddingsApi {
    /// Create a new Embeddings API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// Create embeddings
    pub async fn create_embeddings(&self, request: &EmbeddingRequest) -> Result<EmbeddingResponse, MistralError> {
        let response = self.client.post("/v1/embeddings", request).await?;
        let embeddings: EmbeddingResponse = serde_json::from_str(&response)?;
        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_embedding_request_serialization() {
        let request = EmbeddingRequest {
            input: "Hello world".to_string(),
            model: Some("mistral-embed".to_string()),
            encoding_format: Some("float".to_string()),
            user: Some("user-123".to_string()),
        };
        
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["input"], "Hello world");
        assert_eq!(json["model"], "mistral-embed");
        assert_eq!(json["encoding_format"], "float");
        assert_eq!(json["user"], "user-123");
    }

    #[test]
    fn test_embedding_response_deserialization() {
        let json = json!({
            "object": "list",
            "data": [
                {
                    "object": "embedding",
                    "embedding": [0.1, 0.2, 0.3, 0.4, 0.5],
                    "index": 0
                }
            ],
            "model": "mistral-embed",
            "usage": {
                "prompt_tokens": 2,
                "total_tokens": 2
            }
        });
        
        let response: EmbeddingResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.object, "list");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].embedding.len(), 5);
        assert_eq!(response.usage.prompt_tokens, 2);
    }

    #[test]
    fn test_embedding_data_deserialization() {
        let json = json!({
            "object": "embedding",
            "embedding": [0.1, 0.2, 0.3],
            "index": 0
        });
        
        let data: EmbeddingData = serde_json::from_value(json).unwrap();
        assert_eq!(data.object, "embedding");
        assert_eq!(data.embedding, vec![0.1, 0.2, 0.3]);
        assert_eq!(data.index, 0);
    }

    #[test]
    fn test_embedding_usage_deserialization() {
        let json = json!({
            "prompt_tokens": 5,
            "total_tokens": 5
        });
        
        let usage: EmbeddingUsage = serde_json::from_value(json).unwrap();
        assert_eq!(usage.prompt_tokens, 5);
        assert_eq!(usage.total_tokens, 5);
    }

    #[test]
    fn test_embeddings_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = EmbeddingsApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }

    #[test]
    fn test_minimal_embedding_request() {
        let request = EmbeddingRequest {
            input: "Test text".to_string(),
            model: None,
            encoding_format: None,
            user: None,
        };
        
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["input"], "Test text");
        assert!(!json.as_object().unwrap().contains_key("model"));
    }

    #[test]
    fn test_embedding_with_multiple_vectors() {
        let json = json!({
            "object": "list",
            "data": [
                {
                    "object": "embedding",
                    "embedding": [0.1, 0.2],
                    "index": 0
                },
                {
                    "object": "embedding",
                    "embedding": [0.3, 0.4],
                    "index": 1
                }
            ],
            "model": "mistral-embed",
            "usage": {
                "prompt_tokens": 4,
                "total_tokens": 4
            }
        });
        
        let response: EmbeddingResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].embedding, vec![0.1, 0.2]);
        assert_eq!(response.data[1].embedding, vec![0.3, 0.4]);
    }
}