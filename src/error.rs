// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Giriş/Çıkış hatası: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Ağ isteği hatası: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON ayrıştırma hatası: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Dil algılanamadı")]
    LanguageDetectionError,

    #[error("Çeviri başarısız: {0}")]
    TranslationError(String),

    #[error("AI model etkileşim hatası: {0}")]
    AIModelError(String),

    #[error("Komut çalıştırma hatası: {0}")]
    CommandExecutionError(String),

    #[error("Geçersiz kullanıcı girdisi: {0}")]
    InvalidInputError(String),

    #[error("API anahtarı bulunamadı veya geçersiz")]
    ApiKeyError,

    #[error("Bilinmeyen bir hata oluştu")]
    UnknownError,
}

pub type Result<T> = std::result::Result<T, AppError>;
