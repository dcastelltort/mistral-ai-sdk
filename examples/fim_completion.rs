//! Demonstrates FIM (Fill-in-the-Middle) completion with Mistral AI API
//!
//! This example shows how to use FIM for code completion tasks.
//!
//! Usage:
//!   cargo run --example fim_completion -- <prompt>
//!   MISTRAL_API_KEY=your_key cargo run --example fim_completion -- "def add_numbers(a: int, b: int) -> int:"
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_sdk::{MistralClient, api::fim::{FIMCompletionRequest, FIMApi}};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get prompt from command line arguments
    let prompt = std::env::args().nth(1)
        .context("Usage: cargo run --example fim_completion -- <prompt>")?;

    println!("Creating FIM completion for prompt: '{}'", prompt);

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create FIM API client
    let fim_api = FIMApi::new(client);

    // Create a FIM completion request
    // Note: FIM uses prompt/suffix structure, not messages
    let request = FIMCompletionRequest {
        model: "codestral-latest".to_string(),
        prompt: prompt.to_string(), // The text to complete
        suffix: Some(" test.".to_string()), // Optional suffix for FIM
        temperature: Some(0.2), // Lower temperature for code completion
        max_tokens: Some(200),
        stream: None,
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        top_p: None,
        user: None,
        random_seed: None,
        min_tokens: None,
        metadata: None,
    };

    // Make the API call
    println!("Sending FIM request to Mistral AI API...");
    let response = fim_api.create_completion(&request).await
        .context("Failed to create FIM completion")?;

    // Pretty print the response
    println!("\nFIM Completion Response:");
    println!("{}", to_string_pretty(&response)?);

    // Display key information
    println!("\nCompletion Details:");
    println!("Model: {}", response.model);
    println!("Usage - Prompt tokens: {}", response.usage.prompt_tokens);
    println!("Usage - Completion tokens: {}", response.usage.completion_tokens);
    println!("Usage - Total tokens: {}", response.usage.total_tokens);

    // Extract and display the completed code
    if let Some(choice) = response.choices.first() {
        println!("\nCompleted Code:");
        println!("{}", choice.content);
    }

    Ok(())
}
