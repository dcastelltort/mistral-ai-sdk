# Mistral AI Rust SDK Examples

This directory contains educational examples demonstrating how to use the Mistral AI Rust SDK.

## Getting Started

### Prerequisites

1. **Rust Toolchain**: Install Rust from [rust-lang.org](https://www.rust-lang.org/)
2. **API Key**: Get your Mistral AI API key from the [Mistral AI platform](https://mistral.ai/)

### Configuration

Copy the `.env.example` file to `.env` and add your API key:

```bash
cp .env.example .env
```

Then edit `.env` and set your API key:

```env
MISTRAL_API_KEY=your_api_key_here
```

## Newly Implemented APIs

The following APIs have been recently implemented and are available for use:

### FIM (Fill-in-the-Middle)

Demonstrates fill-in-the-middle text generation.

**Important API Update:** The FIM API has been completely restructured to match the official Mistral API specification. The old `messages[]` structure has been replaced with a `prompt`/`suffix` approach.

**Example Usage:**
```rust
use mistral_ai_rs::api::fim::{FIMCompletionRequest, FIMApi};

let request = FIMCompletionRequest {
    model: "codestral-latest".to_string(),
    prompt: "def add_numbers(a: int, b: int) -> int:".to_string(),
    suffix: Some("    return a + b".to_string()), // Optional suffix
    temperature: Some(0.2),
    max_tokens: Some(200),
};
```

**Key Changes:**
- ✅ Replaced `messages: Vec<FIMMessage>` with `prompt: String`
- ✅ Added optional `suffix: String` for FIM context
- ✅ Removed `FIMMessage` struct (not used by API)
- ✅ Updated to match official OpenAPI specification
- ✅ All tests updated and passing

**Usage:**
```bash
cargo run --example fim_completion -- <prompt>
```

### OCR (Optical Character Recognition)

Shows how to perform OCR on documents and images.

**Available OCR Models:**
- `mistral-ocr-latest` (recommended)
- `mistral-ocr-2512`
- `mistral-ocr-2505`
- `mistral-ocr-2503` (deprecated)

**Example Usage:**
```rust
use mistral_ai_rs::api::ocr::{OCRRequest, OCRApi, OCRDocument, DocumentURLChunk};

let request = OCRRequest {
    model: Some("mistral-ocr-latest".to_string()),
    id: Some("example-ocr-job".to_string()),
    document: OCRDocument::DocumentURL(DocumentURLChunk {
        type_field: "document_url".to_string(),
        document_url: "https://example.com/document.pdf".to_string(),
        document_name: Some("example_document.pdf".to_string()),
    }),
    pages: Some(vec![0, 1]), // Process first 2 pages
    include_image_base64: Some(false),
    image_limit: Some(5),
};
```

**Usage:**
```bash
cargo run --example ocr_document -- <document_url>
```

### Audio Transcription

Demonstrates audio file transcription.

**Example Usage:**
```rust
use mistral_ai_rs::api::audio::{AudioTranscriptionRequest, AudioApi};

let request = AudioTranscriptionRequest {
    file_id: Some("file-123".to_string()),
    file_url: None,
    language: Some("en".to_string()),
    timestamp_granularities: Some(vec!["word".to_string(), "segment".to_string()]),
};
```

## Available Examples

### 1. Chat Completion

Demonstrates how to create chat completions with the Mistral AI API.

**Usage:**
```bash
cargo run --example chat_completion -- "Hello, how are you?"
```

**Features:**
- Basic chat completion request
- Temperature and max tokens configuration
- Response parsing and display

### 2. Embeddings

Shows how to generate text embeddings using Mistral AI models.

**Usage:**
```bash
cargo run --example embeddings -- "Hello world"
```

**Features:**
- Text embedding generation
- Encoding format selection
- Embedding dimension analysis
- Token usage tracking

### 3. Moderations

Demonstrates content moderation for safety classification.

**Usage:**
```bash
cargo run --example moderations -- "This is test content"
```

**Features:**
- Safety violation detection
- Category-specific analysis (hate, harassment, sexual, violence, self-harm)
- Confidence scoring
- Flagged content identification

### 4. File Upload

Shows how to upload files to the Mistral AI platform.

**Usage:**
```bash
# With custom file and purpose
cargo run --example file_upload -- "data.txt" "fine-tune"

# Using default example file
cargo run --example file_upload
```

**Features:**
- File upload with purpose specification
- Supported purposes: `fine-tune`, `batch`
- Automatic UUID validation and conversion
- File validation and error handling
- Upload progress tracking

**Note:** File must be in newline-delimited JSON format for fine-tuning.

### 5. Batch Processing

Demonstrates batch job creation and management.

**Usage:**
```bash
# With custom parameters
cargo run --example batch_job -- "file-123" "/v1/chat/completions" "24h"

# Using default values (auto-generates valid UUID)
cargo run --example batch_job
```

**Features:**
- Batch job creation with input files
- Automatic UUID validation and conversion
- Completion window configuration
- Job listing and status monitoring
- Metadata support
- Default model selection

**Note:** File IDs must be valid UUIDs. The example automatically converts invalid formats.

### 6. Fine-Tuning

Shows how to create and manage fine-tuning jobs.

**Usage:**
```bash
# Using a model that supports fine-tuning
cargo run --example fine_tuning -- "open-mistral-nemo" "file-train-123" "file-val-456"

# Or with a medium model
cargo run --example fine_tuning -- "mistral-medium-latest" "file-train-123" "file-val-456"
```

**Features:**
- Fine-tuning job creation with array-based file specification
- Training and validation files (arrays)
- Hyperparameter configuration (n_epochs, batch_size, learning_rate)
- Job monitoring and retrieval
- Automatic model suffix generation
- Automatic UUID validation and conversion

**Supported Models:**
- `open-mistral-nemo` (and aliases: `mistral-tiny-latest`, `mistral-tiny-2407`)
- `mistral-medium-latest` (and aliases: `mistral-medium`, `mistral-medium-2508`)
- `mistral-large-2411`

**Note:** File IDs must be valid UUIDs. The example automatically converts invalid formats like "file-train-123" to proper UUIDs.

### 7. Conversations

Demonstrates conversation management with the Mistral AI API.

**Usage:**
```bash
cargo run --example conversations -- "Hello, let's chat!"
```

**Features:**
- Conversation creation with the new input entry format
- Conversation listing
- Individual conversation retrieval
- Message history management
- Proper input entry serialization with entry types

**Note:** The conversations API uses a specific input format. The example demonstrates the correct structure:

- User-provided input entries should NOT include IDs (the API generates them automatically)
- Either `model` or `agent_id` must be specified (they are mutually exclusive)
- The example uses `model: "mistral-medium-latest"` for model-based conversations

**Fixed Issues:**
- ✅ Resolved ID field conflicts with API validation
- ✅ Proper model specification
- ✅ Correct request serialization
- ✅ Example now works with the current API

## Newly Implemented APIs

The following APIs have been recently implemented and are available for use:

### FIM (Fill-in-the-Middle)

Demonstrates fill-in-the-middle text generation.

**Example Usage:**
```rust
use mistral_ai_rs::api::fim::{FIMCompletionRequest, FIMMessage, FIMApi};

let request = FIMCompletionRequest {
    model: "mistral-tiny".to_string(),
    messages: vec![
        FIMMessage {
            role: "user".to_string(),
            content: "Hello, world! This is a "
        }
    ],
    suffix: Some(" test.".to_string()),
    max_tokens: Some(50),
    temperature: Some(0.7),
};
```

### OCR (Optical Character Recognition)

Shows how to perform OCR on documents and images.

**Example Usage:**
```rust
use mistral_ai_rs::api::ocr::{OCRRequest, OCRApi};

let request = OCRRequest {
    file_id: Some("file-123".to_string()),
    document_url: None,
    image_url: None,
    language: Some("en".to_string()),
};
```

### Audio Transcription

Demonstrates audio file transcription.

**Example Usage:**
```rust
use mistral_ai_rs::api::audio::{AudioTranscriptionRequest, AudioApi};

let request = AudioTranscriptionRequest {
    file_id: Some("file-123".to_string()),
    file_url: None,
    language: Some("en".to_string()),
    timestamp_granularities: Some(vec!["word".to_string(), "segment".to_string()]),
};
```

### 8. Agent Management

Demonstrates agent creation and management with the Mistral AI API.

**Usage:**
```bash
cargo run --example agent_management
```

**Features:**
- Agent creation with custom tools and completion parameters
- Agent listing and retrieval
- Agent version management
- Agent alias creation and management
- Agent updating and deletion
- Comprehensive error handling

**Agent Tools Supported:**
- Web search (standard and premium)
- Code interpreter
- Image generation
- Function tools with custom parameters
- Document library integration

### 9. Text Classification

Demonstrates content moderation and safety classification.

**Usage:**
```bash
cargo run --example text_classification
```

**Features:**
- Single and batch text classification
- Chat message classification
- Multi-category safety scoring
- Custom threshold analysis
- Metadata support
- Moderation workflows

**Classification Categories:**
- Hate speech detection
- Harassment detection
- Violence detection
- Sexual content detection
- Self-harm detection
- Sexual/minors detection

### 10. Document Library

Demonstrates document library management for RAG applications.

**Usage:**
```bash
cargo run --example document_library
```

**Features:**
- Library creation and management
- Document upload and processing
- Document text content retrieval
- Chunked document processing
- Document status monitoring
- Library sharing and collaboration
- Signed URL generation for secure access

**RAG Workflow Support:**
- Custom chunk sizes for optimal retrieval
- Metadata preservation and search
- Document reprocessing
- Progress tracking
- Secure document access via signed URLs

### 11. Model Listing

Shows how to list available models and retrieve model details.

**Usage:**
```bash
# List all models to find fine-tuning capable ones
cargo run --example models_list
```

**Features:**
- List all available models
- Model details retrieval
- Capabilities analysis (vision, function calling, fine-tuning, etc.)
- Base and fine-tuned model support
- Identify models that support fine-tuning (`"fine_tuning": true`)

**Finding Fine-Tuning Models:**
Run this example to see which models have `"fine_tuning": true` in their capabilities.
Current fine-tuning capable models include:
- `open-mistral-nemo` and its aliases
- `mistral-medium-latest` and its aliases  
- `mistral-large-2411`

## Running Examples

All examples require the `MISTRAL_API_KEY` environment variable:

```bash
# Set API key and run an example
MISTRAL_API_KEY=your_key cargo run --example chat_completion -- "Hello!"

# Or use the .env file
source .env
cargo run --example embeddings -- "Sample text"
```

## Error Handling

All examples include comprehensive error handling with:

- **Contextual Error Messages**: Clear explanations of what went wrong
- **Graceful Failures**: Proper error propagation and user-friendly messages
- **Configuration Validation**: Input validation and helpful usage instructions

## Best Practices Demonstrated

1. **API Key Management**: Using environment variables for sensitive data
2. **Async/Await**: Proper async patterns with Tokio
3. **Error Handling**: Using `anyhow` for graceful error management
4. **JSON Handling**: Pretty printing responses with `serde_json`
5. **Input Validation**: Command-line argument parsing and validation

## Creating Your Own Examples

To create a new example:

1. Create a new file in the `examples/` directory
2. Add proper documentation with `//!` comments
3. Use the existing examples as templates
4. Follow the pattern of creating API-specific clients
5. Include comprehensive error handling

## Troubleshooting

**Missing API Key:**
```
Error: Missing MISTRAL_API_KEY environment variable
Solution: Set the environment variable or create a .env file
```

**Invalid Arguments:**
```
Error: Usage: cargo run --example <name> -- <required_args>
Solution: Check the example's usage instructions
```

**Network Errors:**
```
Error: Failed to connect to Mistral AI API
Solution: Check your internet connection and API key validity
```

**UUID Validation Errors:**
```
Error: Input should be a valid UUID
Solution: Use proper UUID format (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx) or let the example auto-convert
```

**File Not Found Errors:**
```
Error: File XXX not found
Solution: Ensure the file ID exists and was uploaded successfully
```

## Contributing

Contributions to the examples are welcome! Please ensure:

- Examples are well-documented
- Error handling is comprehensive
- Code follows Rust best practices
- New examples cover unique use cases
