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

    #[error("Language detection failed")]
    LanguageDetectionError,

    #[error("Translation failed: {0}")]
    TranslationError(String),

    #[error("AI model interaction error: {0}")]
    AIModelError(String),

    #[error("Command execution error: {0}")]
    CommandExecutionError(String),

    #[error("Invalid user input: {0}")]
    InvalidInputError(String),

    #[error("API key not found or invalid")]
    ApiKeyError,

    #[error("An unknown error occurred")]
    UnknownError,
}

pub type Result<T> = std::result::Result<T, AppError>;
