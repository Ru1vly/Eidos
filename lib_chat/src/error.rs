// lib_chat/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid API key or configuration")]
    AuthenticationError,

    #[error("Rate limit exceeded")]
    RateLimitError,

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("No API provider configured")]
    NoProviderError,

    #[error("Environment variable not set: {0}")]
    EnvError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, ChatError>;
