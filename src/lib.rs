//! Async Rust client for Mistral AI API
//!
//! # Examples
//!
//! ```no_run
//! use mistral_ai_sdk::MistralClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = MistralClient::new("your-api-key".to_string());
//!     // Use the client to make API calls
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod models;
pub mod client;
pub mod api;

pub use error::MistralError;
pub use client::MistralClient;
pub use models::{ModelCapabilities, BaseModelCard, FTModelCard};