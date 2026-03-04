//! Demonstrates conversation management with Mistral AI API
//!
//! This example shows how to create and manage conversations.
//!
//! Usage:
//!   cargo run --example conversations -- <message>
//!   MISTRAL_API_KEY=your_key cargo run --example conversations -- "Hello, let's chat!"
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::conversations::{CreateConversationRequest, ConversationsApi, InputEntry, InputEntryType}};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get initial message from command line arguments
    let message = std::env::args().nth(1)
        .context("Usage: cargo run --example conversations -- <message>")?;

    println!("Creating conversation with initial message: '{}'", message);

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create conversations API client
    let conversations_api = ConversationsApi::new(client);

    // Create a conversation request with the new input format
    let request = CreateConversationRequest {
        inputs: vec![InputEntry {
            object_type: "entry".to_string(),
            entry_type: InputEntryType::MessageInput,
            id: "".to_string(), // Empty string for user inputs (API will generate)
            role: Some("user".to_string()),
            content: Some(message),
            name: None,
        }],
        model: None, // Try using an agent instead
        agent_id: Some("mistral-vibe-cli-latest".to_string()), // Use as agent_id
        metadata: None,
        temperature: None,
        max_tokens: None,
        instructions: None,
        store: Some(true),
    };

    // Make the API call to create conversation
    println!("Sending conversation request to Mistral AI API...");
    let response = conversations_api.create_conversation(&request).await
        .context("Failed to create conversation")?;

    // Pretty print the response
    println!("\nConversation Creation Response:");
    println!("{}", to_string_pretty(&response)?);

    // Display key information
    println!("\nConversation Created:");
    println!("Conversation ID: {}", response.id);
    println!("Model: {}", response.model);
    println!("Created At: {}", response.created);

    // List all conversations
    println!("\nListing all conversations...");
    let list_response = conversations_api.list_conversations(None, None).await
        .context("Failed to list conversations")?;

    println!("Found {} conversations:", list_response.data.len());
    for conv in &list_response.data {
        println!("- Conversation {}: {} (model: {})", 
            conv.id, conv.created, conv.model);
    }

    // Get details of our conversation if we have the ID
    if !list_response.data.is_empty() {
        let conv_id = &list_response.data[0].id;
        println!("\nGetting details for conversation: {}", conv_id);
        
        let details = conversations_api.get_conversation(conv_id).await
            .context("Failed to get conversation details")?;

        println!("Conversation Details:");
        println!("ID: {}", details.id);
        println!("Model: {}", details.model);
        println!("Created: {}", details.created);
        
        if let Some(title) = &details.title {
            println!("Title: {}", title);
        }
    }

    Ok(())
}
