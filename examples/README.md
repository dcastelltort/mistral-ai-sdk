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
cargo run --example file_upload -- "data.txt" "fine-tune"
```

**Features:**
- File upload with purpose specification
- Supported purposes: `fine-tune`, `batch`
- File validation
- Upload progress tracking

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

## Contributing

Contributions to the examples are welcome! Please ensure:

- Examples are well-documented
- Error handling is comprehensive
- Code follows Rust best practices
- New examples cover unique use cases
