# Model Discovery Skill (Repository-Specific)

## Scope
ONLY applies to current repository

## Purpose
Teach users how to use the models_list example to discover available models, their capabilities, and proper usage patterns for troubleshooting API issues.

## Activation Triggers
- User asks about available models
- User mentions model-related errors ("Invalid model", "model not found", etc.)
- User asks how to find correct model names
- User wants to know model capabilities
- API returns model-related errors

## Teaching Approach

### 1. Proactive Suggestion
When user encounters model-related issues, suggest:
```
💡 Tip: You can list all available models and their capabilities by running:

MISTRAL_API_KEY=your_key cargo run --example models_list

This will show you:
- All available model IDs
- Model capabilities (vision, function_calling, fim, etc.)
- Model aliases and deprecation status
- Proper model names for different APIs
```

### 2. Common Use Cases

#### Finding OCR Models
**Problem:** "OCR API says Invalid model"
**Solution:** Run models_list and look for models with `"vision": true` capability
**Example Output:**
```json
{
  "id": "mistral-ocr-latest",
  "capabilities": {
    "vision": true,
    "function_calling": true
  }
}
```

#### Finding Fine-Tuning Models
**Problem:** "Which models support fine-tuning?"
**Solution:** Run models_list and look for `"fine_tuning": true`
**Example Models:**
- `open-mistral-nemo` and aliases
- `mistral-medium-latest` and aliases
- `mistral-large-2411`

#### Finding FIM Models
**Problem:** "What models support FIM completion?"
**Solution:** Run models_list and look for `"completion_fim": true`
**Example Models:**
- `codestral-latest`
- `codestral-2508`

### 3. Troubleshooting Guide

**When API returns "Invalid model" error:**
1. Run: `MISTRAL_API_KEY=your_key cargo run --example models_list`
2. Search for models with required capabilities
3. Use the exact model ID from the output
4. Check for deprecation warnings

**Example Workflow:**
```bash
# 1. List all models
MISTRAL_API_KEY=your_key cargo run --example models_list

# 2. Find OCR-capable models (look for "vision": true)
# 3. Use the correct model in your API call
MISTRAL_API_KEY=your_key cargo run --example ocr_document -- "https://example.com/doc.pdf"
```

### 4. Model Capabilities Reference

| Capability | API Usage | Example Models |
|------------|-----------|----------------|
| `vision: true` | OCR, image processing | mistral-ocr-latest |
| `fine_tuning: true` | Fine-tuning jobs | open-mistral-nemo |
| `completion_fim: true` | FIM completion | codestral-latest |
| `function_calling: true` | Tool/function calling | mistral-medium-latest |
| `classification: true` | Text classification | mistral-classification-latest |

### 5. Best Practices

✅ **Always check models_list first** when encountering model errors
✅ **Use exact model IDs** from the API response
✅ **Check capabilities** to ensure model supports your use case
✅ **Prefer non-deprecated models** (check deprecation field)
✅ **Use aliases** for future-proofing (e.g., "mistral-ocr-latest")

### 6. Example Output Interpretation

```json
{
  "id": "mistral-ocr-latest",
  "object": "model",
  "capabilities": {
    "vision": true,          // ✅ Supports OCR/image processing
    "function_calling": true, // ✅ Supports function calls
    "fine_tuning": false,    // ❌ No fine-tuning support
    "classification": false  // ❌ No classification support
  },
  "aliases": ["mistral-ocr-2512"],  // Alternative names
  "deprecation": null       // ✅ Not deprecated
}
```

### 7. Common Model Categories

**OCR/Vision Models:**
- `mistral-ocr-latest` (recommended)
- `mistral-ocr-2512`
- `mistral-ocr-2505`

**Fine-Tuning Models:**
- `open-mistral-nemo`
- `mistral-medium-latest`
- `mistral-large-2411`

**FIM Models:**
- `codestral-latest`
- `codestral-2508`

**Classification Models:**
- `mistral-classification-latest`

### 8. Integration with Workflow

**Before implementing any API:**
1. Check models_list for available models
2. Verify model capabilities match your needs
3. Use the correct model ID in your code

**When debugging:**
1. Run models_list to verify model exists
2. Check if model has required capabilities
3. Confirm model isn't deprecated

## Verification

After teaching this skill:
- User understands how to discover models
- User can troubleshoot model-related errors
- User knows where to find capability information
- User can make informed model choices

## Example Responses

**User:** "OCR API says Invalid model"
**Response:** "Let's check what OCR models are available. Run this command to see all models and their capabilities:

```bash
MISTRAL_API_KEY=your_key cargo run --example models_list
```

Look for models with `\"vision\": true` in their capabilities. The OCR models are typically named `mistral-ocr-*`."

**User:** "Which model should I use for fine-tuning?"
**Response:** "Great question! You can list all models with fine-tuning support by running:

```bash
MISTRAL_API_KEY=your_key cargo run --example models_list
```

Look for models where `\"fine_tuning\": true`. The current fine-tuning capable models include `open-mistral-nemo`, `mistral-medium-latest`, and `mistral-large-2411`."

## Success Criteria

✅ User can independently run models_list example
✅ User understands how to interpret model capabilities
✅ User can troubleshoot model-related API errors
✅ User makes informed model selections based on capabilities
