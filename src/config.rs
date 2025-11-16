// src/config.rs
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to the ONNX model file
    pub model_path: PathBuf,
    /// Path to the tokenizer JSON file
    pub tokenizer_path: PathBuf,
}

impl Config {
    /// Load configuration from file, environment variables, or use defaults
    pub fn load() -> Result<Self, String> {
        // Priority 1: Try to load from config file
        if let Ok(config) = Self::from_file("eidos.toml") {
            return Ok(config);
        }

        // Priority 2: Try to load from environment variables
        if let Ok(config) = Self::from_env() {
            return Ok(config);
        }

        // Priority 3: Use defaults (will fail if files don't exist)
        Ok(Self::default())
    }

    /// Load config from a TOML file
    pub fn from_file(path: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file '{}': {}", path, e))?;

        toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file '{}': {}", path, e))
    }

    /// Load config from environment variables
    pub fn from_env() -> Result<Self, String> {
        let model_path = env::var("EIDOS_MODEL_PATH").map_err(|_| "EIDOS_MODEL_PATH not set")?;
        let tokenizer_path =
            env::var("EIDOS_TOKENIZER_PATH").map_err(|_| "EIDOS_TOKENIZER_PATH not set")?;

        Ok(Self {
            model_path: PathBuf::from(model_path),
            tokenizer_path: PathBuf::from(tokenizer_path),
        })
    }

    /// Validate that the configured paths exist
    pub fn validate(&self) -> Result<(), String> {
        if !self.model_path.exists() {
            return Err(format!(
                "Model file not found: {}",
                self.model_path.display()
            ));
        }

        if !self.tokenizer_path.exists() {
            return Err(format!(
                "Tokenizer file not found: {}",
                self.tokenizer_path.display()
            ));
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("model.onnx"),
            tokenizer_path: PathBuf::from("tokenizer.json"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.model_path, PathBuf::from("model.onnx"));
        assert_eq!(config.tokenizer_path, PathBuf::from("tokenizer.json"));
    }

    #[test]
    fn test_config_from_env() {
        env::set_var("EIDOS_MODEL_PATH", "/tmp/test_model.onnx");
        env::set_var("EIDOS_TOKENIZER_PATH", "/tmp/test_tokenizer.json");

        let config = Config::from_env().unwrap();
        assert_eq!(config.model_path, PathBuf::from("/tmp/test_model.onnx"));
        assert_eq!(
            config.tokenizer_path,
            PathBuf::from("/tmp/test_tokenizer.json")
        );

        env::remove_var("EIDOS_MODEL_PATH");
        env::remove_var("EIDOS_TOKENIZER_PATH");
    }
}
