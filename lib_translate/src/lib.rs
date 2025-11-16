pub mod detector;
pub mod error;
pub mod translator;

use crate::detector::{detect_language_code, is_english};
use crate::error::Result;
use crate::translator::{Translator, TranslatorProvider};

pub struct Translate {
    translator: Option<Translator>,
}

impl Translate {
    /// Create a new Translate instance with translator from environment
    pub fn new() -> Self {
        let translator = Translator::from_env().ok();
        if translator.is_none() {
            eprintln!("Warning: Using mock translator. Set LIBRETRANSLATE_URL for real translation");
            // Use mock translator as fallback
            return Self {
                translator: Some(Translator::new(TranslatorProvider::Mock)),
            };
        }
        Self { translator }
    }

    /// Create a Translate instance with a specific provider
    pub fn with_provider(provider: TranslatorProvider) -> Self {
        Self {
            translator: Some(Translator::new(provider)),
        }
    }

    /// Detect language and translate if needed
    pub async fn detect_and_translate_async(&self, text: &str, target_lang: &str) -> Result<TranslationResult> {
        // Detect source language
        let source_lang = detect_language_code(text)?;

        // If already in target language, no translation needed
        if source_lang == target_lang {
            return Ok(TranslationResult {
                original: text.to_string(),
                translated: text.to_string(),
                source_lang: source_lang.clone(),
                target_lang: target_lang.to_string(),
                was_translated: false,
            });
        }

        // Translate
        let translator = self
            .translator
            .as_ref()
            .ok_or_else(|| error::TranslateError::NoTranslatorError)?;

        let translated = translator.translate(text, &source_lang, target_lang).await?;

        Ok(TranslationResult {
            original: text.to_string(),
            translated,
            source_lang,
            target_lang: target_lang.to_string(),
            was_translated: true,
        })
    }

    /// Synchronous wrapper for the main run method
    pub fn run(&self, text: &str) {
        // Detect language
        match detect_language_code(text) {
            Ok(lang_code) => {
                println!("Detected language: {}", lang_code);

                if is_english(text) {
                    println!("Text is already in English");
                    println!("Original: {}", text);
                } else {
                    println!("Translating to English...");

                    // Create runtime for async translation
                    let runtime = tokio::runtime::Runtime::new().unwrap();

                    match runtime.block_on(self.detect_and_translate_async(text, "en")) {
                        Ok(result) => {
                            if result.was_translated {
                                println!("Original ({}): {}", result.source_lang, result.original);
                                println!("Translated ({}): {}", result.target_lang, result.translated);
                            } else {
                                println!("No translation needed");
                            }
                        }
                        Err(e) => {
                            eprintln!("Translation Error: {}", e);
                            eprintln!("Tip: Set LIBRETRANSLATE_URL for translation API");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Language detection failed: {}", e);
            }
        }
    }

    /// Detect if text is in English
    pub fn is_english(text: &str) -> bool {
        is_english(text)
    }

    /// Detect language code
    pub fn detect_language(text: &str) -> Result<String> {
        detect_language_code(text)
    }
}

impl Default for Translate {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a translation operation
#[derive(Debug, Clone)]
pub struct TranslationResult {
    pub original: String,
    pub translated: String,
    pub source_lang: String,
    pub target_lang: String,
    pub was_translated: bool,
}

// Re-export commonly used types
pub use error::TranslateError;
