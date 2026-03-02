use serde::{Deserialize, Serialize};
use crate::models::{ModelCapabilities, BaseModelCard};

/// Fine-tuned model information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FTModelCard {
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
    
    /// Fine-tuning job identifier
    pub job: String,
    
    /// Base model identifier
    pub root: String,
    
    /// Whether the model is archived
    #[serde(default)]
    pub archived: bool,
}

impl FTModelCard {
    /// Returns the model type
    pub fn model_type(&self) -> &str {
        &self.model_type
    }
    
    /// Converts to base model card (losing FT-specific fields)
    pub fn to_base_model(&self) -> BaseModelCard {
        BaseModelCard {
            id: self.id.clone(),
            object: self.object.clone(),
            created: self.created,
            owned_by: self.owned_by.clone(),
            capabilities: self.capabilities.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            max_context_length: self.max_context_length,
            aliases: self.aliases.clone(),
            deprecation: self.deprecation.clone(),
            deprecation_replacement_model: self.deprecation_replacement_model.clone(),
            default_model_temperature: self.default_model_temperature.clone(),
            model_type: self.model_type.clone(),
        }
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
    fn test_deserialize_ft_model() {
        let json = json!({
            "id": "ft:open-mistral-7b:587a6b29:20240514:7e773925",
            "object": "model",
            "created": 1756746619,
            "owned_by": "mistralai",
            "capabilities": {
                "completion_chat": true,
                "function_calling": false,
                "completion_fim": false,
                "fine_tuning": true,
                "vision": false,
                "classification": false
            },
            "name": "My Fine-Tuned Model",
            "description": "A fine-tuned version",
            "max_context_length": 32768,
            "aliases": [],
            "deprecation": null,
            "deprecation_replacement_model": null,
            "default_model_temperature": null,
            "type": "fine-tuned",
            "job": "ftjob-12345",
            "root": "open-mistral-7b",
            "archived": false
        });

        let model: FTModelCard = serde_json::from_value(json).unwrap();
        assert_eq!(model.id, "ft:open-mistral-7b:587a6b29:20240514:7e773925");
        assert_eq!(model.model_type(), "fine-tuned");
        assert_eq!(model.job, "ftjob-12345");
        assert_eq!(model.root, "open-mistral-7b");
        assert!(!model.archived);
    }

    #[test]
    fn test_serialize_ft_model() {
        let capabilities = ModelCapabilities {
            completion_chat: true,
            function_calling: true,
            completion_fim: false,
            fine_tuning: true,
            vision: false,
            classification: false,
        };

        let model = FTModelCard {
            id: "ft:test-model:123".to_string(),
            object: "model".to_string(),
            created: 1234567890,
            owned_by: "test".to_string(),
            capabilities,
            name: Some("FT Test Model".to_string()),
            description: Some("A fine-tuned test model".to_string()),
            max_context_length: 16384,
            aliases: vec![],
            deprecation: None,
            deprecation_replacement_model: None,
            default_model_temperature: None,
            model_type: "fine-tuned".to_string(),
            job: "ftjob-test".to_string(),
            root: "test-base".to_string(),
            archived: true,
        };

        let json = serde_json::to_value(&model).unwrap();
        assert_eq!(json["id"], "ft:test-model:123");
        assert_eq!(json["type"], "fine-tuned");
        assert_eq!(json["job"], "ftjob-test");
        assert_eq!(json["root"], "test-base");
        assert_eq!(json["archived"], true);
    }

    #[test]
    fn test_to_base_model_conversion() {
        let ft_model = FTModelCard {
            id: "test".to_string(),
            object: "model".to_string(),
            created: 123,
            owned_by: "test".to_string(),
            capabilities: ModelCapabilities::default(),
            name: None,
            description: None,
            max_context_length: 32768,
            aliases: vec![],
            deprecation: None,
            deprecation_replacement_model: None,
            default_model_temperature: None,
            model_type: "fine-tuned".to_string(),
            job: "job1".to_string(),
            root: "base".to_string(),
            archived: false,
        };

        let base_model = ft_model.to_base_model();
        assert_eq!(base_model.id, ft_model.id);
        assert_eq!(base_model.created, ft_model.created);
        assert_eq!(base_model.model_type(), ft_model.model_type());
        // Verify it's a proper BaseModelCard
        assert_eq!(base_model.model_type(), "fine-tuned");
    }
}