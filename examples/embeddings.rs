//! Demonstrates text embeddings with Mistral AI API
//!
//! This example shows how to generate text embeddings and handle the response.
//!
//! Usage:
//!   cargo run --example embeddings -- <text>
//!   MISTRAL_API_KEY=your_key cargo run --example embeddings -- "Hello world"
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::embeddings::{EmbeddingRequest, EmbeddingResponse, EmbeddingsApi}};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get text from command line arguments
    let text = std::env::args().nth(1)
        .context("Usage: cargo run --example embeddings -- <text>")?;

    println!("Generating embeddings for text: '{}'", text);

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create embeddings API client
    let embeddings_api = EmbeddingsApi::new(client);

    // Create an embedding request
    let request = EmbeddingRequest {
        input: text,
        model: Some("mistral-embed".to_string()), // Use an appropriate embedding model
        encoding_format: Some("float".to_string()),
        user: None,
    };

    // Make the API call
    println!("Sending request to Mistral AI API...");
    let response: EmbeddingResponse = embeddings_api.create_embeddings(&request).await
        .context("Failed to create embeddings")?;

    // Pretty print the response
    println!("\nAPI Response:");
    println!("{}", to_string_pretty(&response)?);

    // Display embedding information
    if let Some(data) = response.data.first() {
        println!("\nEmbedding Information:");
        println!("Index: {}", data.index);
        println!("Embedding dimension: {}", data.embedding.len());
        println!("First 5 values: {:?}", &data.embedding[..5]);
    }

    // Display usage information
    println!("\nUsage:");
    println!("Prompt tokens: {}", response.usage.prompt_tokens);
    println!("Total tokens: {}", response.usage.total_tokens);

    Ok(())
}
