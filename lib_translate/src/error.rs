// lib_translate/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranslateError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Language detection failed: {0}")]
    DetectionError(String),

    #[error("Translation failed: {0}")]
    TranslationFailed(String),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("No translator configured")]
    NoTranslatorError,

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, TranslateError>;
