// Agents API implementation
// This module provides functionality for managing Mistral AI agents

use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent tool type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentTool {
    /// Function tool
    Function {
        /// Function name
        name: String,
        /// Function description
        description: String,
        /// Function parameters
        parameters: serde_json::Value,
    },
    /// Web search tool
    WebSearch,
    /// Web search premium tool
    WebSearchPremium,
    /// Code interpreter tool
    CodeInterpreter,
    /// Image generation tool
    ImageGeneration,
    /// Document library tool
    DocumentLibrary,
}

/// Completion arguments for agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionArgs {
    /// Temperature for completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    
    /// Maximum tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    
    /// Top probability mass
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

/// Agent creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    /// Model to use for the agent
    pub model: String,
    
    /// Agent name
    pub name: String,
    
    /// Instruction prompt the model will follow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    
    /// List of tools available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AgentTool>>,
    
    /// Completion arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    
    /// Agent description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Handoff configurations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoffs: Option<Vec<String>>,
    
    /// Agent metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Agent response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Agent ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Model used
    pub model: String,
    
    /// Agent name
    pub name: String,
    
    /// Current version
    pub version: i32,
    
    /// All versions
    pub versions: Vec<i32>,
    
    /// Creation timestamp
    pub created_at: String,
    
    /// Update timestamp
    pub updated_at: String,
    
    /// Instruction prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    
    /// Available tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AgentTool>>,
    
    /// Completion arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    
    /// Agent description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Handoff configurations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoffs: Option<Vec<String>>,
    
    /// Deployment chat enabled
    pub deployment_chat: bool,
    
    /// Agent source
    pub source: String,
    
    /// Agent metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// List agents response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentsResponse {
    /// List of agents
    pub data: Vec<Agent>,
    
    /// Pagination information
    pub has_more: bool,
}

/// Agent alias response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAliasResponse {
    /// Agent ID
    pub agent_id: String,
    
    /// Alias name
    pub alias: String,
    
    /// Target version
    pub version: i32,
}

/// List agent aliases response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentAliasesResponse {
    /// List of aliases
    pub data: Vec<AgentAliasResponse>,
}

/// Agents API client
#[derive(Debug)]
pub struct AgentsApi {
    client: MistralClient,
}

impl AgentsApi {
    /// Create a new Agents API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// Create a new agent
    pub async fn create_agent(&self, request: &CreateAgentRequest) -> Result<Agent, MistralError> {
        let response = self.client.post("/v1/agents", request).await?;
        let agent: Agent = serde_json::from_str(&response)?;
        Ok(agent)
    }
    
    /// List all agents
    pub async fn list_agents(&self) -> Result<ListAgentsResponse, MistralError> {
        let response = self.client.get("/v1/agents", None).await?;
        let agents: ListAgentsResponse = serde_json::from_str(&response)?;
        Ok(agents)
    }
    
    /// Get a specific agent by ID
    pub async fn get_agent(&self, agent_id: &str) -> Result<Agent, MistralError> {
        let path = format!("/v1/agents/{}", agent_id);
        let response = self.client.get(&path, None).await?;
        let agent: Agent = serde_json::from_str(&response)?;
        Ok(agent)
    }
    
    /// Update an agent
    pub async fn update_agent(&self, agent_id: &str, request: &CreateAgentRequest) -> Result<Agent, MistralError> {
        let path = format!("/v1/agents/{}", agent_id);
        let response = self.client.put(&path, request).await?;
        let agent: Agent = serde_json::from_str(&response)?;
        Ok(agent)
    }
    
    /// Delete an agent
    pub async fn delete_agent(&self, agent_id: &str) -> Result<(), MistralError> {
        let path = format!("/v1/agents/{}", agent_id);
        self.client.delete(&path).await?;
        Ok(())
    }
    
    /// Update agent version
    pub async fn update_agent_version(&self, agent_id: &str, version: i32) -> Result<Agent, MistralError> {
        let path = format!("/v1/agents/{}", agent_id);
        let response = self.client.patch(&path, &serde_json::json!({"version": version})).await?;
        let agent: Agent = serde_json::from_str(&response)?;
        Ok(agent)
    }
    
    /// List agent versions
    pub async fn list_agent_versions(&self, agent_id: &str) -> Result<Vec<i32>, MistralError> {
        let path = format!("/v1/agents/{}/versions", agent_id);
        let response = self.client.get(&path, None).await?;
        let versions: Vec<i32> = serde_json::from_str(&response)?;
        Ok(versions)
    }
    
    /// Get specific agent version
    pub async fn get_agent_version(&self, agent_id: &str, version: i32) -> Result<Agent, MistralError> {
        let path = format!("/v1/agents/{}/versions/{}", agent_id, version);
        let response = self.client.get(&path, None).await?;
        let agent: Agent = serde_json::from_str(&response)?;
        Ok(agent)
    }
    
    /// Create or update agent alias
    pub async fn create_or_update_alias(
        &self,
        agent_id: &str,
        alias: &str,
        version: i32,
    ) -> Result<AgentAliasResponse, MistralError> {
        let path = format!("/v1/agents/{}/aliases", agent_id);
        let alias_param = alias.to_string();
        let version_param = version.to_string();
        let params = vec![
            ("alias", alias_param.as_str()),
            ("version", version_param.as_str()),
        ];
        // Note: This endpoint uses query parameters, not JSON body
        let response = self.client.put_with_params(&path, &params).await?;
        let alias_response: AgentAliasResponse = serde_json::from_str(&response)?;
        Ok(alias_response)
    }
    
    /// List agent version aliases
    pub async fn list_agent_aliases(&self, agent_id: &str) -> Result<ListAgentAliasesResponse, MistralError> {
        let path = format!("/v1/agents/{}/aliases", agent_id);
        let response = self.client.get(&path, None).await?;
        let aliases: ListAgentAliasesResponse = serde_json::from_str(&response)?;
        Ok(aliases)
    }
}

// Need to add the put_with_params method to the client
#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_agent_tool_serialization() {
        let function_tool = AgentTool::Function {
            name: "test_function".to_string(),
            description: "Test function".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "param1": {"type": "string"}
                }
            }),
        };

        let json = serde_json::to_value(function_tool).unwrap();
        assert_eq!(json["type"], "function");
        assert_eq!(json["name"], "test_function");
    }

    #[test]
    fn test_completion_args_serialization() {
        let args = CompletionArgs {
            temperature: Some(0.7),
            max_tokens: Some(100),
            top_p: Some(0.9),
            stop: Some(vec!["\n".to_string()]),
        };

        let json = serde_json::to_value(args).unwrap();
        assert!(json["temperature"].as_f64().unwrap().abs() - 0.7 < f64::EPSILON);
        assert_eq!(json["max_tokens"], 100);
        assert_eq!(json["stop"].as_array().unwrap(), &vec![serde_json::Value::from("\n")]);
    }

    #[test]
    fn test_create_agent_request_serialization() {
        let request = CreateAgentRequest {
            model: "mistral-tiny".to_string(),
            name: "test-agent".to_string(),
            instructions: Some("Follow these instructions".to_string()),
            tools: Some(vec![AgentTool::WebSearch]),
            completion_args: Some(CompletionArgs {
                temperature: Some(0.7),
                max_tokens: Some(100),
                top_p: None,
                stop: None,
            }),
            description: Some("Test agent description".to_string()),
            handoffs: Some(vec!["fallback".to_string()]),
            metadata: Some(HashMap::from([("key".to_string(), json!("value"))])),
        };

        let json = serde_json::to_value(request).unwrap();
        assert_eq!(json["model"], "mistral-tiny");
        assert_eq!(json["name"], "test-agent");
        assert_eq!(json["tools"][0]["type"], "web_search");
    }

    #[test]
    fn test_agent_deserialization() {
        let json = json!({
            "id": "agent-123",
            "object": "agent",
            "model": "mistral-tiny",
            "name": "test-agent",
            "version": 1,
            "versions": [1],
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z",
            "deployment_chat": true,
            "source": "user",
            "instructions": "Test instructions",
            "tools": [{"type": "web_search"}],
            "completion_args": {"temperature": 0.7},
            "description": "Test description",
            "handoffs": ["fallback"],
            "metadata": {"key": "value"}
        });

        let agent: Agent = serde_json::from_value(json).unwrap();
        assert_eq!(agent.id, "agent-123");
        assert_eq!(agent.name, "test-agent");
        assert_eq!(agent.version, 1);
    }

    #[test]
    fn test_agents_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = AgentsApi::new(client);
        assert_eq!(api.client.api_key, "test-key");
    }
}