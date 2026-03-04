use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Fine-tuning job object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuningJob {
    /// Job ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Model being fine-tuned
    pub model: String,
    
    /// Creation timestamp
    pub created_at: i64,
    
    /// Job status
    pub status: String,
    
    /// Training file IDs (array)
    pub training_files: Vec<String>,
    
    /// Validation file IDs (array, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_files: Option<Vec<String>>,
    
    /// Hyperparameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<HashMap<String, serde_json::Value>>,
    
    /// Fine-tuned model name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuned_model: Option<String>,
    
    /// Error message (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Fine-tuning job type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuningJobType {
    /// Completion fine-tuning
    Completion,
    /// Classifier fine-tuning
    Classifier,
}

/// Create fine-tuning job request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFineTuningJobRequest {
    /// Model to fine-tune
    pub model: String,
    
    /// Training file IDs (array)
    pub training_files: Vec<String>,
    
    /// Validation file IDs (array, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_files: Option<Vec<String>>,
    
    /// Job type (completion or classifier)
    #[serde(rename = "job_type")]
    pub job_type: FineTuningJobType,
    
    /// Hyperparameters (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<HashMap<String, serde_json::Value>>,
    
    /// Suffix for fine-tuned model name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    
    /// Auto start the job (default: true)
    #[serde(default = "default_auto_start")]
    pub auto_start: bool,
}

fn default_auto_start() -> bool {
    true
}

/// List fine-tuning jobs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFineTuningJobsResponse {
    /// List of jobs
    pub data: Vec<FineTuningJob>,
    
    /// Object type
    pub object: String,
}

/// Fine-tuning job event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuningEvent {
    /// Event ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Creation timestamp
    pub created_at: i64,
    
    /// Event level
    pub level: String,
    
    /// Event message
    pub message: String,
    
    /// Job ID
    pub job_id: String,
}

/// Fine-tuning job metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuningMetrics {
    /// Job ID
    pub id: String,
    
    /// Object type
    pub object: String,
    
    /// Training metrics
    pub training_metrics: HashMap<String, f64>,
    
    /// Validation metrics (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_metrics: Option<HashMap<String, f64>>,
}

/// Fine-tuning API client
#[derive(Debug)]
pub struct FineTuningApi {
    client: MistralClient,
}

impl FineTuningApi {
    /// Create a new Fine-Tuning API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }
    
    /// Create a fine-tuning job
    pub async fn create_job(&self, request: &CreateFineTuningJobRequest) -> Result<FineTuningJob, MistralError> {
        let response = self.client.post("/v1/fine_tuning/jobs", request).await?;
        let job: FineTuningJob = serde_json::from_str(&response)?;
        Ok(job)
    }
    
    /// List fine-tuning jobs
    pub async fn list_jobs(&self) -> Result<ListFineTuningJobsResponse, MistralError> {
        let response = self.client.get("/v1/fine_tuning/jobs", None).await?;
        let jobs: ListFineTuningJobsResponse = serde_json::from_str(&response)?;
        Ok(jobs)
    }
    
    /// Retrieve a fine-tuning job
    pub async fn retrieve_job(&self, job_id: &str) -> Result<FineTuningJob, MistralError> {
        let path = format!("/v1/fine_tuning/jobs/{}", job_id);
        let response = self.client.get(&path, None).await?;
        let job: FineTuningJob = serde_json::from_str(&response)?;
        Ok(job)
    }
    
    /// Cancel a fine-tuning job
    pub async fn cancel_job(&self, job_id: &str) -> Result<FineTuningJob, MistralError> {
        let path = format!("/v1/fine_tuning/jobs/{}/cancel", job_id);
        let response = self.client.post(&path, &serde_json::json!({})).await?;
        let job: FineTuningJob = serde_json::from_str(&response)?;
        Ok(job)
    }
    
    /// Start a fine-tuning job
    pub async fn start_job(&self, job_id: &str) -> Result<FineTuningJob, MistralError> {
        let path = format!("/v1/fine_tuning/jobs/{}/start", job_id);
        let response = self.client.post(&path, &serde_json::json!({})).await?;
        let job: FineTuningJob = serde_json::from_str(&response)?;
        Ok(job)
    }
    
    /// Get fine-tuning job events
    pub async fn get_job_events(&self, job_id: &str) -> Result<Vec<FineTuningEvent>, MistralError> {
        let path = format!("/v1/fine_tuning/jobs/{}/events", job_id);
        let response = self.client.get(&path, None).await?;
        let events: Vec<FineTuningEvent> = serde_json::from_str(&response)?;
        Ok(events)
    }
    
    /// Get fine-tuning job metrics
    pub async fn get_job_metrics(&self, job_id: &str) -> Result<FineTuningMetrics, MistralError> {
        let path = format!("/v1/fine_tuning/jobs/{}/metrics", job_id);
        let response = self.client.get(&path, None).await?;
        let metrics: FineTuningMetrics = serde_json::from_str(&response)?;
        Ok(metrics)
    }
    
    /// Archive a fine-tuned model
    pub async fn archive_model(&self, model_id: &str) -> Result<FineTuningJob, MistralError> {
        let path = format!("/v1/fine_tuning/models/{}/archive", model_id);
        let response = self.client.post(&path, &serde_json::json!({})).await?;
        let job: FineTuningJob = serde_json::from_str(&response)?;
        Ok(job)
    }
    
    /// Unarchive a fine-tuned model
    pub async fn unarchive_model(&self, model_id: &str) -> Result<FineTuningJob, MistralError> {
        let path = format!("/v1/fine_tuning/models/{}/unarchive", model_id);
        let response = self.client.post(&path, &serde_json::json!({})).await?;
        let job: FineTuningJob = serde_json::from_str(&response)?;
        Ok(job)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;
    use serde_json::json;

    #[test]
    fn test_fine_tuning_job_deserialization() {
        let json = json!({
            "id": "ftjob-123",
            "object": "fine_tuning.job",
            "model": "mistral-tiny",
            "created_at": 1234567890,
            "status": "running",
            "training_files": ["file-123"],
            "validation_files": ["file-456"],
            "hyperparameters": {
                "n_epochs": 3,
                "batch_size": 16
            },
            "fine_tuned_model": "ft:mistral-tiny:123",
            "error": null
        });
        
        let job: FineTuningJob = serde_json::from_value(json).unwrap();
        assert_eq!(job.id, "ftjob-123");
        assert_eq!(job.model, "mistral-tiny");
        assert_eq!(job.status, "running");
        assert_eq!(job.training_files, vec!["file-123"]);
        assert_eq!(job.validation_files.as_deref(), Some(&vec!["file-456".to_string()][..]));
        assert_eq!(job.fine_tuned_model.as_deref(), Some("ft:mistral-tiny:123"));
    }

    #[test]
    fn test_fine_tuning_job_type_serialization() {
        let completion_job = FineTuningJobType::Completion;
        let classifier_job = FineTuningJobType::Classifier;
        
        assert_eq!(serde_json::to_value(completion_job).unwrap(), json!("completion"));
        assert_eq!(serde_json::to_value(classifier_job).unwrap(), json!("classifier"));
    }

    #[test]
    fn test_create_fine_tuning_job_request() {
        let request = CreateFineTuningJobRequest {
            model: "open-mistral-7b".to_string(),
            training_files: vec!["file-123".to_string()],
            validation_files: Some(vec!["file-456".to_string()]),
            job_type: FineTuningJobType::Completion,
            hyperparameters: Some(HashMap::from([
                ("n_epochs".to_string(), json!(3)),
                ("batch_size".to_string(), json!(16)),
            ])),
            suffix: Some("custom-suffix".to_string()),
            auto_start: true,
        };
        
        assert_eq!(request.model, "open-mistral-7b");
        assert_eq!(request.training_files, vec!["file-123"]);
        assert_eq!(request.suffix.as_deref(), Some("custom-suffix"));
        assert!(matches!(request.job_type, FineTuningJobType::Completion));
        assert!(request.auto_start);
    }

    #[test]
    fn test_list_fine_tuning_jobs_response() {
        let json = json!({
            "data": [
                {
                    "id": "ftjob-1",
                    "object": "fine_tuning.job",
                    "model": "mistral-tiny",
                    "created_at": 1234567890,
                    "status": "completed",
                    "training_files": ["file-1"]
                },
                {
                    "id": "ftjob-2",
                    "object": "fine_tuning.job",
                    "model": "mistral-small",
                    "created_at": 1234567891,
                    "status": "failed",
                    "training_files": ["file-2"],
                    "error": "Training failed"
                }
            ],
            "object": "list"
        });
        
        let response: ListFineTuningJobsResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id, "ftjob-1");
        assert_eq!(response.data[1].status, "failed");
    }

    #[test]
    fn test_fine_tuning_event_deserialization() {
        let json = json!({
            "id": "ftevent-123",
            "object": "fine_tuning.event",
            "created_at": 1234567890,
            "level": "info",
            "message": "Training started",
            "job_id": "ftjob-123"
        });
        
        let event: FineTuningEvent = serde_json::from_value(json).unwrap();
        assert_eq!(event.id, "ftevent-123");
        assert_eq!(event.level, "info");
        assert_eq!(event.message, "Training started");
    }

    #[test]
    fn test_fine_tuning_metrics_deserialization() {
        let json = json!({
            "id": "ftjob-123",
            "object": "fine_tuning.metrics",
            "training_metrics": {
                "loss": 0.123,
                "accuracy": 0.95
            },
            "validation_metrics": {
                "loss": 0.145,
                "accuracy": 0.93
            }
        });
        
        let metrics: FineTuningMetrics = serde_json::from_value(json).unwrap();
        assert_eq!(metrics.id, "ftjob-123");
        assert_eq!(metrics.training_metrics["loss"], 0.123);
        assert_eq!(metrics.validation_metrics.as_ref().unwrap()["accuracy"], 0.93);
    }

    #[test]
    fn test_fine_tuning_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = FineTuningApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }

    #[test]
    fn test_classifier_job_request() {
        let request = CreateFineTuningJobRequest {
            model: "open-mistral-7b".to_string(),
            training_files: vec!["file-123".to_string()],
            validation_files: Some(vec!["file-456".to_string()]),
            job_type: FineTuningJobType::Classifier,
            hyperparameters: Some(HashMap::from([
                ("n_epochs".to_string(), json!(3)),
            ])),
            suffix: Some("test-classifier".to_string()),
            auto_start: false,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"job_type\":\"classifier\""));
        assert!(json.contains("\"auto_start\":false"));
    }
}