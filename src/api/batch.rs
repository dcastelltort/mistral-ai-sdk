use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Batch job object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJob {
    /// Job ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Job type
    pub job_type: String,
    
    /// Creation timestamp
    pub created_at: i64,
    
    /// Job status
    pub status: String,
    
    /// Input file ID
    pub input_file: String,
    
    /// Output file ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file: Option<String>,
    
    /// Error file ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_file: Option<String>,
    
    /// Completion window
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window: Option<String>,
    
    /// Metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    
    /// Error message (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Create batch job request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchJobRequest {
    /// Input file IDs (array)
    pub input_files: Vec<String>,
    
    /// Completion window (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window: Option<String>,
    
    /// Metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    
    /// Endpoint parameters (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    
    /// Model (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// List batch jobs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBatchJobsResponse {
    /// List of jobs
    pub data: Vec<BatchJob>,
    
    /// Object type
    pub object: String,
}

/// Batch API client
#[derive(Debug)]
pub struct BatchApi {
    client: MistralClient,
}

impl BatchApi {
    /// Create a new Batch API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// Create a batch job
    pub async fn create_job(&self, request: &CreateBatchJobRequest) -> Result<BatchJob, MistralError> {
        let response = self.client.post("/v1/batch/jobs", request).await?;
        let job: BatchJob = serde_json::from_str(&response)?;
        Ok(job)
    }
    
    /// List batch jobs
    pub async fn list_jobs(&self) -> Result<ListBatchJobsResponse, MistralError> {
        let response = self.client.get("/v1/batch/jobs", None).await?;
        let jobs: ListBatchJobsResponse = serde_json::from_str(&response)?;
        Ok(jobs)
    }
    
    /// Retrieve a batch job
    pub async fn retrieve_job(&self, job_id: &str) -> Result<BatchJob, MistralError> {
        let path = format!("/v1/batch/jobs/{}", job_id);
        let response = self.client.get(&path, None).await?;
        let job: BatchJob = serde_json::from_str(&response)?;
        Ok(job)
    }
    
    /// Cancel a batch job
    pub async fn cancel_job(&self, job_id: &str) -> Result<BatchJob, MistralError> {
        let path = format!("/v1/batch/jobs/{}/cancel", job_id);
        let response = self.client.post(&path, &serde_json::json!({})).await?;
        let job: BatchJob = serde_json::from_str(&response)?;
        Ok(job)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_batch_job_deserialization() {
        let json = json!({
            "id": "batch-123",
            "object": "batch.job",
            "job_type": "completion",
            "created_at": 1234567890,
            "status": "running",
            "input_file": "file-123",
            "output_file": "file-456",
            "error_file": "file-789",
            "completion_window": "24h",
            "metadata": {
                "user_id": "123",
                "priority": "high"
            },
            "error": null
        });
        
        let job: BatchJob = serde_json::from_value(json).unwrap();
        assert_eq!(job.id, "batch-123");
        assert_eq!(job.job_type, "completion");
        assert_eq!(job.status, "running");
        assert_eq!(job.input_file, "file-123");
        assert_eq!(job.metadata.as_ref().unwrap()["user_id"], "123");
    }

    #[test]
    fn test_create_batch_job_request() {
        let request = CreateBatchJobRequest {
            input_files: vec!["file-123".to_string()],
            completion_window: Some("24h".to_string()),
            metadata: Some(HashMap::from([
                ("user_id".to_string(), json!("123")),
                ("priority".to_string(), json!("high")),
            ])),
            endpoint: Some("/v1/chat/completions".to_string()),
            model: None,
        };
        
        assert_eq!(request.input_files, vec!["file-123"]);
        assert_eq!(request.completion_window.as_deref(), Some("24h"));
        assert_eq!(request.endpoint.as_deref(), Some("/v1/chat/completions"));
    }

    #[test]
    fn test_list_batch_jobs_response() {
        let json = json!({
            "data": [
                {
                    "id": "batch-1",
                    "object": "batch.job",
                    "job_type": "completion",
                    "created_at": 1234567890,
                    "status": "completed",
                    "input_file": "file-1"
                },
                {
                    "id": "batch-2",
                    "object": "batch.job",
                    "job_type": "embedding",
                    "created_at": 1234567891,
                    "status": "failed",
                    "input_file": "file-2",
                    "error": "Processing failed"
                }
            ],
            "object": "list"
        });
        
        let response: ListBatchJobsResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id, "batch-1");
        assert_eq!(response.data[1].status, "failed");
    }

    #[test]
    fn test_batch_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = BatchApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }

    #[test]
    fn test_batch_job_with_minimal_fields() {
        let json = json!({
            "id": "batch-123",
            "object": "batch.job",
            "job_type": "completion",
            "created_at": 1234567890,
            "status": "pending",
            "input_file": "file-123"
        });
        
        let job: BatchJob = serde_json::from_value(json).unwrap();
        assert_eq!(job.id, "batch-123");
        assert_eq!(job.status, "pending");
        assert!(job.output_file.is_none());
        assert!(job.error_file.is_none());
    }
}