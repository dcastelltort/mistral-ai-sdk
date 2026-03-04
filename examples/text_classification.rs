//! Demonstrates text classification with Mistral AI API
//!
//! This example shows how to classify text content for moderation and safety.
//!
//! Usage:
//!   cargo run --example text_classification
//!   MISTRAL_API_KEY=your_key cargo run --example text_classification
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_rs::{MistralClient, api::classifications::{ClassificationsApi, ClassificationRequest, ClassificationInput, ChatClassificationRequest, ChatMessage}};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    println!("Text Classification Example");
    println!("==========================");

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create classifications API client
    let classifications_api = ClassificationsApi::new(client);

    // Example 1: Single text classification
    println!("\n1. Classifying single text...");
    let single_request = ClassificationRequest {
        model: "mistral-moderation-latest".to_string(),
        input: ClassificationInput::Single("This is a test message to classify for safety".to_string()),
        metadata: Some(vec![
            ("user_id".to_string(), json!("test-user-123")),
            ("context".to_string(), json!("example")),
        ].into_iter().collect()),
    };

    match classifications_api.classify(&single_request).await {
        Ok(classification) => {
            println!("✓ Classification successful!");
            println!("  Classification ID: {}", classification.id);
            println!("  Model: {}", classification.model);
            println!("  Results:");
            for (i, result) in classification.results.iter().enumerate() {
                println!("    Result {}:", i + 1);
                for (category, score) in &result.scores {
                    println!("      {}: {:.6}", category, score);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Single text classification failed: {}", e);
        }
    }

    // Example 2: Multiple text classification
    println!("\n2. Classifying multiple texts...");
    let multiple_request = ClassificationRequest {
        model: "mistral-moderation-latest".to_string(),
        input: ClassificationInput::Multiple(vec![
            "First message to classify".to_string(),
            "Second message with different content".to_string(),
            "Third message for batch processing".to_string(),
        ]),
        metadata: None,
    };

    match classifications_api.classify(&multiple_request).await {
        Ok(classification) => {
            println!("✓ Batch classification successful!");
            println!("  Processed {} inputs, got {} results", 
                match &multiple_request.input {
                    ClassificationInput::Multiple(vec) => vec.len(),
                    _ => 1,
                },
                classification.results.len()
            );
        }
        Err(e) => {
            eprintln!("❌ Multiple text classification failed: {}", e);
        }
    }

    // Example 3: Chat message classification
    println!("\n3. Classifying chat messages...");
    let chat_request = ChatClassificationRequest {
        model: "mistral-moderation-latest".to_string(),
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello, can you help me with something?".to_string(),
                name: None,
            },
            ChatMessage {
                role: "assistant".to_string(),
                content: "Of course! What do you need help with?".to_string(),
                name: Some("assistant".to_string()),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "I need information about your services".to_string(),
                name: None,
            },
        ],
        metadata: Some(vec![
            ("conversation_id".to_string(), json!("conv-123")),
            ("platform".to_string(), json!("web")),
        ].into_iter().collect()),
    };

    match classifications_api.classify_chat(&chat_request).await {
        Ok(classification) => {
            println!("✓ Chat classification successful!");
            println!("  Classified {} messages", chat_request.messages.len());
            println!("  Safety analysis:");
            
            // Analyze the results for safety concerns
            for (i, result) in classification.results.iter().enumerate() {
                let mut has_safety_issues = false;
                for (category, score) in &result.scores {
                    if *score > 0.5 {
                        println!("    ⚠️  Message {}: High {} score: {:.4}", i + 1, category, score);
                        has_safety_issues = true;
                    }
                }
                if !has_safety_issues {
                    println!("    ✓ Message {}: No safety concerns detected", i + 1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Chat classification failed: {}", e);
        }
    }

    // Example 4: Moderation classification with threshold analysis
    println!("\n4. Advanced moderation analysis...");
    let moderation_texts = vec![
        "Normal friendly message".to_string(),
        "Message with mild negative content".to_string(),
        "Message containing potentially harmful content".to_string(),
    ];
    
    let moderation_request = ClassificationRequest {
        model: "mistral-moderation-latest".to_string(),
        input: ClassificationInput::Multiple(moderation_texts.clone()),
        metadata: Some(vec![
            ("purpose".to_string(), json!("moderation")),
            ("severity".to_string(), json!("high")),
        ].into_iter().collect()),
    };

    match classifications_api.classify(&moderation_request).await {
        Ok(classification) => {
            println!("✓ Moderation analysis complete!");
            
            // Analyze each text with thresholds
            const SAFETY_THRESHOLDS: &[(&str, f64)] = &[
                ("hate", 0.3),
                ("harassment", 0.3),
                ("violence", 0.1),
                ("sexual", 0.2),
                ("self_harm", 0.1),
                ("sexual_minors", 0.05),
            ];
            
            for (i, text) in moderation_texts.iter().enumerate() {
                println!("\n  Text {}: \'{}\'", i + 1, text);
                if let Some(result) = classification.results.get(i) {
                    let mut flagged = false;
                    for &(category, threshold) in SAFETY_THRESHOLDS {
                        if let Some(&score) = result.scores.get(category) {
                            if score > threshold {
                                println!("    ⚠️  {}: {:.4} (above threshold {:.2})", category, score, threshold);
                                flagged = true;
                            }
                        }
                    }
                    if !flagged {
                        println!("    ✓ All safety checks passed");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Moderation analysis failed: {}", e);
        }
    }

    println!("\nText classification example completed!");
    println!("The Classification API can be used for:");
    println!("  - Content moderation");
    println!("  - Safety filtering");
    println!("  - Sentiment analysis");
    println!("  - Topic classification");
    println!("  - Custom classification tasks");
    
    Ok(())
}