// Libraries API implementation
// This module provides functionality for document library management

use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Library creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryIn {
    /// Library name
    pub name: String,
    
    /// Library description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Chunk size for document processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<i32>,
}

/// Library update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryInUpdate {
    /// Library name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Library description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Library response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryOut {
    /// Library ID
    pub id: String,
    
    /// Library name
    pub name: String,
    
    /// Library description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Creation timestamp
    pub created_at: String,
    
    /// Update timestamp
    pub updated_at: String,
    
    /// Owner ID
    pub owner_id: String,
    
    /// Chunk size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<i32>,
    
    /// Document count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_count: Option<i32>,
    
    /// Total chunks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_chunks: Option<i32>,
    
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// List libraries response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLibraryOut {
    /// List of libraries
    pub data: Vec<LibraryOut>,
    
    /// Pagination information
    #[serde(default)]
    pub has_more: bool,
}

/// Document upload request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUploadRequest {
    /// Document file (URL or base64 encoded content)
    pub file: String,
    
    /// Document metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Document response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentOut {
    /// Document ID
    pub id: String,
    
    /// Document URL
    pub url: String,
    
    /// Library ID
    pub library_id: String,
    
    /// Document status
    pub status: String,
    
    /// Creation timestamp
    pub created_at: String,
    
    /// Update timestamp
    pub updated_at: String,
    
    /// Document metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    
    /// Chunk count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_count: Option<i32>,
    
    /// Error message (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// List documents response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentsOut {
    /// List of documents
    pub data: Vec<DocumentOut>,
    
    /// Pagination information
    pub has_more: bool,
}

/// Document text content response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTextContent {
    /// Document ID
    pub id: String,
    
    /// Text content
    pub text: String,
    
    /// Chunk information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk: Option<DocumentChunk>,
}

/// Document chunk information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    /// Chunk index
    pub index: i32,
    
    /// Total chunks
    pub total: i32,
    
    /// Chunk text
    pub text: String,
}

/// Document status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStatus {
    /// Document ID
    pub id: String,
    
    /// Document status
    pub status: String,
    
    /// Error message (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    
    /// Progress information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
}

/// Signed URL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedUrlResponse {
    /// Signed URL
    pub url: String,
    
    /// Expiration timestamp
    pub expires_at: String,
}

/// Share library request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareLibraryRequest {
    /// Organization ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    
    /// Access level (Viewer or Editor)
    pub level: String,
    
    /// UUID of the entity to share with
    pub share_with_uuid: String,
    
    /// Type of entity (User, Workspace, or Org)
    pub share_with_type: String,
}

/// Libraries API client
#[derive(Debug)]
pub struct LibrariesApi {
    client: MistralClient,
}

impl LibrariesApi {
    /// Create a new Libraries API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// Create a new library
    pub async fn create_library(&self, request: &LibraryIn) -> Result<LibraryOut, MistralError> {
        let response = self.client.post("/v1/libraries", request).await?;
        let library: LibraryOut = serde_json::from_str(&response)?;
        Ok(library)
    }
    
    /// List all libraries
    pub async fn list_libraries(&self) -> Result<ListLibraryOut, MistralError> {
        let response = self.client.get("/v1/libraries", None).await?;
        let libraries: ListLibraryOut = serde_json::from_str(&response)?;
        Ok(libraries)
    }
    
    /// Get a specific library by ID
    pub async fn get_library(&self, library_id: &str) -> Result<LibraryOut, MistralError> {
        let path = format!("/v1/libraries/{}", library_id);
        let response = self.client.get(&path, None).await?;
        let library: LibraryOut = serde_json::from_str(&response)?;
        Ok(library)
    }
    
    /// Update a library
    pub async fn update_library(&self, library_id: &str, request: &LibraryInUpdate) -> Result<LibraryOut, MistralError> {
        let path = format!("/v1/libraries/{}", library_id);
        let response = self.client.put(&path, request).await?;
        let library: LibraryOut = serde_json::from_str(&response)?;
        Ok(library)
    }
    
    /// Delete a library
    pub async fn delete_library(&self, library_id: &str) -> Result<(), MistralError> {
        let path = format!("/v1/libraries/{}", library_id);
        self.client.delete(&path).await?;
        Ok(())
    }
    
    /// Upload a document to a library
    pub async fn upload_document(&self, library_id: &str, request: &DocumentUploadRequest) -> Result<DocumentOut, MistralError> {
        let path = format!("/v1/libraries/{}/documents", library_id);
        let response = self.client.post(&path, request).await?;
        let document: DocumentOut = serde_json::from_str(&response)?;
        Ok(document)
    }
    
    /// List documents in a library
    pub async fn list_documents(&self, library_id: &str) -> Result<ListDocumentsOut, MistralError> {
        let path = format!("/v1/libraries/{}/documents", library_id);
        let response = self.client.get(&path, None).await?;
        let documents: ListDocumentsOut = serde_json::from_str(&response)?;
        Ok(documents)
    }
    
    /// Get a specific document
    pub async fn get_document(&self, library_id: &str, document_id: &str) -> Result<DocumentOut, MistralError> {
        let path = format!("/v1/libraries/{}/documents/{}", library_id, document_id);
        let response = self.client.get(&path, None).await?;
        let document: DocumentOut = serde_json::from_str(&response)?;
        Ok(document)
    }
    
    /// Get document text content
    pub async fn get_document_text(&self, library_id: &str, document_id: &str) -> Result<DocumentTextContent, MistralError> {
        let path = format!("/v1/libraries/{}/documents/{}/text_content", library_id, document_id);
        let response = self.client.get(&path, None).await?;
        let text_content: DocumentTextContent = serde_json::from_str(&response)?;
        Ok(text_content)
    }
    
    /// Get document status
    pub async fn get_document_status(&self, library_id: &str, document_id: &str) -> Result<DocumentStatus, MistralError> {
        let path = format!("/v1/libraries/{}/documents/{}/status", library_id, document_id);
        let response = self.client.get(&path, None).await?;
        let status: DocumentStatus = serde_json::from_str(&response)?;
        Ok(status)
    }
    
    /// Get signed URL for document
    pub async fn get_signed_url(&self, library_id: &str, document_id: &str) -> Result<SignedUrlResponse, MistralError> {
        let path = format!("/v1/libraries/{}/documents/{}/signed-url", library_id, document_id);
        let response = self.client.get(&path, None).await?;
        let signed_url: SignedUrlResponse = serde_json::from_str(&response)?;
        Ok(signed_url)
    }
    
    /// Get extracted text signed URL
    pub async fn get_extracted_text_signed_url(&self, library_id: &str, document_id: &str) -> Result<SignedUrlResponse, MistralError> {
        let path = format!("/v1/libraries/{}/documents/{}/extracted-text-signed-url", library_id, document_id);
        let response = self.client.get(&path, None).await?;
        let signed_url: SignedUrlResponse = serde_json::from_str(&response)?;
        Ok(signed_url)
    }
    
    /// Reprocess a document
    pub async fn reprocess_document(&self, library_id: &str, document_id: &str) -> Result<DocumentOut, MistralError> {
        let path = format!("/v1/libraries/{}/documents/{}/reprocess", library_id, document_id);
        let response = self.client.post(&path, &serde_json::json!({})).await?;
        let document: DocumentOut = serde_json::from_str(&response)?;
        Ok(document)
    }
    
    /// Share a library
    pub async fn share_library(&self, library_id: &str, request: &ShareLibraryRequest) -> Result<(), MistralError> {
        let path = format!("/v1/libraries/{}/share", library_id);
        self.client.put(&path, request).await?;
        Ok(())
    }
    
    /// Delete a document
    pub async fn delete_document(&self, library_id: &str, document_id: &str) -> Result<(), MistralError> {
        let path = format!("/v1/libraries/{}/documents/{}", library_id, document_id);
        self.client.delete(&path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_library_in_serialization() {
        let request = LibraryIn {
            name: "Test Library".to_string(),
            description: Some("A test library for documents".to_string()),
            chunk_size: Some(512),
        };

        let json = serde_json::to_value(request).unwrap();
        assert_eq!(json["name"], "Test Library");
        assert_eq!(json["description"], "A test library for documents");
        assert_eq!(json["chunk_size"], 512);
    }

    #[test]
    fn test_library_out_deserialization() {
        let json = json!({
            "id": "lib-123",
            "name": "Test Library",
            "description": "A test library",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z",
            "owner_id": "user-123",
            "chunk_size": 512,
            "document_count": 10,
            "total_chunks": 100,
            "status": "active"
        });

        let library: LibraryOut = serde_json::from_value(json).unwrap();
        assert_eq!(library.id, "lib-123");
        assert_eq!(library.name, "Test Library");
        assert_eq!(library.document_count.unwrap(), 10);
    }

    #[test]
    fn test_document_upload_request_serialization() {
        let request = DocumentUploadRequest {
            file: "https://example.com/document.pdf".to_string(),
            metadata: Some(HashMap::from([
                ("title".to_string(), json!("Test Document")),
                ("author".to_string(), json!("John Doe")),
            ])),
        };

        let json = serde_json::to_value(request).unwrap();
        assert_eq!(json["file"], "https://example.com/document.pdf");
        assert_eq!(json["metadata"]["title"], "Test Document");
    }

    #[test]
    fn test_document_out_deserialization() {
        let json = json!({
            "id": "doc-123",
            "url": "https://example.com/document.pdf",
            "library_id": "lib-123",
            "status": "processed",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z",
            "metadata": {"title": "Test Document"},
            "chunk_count": 42,
            "error": null
        });

        let document: DocumentOut = serde_json::from_value(json).unwrap();
        assert_eq!(document.id, "doc-123");
        assert_eq!(document.url, "https://example.com/document.pdf");
        assert_eq!(document.status, "processed");
    }

    #[test]
    fn test_document_text_content_deserialization() {
        let json = json!({
            "id": "doc-123",
            "text": "This is the extracted text content from the document.",
            "chunk": {
                "index": 0,
                "total": 5,
                "text": "This is the extracted text content from the document."
            }
        });

        let text_content: DocumentTextContent = serde_json::from_value(json).unwrap();
        assert_eq!(text_content.id, "doc-123");
        assert!(text_content.text.contains("extracted text"));
        assert_eq!(text_content.chunk.unwrap().index, 0);
    }

    #[test]
    fn test_libraries_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = LibrariesApi::new(client);
        assert_eq!(api.client.api_key, "test-key");
    }
}