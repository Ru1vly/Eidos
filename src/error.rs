// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Network request error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Invalid user input: {0}")]
    InvalidInputError(String),

    // Future error types - planned for Phase 9.2 (Unified Error Handling)
    #[allow(dead_code)]
    #[error("Language detection failed")]
    LanguageDetectionError,

    #[allow(dead_code)]
    #[error("Translation failed: {0}")]
    TranslationError(String),

    #[allow(dead_code)]
    #[error("AI model interaction error: {0}")]
    AIModelError(String),

    #[allow(dead_code)]
    #[error("API key not found or invalid")]
    ApiKeyError,
}

pub type Result<T> = std::result::Result<T, AppError>;
