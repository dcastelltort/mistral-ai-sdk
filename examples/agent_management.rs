//! Demonstrates agent management with Mistral AI API
//!
//! This example shows how to create, manage, and use agents.
//!
//! Usage:
//!   cargo run --example agent_management
//!   MISTRAL_API_KEY=your_key cargo run --example agent_management
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::agents::{AgentsApi, CreateAgentRequest, AgentTool, CompletionArgs}};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    println!("Creating agent management example...");

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create agents API client
    let agents_api = AgentsApi::new(client);

    // Create an agent with web search capability
    let agent_request = CreateAgentRequest {
        model: "mistral-medium-latest".to_string(),
        name: "research-assistant".to_string(),
        instructions: Some("You are a research assistant. Provide detailed, well-researched answers with sources when possible.".to_string()),
        tools: Some(vec![
            AgentTool::WebSearch,
            AgentTool::WebSearchPremium,
        ]),
        completion_args: Some(CompletionArgs {
            temperature: Some(0.3),
            max_tokens: Some(1000),
            top_p: Some(0.9),
            stop: Some(vec!["\n".to_string()]),
        }),
        description: Some("Research assistant with web search capabilities".to_string()),
        handoffs: Some(vec!["fallback".to_string()]),
        metadata: Some(vec![
            ("category".to_string(), json!("research")),
            ("version".to_string(), json!("1.0")),
        ].into_iter().collect()),
    };

    // Create the agent
    println!("Creating research assistant agent...");
    match agents_api.create_agent(&agent_request).await {
        Ok(agent) => {
            println!("✓ Agent created successfully!");
            println!("Agent ID: {}", agent.id);
            println!("Agent Name: {}", agent.name);
            println!("Current Version: {}", agent.version);
            println!("Model: {}", agent.model);
            
            // List all agents
            println!("\nListing all agents...");
            match agents_api.list_agents().await {
                Ok(agents_list) => {
                    println!("Found {} agents:", agents_list.data.len());
                    for agent in &agents_list.data {
                        println!("- {} (v{}) - {}", agent.name, agent.version, agent.model);
                    }
                }
                Err(e) => {
                    eprintln!("⚠ Could not list agents: {}", e);
                }
            }
            
            // Get the specific agent we created
            println!("\nGetting details for agent: {}", agent.id);
            match agents_api.get_agent(&agent.id).await {
                Ok(detailed_agent) => {
                    println!("Agent Details:");
                    println!("  ID: {}", detailed_agent.id);
                    println!("  Name: {}", detailed_agent.name);
                    println!("  Version: {}", detailed_agent.version);
                    println!("  Model: {}", detailed_agent.model);
                    if let Some(instructions) = &detailed_agent.instructions {
                        println!("  Instructions: {}", instructions);
                    }
                    if let Some(tools) = &detailed_agent.tools {
                        println!("  Tools: {:?}", tools);
                    }
                }
                Err(e) => {
                    eprintln!("⚠ Could not get agent details: {}", e);
                }
            }
            
            // List agent versions
            println!("\nListing versions for agent: {}", agent.id);
            match agents_api.list_agent_versions(&agent.id).await {
                Ok(versions) => {
                    println!("Agent versions: {:?}", versions);
                }
                Err(e) => {
                    eprintln!("⚠ Could not list agent versions: {}", e);
                }
            }
            
            // Create an alias for the agent
            println!("\nCreating alias 'latest' for agent version {}", agent.version);
            match agents_api.create_or_update_alias(&agent.id, "latest", agent.version).await {
                Ok(alias_response) => {
                    println!("✓ Alias created: {} -> version {}", alias_response.alias, alias_response.version);
                    
                    // List all aliases for the agent
                    match agents_api.list_agent_aliases(&agent.id).await {
                        Ok(aliases) => {
                            println!("Agent aliases:");
                            for alias in &aliases.data {
                                println!("  {} -> version {}", alias.alias, alias.version);
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠ Could not list agent aliases: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("⚠ Could not create alias: {}", e);
                }
            }
            
            // Update the agent
            println!("\nUpdating agent instructions...");
            let mut updated_request = agent_request.clone();
            updated_request.instructions = Some("Updated: You are a research assistant with enhanced capabilities.".to_string());
            
            match agents_api.update_agent(&agent.id, &updated_request).await {
                Ok(updated_agent) => {
                    println!("✓ Agent updated successfully!");
                    println!("New version: {}", updated_agent.version);
                }
                Err(e) => {
                    eprintln!("⚠ Could not update agent: {}", e);
                }
            }
            
            // Clean up: Delete the agent
            println!("\nCleaning up - deleting agent...");
            match agents_api.delete_agent(&agent.id).await {
                Ok(_) => {
                    println!("✓ Agent deleted successfully!");
                }
                Err(e) => {
                    eprintln!("⚠ Could not delete agent: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create agent: {}", e);
            eprintln!("This might be expected if you don't have agents API access.");
        }
    }

    println!("\nAgent management example completed!");
    Ok(())
}