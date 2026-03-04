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
use mistral_ai_rs::{MistralClient, api::libraries::{LibrariesApi, LibraryIn, LibraryInUpdate, DocumentUploadRequest, ShareLibraryRequest}};
use serde_json::json;

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
            
            // Example 3: Upload documents to the library
            println!("\n3. Uploading documents to the library...");
            let document_requests = vec![
                DocumentUploadRequest {
                    url: "https://example.com/research-paper1.pdf".to_string(),
                    metadata: Some(vec![
                        ("title".to_string(), json!("Advanced Machine Learning Techniques")),
                        ("author".to_string(), json!("Jane Doe")),
                        ("year".to_string(), json!(2023)),
                        ("keywords".to_string(), json!(["AI", "ML", "Deep Learning"])),
                    ].into_iter().collect()),
                },
                DocumentUploadRequest {
                    url: "https://example.com/research-paper2.pdf".to_string(),
                    metadata: Some(vec![
                        ("title".to_string(), json!("Natural Language Processing Trends")),
                        ("author".to_string(), json!("John Smith")),
                        ("year".to_string(), json!(2024)),
                        ("keywords".to_string(), json!(["NLP", "LLM", "Transformers"])),
                    ].into_iter().collect()),
                },
            ];
            
            let mut uploaded_documents = Vec::new();
            for doc_request in document_requests {
                match libraries_api.upload_document(&library.id, &doc_request).await {
                    Ok(document) => {
                        let title = doc_request.metadata.as_ref().and_then(|m| m.get("title")).map(|v| v.to_string()).unwrap_or("unknown".to_string());
                        println!("  ✓ Uploaded document: {} (ID: {})", title, document.id);
                        uploaded_documents.push(document);
                    }
                    Err(e) => {
                        eprintln!("  ⚠ Could not upload document: {}", e);
                    }
                }
            }
            
            // Example 4: List documents in the library
            if !uploaded_documents.is_empty() {
                println!("\n4. Listing documents in the library...");
                match libraries_api.list_documents(&library.id).await {
                    Ok(documents) => {
                        println!("Found {} documents:", documents.data.len());
                        for doc in &documents.data {
                            let title = doc.metadata.as_ref().and_then(|m| m.get("title")).map(|v| v.to_string()).unwrap_or("untitled".to_string());
                            println!("  - {} (ID: {}, status: {})", title, doc.id, doc.status);
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠ Could not list documents: {}", e);
                    }
                }
                
                // Example 5: Get document details and text content
                if let Some(first_doc) = uploaded_documents.first() {
                    println!("\n5. Getting document details and content...");
                    
                    // Get document details
                    match libraries_api.get_document(&library.id, &first_doc.id).await {
                        Ok(detailed_doc) => {
                            println!("Document Details:");
                            println!("  ID: {}", detailed_doc.id);
                            println!("  URL: {}", detailed_doc.url);
                            println!("  Status: {}", detailed_doc.status);
                            println!("  Chunks: {}", detailed_doc.chunk_count.unwrap_or(0));
                            
                            if let Some(metadata) = &detailed_doc.metadata {
                                println!("  Metadata:");
                                for (key, value) in metadata {
                                    println!("    {}: {}", key, value);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠ Could not get document details: {}", e);
                        }
                    }
                    
                    // Get document text content
                    match libraries_api.get_document_text(&library.id, &first_doc.id).await {
                        Ok(text_content) => {
                            println!("\n  Document Text Content:");
                            println!("  First {} characters: {}", 
                                text_content.text.len().min(100),
                                &text_content.text[..text_content.text.len().min(100)]);
                            if let Some(chunk) = &text_content.chunk {
                                println!("  Chunk {}/{}", chunk.index + 1, chunk.total);
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠ Could not get document text: {}", e);
                        }
                    }
                    
                    // Example 6: Get document status
                    match libraries_api.get_document_status(&library.id, &first_doc.id).await {
                        Ok(status) => {
                            println!("\n  Document Status: {}", status.status);
                            if let Some(progress) = status.progress {
                                println!("  Progress: {:.1}%", progress * 100.0);
                            }
                            if let Some(error) = status.error {
                                println!("  Error: {}", error);
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠ Could not get document status: {}", e);
                        }
                    }
                }
                
                // Example 7: Update the library
                println!("\n6. Updating library information...");
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
                
                // Example 8: Share the library (if supported)
                println!("\n7. Sharing the library with collaborators...");
                let share_request = ShareLibraryRequest {
                    user_ids: vec!["user1@example.com".to_string(), "user2@example.com".to_string()],
                    permission: "read".to_string(),
                };
                
                match libraries_api.share_library(&library.id, &share_request).await {
                    Ok(_) => {
                        println!("✓ Library shared with {} users", share_request.user_ids.len());
                    }
                    Err(e) => {
                        eprintln!("⚠ Could not share library: {}", e);
                    }
                }
                
                // Example 9: Get signed URLs for document access
                if let Some(first_doc) = uploaded_documents.first() {
                    println!("\n8. Getting signed URLs for document access...");
                    
                    match libraries_api.get_signed_url(&library.id, &first_doc.id).await {
                        Ok(signed_url) => {
                            println!("✓ Signed URL for document: {}", signed_url.url);
                            println!("  Expires at: {}", signed_url.expires_at);
                        }
                        Err(e) => {
                            eprintln!("⚠ Could not get signed URL: {}", e);
                        }
                    }
                    
                    match libraries_api.get_extracted_text_signed_url(&library.id, &first_doc.id).await {
                        Ok(signed_url) => {
                            println!("✓ Signed URL for extracted text: {}", signed_url.url);
                        }
                        Err(e) => {
                            eprintln!("⚠ Could not get extracted text signed URL: {}", e);
                        }
                    }
                }
                
                // Example 10: Clean up - delete documents and library
                println!("\n9. Cleaning up - deleting documents and library...");
                
                for doc in &uploaded_documents {
                    match libraries_api.delete_document(&library.id, &doc.id).await {
                        Ok(_) => {
                            println!("  ✓ Deleted document: {}", doc.id);
                        }
                        Err(e) => {
                            eprintln!("  ⚠ Could not delete document {}: {}", doc.id, e);
                        }
                    }
                }
                
                match libraries_api.delete_library(&library.id).await {
                    Ok(_) => {
                        println!("  ✓ Deleted library: {}", library.id);
                    }
                    Err(e) => {
                        eprintln!("  ⚠ Could not delete library: {}", e);
                    }
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
    
    Ok(())
}