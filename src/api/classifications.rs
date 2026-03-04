// Classifications API implementation
// This module provides functionality for text classification using Mistral AI models

use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Classification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationRequest {
    /// ID of the model to use
    pub model: String,
    
    /// Text to classify (can be string or array of strings)
    pub input: ClassificationInput,
    
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Classification input type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClassificationInput {
    /// Single text input
    Single(String),
    /// Multiple text inputs
    Multiple(Vec<String>),
}

/// Chat classification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatClassificationRequest {
    /// ID of the model to use
    pub model: String,
    
    /// Chat messages to classify
    pub messages: Vec<ChatMessage>,
    
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Chat message for classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of the message sender
    pub role: String,
    
    /// Content of the message
    pub content: String,
    
    /// Optional name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Classification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResponse {
    /// Classification ID
    pub id: String,
    
    /// Model used
    pub model: String,
    
    /// Classification results
    pub results: Vec<ClassificationResult>,
}

/// Classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    /// Classification scores by category
    pub scores: HashMap<String, f64>,
}

/// Classifications API client
#[derive(Debug)]
pub struct ClassificationsApi {
    client: MistralClient,
}

impl ClassificationsApi {
    /// Create a new Classifications API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// Perform text classification
    pub async fn classify(&self, request: &ClassificationRequest) -> Result<ClassificationResponse, MistralError> {
        let response = self.client.post("/v1/classifications", request).await?;
        let classification: ClassificationResponse = serde_json::from_str(&response)?;
        Ok(classification)
    }
    
    /// Perform chat message classification
    pub async fn classify_chat(&self, request: &ChatClassificationRequest) -> Result<ClassificationResponse, MistralError> {
        let response = self.client.post("/v1/chat/classifications", request).await?;
        let classification: ClassificationResponse = serde_json::from_str(&response)?;
        Ok(classification)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_classification_request_serialization() {
        let request = ClassificationRequest {
            model: "mistral-moderation-latest".to_string(),
            input: ClassificationInput::Single("Test content to classify".to_string()),
            metadata: Some(HashMap::from([
                ("user_id".to_string(), json!("123")),
                ("context".to_string(), json!("test")),
            ])),
        };

        let json = serde_json::to_value(request).unwrap();
        assert_eq!(json["model"], "mistral-moderation-latest");
        assert_eq!(json["input"], "Test content to classify");
        assert_eq!(json["metadata"]["user_id"], "123");
    }

    #[test]
    fn test_classification_input_serialization() {
        // Test single input
        let single_input = ClassificationInput::Single("Single text".to_string());
        let json_single = serde_json::to_value(single_input).unwrap();
        assert_eq!(json_single, "Single text");

        // Test multiple inputs
        let multiple_input = ClassificationInput::Multiple(vec![
            "Text 1".to_string(),
            "Text 2".to_string(),
        ]);
        let json_multiple = serde_json::to_value(multiple_input).unwrap();
        assert_eq!(json_multiple, json!( ["Text 1", "Text 2"] ));
    }

    #[test]
    fn test_chat_classification_request_serialization() {
        let request = ChatClassificationRequest {
            model: "mistral-moderation-latest".to_string(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: "Hello, how are you?".to_string(),
                    name: None,
                },
                ChatMessage {
                    role: "assistant".to_string(),
                    content: "I'm doing well, thank you!".to_string(),
                    name: Some("assistant".to_string()),
                },
            ],
            metadata: None,
        };

        let json = serde_json::to_value(request).unwrap();
        assert_eq!(json["model"], "mistral-moderation-latest");
        assert_eq!(json["messages"][0]["role"], "user");
        assert_eq!(json["messages"][1]["name"], "assistant");
    }

    #[test]
    fn test_classification_response_deserialization() {
        let json = json!({
            "id": "mod-e5cc70bb28c444948073e77776eb30ef",
            "model": "mistral-moderation-latest",
            "results": [
                {
                    "scores": {
                        "hate": 0.0001,
                        "harassment": 0.0002,
                        "violence": 0.00005,
                        "sexual": 0.00001,
                        "self_harm": 0.000001,
                        "sexual_minors": 0.0000001
                    }
                }
            ]
        });

        let response: ClassificationResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.id, "mod-e5cc70bb28c444948073e77776eb30ef");
        assert_eq!(response.model, "mistral-moderation-latest");
        assert_eq!(response.results.len(), 1);
        assert!(response.results[0].scores.contains_key("hate"));
        assert!(response.results[0].scores["hate"] > 0.0);
    }

    #[test]
    fn test_classifications_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = ClassificationsApi::new(client);
        assert_eq!(api.client.api_key, "test-key");
    }
}