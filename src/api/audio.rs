use crate::client::MistralClient;
use crate::error::MistralError;
use serde::{Deserialize, Serialize};

/// Audio transcription request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTranscriptionRequest {
    /// Model to use for transcription
    pub model: String,
    
    /// Audio file (optional - use file_url or file_id instead for API)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    
    /// URL of audio file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    
    /// ID of uploaded file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    
    /// Language (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    
    /// Temperature (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    
    /// Response format (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    
    /// Prompt (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    
    /// Timestamp granularities (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<String>>,
}

/// Audio transcription response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResponse {
    /// Model used
    pub model: String,
    
    /// Transcribed text
    pub text: String,
    
    /// Language
    pub language: String,
    
    /// Segments (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegment>>,
    
    /// Usage information
    pub usage: UsageInfo,
}

/// Transcription segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    /// Segment text
    pub text: String,
    
    /// Start time
    pub start: f32,
    
    /// End time
    pub end: f32,
    
    /// Confidence score (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    
    /// Speaker ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
    
    /// Segment type
    #[serde(default = "default_segment_type")]
    pub r#type: String,
}

fn default_segment_type() -> String {
    "transcription_segment".to_string()
}

/// Usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    /// Prompt audio seconds
    pub prompt_audio_seconds: f32,
    
    /// Prompt tokens
    pub prompt_tokens: i32,
    
    /// Total tokens
    pub total_tokens: i32,
    
    /// Completion tokens
    pub completion_tokens: i32,
}

/// Audio API client
#[derive(Debug)]
pub struct AudioApi {
    client: MistralClient,
}

impl AudioApi {
    /// Create a new Audio API client
    pub fn new(client: MistralClient) -> Self {
        Self { client }
    }

    /// Create audio transcription
    pub async fn create_transcription(&self, request: &AudioTranscriptionRequest) -> Result<TranscriptionResponse, MistralError> {
        let response = self.client.post("/v1/audio/transcriptions", request).await?;
        let transcription: TranscriptionResponse = serde_json::from_str(&response)?;
        Ok(transcription)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MistralClient;

    #[test]
    fn test_audio_transcription_request_serialization() {
        let request = AudioTranscriptionRequest {
            model: "voxtral-mini-latest".to_string(),
            file: None,
            file_url: Some("https://example.com/audio.mp3".to_string()),
            file_id: None,
            language: Some("en".to_string()),
            temperature: Some(0.2),
            response_format: Some("json".to_string()),
            prompt: Some("Transcribe this audio".to_string()),
            timestamp_granularities: Some(vec!["word".to_string(), "segment".to_string()]),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("voxtral-mini-latest"));
        assert!(json.contains("https://example.com/audio.mp3"));
        assert!(json.contains("en"));

        let deserialized: AudioTranscriptionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.model, "voxtral-mini-latest");
        assert_eq!(deserialized.file_url, Some("https://example.com/audio.mp3".to_string()));
        assert_eq!(deserialized.language, Some("en".to_string()));
    }

    #[test]
    fn test_transcription_response_deserialization() {
        let json_response = r#"{
            "model": "voxtral-mini-2507",
            "text": "This is a test transcription of the audio content.",
            "language": "en",
            "segments": [
                {
                    "text": "This is a test",
                    "start": 0.0,
                    "end": 1.5,
                    "score": 0.95,
                    "speaker_id": "speaker_1",
                    "type": "transcription_segment"
                },
                {
                    "text": "transcription of the audio content.",
                    "start": 1.5,
                    "end": 3.2,
                    "score": 0.92,
                    "type": "transcription_segment"
                }
            ],
            "usage": {
                "prompt_audio_seconds": 120.5,
                "prompt_tokens": 4,
                "total_tokens": 3264,
                "completion_tokens": 635
            }
        }"#;

        let response: TranscriptionResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.model, "voxtral-mini-2507");
        assert_eq!(response.language, "en");
        assert!(response.text.contains("test transcription"));
        assert_eq!(response.segments.as_ref().unwrap().len(), 2);
        assert_eq!(response.usage.prompt_audio_seconds, 120.5);
    }

    #[test]
    fn test_audio_api_creation() {
        let client = MistralClient::new("test-key".to_string());
        let api = AudioApi::new(client);
        
        // Just verify it compiles and can be created
        assert_eq!(api.client.api_key, "test-key");
    }
}
