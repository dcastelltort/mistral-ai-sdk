use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chat completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// Model to use
    pub model: String,
    
    /// List of messages
    pub messages: Vec<ChatMessage>,
    
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
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of the message sender
    pub role: String,
    
    /// Content of the message
    pub content: String,
    
    /// Name of the sender (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Function call (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<HashMap<String, serde_json::Value>>,
}

/// Chat completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    /// Response ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Creation timestamp
    pub created: i64,
    
    /// Model used
    pub model: String,
    
    /// List of choices
    pub choices: Vec<ChatCompletionChoice>,
    
    /// Usage information
    pub usage: ChatCompletionUsage,
}

/// Chat completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    /// Index of the choice
    pub index: i32,
    
    /// Message content
    pub message: ChatMessage,
    
    /// Finish reason
    pub finish_reason: String,
}

/// Usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionUsage {
    /// Number of prompt tokens
    pub prompt_tokens: i32,
    
    /// Number of completion tokens
    pub completion_tokens: i32,
    
    /// Total number of tokens
    pub total_tokens: i32,
}

/// Chat API client
#[derive(Debug)]
pub struct ChatApi {
    client: MistralClient,
}

impl ChatApi {
    /// Create a new Chat API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// Create a chat completion
    pub async fn create_completion(&self, request: &ChatCompletionRequest) -> Result<ChatCompletionResponse, MistralError> {
        let response = self.client.post("/v1/chat/completions", request).await?;
        let completion: ChatCompletionResponse = serde_json::from_str(&response)?;
        Ok(completion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_chat_completion_request_serialization() {
        let request = ChatCompletionRequest {
            model: "mistral-tiny".to_string(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: "Hello!".to_string(),
                    name: None,
                    function_call: None,
                },
                ChatMessage {
                    role: "assistant".to_string(),
                    content: "Hi there!".to_string(),
                    name: Some("assistant".to_string()),
                    function_call: None,
                },
            ],
            temperature: Some(0.7),
            top_p: Some(0.9),
            max_tokens: Some(100),
            stream: Some(false),
            stop: Some(vec!["\\n".to_string()]),
            presence_penalty: Some(0.1),
            frequency_penalty: Some(0.2),
            user: Some("user-123".to_string()),
        };
        
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["model"], "mistral-tiny");
        assert_eq!(json["messages"][0]["content"], "Hello!");
        assert_eq!(json["messages"][1]["name"], "assistant");
        assert!(json["temperature"].as_f64().unwrap().abs() - 0.7 < f64::EPSILON);
        assert_eq!(json["max_tokens"], 100);
    }

    #[test]
    fn test_chat_message_serialization() {
        let message = ChatMessage {
            role: "function".to_string(),
            content: "Function response".to_string(),
            name: Some("get_weather".to_string()),
            function_call: Some(HashMap::from([
                ("name".to_string(), json!("get_weather")),
                ("arguments".to_string(), json!({"location": "Paris"}))
            ])),
        };
        
        let json = serde_json::to_value(&message).unwrap();
        assert_eq!(json["role"], "function");
        assert_eq!(json["name"], "get_weather");
        assert!(json["function_call"].is_object());
    }

    #[test]
    fn test_chat_completion_response_deserialization() {
        let json = json!({
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1234567890,
            "model": "mistral-tiny",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "Hello! How can I help you today?"
                    },
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20,
                "total_tokens": 30
            }
        });
        
        let response: ChatCompletionResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.id, "chatcmpl-123");
        assert_eq!(response.model, "mistral-tiny");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.usage.total_tokens, 30);
    }

    #[test]
    fn test_chat_completion_choice_deserialization() {
        let json = json!({
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Response content"
            },
            "finish_reason": "length"
        });
        
        let choice: ChatCompletionChoice = serde_json::from_value(json).unwrap();
        assert_eq!(choice.index, 0);
        assert_eq!(choice.message.content, "Response content");
        assert_eq!(choice.finish_reason, "length");
    }

    #[test]
    fn test_chat_usage_deserialization() {
        let json = json!({
            "prompt_tokens": 15,
            "completion_tokens": 25,
            "total_tokens": 40
        });
        
        let usage: ChatCompletionUsage = serde_json::from_value(json).unwrap();
        assert_eq!(usage.prompt_tokens, 15);
        assert_eq!(usage.completion_tokens, 25);
        assert_eq!(usage.total_tokens, 40);
    }

    #[test]
    fn test_chat_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = ChatApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }

    #[test]
    fn test_minimal_chat_request() {
        let request = ChatCompletionRequest {
            model: "mistral-tiny".to_string(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: "Hello!".to_string(),
                    name: None,
                    function_call: None,
                }
            ],
            temperature: None,
            top_p: None,
            max_tokens: None,
            stream: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            user: None,
        };
        
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["model"], "mistral-tiny");
        assert_eq!(json["messages"][0]["content"], "Hello!");
        assert!(!json.as_object().unwrap().contains_key("temperature"));
    }
}