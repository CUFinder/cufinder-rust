use thiserror::Error;

/// CUFinder SDK error types
#[derive(Error, Debug)]
pub enum CufinderError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("API error: status {status}, message: {message}")]
    ApiError { status: u16, message: String },
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),
    
    #[error("Credit limit exceeded: {0}")]
    CreditLimitError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

/// Result type alias for CUFinder operations
pub type Result<T> = std::result::Result<T, CufinderError>;
