use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};

/// OCR request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRRequest {
    /// Model to use for OCR (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    
    /// Request ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    /// Document to process
    pub document: OCRDocument,
    
    /// Specific pages to process (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<i32>>,
    
    /// Include image base64 in response (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_image_base64: Option<bool>,
    
    /// Image limit (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_limit: Option<i32>,
}

/// OCR document source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OCRDocument {
    File(FileChunk),
    DocumentURL(DocumentURLChunk),
    ImageURL(ImageURLChunk),
}

/// File chunk for OCR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    #[serde(rename = "type")]
    pub type_field: String,
    
    pub file_id: String,
}

/// Document URL chunk for OCR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentURLChunk {
    #[serde(rename = "type")]
    pub type_field: String,
    
    pub document_url: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
}

/// Image URL chunk for OCR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageURLChunk {
    #[serde(rename = "type")]
    pub type_field: String,
    
    pub image_url: String,
}

/// OCR response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRResponse {
    /// List of OCR pages
    pub pages: Vec<OCRPageObject>,
    
    /// Model used
    pub model: String,
    
    /// Document annotation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_annotation: Option<String>,
    
    /// Usage information
    pub usage_info: OCRUsageInfo,
}

/// OCR page object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRPageObject {
    /// Page index
    pub index: i32,
    
    /// Markdown content
    pub markdown: String,
    
    /// Extracted images (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<OCRImageObject>>,
    
    /// Extracted tables (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<OCRTableObject>>,
    
    /// Page dimensions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<OCRDimensions>,
}

/// OCR image object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRImageObject {
    /// Image ID
    pub id: String,
    
    /// Image format
    pub format: String,
    
    /// Image content
    pub content: String,
    
    /// Bounding box
    pub bounding_box: OCRBoundingBox,
}

/// OCR table object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRTableObject {
    /// Table ID
    pub id: String,
    
    /// Table content
    pub content: String,
    
    /// Table format
    pub format: String,
}

/// OCR dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRDimensions {
    /// Width
    pub width: f32,
    
    /// Height
    pub height: f32,
    
    /// Unit
    pub unit: String,
}

/// OCR bounding box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRBoundingBox {
    /// X coordinate
    pub x: f32,
    
    /// Y coordinate
    pub y: f32,
    
    /// Width
    pub width: f32,
    
    /// Height
    pub height: f32,
}

/// OCR usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRUsageInfo {
    /// Number of pages processed
    pub pages_processed: i32,
}

/// OCR API client
#[derive(Debug)]
pub struct OCRApi {
    client: MistralClient,
}

impl OCRApi {
    /// Create a new OCR API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }

    /// Perform OCR on a document
    pub async fn perform_ocr(&self, request: &OCRRequest) -> Result<OCRResponse, MistralError> {
        let response = self.client.post("/v1/ocr", request).await?;
        let ocr_result: OCRResponse = serde_json::from_str(&response)?;
        Ok(ocr_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;

    #[test]
    fn test_ocr_request_serialization() {
        let request = OCRRequest {
            model: Some("ocr-model-latest".to_string()),
            id: Some("test-ocr-123".to_string()),
            document: OCRDocument::DocumentURL(DocumentURLChunk {
                type_field: "document_url".to_string(),
                document_url: "https://example.com/document.pdf".to_string(),
                document_name: Some("test_document.pdf".to_string()),
            }),
            pages: Some(vec![0, 1, 2]),
            include_image_base64: Some(true),
            image_limit: Some(10),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ocr-model-latest"));
        assert!(json.contains("https://example.com/document.pdf"));
        assert!(json.contains("test_document.pdf"));

        let deserialized: OCRRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.model, Some("ocr-model-latest".to_string()));
        assert_eq!(deserialized.id, Some("test-ocr-123".to_string()));
        assert_eq!(deserialized.pages, Some(vec![0, 1, 2]));
    }


    #[test]
    fn test_ocr_response_deserialization() {
        let json_response = r#"{
            "pages": [{
                "index": 0,
                "markdown": "Test Document Content",
                "images": [{
                    "id": "img-001",
                    "format": "png",
                    "content": "base64data",
                    "bounding_box": {"x": 10.5, "y": 20.3, "width": 100.0, "height": 50.0}
                }],
                "tables": [{
                    "id": "table-001",
                    "content": "Table Content",
                    "format": "markdown"
                }],
                "dimensions": {"width": 8.5, "height": 11.0, "unit": "in"}
            }],
            "model": "ocr-model-v1",
            "document_annotation": null,
            "usage_info": {"pages_processed": 1}
        }"#;

        let response: OCRResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.pages.len(), 1);
        assert_eq!(response.model, "ocr-model-v1");
        assert_eq!(response.usage_info.pages_processed, 1);
        
        let page = &response.pages[0];
        assert_eq!(page.index, 0);
        assert_eq!(page.markdown, "Test Document Content");
        assert_eq!(page.images.as_ref().unwrap().len(), 1);
        assert_eq!(page.tables.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_ocr_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = OCRApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }
}
