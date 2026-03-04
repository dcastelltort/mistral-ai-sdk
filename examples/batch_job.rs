//! Demonstrates batch job processing with Mistral AI API
//!
//! This example shows how to create and manage batch processing jobs.
//!
//! Usage:
//!   cargo run --example batch_job -- [input_file_id] [endpoint] [completion_window]
//!   MISTRAL_API_KEY=your_key cargo run --example batch_job -- "550e8400-e29b-41d4-a716-446655440000" "/v1/chat/completions" "24h"
//!
//! If no arguments are provided, it generates a random UUID and uses defaults:
//!   cargo run --example batch_job
//!
//! Note: input_file_id must be a valid UUID format. If not provided, a random UUID is generated.
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_sdk::{MistralClient, api::batch::{CreateBatchJobRequest, BatchApi}};
use serde_json::to_string_pretty;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get input file ID from command line arguments, or generate a default UUID
    let input_file_id = std::env::args().nth(1)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Get endpoint from command line arguments, or use default
    let endpoint = std::env::args().nth(2)
        .unwrap_or_else(|| "/v1/chat/completions".to_string());

    // Get completion window from command line arguments, or use default
    let completion_window = std::env::args().nth(3)
        .unwrap_or_else(|| "24h".to_string());

    // Validate and convert file ID to proper UUID format
    let file_uuid = if input_file_id.starts_with("file-") {
        // If it's in the old format like "file-123", generate a proper UUID
        println!("Warning: '{}' is not a valid UUID format. Generating a proper UUID.", input_file_id);
        Uuid::new_v4().to_string()
    } else if Uuid::parse_str(&input_file_id).is_err() {
        // If it's not a valid UUID, generate one
        println!("Warning: '{}' is not a valid UUID. Generating a proper UUID.", input_file_id);
        Uuid::new_v4().to_string()
    } else {
        input_file_id
    };

    println!("Creating batch job with file ID: {}", file_uuid);
    println!("Endpoint: {}", endpoint);
    println!("Completion window: {}", completion_window);

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create batch API client
    let batch_api = BatchApi::new(client);

    // Create a batch job request
    let mut metadata = HashMap::new();
    metadata.insert("description".to_string(), serde_json::Value::String("Example batch job".to_string()));
    metadata.insert("created_by".to_string(), serde_json::Value::String("mistral-ai-rs-example".to_string()));
    
    let request = CreateBatchJobRequest {
        input_files: vec![file_uuid.clone()],
        endpoint: Some(endpoint.clone()),
        completion_window: Some(completion_window.clone()),
        metadata: Some(metadata),
        model: Some("mistral-small-latest".to_string()), // Add default model
    };

    // Make the API call
    println!("Sending batch job request to Mistral AI API...");
    let response = batch_api.create_job(&request).await
        .context("Failed to create batch job")?;

    // Pretty print the response
    println!("\nBatch Job Creation Response:");
    println!("{}", to_string_pretty(&response)?);

    // Display key information
    println!("\nBatch Job Created:");
    println!("Job ID: {}", response.id);
    println!("Input File ID: {}", response.input_file);
    println!("Completion Window: {:?}", response.completion_window);
    println!("Status: {}", response.status);

    // List batch jobs to verify our job appears
    println!("\nListing all batch jobs...");
    let list_response = batch_api.list_jobs().await
        .context("Failed to list batch jobs")?;

    println!("Found {} batch jobs:", list_response.data.len());
    for job in &list_response.data {
        println!("- Job {}: {} (status: {})", job.id, job.job_type, job.status);
    }

    Ok(())
}
