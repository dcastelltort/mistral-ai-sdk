use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// File object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObject {
    /// File ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// File purpose
    pub purpose: String,
    
    /// File name
    pub filename: String,
    
    /// File size in bytes
    pub bytes: i64,
    
    /// Creation timestamp
    pub created_at: i64,
    
    /// File status
    pub status: String,
    
    /// File status details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

/// File upload request
#[derive(Debug, Clone)]
pub struct FileUploadRequest {
    /// File path
    pub file_path: String,
    
    /// File purpose
    pub purpose: String,
}

/// File upload response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResponse {
    /// File ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// File purpose
    pub purpose: String,
    
    /// File name
    pub filename: String,
    
    /// File size in bytes
    pub bytes: i64,
    
    /// Creation timestamp
    pub created_at: i64,
    
    /// File status
    pub status: String,
    
    /// File status details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

/// List files response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResponse {
    /// List of files
    pub data: Vec<FileObject>,
    
    /// Object type
    pub object: String,
}

/// File content response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContentResponse {
    /// File content
    pub content: String,
    
    /// File ID
    pub id: String,
    
    /// Object type
    pub object: String,
}

/// Signed URL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedUrlResponse {
    /// Signed URL
    pub url: String,
    
    /// Expiration timestamp
    pub expires_at: i64,
    
    /// File ID
    pub id: String,
    
    /// Object type
    pub object: String,
}

/// Files API client
#[derive(Debug)]
pub struct FilesApi {
    client: MistralClient,
}

impl FilesApi {
    /// Create a new Files API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// List all files
    pub async fn list_files(&self) -> Result<ListFilesResponse, MistralError> {
        let response = self.client.get("/v1/files", None).await?;
        let files: ListFilesResponse = serde_json::from_str(&response)?;
        Ok(files)
    }
    
    /// Upload a file
    pub async fn upload_file(&self, request: &FileUploadRequest) -> Result<FileUploadResponse, MistralError> {
        let file_path = Path::new(&request.file_path);
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| MistralError::InvalidConfiguration("Invalid file path".to_string()))?;
        
        let file_content = tokio::fs::read(&request.file_path).await
            .map_err(|e| MistralError::NetworkError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        
        let part = reqwest::multipart::Part::bytes(file_content)
            .file_name(file_name.to_string())
            .mime_str("application/octet-stream")?;
        
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("purpose", request.purpose.clone());
        
        let response = self.client.client.post("https://api.mistral.ai/v1/files")
            .multipart(form)
            .send()
            .await
            .map_err(|e| MistralError::NetworkError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        
        let status = response.status();
        let body = response.text().await
            .map_err(|e| MistralError::NetworkError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        
        if !status.is_success() {
            return Err(MistralError::from_status(status, &body));
        }
        
        let file_response: FileUploadResponse = serde_json::from_str(&body)?;
        Ok(file_response)
    }
    
    /// Retrieve a file
    pub async fn retrieve_file(&self, file_id: &str) -> Result<FileObject, MistralError> {
        let path = format!("/v1/files/{}", file_id);
        let response = self.client.get(&path, None).await?;
        let file: FileObject = serde_json::from_str(&response)?;
        Ok(file)
    }
    
    /// Delete a file
    pub async fn delete_file(&self, file_id: &str) -> Result<(), MistralError> {
        let path = format!("/v1/files/{}", file_id);
        self.client.delete(&path).await?;
        Ok(())
    }
    
    /// Get file content
    pub async fn get_file_content(&self, file_id: &str) -> Result<FileContentResponse, MistralError> {
        let path = format!("/v1/files/{}/content", file_id);
        let response = self.client.get(&path, None).await?;
        let content: FileContentResponse = serde_json::from_str(&response)?;
        Ok(content)
    }
    
    /// Get signed URL for file upload
    pub async fn get_signed_url(&self, file_name: &str, purpose: &str) -> Result<SignedUrlResponse, MistralError> {
        let path = "/v1/files/signed-url";
        let params = [("file_name", file_name), ("purpose", purpose)];
        let response = self.client.get(path, Some(&params)).await?;
        let url_response: SignedUrlResponse = serde_json::from_str(&response)?;
        Ok(url_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_file_object_deserialization() {
        let json = json!({
            "id": "file-123",
            "object": "file",
            "purpose": "fine-tune",
            "filename": "data.jsonl",
            "bytes": 1024,
            "created_at": 1234567890,
            "status": "uploaded",
            "status_details": null
        });
        
        let file: FileObject = serde_json::from_value(json).unwrap();
        assert_eq!(file.id, "file-123");
        assert_eq!(file.purpose, "fine-tune");
        assert_eq!(file.filename, "data.jsonl");
        assert_eq!(file.bytes, 1024);
    }

    #[test]
    fn test_list_files_response() {
        let json = json!({
            "data": [
                {
                    "id": "file-1",
                    "object": "file",
                    "purpose": "fine-tune",
                    "filename": "data1.jsonl",
                    "bytes": 1024,
                    "created_at": 1234567890,
                    "status": "uploaded"
                },
                {
                    "id": "file-2",
                    "object": "file",
                    "purpose": "batch",
                    "filename": "data2.jsonl",
                    "bytes": 2048,
                    "created_at": 1234567891,
                    "status": "processed"
                }
            ],
            "object": "list"
        });
        
        let response: ListFilesResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id, "file-1");
        assert_eq!(response.data[1].purpose, "batch");
    }

    #[test]
    fn test_file_upload_response() {
        let json = json!({
            "id": "file-123",
            "object": "file",
            "purpose": "fine-tune",
            "filename": "data.jsonl",
            "bytes": 1024,
            "created_at": 1234567890,
            "status": "uploaded",
            "status_details": null
        });
        
        let response: FileUploadResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.id, "file-123");
        assert_eq!(response.status, "uploaded");
    }

    #[test]
    fn test_file_content_response() {
        let json = json!({
            "content": "file content here",
            "id": "file-123",
            "object": "file.content"
        });
        
        let response: FileContentResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.content, "file content here");
        assert_eq!(response.id, "file-123");
    }

    #[test]
    fn test_signed_url_response() {
        let json = json!({
            "url": "https://signed-url.example.com",
            "expires_at": 1234567890,
            "id": "file-123",
            "object": "file.signed_url"
        });
        
        let response: SignedUrlResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.url, "https://signed-url.example.com");
        assert_eq!(response.expires_at, 1234567890);
    }

    #[test]
    fn test_files_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = FilesApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }

    #[test]
    fn test_file_upload_request() {
        let request = FileUploadRequest {
            file_path: "/path/to/file.jsonl".to_string(),
            purpose: "fine-tune".to_string(),
        };
        
        assert_eq!(request.file_path, "/path/to/file.jsonl");
        assert_eq!(request.purpose, "fine-tune");
    }
}