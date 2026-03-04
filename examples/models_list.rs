//! Demonstrates model listing and management with Mistral AI API
//!
//! This example shows how to list available models and retrieve model details.
//!
//! Usage:
//!   cargo run --example models_list
//!   MISTRAL_API_KEY=your_key cargo run --example models_list
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_sdk::{MistralClient, api::models::{ModelsApi, ModelListItem}};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    println!("Listing available Mistral AI models...");

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create models API client
    let models_api = ModelsApi::new(client);

    // List all available models
    println!("Fetching model list from Mistral AI API...");
    let response = models_api.list_models().await
        .context("Failed to list models")?;

    // Pretty print the response
    println!("\nFull API Response:");
    println!("{}", to_string_pretty(&response)?);

    // Display model information in a user-friendly format
    println!("\nAvailable Models ({}):", response.data.len());
    
    for (i, model) in response.data.iter().enumerate() {
        // Handle the enum variants
        let (id, created, owned_by, capabilities) = match model {
            ModelListItem::Base(base) => (
                &base.id,
                base.created,
                &base.owned_by,
                &base.capabilities
            ),
            ModelListItem::FineTuned(ft) => (
                &ft.id,
                ft.created,
                &ft.owned_by,
                &ft.capabilities
            ),
        };
        
        println!("\n{}. {}", i + 1, id);
        println!("   Created: {}", created);
        println!("   Owned by: {}", owned_by);
        
        // Capabilities are always present (not optional)
        println!("   Capabilities:");
        println!("     - Vision: {}", capabilities.vision);
        println!("     - Function Calling: {}", capabilities.function_calling);
        println!("     - Fine Tuning: {}", capabilities.fine_tuning);
        println!("     - Classification: {}", capabilities.classification);
        println!("     - Chat Completion: {}", capabilities.completion_chat);
        println!("     - FIM Completion: {}", capabilities.completion_fim);
    }

    // If we have models, get details of the first one
    if !response.data.is_empty() {
        let first_model = &response.data[0];
        let model_id = match first_model {
            ModelListItem::Base(base) => &base.id,
            ModelListItem::FineTuned(ft) => &ft.id,
        };
        println!("\nGetting details for model: {}", model_id);
        
        // Retrieve the model details
        let model_details = models_api.retrieve_model(model_id).await
            .context("Failed to get model details")?;

        // Handle the enum variant again for details
        match model_details {
            ModelListItem::Base(base) => {
                println!("Model Details:");
                println!("ID: {}", base.id);
                println!("Object: {}", base.object);
                println!("Created: {}", base.created);
                println!("Owned by: {}", base.owned_by);
                
                // Capabilities are always present
                println!("\nCapabilities:");
                println!("- Vision: {}", base.capabilities.vision);
                println!("- Function Calling: {}", base.capabilities.function_calling);
                println!("- Fine Tuning: {}", base.capabilities.fine_tuning);
                println!("- Classification: {}", base.capabilities.classification);
                println!("- Chat Completion: {}", base.capabilities.completion_chat);
                println!("- FIM Completion: {}", base.capabilities.completion_fim);
            },
            ModelListItem::FineTuned(ft) => {
                println!("Model Details:");
                println!("ID: {}", ft.id);
                println!("Object: {}", ft.object);
                println!("Created: {}", ft.created);
                println!("Owned by: {}", ft.owned_by);
                
                // Capabilities are always present
                println!("\nCapabilities:");
                println!("- Vision: {}", ft.capabilities.vision);
                println!("- Function Calling: {}", ft.capabilities.function_calling);
                println!("- Fine Tuning: {}", ft.capabilities.fine_tuning);
                println!("- Classification: {}", ft.capabilities.classification);
                println!("- Chat Completion: {}", ft.capabilities.completion_chat);
                println!("- FIM Completion: {}", ft.capabilities.completion_fim);
            }
        }
    }

    println!("\nModel listing complete!");
    println!("You can use these model IDs in other API calls.");

    Ok(())
}
