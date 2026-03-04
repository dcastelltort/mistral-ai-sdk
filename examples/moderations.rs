//! Demonstrates content moderation with Mistral AI API
//!
//! This example shows how to check text content for safety violations.
//!
//! Usage:
//!   cargo run --example moderations -- <text>
//!   MISTRAL_API_KEY=your_key cargo run --example moderations -- "This is test content"
//!
//! Requirements:
//!   - MISTRAL_API_KEY environment variable with moderation scope
//!   - Moderation API access (contact Mistral AI support if needed)
//!
//! Note: The moderation API requires a special API key scope.
//! If you get a permission error, your key may not have moderation access.

use anyhow::{Context, Result};
use mistral_ai_rs::{
    api::moderations::{ModerationRequest, ModerationsApi},
    MistralClient,
};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get text from command line arguments
    let text = std::env::args()
        .nth(1)
        .context("Usage: cargo run --example moderations -- <text>")?;

    println!("Checking moderation for text: '{}'", text);

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create a moderation request
    let request = ModerationRequest {
        input: text,
        model: Some("mistral-moderation-latest".to_string()), // Use the moderation model
    };

    // Create moderations API client
    let moderations_api = ModerationsApi::new(&client);

    // Make the API call
    println!("Sending moderation request to Mistral AI API...");
    let response = moderations_api.create_moderation(&request).await
        .context("Failed to create moderation")?;

    // Pretty print the response
    println!("\nFull API Response:");
    println!("{}", to_string_pretty(&response)?);

    // Analyze and display moderation results
    println!("\nModeration Analysis:");
    for (i, result) in response.results.iter().enumerate() {
        println!("Result {}: Flagged = {}", i + 1, result.flagged);

        if result.flagged {
            println!("  Safety violations detected:");
            if result.categories.hate {
                println!(
                    "  - Hate content (score: {:.3})",
                    result.category_scores.hate
                );
            }
            if result.categories.harassment {
                println!(
                    "  - Harassment (score: {:.3})",
                    result.category_scores.harassment
                );
            }
            if result.categories.sexual {
                println!(
                    "  - Sexual content (score: {:.3})",
                    result.category_scores.sexual
                );
            }
            if result.categories.violence {
                println!(
                    "  - Violence (score: {:.3})",
                    result.category_scores.violence
                );
            }
            if result.categories.self_harm {
                println!(
                    "  - Self-harm (score: {:.3})",
                    result.category_scores.self_harm
                );
            }
        } else {
            println!("  No safety violations detected - content is safe");
        }
    }

    Ok(())
}
