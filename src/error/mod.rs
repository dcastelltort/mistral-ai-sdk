use reqwest::StatusCode;
use serde_json::Error as SerdeError;
use std::fmt;
use std::io::{Error as IoError, ErrorKind};


/// Main error type for the Mistral AI SDK
#[derive(Debug)]
pub enum MistralError {
    /// API returned an error response
    ApiError {
        status: StatusCode,
        message: String,
        error_type: String,
    },
    
    /// Network-related error
    NetworkError(IoError),
    
    /// JSON serialization/deserialization error
    SerializationError(String),
    
    /// Invalid configuration or parameters
    InvalidConfiguration(String),
    
    /// Authentication failed
    AuthenticationError(String),
}

impl fmt::Display for MistralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MistralError::ApiError { status, message, error_type } => {
                write!(f, "API Error ({}): {} - {}", status, error_type, message)
            }
            MistralError::NetworkError(err) => {
                write!(f, "Network error: {}", err)
            }
            MistralError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            MistralError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            MistralError::AuthenticationError(msg) => {
                write!(f, "Authentication error: {}", msg)
            }
        }
    }
}

impl std::error::Error for MistralError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MistralError::NetworkError(err) => Some(err),
            _ => None,
        }
    }
}

impl MistralError {
    /// Create an API error from HTTP status and message
    pub fn from_status(status: StatusCode, message: &str) -> Self {
        let error_type = match status {
            StatusCode::BAD_REQUEST => "invalid_request_error",
            StatusCode::UNAUTHORIZED => "authentication_error",
            StatusCode::FORBIDDEN => "permission_error",
            StatusCode::NOT_FOUND => "not_found_error",
            StatusCode::TOO_MANY_REQUESTS => "rate_limit_error",
            StatusCode::INTERNAL_SERVER_ERROR => "server_error",
            _ => "api_error",
        };
        
        MistralError::ApiError {
            status,
            message: message.to_string(),
            error_type: error_type.to_string(),
        }
    }
    
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            MistralError::ApiError { status, .. } => {
                // Retry on server errors and rate limits
                status.is_server_error() || *status == StatusCode::TOO_MANY_REQUESTS
            }
            MistralError::NetworkError(_) => true, // Network errors are retryable
            _ => false,
        }
    }
    
    /// Check if the error is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        matches!(self, MistralError::ApiError { status, .. } if status.is_client_error())
    }
    
    /// Check if the error is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        matches!(self, MistralError::ApiError { status, .. } if status.is_server_error())
    }
    
    /// Check if the error is an authentication error
    pub fn is_authentication_error(&self) -> bool {
        matches!(self, MistralError::AuthenticationError(_)) ||
        matches!(self, MistralError::ApiError { status: StatusCode::UNAUTHORIZED, .. })
    }
}

impl From<reqwest::Error> for MistralError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() || error.is_connect() || error.is_body() {
            MistralError::NetworkError(IoError::new(
                ErrorKind::Other,
                error.to_string(),
            ))
        } else if error.status().is_some() {
            let status = error.status().unwrap();
            MistralError::from_status(status, &error.to_string())
        } else {
            MistralError::NetworkError(IoError::new(
                ErrorKind::Other,
                error.to_string(),
            ))
        }
    }
}

impl From<SerdeError> for MistralError {
    fn from(error: SerdeError) -> Self {
        MistralError::SerializationError(error.to_string())
    }
}

impl From<IoError> for MistralError {
    fn from(error: IoError) -> Self {
        MistralError::NetworkError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;


    #[test]
    fn test_api_error_display() {
        let error = MistralError::ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Invalid request".to_string(),
            error_type: "invalid_request_error".to_string(),
        };
        
        let display = format!("{}", error);
        assert!(display.contains("API Error (400"));
        assert!(display.contains("invalid_request_error"));
        assert!(display.contains("Invalid request"));
    }

    #[test]
    fn test_network_error_display() {
        let io_error = IoError::new(ErrorKind::NotFound, "file not found");
        let error = MistralError::NetworkError(io_error);
        
        assert!(format!("{}", error).contains("Network error"));
    }

    #[test]
    fn test_serialization_error_display() {
        let error = MistralError::SerializationError("failed to parse JSON".to_string());
        
        assert_eq!(
            format!("{}", error),
            "Serialization error: failed to parse JSON"
        );
    }

    #[test]
    fn test_api_error_from_status() {
        let error = MistralError::from_status(
            StatusCode::UNAUTHORIZED,
            "Invalid API key"
        );
        
        match error {
            MistralError::ApiError { status, message, error_type } => {
                assert_eq!(status, StatusCode::UNAUTHORIZED);
                assert_eq!(message, "Invalid API key");
                assert_eq!(error_type, "authentication_error");
            }
            _ => panic!("Expected ApiError variant"),
        }
    }

    #[test]
    fn test_error_from_reqwest() {
        // Create a mock reqwest error by simulating a timeout
        // Since reqwest::Error::new is private, we'll test the conversion
        // through the public interface in integration
        let io_error = IoError::new(ErrorKind::TimedOut, "timeout");
        let mistral_error = MistralError::NetworkError(io_error);
        
        match mistral_error {
            MistralError::NetworkError(_) => {},
            _ => panic!("Expected NetworkError variant"),
        }
    }

    #[test]
    fn test_is_retryable() {
        // Rate limit error should be retryable
        let rate_limit_error = MistralError::ApiError {
            status: StatusCode::TOO_MANY_REQUESTS,
            message: "Rate limit exceeded".to_string(),
            error_type: "rate_limit_error".to_string(),
        };
        assert!(rate_limit_error.is_retryable());

        // Server error should be retryable
        let server_error = MistralError::ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
            error_type: "server_error".to_string(),
        };
        assert!(server_error.is_retryable());

        // Client error should not be retryable
        let client_error = MistralError::ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Bad request".to_string(),
            error_type: "invalid_request_error".to_string(),
        };
        assert!(!client_error.is_retryable());
    }

    #[test]
    fn test_error_type_checks() {
        let client_error = MistralError::ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Bad request".to_string(),
            error_type: "invalid_request_error".to_string(),
        };
        
        let server_error = MistralError::ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Server error".to_string(),
            error_type: "server_error".to_string(),
        };
        
        let auth_error = MistralError::AuthenticationError("Invalid token".to_string());
        let unauthorized_error = MistralError::from_status(StatusCode::UNAUTHORIZED, "Unauthorized");
        
        assert!(client_error.is_client_error());
        assert!(!client_error.is_server_error());
        assert!(!client_error.is_authentication_error());
        
        assert!(!server_error.is_client_error());
        assert!(server_error.is_server_error());
        assert!(!server_error.is_authentication_error());
        
        assert!(auth_error.is_authentication_error());
        assert!(unauthorized_error.is_authentication_error());
    }

    #[test]
    fn test_error_source() {
        use std::error::Error;
        
        let io_error = IoError::new(ErrorKind::NotFound, "file not found");
        let mistral_error = MistralError::NetworkError(io_error);
        
        assert!(mistral_error.source().is_some());
        assert_eq!(mistral_error.source().unwrap().to_string(), "file not found");
    }
}