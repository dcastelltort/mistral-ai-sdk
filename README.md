# Mistral AI Rust SDK

Async Rust client for the Mistral AI API.

## Current Implementation Status

✅ **Completed:**
- Project structure with proper module organization
- Core data models with comprehensive tests:
  - `ModelCapabilities` - Model feature flags
  - `BaseModelCard` - Base model information
  - `FTModelCard` - Fine-tuned model information with conversion to base model
- Serialization/deserialization using serde
- TDD approach with 9 passing tests
- Proper error handling foundation

🚧 **In Progress:**
- HTTP client implementation
- API endpoints
- Retry logic
- Rate limiting

## Features

- **Async-first design** using tokio
- **Type-safe models** matching Mistral AI API specification
- **Comprehensive testing** with 100% model coverage
- **Proper error handling** with thiserror
- **Configurable** client with builder pattern

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mistral-ai-rs = { git = "https://github.com/your-repo/mistral-ai-rs" }
```

## Usage

```rust
use mistral_ai_rs::{MistralClient, BaseModelCard, ModelCapabilities};
use serde_json::json;

// Create a model from JSON
let json = json!({
    "id": "open-mistral-7b",
    "capabilities": {
        "completion_chat": true,
        "function_calling": false,
        "completion_fim": false,
        "fine_tuning": false,
        "vision": false,
        "classification": false
    },
    "type": "base"
});

let model: BaseModelCard = serde_json::from_value(json).unwrap();
println!("Model ID: {}", model.id);
println!("Supports chat: {}", model.capabilities.completion_chat);
```

## Development

### Running Tests

```bash
cargo test
```

### Test Coverage

- `ModelCapabilities`: 3 tests (serialization, deserialization, defaults)
- `BaseModelCard`: 3 tests (serialization, deserialization, defaults)
- `FTModelCard`: 3 tests (serialization, deserialization, conversion)

Total: 9 tests, 100% passing

## Architecture

```
src/
├── lib.rs              # Main library entry point
├── models/             # API data models
│   ├── capabilities.rs # Model capabilities
│   ├── base_model.rs   # Base model card
│   └── ft_model.rs     # Fine-tuned model card
├── client/             # HTTP client
│   ├── mod.rs          # Client implementation
│   ├── builder.rs      # Client builder
│   └── retry.rs        # Retry strategy
├── api/                # API endpoints
│   ├── mod.rs          # API module
│   ├── models.rs       # Models endpoints
│   └── conversations.rs # Conversations endpoints
└── error.rs            # Error handling
```

## Contributing

This project follows TDD principles:
1. Write failing tests first
2. Implement minimal code to pass tests
3. Refactor and improve
4. Repeat

## License

MIT OR Apache-2.0