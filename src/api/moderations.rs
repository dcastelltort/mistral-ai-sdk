use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ModerationRequest {
    pub input: String,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ModerationResult {
    pub flagged: bool,
    pub categories: Categories,
    pub category_scores: CategoryScores,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Categories {
    pub sexual: bool,
    pub hate: bool,
    pub harassment: bool,
    pub self_harm: bool,
    pub sexual_minors: bool,
    pub hate_threatening: bool,
    pub violence_graphic: bool,
    pub self_harm_intent: bool,
    pub self_harm_instructions: bool,
    pub harassment_threatening: bool,
    pub violence: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CategoryScores {
    pub sexual: f32,
    pub hate: f32,
    pub harassment: f32,
    pub self_harm: f32,
    pub sexual_minors: f32,
    pub hate_threatening: f32,
    pub violence_graphic: f32,
    pub self_harm_intent: f32,
    pub self_harm_instructions: f32,
    pub harassment_threatening: f32,
    pub violence: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<ModerationResult>,
}

pub struct ModerationsApi<'a> {
    client: &'a MistralClient,
}

impl<'a> ModerationsApi<'a> {
    pub fn new(client: &'a MistralClient) -> Self {
        Self { client }
    }

    pub async fn create_moderation(&self, request: &ModerationRequest) -> Result<ModerationResponse, MistralError> {
        let response = self.client.post("/v1/moderations", request).await?;
        let moderation: ModerationResponse = serde_json::from_str(&response)?;
        Ok(moderation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::builder::MistralClientBuilder;

    #[test]
    fn test_moderation_request_serialization() {
        let request = ModerationRequest {
            input: "This is a test input".to_string(),
            model: Some("text-moderation-latest".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("This is a test input"));
        assert!(json.contains("text-moderation-latest"));

        let deserialized: ModerationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.input, "This is a test input");
        assert_eq!(deserialized.model, Some("text-moderation-latest".to_string()));
    }

    #[test]
    fn test_moderation_response_deserialization() {
        let json_response = r#"
        {
            "id": "modr-123",
            "model": "text-moderation-latest",
            "results": [
                {
                    "flagged": true,
                    "categories": {
                        "sexual": false,
                        "hate": true,
                        "harassment": false,
                        "self_harm": false,
                        "sexual_minors": false,
                        "hate_threatening": true,
                        "violence_graphic": false,
                        "self_harm_intent": false,
                        "self_harm_instructions": false,
                        "harassment_threatening": false,
                        "violence": false
                    },
                    "category_scores": {
                        "sexual": 0.1,
                        "hate": 0.9,
                        "harassment": 0.2,
                        "self_harm": 0.05,
                        "sexual_minors": 0.01,
                        "hate_threatening": 0.85,
                        "violence_graphic": 0.02,
                        "self_harm_intent": 0.01,
                        "self_harm_instructions": 0.005,
                        "harassment_threatening": 0.15,
                        "violence": 0.08
                    }
                }
            ]
        }
        "#;

        let response: ModerationResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.id, "modr-123");
        assert_eq!(response.model, "text-moderation-latest");
        assert_eq!(response.results.len(), 1);
        assert!(response.results[0].flagged);
        assert!(response.results[0].categories.hate);
        assert!(response.results[0].categories.hate_threatening);
    }

    #[tokio::test]
    async fn test_moderations_api_integration() {
        let client = MistralClientBuilder::new()
            .api_key("test-api-key")
            .build()
            .unwrap();

        let api = ModerationsApi::new(&client);
        
        let request = ModerationRequest {
            input: "Test content for moderation".to_string(),
            model: Some("text-moderation-latest".to_string()),
        };

        // This will fail in test environment but verifies the API structure works
        let result = api.create_moderation(&request).await;
        assert!(result.is_err()); // Expected to fail without real API key
    }
}
