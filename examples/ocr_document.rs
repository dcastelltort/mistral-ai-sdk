//! Demonstrates OCR (Optical Character Recognition) with Mistral AI API
//!
//! This example shows how to perform OCR on documents using different input sources.
//!
//! Usage:
//!   cargo run --example ocr_document -- <document_url> <output_format>
//!   MISTRAL_API_KEY=your_key cargo run --example ocr_document -- "https://example.com/doc.pdf" "markdown"
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::ocr::{OCRRequest, OCRDocument, DocumentURLChunk, OCRApi}};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get document URL from command line arguments
    let document_url = std::env::args().nth(1)
        .context("Usage: cargo run --example ocr_document -- <document_url>")?;

    println!("Performing OCR on document: {}", document_url);
    // Note: OCR API returns results in its standard format
    // The response_format parameter has been removed as it's not supported by the API

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create OCR API client
    let ocr_api = OCRApi::new(client);

    // Create an OCR request with document URL
    let request = OCRRequest {
        model: Some("mistral-ocr-latest".to_string()), // Use the latest OCR model
        id: Some("example-ocr-job".to_string()),
        document: OCRDocument::DocumentURL(DocumentURLChunk {
            type_field: "document_url".to_string(),
            document_url: document_url.clone(),
            document_name: Some("example_document.pdf".to_string()),
        }),
        pages: Some(vec![0, 1]), // Process first 2 pages
        include_image_base64: Some(false),
        image_limit: Some(5),
        // Note: response_format is not supported by the OCR API
        // The API returns results in its standard format
    };

    // Make the API call
    println!("Sending OCR request to Mistral AI API...");
    let response = ocr_api.perform_ocr(&request).await
        .context("Failed to perform OCR")?;

    // Pretty print the response
    println!("\nOCR Response:");
    println!("{}", to_string_pretty(&response)?);

    // Display key information
    println!("\nOCR Results:");
    println!("Model used: {}", response.model);
    println!("Pages processed: {}", response.usage_info.pages_processed);

    // Display extracted content from each page
    for (i, page) in response.pages.iter().enumerate() {
        println!("\n=== Page {} ===", i + 1);
        println!("Index: {}", page.index);
        println!("Extracted Text:");
        println!("{}", page.markdown);
        
        if let Some(images) = &page.images {
            println!("\nFound {} images:", images.len());
            for image in images {
                println!("- Image {} ({}): {}x{}", 
                    image.id, image.format, image.bounding_box.width, image.bounding_box.height);
            }
        }
        
        if let Some(tables) = &page.tables {
            println!("\nFound {} tables:", tables.len());
            for table in tables {
                println!("- Table {} ({}):", table.id, table.format);
                println!("  {}", table.content);
            }
        }
        
        if let Some(dimensions) = &page.dimensions {
            println!("\nPage Dimensions: {} {} x {}", 
                dimensions.width, dimensions.unit, dimensions.height);
        }
    }

    Ok(())
}
