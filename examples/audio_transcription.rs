//! Demonstrates audio transcription with Mistral AI API
//!
//! This example shows how to transcribe audio files using different input sources.
//!
//! Usage:
//!   cargo run --example audio_transcription -- <audio_url> <language>
//!   MISTRAL_API_KEY=your_key cargo run --example audio_transcription -- "https://example.com/audio.mp3" "en"
//!
//! The example requires the MISTRAL_API_KEY environment variable to be set.

use anyhow::{Context, Result};
use mistral_ai_sdk::{MistralClient, api::audio::{AudioTranscriptionRequest, AudioApi}};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable
    let api_key = std::env::var("MISTRAL_API_KEY")
        .context("Missing MISTRAL_API_KEY environment variable.\nPlease set it or create a .env file from .env.example")?;

    // Get audio URL from command line arguments
    let audio_url = std::env::args().nth(1)
        .context("Usage: cargo run --example audio_transcription -- <audio_url> <language>")?;

    // Get language from command line arguments (optional)
    let language = std::env::args().nth(2).unwrap_or_else(|| "en".to_string());

    println!("Transcribing audio from: {}", audio_url);
    println!("Language: {}", language);

    // Create the Mistral client
    let client = MistralClient::new(api_key);

    // Create audio API client
    let audio_api = AudioApi::new(client);

    // Create an audio transcription request
    let request = AudioTranscriptionRequest {
        model: "voxtral-mini-latest".to_string(),
        file: None, // Not used when using file_url
        file_url: Some(audio_url.clone()),
        file_id: None, // Alternative: use file_id for pre-uploaded files
        language: Some(language),
        temperature: Some(0.2), // Lower temperature for more accurate transcription
        response_format: Some("json".to_string()),
        prompt: Some("Transcribe this audio clearly and accurately".to_string()),
        timestamp_granularities: Some(vec!["word".to_string(), "segment".to_string()]),
    };

    // Make the API call
    println!("Sending transcription request to Mistral AI API...");
    let response = audio_api.create_transcription(&request).await
        .context("Failed to create transcription")?;

    // Pretty print the response
    println!("\nTranscription Response:");
    println!("{}", to_string_pretty(&response)?);

    // Display key information
    println!("\nTranscription Details:");
    println!("Model: {}", response.model);
    println!("Language: {}", response.language);
    println!("Audio duration: {} seconds", response.usage.prompt_audio_seconds);
    println!("Tokens used: {}", response.usage.total_tokens);

    // Display the transcribed text
    println!("\n=== Transcribed Text ===");
    println!("{}", response.text);

    // Display segments if available
    if let Some(segments) = &response.segments {
        println!("\n=== Segments ({}) ===", segments.len());
        for (i, segment) in segments.iter().enumerate() {
            println!("Segment {}: [{:.2}s - {:.2}s]", i + 1, segment.start, segment.end);
            println!("  {}", segment.text);
            if let Some(score) = segment.score {
                println!("  Confidence: {:.2}%", score * 100.0);
            }
            if let Some(speaker) = &segment.speaker_id {
                println!("  Speaker: {}", speaker);
            }
            println!();
        }
    }

    Ok(())
}
