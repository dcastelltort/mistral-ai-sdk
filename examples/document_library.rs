//! Demonstrates document library management with Mistral AI API
//!
//! This example shows how to create and manage document libraries for RAG (Retrieval-Augmented Generation).
//!
//! Usage:
//!   cargo run --example document_library
//!   MISTRAL_API_KEY=your_key cargo run --example document_library
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::libraries::{LibrariesApi, LibraryIn, LibraryInUpdate, ShareLibraryRequest}};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    println!("Document Library Management Example");
    println!("===================================");

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create libraries API client
    let libraries_api = LibrariesApi::new(client);

    // Example 1: Create a new library
    println!("\n1. Creating a new document library...");
    let library_request = LibraryIn {
        name: "Research Papers".to_string(),
        description: Some("A library for storing and retrieving research papers".to_string()),
        chunk_size: Some(512),
    };

    match libraries_api.create_library(&library_request).await {
        Ok(library) => {
            println!("✓ Library created successfully!");
            println!("  Library ID: {}", library.id);
            println!("  Library Name: {}", library.name);
            println!("  Status: {}", library.status.clone().unwrap_or("active".to_string()));
            
            // Example 2: List all libraries
            println!("\n2. Listing all libraries...");
            match libraries_api.list_libraries().await {
                Ok(libraries) => {
                    println!("Found {} libraries:", libraries.data.len());
                    for lib in &libraries.data {
                        println!("  - {} (ID: {}, {} documents)", 
                            lib.name, lib.id, lib.document_count.unwrap_or(0));
                    }
                }
                Err(e) => {
                    eprintln!("⚠ Could not list libraries: {}", e);
                }
            }
            
            // Example 3: Update the library
            println!("\n3. Updating library information...");
            let update_request = LibraryInUpdate {
                name: Some("Research Papers - Updated".to_string()),
                description: Some("An updated library for research papers with enhanced metadata".to_string()),
            };
            
            match libraries_api.update_library(&library.id, &update_request).await {
                Ok(updated_library) => {
                    println!("✓ Library updated successfully!");
                    println!("  New name: {}", updated_library.name);
                    println!("  New description: {}", updated_library.description.unwrap_or("None".to_string()));
                }
                Err(e) => {
                    eprintln!("⚠ Could not update library: {}", e);
                }
            }
            
            // Example 4: Share the library (if supported)
            println!("\n4. Sharing the library with collaborators...");
            let user_uuid = Uuid::new_v4(); // Generate a valid UUID for each run
            let share_request = ShareLibraryRequest {
                org_id: None,
                level: "Viewer".to_string(),  // Must be "Viewer" or "Editor"
                share_with_uuid: user_uuid.to_string(),  // Use generated UUID
                share_with_type: "User".to_string(),  // Must be "User", "Workspace", or "Org"
            };
            
            match libraries_api.share_library(&library.id, &share_request).await {
                Ok(_) => {
                    println!("✓ Library shared with user {}", user_uuid);
                }
                Err(e) => {
                    eprintln!("⚠ Could not share library: {}", e);
                }
            }
            
            // Example 5: Clean up - delete the library
            println!("\n5. Cleaning up - deleting library...");
            match libraries_api.delete_library(&library.id).await {
                Ok(_) => {
                    println!("✓ Deleted library: {}", library.id);
                }
                Err(e) => {
                    eprintln!("⚠ Could not delete library: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create library: {}", e);
            eprintln!("This might be expected if you don't have libraries API access.");
        }
    }

    println!("\nDocument library example completed!");
    println!("The Libraries API provides powerful document management for RAG applications:");
    println!("  - Create and manage document libraries");
    println!("  - Upload and process documents");
    println!("  - Retrieve document text content and metadata");
    println!("  - Monitor document processing status");
    println!("  - Share libraries with collaborators");
    println!("  - Generate signed URLs for secure access");
    println!("  - Support for chunked document processing");
    
    println!("\nNote: Document upload functionality requires multipart/form-data support");
    println!("which will be implemented in a future update. The current example demonstrates");
    println!("library creation, management, and sharing capabilities.");
    
    Ok(())
}
