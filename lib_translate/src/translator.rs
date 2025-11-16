// lib_translate/src/translator.rs
use crate::error::{Result, TranslateError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
pub enum TranslatorProvider {
    LibreTranslate { url: String, api_key: Option<String> },
    Mock, // For testing without API
}

impl TranslatorProvider {
    /// Load translator from environment variables
    pub fn from_env() -> Result<Self> {
        // Check for LibreTranslate configuration
        if let Ok(url) = env::var("LIBRETRANSLATE_URL") {
            let api_key = env::var("LIBRETRANSLATE_API_KEY").ok();
            return Ok(TranslatorProvider::LibreTranslate { url, api_key });
        }

        // Default to public LibreTranslate instance (with limitations)
        Ok(TranslatorProvider::LibreTranslate {
            url: "https://libretranslate.com".to_string(),
            api_key: None,
        })
    }
}

#[derive(Debug, Serialize)]
struct LibreTranslateRequest {
    q: String,
    source: String,
    target: String,
    format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LibreTranslateResponse {
    Success {
        #[serde(rename = "translatedText")]
        translated_text: String,
    },
    Error {
        error: String,
    },
}

pub struct Translator {
    provider: TranslatorProvider,
    client: Client,
}

impl Translator {
    pub fn new(provider: TranslatorProvider) -> Self {
        Self {
            provider,
            client: Client::new(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let provider = TranslatorProvider::from_env()?;
        Ok(Self::new(provider))
    }

    pub async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String> {
        match &self.provider {
            TranslatorProvider::LibreTranslate { url, api_key } => {
                self.translate_libretranslate(url, api_key.as_deref(), text, source_lang, target_lang)
                    .await
            }
            TranslatorProvider::Mock => {
                // Mock translator for testing - just returns original text with prefix
                Ok(format!("[Translated from {} to {}] {}", source_lang, target_lang, text))
            }
        }
    }

    async fn translate_libretranslate(
        &self,
        base_url: &str,
        api_key: Option<&str>,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String> {
        let url = format!("{}/translate", base_url);

        let request_body = LibreTranslateRequest {
            q: text.to_string(),
            source: source_lang.to_string(),
            target: target_lang.to_string(),
            format: "text".to_string(),
            api_key: api_key.map(|s| s.to_string()),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(TranslateError::ApiError(format!(
                "Translation API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_data: LibreTranslateResponse = response.json().await?;

        match response_data {
            LibreTranslateResponse::Success { translated_text } => Ok(translated_text),
            LibreTranslateResponse::Error { error } => {
                Err(TranslateError::TranslationFailed(error))
            }
        }
    }

    /// Translate to English if not already in English
    pub async fn translate_to_english(&self, text: &str, source_lang: &str) -> Result<String> {
        if source_lang == "en" {
            return Ok(text.to_string());
        }
        self.translate(text, source_lang, "en").await
    }

    /// Translate from English to target language
    pub async fn translate_from_english(&self, text: &str, target_lang: &str) -> Result<String> {
        if target_lang == "en" {
            return Ok(text.to_string());
        }
        self.translate(text, "en", target_lang).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_translator() {
        let translator = Translator::new(TranslatorProvider::Mock);
        let result = translator.translate("Hello", "en", "es").await.unwrap();
        assert!(result.contains("Hello"));
        assert!(result.contains("en"));
        assert!(result.contains("es"));
    }

    #[tokio::test]
    async fn test_translate_to_english_same_language() {
        let translator = Translator::new(TranslatorProvider::Mock);
        let result = translator.translate_to_english("Hello", "en").await.unwrap();
        assert_eq!(result, "Hello");
    }
}
