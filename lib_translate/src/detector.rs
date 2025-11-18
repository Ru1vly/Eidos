// lib_translate/src/detector.rs
use crate::error::{Result, TranslateError};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use std::sync::OnceLock;

static DETECTOR: OnceLock<LanguageDetector> = OnceLock::new();

/// Get or initialize the language detector
fn get_detector() -> &'static LanguageDetector {
    DETECTOR.get_or_init(|| {
        LanguageDetectorBuilder::from_all_languages()
            .with_minimum_relative_distance(0.25)
            .build()
    })
}

/// Detect the language of the given text
pub fn detect_language(text: &str) -> Result<Language> {
    let detector = get_detector();

    detector
        .detect_language_of(text)
        .ok_or_else(|| TranslateError::DetectionError("Could not detect language".to_string()))
}

/// Detect language and return ISO 639-1 code (e.g., "en", "es", "fr")
pub fn detect_language_code(text: &str) -> Result<String> {
    let language = detect_language(text)?;
    Ok(language.iso_code_639_1().to_string().to_lowercase())
}

/// Check if text is in English
pub fn is_english(text: &str) -> bool {
    detect_language(text)
        .map(|lang| lang == Language::English)
        .unwrap_or(false)
}

/// Get confidence scores for multiple languages
pub fn detect_with_confidence(text: &str) -> Vec<(Language, f64)> {
    let detector = get_detector();
    detector
        .compute_language_confidence_values(text)
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_english() {
        // Use longer text for better detection accuracy
        let text = "Hello, how are you doing today? This is a longer English text sample that should be easier to detect.";
        let result = detect_language(text).unwrap();
        assert_eq!(result, Language::English);
    }

    #[test]
    fn test_detect_spanish() {
        let text = "Hola, ¿cómo estás hoy? Este es un texto más largo en español que debería ser más fácil de detectar.";
        let result = detect_language(text).unwrap();
        assert_eq!(result, Language::Spanish);
    }

    #[test]
    fn test_detect_french() {
        let text = "Bonjour, comment allez-vous aujourd'hui? Ceci est un texte plus long en français qui devrait être plus facile à détecter.";
        let result = detect_language(text).unwrap();
        assert_eq!(result, Language::French);
    }

    #[test]
    fn test_detect_language_code() {
        let text =
            "Hello world, this is a test of the language detection system with English text.";
        let code = detect_language_code(text).unwrap();
        assert_eq!(code, "en");

        let text = "Hola mundo, esta es una prueba del sistema de detección de idioma con texto en español.";
        let code = detect_language_code(text).unwrap();
        assert_eq!(code, "es");
    }

    #[test]
    fn test_is_english() {
        assert!(is_english(
            "This is English text that is long enough to be detected properly with good accuracy."
        ));
        assert!(!is_english(
            "Ceci est du texte français qui est assez long pour être détecté correctement."
        ));
    }
}
