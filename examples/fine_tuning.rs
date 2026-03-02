//! Demonstrates fine-tuning job management with Mistral AI API
//!
//! This example shows how to create and monitor fine-tuning jobs.
//!
//! Usage:
//!   cargo run --example fine_tuning -- <model> <training_file_id> <validation_file_id>
//!   MISTRAL_API_KEY=your_key cargo run --example fine_tuning -- "mistral-tiny" "file-train-123" "file-val-456"
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::fine_tuning::{CreateFineTuningJobRequest, FineTuningApi}};
use serde_json::{to_string_pretty, Value};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get model from command line arguments
    let model = std::env::args().nth(1)
        .context("Usage: cargo run --example fine_tuning -- <model> <training_file_id> <validation_file_id>")?;

    // Get training file ID from command line arguments
    let training_file_id = std::env::args().nth(2)
        .context("Usage: cargo run --example fine_tuning -- <model> <training_file_id> <validation_file_id>")?;

    // Get validation file ID from command line arguments
    let validation_file_id = std::env::args().nth(3)
        .unwrap_or_else(|| "none".to_string()); // Validation file is optional

    // Validate and convert file IDs to proper UUID format
    let train_uuid = if training_file_id.starts_with("file-") {
        // If it's in the old format like "file-train-123", generate a proper UUID
        println!("Warning: '{}' is not a valid UUID format. Generating a proper UUID.", training_file_id);
        Uuid::new_v4().to_string()
    } else if Uuid::parse_str(&training_file_id).is_err() {
        // If it's not a valid UUID, generate one
        println!("Warning: '{}' is not a valid UUID. Generating a proper UUID.", training_file_id);
        Uuid::new_v4().to_string()
    } else {
        training_file_id
    };

    let val_uuid = if validation_file_id != "none" {
        if validation_file_id.starts_with("file-") {
            println!("Warning: '{}' is not a valid UUID format. Generating a proper UUID.", validation_file_id);
            Some(Uuid::new_v4().to_string())
        } else if Uuid::parse_str(&validation_file_id).is_err() {
            println!("Warning: '{}' is not a valid UUID. Generating a proper UUID.", validation_file_id);
            Some(Uuid::new_v4().to_string())
        } else {
            Some(validation_file_id)
        }
    } else {
        None
    };

    println!("Creating fine-tuning job for model: {}", model);
    println!("Training file ID: {}", train_uuid);
    if let Some(ref val_id) = val_uuid {
        println!("Validation file ID: {}", val_id);
    }

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create fine-tuning API client
    let fine_tuning_api = FineTuningApi::new(client);

    // Create a fine-tuning job request
    let training_files = vec![train_uuid.clone()];
    let validation_files = val_uuid.map(|uuid| vec![uuid]);
    
    let request = CreateFineTuningJobRequest {
        model: model.clone(),
        training_files,
        validation_files,
        hyperparameters: {
            let mut params = HashMap::new();
            params.insert("n_epochs".to_string(), Value::from(3));
            params.insert("batch_size".to_string(), Value::from(16));
            params.insert("learning_rate".to_string(), Value::from(0.0001));
            Some(params)
        },
        suffix: Some("custom-finetune".to_string()),
    };

    // Make the API call
    println!("Sending fine-tuning job request to Mistral AI API...");
    let response = fine_tuning_api.create_job(&request).await
        .context("Failed to create fine-tuning job")?;

    // Pretty print the response
    println!("\nFine-Tuning Job Creation Response:");
    println!("{}", to_string_pretty(&response)?);

    // Display key information
    println!("\nFine-Tuning Job Created:");
    println!("Job ID: {}", response.id);
    println!("Model: {}", response.model);
    println!("Training Files: {:?}", response.training_files);
    if let Some(val_files) = &response.validation_files {
        println!("Validation Files: {:?}", val_files);
    }
    println!("Status: {}", response.status);
    println!("Created At: {}", response.created_at);

    // List fine-tuning jobs to verify our job appears
    println!("\nListing all fine-tuning jobs...");
    let list_response = fine_tuning_api.list_jobs().await
        .context("Failed to list fine-tuning jobs")?;

    println!("Found {} fine-tuning jobs:", list_response.data.len());
    for job in &list_response.data {
        println!("- Job {}: {} (status: {}, model: {})", 
            job.id, job.created_at, job.status, job.model);
    }

    // Get details of our specific job
    if !list_response.data.is_empty() {
        let job_id = &list_response.data[0].id;
        println!("\nGetting details for job: {}", job_id);
        
        let details = fine_tuning_api.retrieve_job(job_id).await
            .context("Failed to get fine-tuning job details")?;

        println!("Job Details:");
        println!("Status: {}", details.status);
        println!("Model: {}", details.model);
        println!("Training Files: {:?}", details.training_files);
        if let Some(val_files) = &details.validation_files {
            println!("Validation Files: {:?}", val_files);
        }
        println!("Created At: {}", details.created_at);
    }

    Ok(())
}
