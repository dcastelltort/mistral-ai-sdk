//! Demonstrates file upload with Mistral AI API
//!
//! This example shows how to upload a file to the Mistral AI platform.
//!
//! Usage:
//!   cargo run --example file_upload -- [file_path] [purpose]
//!   MISTRAL_API_KEY=your_key cargo run --example file_upload -- "data.txt" "fine-tune"
//!
//! If no arguments are provided, it uses the default example file:
//!   cargo run --example file_upload
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.
//!
//! Purpose options: fine-tune, batch, etc.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::files::{FileUploadRequest, FilesApi}};
use serde_json::to_string_pretty;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get file path from command line arguments, or use default example file
    let file_path = std::env::args().nth(1).unwrap_or_else(|| "examples/data.txt".to_string());

    // Get purpose from command line arguments, or use default purpose
    let purpose_str = std::env::args().nth(2).unwrap_or_else(|| "fine-tune".to_string());

    // Purpose is now a string in the API
    let purpose = purpose_str;

    println!("Uploading file '{}' for purpose: {:?}", file_path, purpose);

    // Verify file exists
    if !Path::new(&file_path).exists() {
        return Err(anyhow::anyhow!("File not found: {}", file_path));
    }

    // Verify file exists and is readable
    let _ = fs::metadata(&file_path)
        .context("Failed to access file")?;

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create files API client
    let files_api = FilesApi::new(client);

    // Create file upload request
    let request = FileUploadRequest {
        file_path: file_path.clone(),
        purpose,
    };

    // Make the API call
    println!("Uploading file to Mistral AI API...");
    let response = files_api.upload_file(&request).await
        .context("Failed to upload file")?;

    // Pretty print the response
    println!("\nUpload Response:");
    println!("{}", to_string_pretty(&response)?);

    println!("\nFile uploaded successfully!");
    println!("File ID: {}", response.id);
    println!("File name: {}", response.filename);
    println!("File purpose: {:?}", response.purpose);

    Ok(())
}
