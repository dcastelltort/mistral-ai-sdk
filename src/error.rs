use thiserror::Error;

#[derive(Error, Debug)]
pub enum MistralError {
    #[error("API Error: {0}")]
    ApiError(String),
    #[error("Network Error: {0}")]
    NetworkError(String),
}