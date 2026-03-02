use serde::{Deserialize, Serialize};
use crate::models::ModelCapabilities;

/// Base model information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseModelCard {
    /// Model identifier
    pub id: String,
    
    /// Object type
    #[serde(default = "default_object")]
    pub object: String,
    
    /// Creation timestamp
    pub created: i64,
    
    /// Organization that owns the model
    #[serde(default = "default_owned_by")]
    pub owned_by: String,
    
    /// Model capabilities
    pub capabilities: ModelCapabilities,
    
    /// Optional model name
    #[serde(default)]
    pub name: Option<String>,
    
    /// Optional model description
    #[serde(default)]
    pub description: Option<String>,
    
    /// Maximum context length
    #[serde(default = "default_max_context_length")]
    pub max_context_length: i32,
    
    /// Model aliases
    #[serde(default)]
    pub aliases: Vec<String>,
    
    /// Optional deprecation timestamp
    #[serde(default)]
    pub deprecation: Option<String>,
    
    /// Optional deprecation replacement model
    #[serde(default)]
    pub deprecation_replacement_model: Option<String>,
    
    /// Optional default model temperature
    #[serde(default)]
    pub default_model_temperature: Option<f32>,
    
    /// Model type discriminator
    #[serde(rename = "type")]
    pub model_type: String,
}

impl BaseModelCard {
    /// Returns the model type
    pub fn model_type(&self) -> &str {
        &self.model_type
    }
}

fn default_object() -> String {
    "model".to_string()
}

fn default_owned_by() -> String {
    "mistralai".to_string()
}

fn default_max_context_length() -> i32 {
    32768
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::models::ModelCapabilities;

    #[test]
    fn test_deserialize_base_model() {
        let json = json!({
            "id": "open-mistral-7b",
            "object": "model",
            "created": 1756746619,
            "owned_by": "mistralai",
            "capabilities": {
                "completion_chat": true,
                "function_calling": false,
                "completion_fim": false,
                "fine_tuning": false,
                "vision": false,
                "classification": false
            },
            "name": "Mistral 7B",
            "description": "A 7B parameter model",
            "max_context_length": 32768,
            "aliases": ["mistral-7b"],
            "deprecation": null,
            "deprecation_replacement_model": null,
            "default_model_temperature": 0.7,
            "type": "base"
        });

        let model: BaseModelCard = serde_json::from_value(json).unwrap();
        assert_eq!(model.id, "open-mistral-7b");
        assert_eq!(model.object, "model");
        assert_eq!(model.created, 1756746619);
        assert_eq!(model.owned_by, "mistralai");
        assert!(model.capabilities.completion_chat);
        assert_eq!(model.name.as_deref(), Some("Mistral 7B"));
        assert_eq!(model.max_context_length, 32768);
        assert_eq!(model.aliases, vec!["mistral-7b"]);
        assert_eq!(model.model_type(), "base");
    }

    #[test]
    fn test_serialize_base_model() {
        let capabilities = ModelCapabilities {
            completion_chat: true,
            function_calling: false,
            completion_fim: false,
            fine_tuning: false,
            vision: false,
            classification: false,
        };

        let model = BaseModelCard {
            id: "test-model".to_string(),
            object: "model".to_string(),
            created: 1234567890,
            owned_by: "test".to_string(),
            capabilities,
            name: Some("Test Model".to_string()),
            description: Some("A test model".to_string()),
            max_context_length: 8192,
            aliases: vec!["test-alias".to_string()],
            deprecation: None,
            deprecation_replacement_model: None,
            default_model_temperature: Some(0.5),
            model_type: "base".to_string(),
        };

        let json = serde_json::to_value(&model).unwrap();
        assert_eq!(json["id"], "test-model");
        assert_eq!(json["type"], "base");
    }

    #[test]
    fn test_default_values() {
        let capabilities = ModelCapabilities::default();
        let model = BaseModelCard {
            id: "test".to_string(),
            object: "custom".to_string(), // Should be overridden by default
            created: 0,
            owned_by: "custom".to_string(), // Should be overridden by default
            capabilities,
            name: None,
            description: None,
            max_context_length: 1000, // Should be overridden by default
            aliases: vec![],
            deprecation: None,
            deprecation_replacement_model: None,
            default_model_temperature: None,
            model_type: "base".to_string(),
        };

        let json = serde_json::to_value(&model).unwrap();
        // These should use the default values from serde attributes
        assert_eq!(json["object"], "custom"); // Not overridden by serde default
        assert_eq!(json["owned_by"], "custom"); // Not overridden by serde default
        assert_eq!(json["max_context_length"], 1000); // Not overridden by serde default
    }
}