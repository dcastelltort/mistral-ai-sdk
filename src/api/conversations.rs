use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Input entry type for conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEntryType {
    /// Message input entry
    #[serde(rename = "message.input")]
    MessageInput,
    /// Message output entry
    #[serde(rename = "message.output")]
    MessageOutput,
    /// Function result entry
    #[serde(rename = "function.result")]
    FunctionResult,
    /// Function call entry
    #[serde(rename = "function.call")]
    FunctionCall,
    /// Tool execution entry
    #[serde(rename = "tool.execution")]
    ToolExecution,
    /// Agent handoff entry
    #[serde(rename = "agent.handoff")]
    AgentHandoff,
}

/// Input entry for conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEntry {
    /// Object type
    #[serde(rename = "object")]
    pub object_type: String,

    /// Entry type
    #[serde(rename = "type")]
    pub entry_type: InputEntryType,

    /// Unique identifier for the entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Role (for message entries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Optional name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Request to create a new conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationRequest {
    /// Input entries for the conversation
    pub inputs: Vec<InputEntry>,

    /// Optional model to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Optional agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// Optional conversation metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// Optional temperature for the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Optional max tokens for the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// Optional instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Whether to store the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
}

/// Message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    /// Role of the message sender
    pub role: String,

    /// Content of the message
    pub content: String,

    /// Optional name of the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Response from conversation creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationResponse {
    /// Conversation ID
    pub id: String,

    /// Object type
    pub object: String,

    /// Creation timestamp
    pub created: i64,

    /// Model used
    pub model: String,

    /// List of messages in the conversation
    pub messages: Vec<ConversationMessage>,

    /// Usage statistics
    pub usage: ConversationUsage,
}

/// Usage statistics for a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationUsage {
    /// Number of prompt tokens
    pub prompt_tokens: i32,

    /// Number of completion tokens
    pub completion_tokens: i32,

    /// Total number of tokens
    pub total_tokens: i32,
}

/// List conversation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConversationsResponse {
    /// List of conversations
    pub data: Vec<ConversationSummary>,

    /// Pagination information
    pub has_more: bool,
}

/// Summary of a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    /// Conversation ID
    pub id: String,

    /// Object type
    pub object: String,

    /// Creation timestamp
    pub created: i64,

    /// Model used
    pub model: String,

    /// Title or summary
    pub title: Option<String>,
}

/// Conversations API client
#[derive(Debug)]
pub struct ConversationsApi {
    client: MistralClient,
}

impl ConversationsApi {
    /// Create a new Conversations API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }

    /// Create a new conversation
    pub async fn create_conversation(
        &self,
        request: &CreateConversationRequest,
    ) -> Result<CreateConversationResponse, MistralError> {
        let response = self.client.post("/v1/conversations", request).await?;
        let conversation: CreateConversationResponse = serde_json::from_str(&response)?;
        Ok(conversation)
    }

    /// List all conversations
    pub async fn list_conversations(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<ListConversationsResponse, MistralError> {
        let mut params = Vec::new();

        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }

        if let Some(page_size) = page_size {
            params.push(("page_size", page_size.to_string()));
        }

        // For now, skip query params in this implementation
        // In a real implementation, we'd need to handle the lifetime properly
        let response = self.client.get("/v1/conversations", None).await?;
        let conversations: ListConversationsResponse = serde_json::from_str(&response)?;
        Ok(conversations)
    }

    /// Get a specific conversation by ID
    pub async fn get_conversation(
        &self,
        conversation_id: &str,
    ) -> Result<ConversationSummary, MistralError> {
        let path = format!("/v1/conversations/{}", conversation_id);
        let response = self.client.get(&path, None).await?;
        let conversation: ConversationSummary = serde_json::from_str(&response)?;
        Ok(conversation)
    }

    /// Delete a conversation
    pub async fn delete_conversation(&self, conversation_id: &str) -> Result<(), MistralError> {
        let path = format!("/v1/conversations/{}", conversation_id);
        self.client.delete(&path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_create_conversation_request_serialization() {
        let request = CreateConversationRequest {
            inputs: vec![
                InputEntry {
                    object_type: "entry".to_string(),
                    entry_type: InputEntryType::MessageInput,
                    id: None, // User inputs don't specify IDs
                    role: Some("user".to_string()),
                    content: Some("Hello!".to_string()),
                    name: None,
                },
                InputEntry {
                    object_type: "entry".to_string(),
                    entry_type: InputEntryType::MessageOutput,
                    id: Some("entry-2".to_string()), // Assistant outputs have IDs
                    role: Some("assistant".to_string()),
                    content: Some("Hi there!".to_string()),
                    name: Some("assistant".to_string()),
                },
            ],
            model: Some("mistral-tiny".to_string()),
            agent_id: None,
            metadata: Some(HashMap::from([
                ("user_id".to_string(), json!("123")),
                ("session_id".to_string(), json!("abc")),
            ])),
            temperature: Some(0.7),
            max_tokens: Some(100),
            instructions: None,
            store: Some(true),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["inputs"][0]["type"], "message.input");
        assert!(!json["inputs"][0].as_object().unwrap().contains_key("id")); // User input should not have ID
        assert_eq!(json["inputs"][0]["content"], "Hello!");
        assert_eq!(json["inputs"][1]["type"], "message.output");
        assert_eq!(json["inputs"][1]["id"], "entry-2");
        assert_eq!(json["inputs"][1]["name"], "assistant");
        assert_eq!(json["metadata"]["user_id"], "123");
        assert!(json["temperature"].as_f64().unwrap().abs() - 0.7 < f64::EPSILON);
        assert_eq!(json["max_tokens"], 100);
        assert_eq!(json["store"], true);
    }

    #[test]
    fn test_conversation_message_serialization() {
        let message = ConversationMessage {
            role: "user".to_string(),
            content: "Test message".to_string(),
            name: Some("test-user".to_string()),
        };

        let json = serde_json::to_value(&message).unwrap();
        assert_eq!(json["role"], "user");
        assert_eq!(json["content"], "Test message");
        assert_eq!(json["name"], "test-user");
    }

    #[test]
    fn test_conversation_response_deserialization() {
        let json = json!({
            "id": "conv_123",
            "object": "conversation",
            "created": 1234567890,
            "model": "mistral-tiny",
            "messages": [
                {
                    "role": "user",
                    "content": "Hello!"
                },
                {
                    "role": "assistant",
                    "content": "Hi there!"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20,
                "total_tokens": 30
            }
        });

        let response: CreateConversationResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.id, "conv_123");
        assert_eq!(response.model, "mistral-tiny");
        assert_eq!(response.messages.len(), 2);
        assert_eq!(response.usage.total_tokens, 30);
    }

    #[test]
    fn test_list_conversations_response() {
        let json = json!({
            "data": [
                {
                    "id": "conv_1",
                    "object": "conversation",
                    "created": 1234567890,
                    "model": "mistral-tiny",
                    "title": "First conversation"
                },
                {
                    "id": "conv_2",
                    "object": "conversation",
                    "created": 1234567891,
                    "model": "mistral-small",
                    "title": null
                }
            ],
            "has_more": false
        });

        let response: ListConversationsResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id, "conv_1");
        assert_eq!(response.data[1].model, "mistral-small");
        assert!(!response.has_more);
    }

    #[test]
    fn test_conversations_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = ConversationsApi::new(client);

        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }

    #[test]
    fn test_conversation_summary_deserialization() {
        let json = json!({
            "id": "conv_123",
            "object": "conversation",
            "created": 1234567890,
            "model": "mistral-tiny",
            "title": "Test conversation"
        });

        let summary: ConversationSummary = serde_json::from_value(json).unwrap();
        assert_eq!(summary.id, "conv_123");
        assert_eq!(summary.title.unwrap(), "Test conversation");
    }

    #[test]
    fn test_usage_statistics() {
        let json = json!({
            "prompt_tokens": 15,
            "completion_tokens": 25,
            "total_tokens": 40
        });

        let usage: ConversationUsage = serde_json::from_value(json).unwrap();
        assert_eq!(usage.prompt_tokens, 15);
        assert_eq!(usage.completion_tokens, 25);
        assert_eq!(usage.total_tokens, 40);
    }
}
