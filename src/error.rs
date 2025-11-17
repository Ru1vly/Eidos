// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network request error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Invalid user input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
