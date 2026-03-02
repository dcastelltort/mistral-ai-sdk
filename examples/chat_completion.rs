//! Demonstrates basic chat completion with Mistral AI API
//!
//! This example shows how to create a chat completion request and handle the response.
//!
//! Usage:
//!   cargo run --example chat_completion -- <prompt>
//!   MISTRAL_API_KEY=your_key cargo run --example chat_completion -- "Hello, how are you?"
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::chat::{ChatCompletionRequest, ChatMessage, ChatApi}};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get prompt from command line arguments
    let prompt = std::env::args().nth(1)
        .context("Usage: cargo run --example chat_completion -- <prompt>")?;

    println!("Creating chat completion for prompt: '{}'", prompt);

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create chat API client
    let chat_api = ChatApi::new(client);

    // Create a chat completion request
    let request = ChatCompletionRequest {
        model: "mistral-tiny".to_string(), // Use an appropriate model
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
            name: None,
            function_call: None,
        }],
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: None,
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        top_p: None,
        user: None,
    };

    // Make the API call
    println!("Sending request to Mistral AI API...");
    let response = chat_api.create_completion(&request).await
        .context("Failed to create chat completion")?;

    // Pretty print the response
    println!("\nAPI Response:");
    println!("{}", to_string_pretty(&response)?);

    // Extract and display the assistant's reply
    if let Some(choice) = response.choices.first() {
        println!("\nAssistant's Reply:");
        println!("{}", choice.message.content);
    }

    Ok(())
}
