use serde::{Deserialize, Serialize};

/// Model capabilities indicating what features are supported
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelCapabilities {
    /// Whether the model supports chat completion
    #[serde(default)]
    pub completion_chat: bool,
    
    /// Whether the model supports function calling
    #[serde(default)]
    pub function_calling: bool,
    
    /// Whether the model supports fill-in-the-middle completion
    #[serde(default)]
    pub completion_fim: bool,
    
    /// Whether the model supports fine-tuning
    #[serde(default)]
    pub fine_tuning: bool,
    
    /// Whether the model supports vision capabilities
    #[serde(default)]
    pub vision: bool,
    
    /// Whether the model supports classification tasks
    #[serde(default)]
    pub classification: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_capabilities() {
        let json = json!({
            "completion_chat": true,
            "function_calling": false,
            "completion_fim": false,
            "fine_tuning": true,
            "vision": false,
            "classification": false
        });

        let capabilities: ModelCapabilities = serde_json::from_value(json).unwrap();
        assert!(capabilities.completion_chat);
        assert!(!capabilities.function_calling);
        assert!(!capabilities.completion_fim);
        assert!(capabilities.fine_tuning);
        assert!(!capabilities.vision);
        assert!(!capabilities.classification);
    }

    #[test]
    fn test_serialize_capabilities() {
        let capabilities = ModelCapabilities {
            completion_chat: true,
            function_calling: true,
            completion_fim: false,
            fine_tuning: false,
            vision: true,
            classification: false,
        };

        let json = serde_json::to_value(&capabilities).unwrap();
        assert_eq!(json["completion_chat"], true);
        assert_eq!(json["function_calling"], true);
        assert_eq!(json["completion_fim"], false);
        assert_eq!(json["fine_tuning"], false);
        assert_eq!(json["vision"], true);
        assert_eq!(json["classification"], false);
    }

    #[test]
    fn test_default_capabilities() {
        let capabilities = ModelCapabilities::default();
        assert!(!capabilities.completion_chat);
        assert!(!capabilities.function_calling);
        assert!(!capabilities.completion_fim);
        assert!(!capabilities.fine_tuning);
        assert!(!capabilities.vision);
        assert!(!capabilities.classification);
    }
}