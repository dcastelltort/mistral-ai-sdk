use crate::client::MistralClient;
use crate::error::MistralError;
use crate::models::{BaseModelCard, FTModelCard};
use serde::{Deserialize, Serialize};

/// Response type for listing models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListResponse {
    /// List of models
    pub data: Vec<ModelListItem>,
}

/// Individual model item in the list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelListItem {
    Base(BaseModelCard),
    FineTuned(FTModelCard),
}

/// Response type for deleting a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelResponse {
    /// Model ID that was deleted
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Deletion status
    pub deleted: bool,
}

/// Models API client
#[derive(Debug)]
pub struct ModelsApi {
    client: MistralClient,
}

impl ModelsApi {
    /// Create a new Models API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// List all available models
    pub async fn list_models(&self) -> Result<ModelListResponse, MistralError> {
        let response = self.client.get("/v1/models", None).await?;
        let models: ModelListResponse = serde_json::from_str(&response)?;
        Ok(models)
    }
    
    /// Retrieve a specific model by ID
    pub async fn retrieve_model(&self, model_id: &str) -> Result<ModelListItem, MistralError> {
        let path = format!("/v1/models/{}", model_id);
        let response = self.client.get(&path, None).await?;
        
        // Try to deserialize as FTModelCard first, then BaseModelCard
        if let Ok(ft_model) = serde_json::from_str::<FTModelCard>(&response) {
            return Ok(ModelListItem::FineTuned(ft_model));
        }
        
        let base_model = serde_json::from_str::<BaseModelCard>(&response)?;
        Ok(ModelListItem::Base(base_model))
    }
    
    /// Delete a fine-tuned model
    pub async fn delete_model(&self, model_id: &str) -> Result<DeleteModelResponse, MistralError> {
        let path = format!("/v1/models/{}", model_id);
        let response = self.client.delete(&path).await?;
        let result: DeleteModelResponse = serde_json::from_str(&response)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use crate::models::{ModelCapabilities, BaseModelCard, FTModelCard};
    use serde_json::json;

    #[test]
    fn test_model_list_item_serialization() {
        let base_model = BaseModelCard {
            id: "base-model".to_string(),
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
            model_type: "base".to_string(),
        };
        
        let ft_model = FTModelCard {
            id: "ft-model".to_string(),
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
            job: "job-123".to_string(),
            root: "base-model".to_string(),
            archived: false,
        };
        
        let list_response = ModelListResponse {
            data: vec![
                ModelListItem::Base(base_model),
                ModelListItem::FineTuned(ft_model),
            ],
        };
        
        let json = serde_json::to_value(&list_response).unwrap();
        assert!(json["data"][0]["id"] == "base-model");
        assert!(json["data"][1]["id"] == "ft-model");
    }

    #[test]
    fn test_delete_model_response() {
        let json = json!({
            "id": "ft:model:123",
            "object": "model",
            "deleted": true
        });
        
        let response: DeleteModelResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.id, "ft:model:123");
        assert_eq!(response.object, "model");
        assert!(response.deleted);
    }

    #[test]
    fn test_models_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = ModelsApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }

    #[test]
    fn test_model_list_item_creation() {
        let base_model = BaseModelCard {
            id: "base-model".to_string(),
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
            model_type: "base".to_string(),
        };
        
        let ft_model = FTModelCard {
            id: "ft-model".to_string(),
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
            job: "job-123".to_string(),
            root: "base-model".to_string(),
            archived: false,
        };
        
        let base_item = ModelListItem::Base(base_model);
        let ft_item = ModelListItem::FineTuned(ft_model);
        
        match base_item {
            ModelListItem::Base(model) => assert_eq!(model.id, "base-model"),
            _ => panic!("Expected base model"),
        }
        
        match ft_item {
            ModelListItem::FineTuned(model) => assert_eq!(model.id, "ft-model"),
            _ => panic!("Expected fine-tuned model"),
        }
    }
}