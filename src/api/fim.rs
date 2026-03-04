use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};

/// FIM (Fill-in-the-Middle) completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FIMCompletionRequest {
    /// Model to use for FIM completion
    pub model: String,
    
    /// The text/code to complete (required)
    pub prompt: String,
    
    /// Temperature (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    
    /// Top P (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    
    /// Maximum tokens (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    
    /// Stream response (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    
    /// Stop sequences (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    
    /// Presence penalty (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    
    /// Frequency penalty (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    
    /// User identifier (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    
    /// Suffix to use for FIM (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    
    /// Random seed (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i32>,
    
    /// Minimum tokens (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<i32>,
    
    /// Metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// FIM completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FIMCompletionResponse {
    /// Response ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Creation timestamp
    pub created: i64,
    
    /// Model used
    pub model: String,
    
    /// Usage information
    pub usage: FIMUsage,
    
    /// List of choices
    pub choices: Vec<FIMChoice>,
}

/// FIM usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FIMUsage {
    /// Number of prompt tokens
    pub prompt_tokens: i32,
    
    /// Number of completion tokens
    pub completion_tokens: i32,
    
    /// Total number of tokens
    pub total_tokens: i32,
}

/// FIM completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FIMChoice {
    /// Choice index
    pub index: i32,
    
    /// Generated message content
    pub content: String,
    
    /// Finish reason
    pub finish_reason: String,
}

/// FIM API client
#[derive(Debug)]
pub struct FIMApi {
    client: MistralClient,
}

impl FIMApi {
    /// Create a new FIM API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }

    /// Create a FIM completion
    pub async fn create_completion(&self, request: &FIMCompletionRequest) -> Result<FIMCompletionResponse, MistralError> {
        let response = self.client.post("/v1/fim/completions", request).await?;
        let completion: FIMCompletionResponse = serde_json::from_str(&response)?;
        Ok(completion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;

    #[test]
    fn test_fim_completion_request_serialization() {
        let request = FIMCompletionRequest {
            model: "codestral-latest".to_string(),
            prompt: "def add_numbers(a: int, b: int) -> int:".to_string(),
            suffix: Some("    \"\"\"Add two numbers\"\"\"\n    ".to_string()),
            temperature: Some(0.7),
            max_tokens: Some(100),
            stream: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            top_p: None,
            user: None,
            random_seed: None,
            min_tokens: None,
            metadata: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("codestral-latest"));
        assert!(json.contains("def add_numbers"));
        assert!(json.contains("0.7"));

        let deserialized: FIMCompletionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.model, "codestral-latest");
        assert_eq!(deserialized.prompt, "def add_numbers(a: int, b: int) -> int:");
        assert_eq!(deserialized.temperature, Some(0.7));
    }

    #[test]
    fn test_fim_completion_response_deserialization() {
        let json_response = r#"
        {
            "id": "447e3e0d457e42e98248b5d2ef52a2a3",
            "object": "chat.completion",
            "model": "codestral-latest",
            "usage": {
                "prompt_tokens": 8,
                "completion_tokens": 91,
                "total_tokens": 99
            },
            "created": 1759496862,
            "choices": [
                {
                    "index": 0,
                    "content": "    return a + b",
                    "finish_reason": "stop"
                }
            ]
        }
        "#;

        let response: FIMCompletionResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.id, "447e3e0d457e42e98248b5d2ef52a2a3");
        assert_eq!(response.model, "codestral-latest");
        assert_eq!(response.usage.prompt_tokens, 8);
        assert_eq!(response.usage.completion_tokens, 91);
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.choices[0].content, "    return a + b");
    }

    #[test]
    fn test_fim_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = FIMApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }
}
