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
    ///
    /// Priority order (highest to lowest):
    /// 1. Environment variables (EIDOS_MODEL_PATH, EIDOS_TOKENIZER_PATH)
    /// 2. Local config file (./eidos.toml)
    /// 3. User config file (~/.config/eidos/eidos.toml)
    /// 4. Built-in defaults
    pub fn load() -> Result<Self, String> {
        // Priority 1: Environment variables (highest priority)
        if let Ok(config) = Self::from_env() {
            return Ok(config);
        }

        // Priority 2: Local config file
        if let Ok(config) = Self::from_file("eidos.toml") {
            return Ok(config);
        }

        // Priority 3: User config file
        if let Some(user_config_path) = Self::get_user_config_path() {
            if let Ok(config) = Self::from_file(&user_config_path.to_string_lossy()) {
                return Ok(config);
            }
        }

        // Priority 4: Use defaults (will fail validation if files don't exist)
        Ok(Self::default())
    }

    /// Get the path to the user config file (~/.config/eidos/eidos.toml)
    fn get_user_config_path() -> Option<PathBuf> {
        let home = env::var("HOME").ok()?;
        Some(PathBuf::from(home).join(".config/eidos/eidos.toml"))
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

    /// Validate that the configured paths exist and are safe to use
    pub fn validate(&self) -> Result<(), String> {
        // Validate model path
        Self::validate_file_path(&self.model_path, "Model", 2 * 1024 * 1024 * 1024)?; // 2GB max

        // Validate tokenizer path
        Self::validate_file_path(&self.tokenizer_path, "Tokenizer", 100 * 1024 * 1024)?; // 100MB max

        Ok(())
    }

    /// Validate a file path for security and safety
    fn validate_file_path(path: &PathBuf, file_type: &str, max_size: u64) -> Result<(), String> {
        // Check if file exists
        if !path.exists() {
            return Err(format!("{} file not found: {}", file_type, path.display()));
        }

        // Canonicalize path to resolve symlinks and check for path traversal
        let canonical_path = path.canonicalize().map_err(|e| {
            format!(
                "Failed to resolve {} path {}: {}",
                file_type,
                path.display(),
                e
            )
        })?;

        // Check if path contains suspicious patterns (after canonicalization)
        let path_str = canonical_path.to_string_lossy();
        if path_str.contains("..") {
            return Err(format!(
                "{} path contains suspicious patterns: {}",
                file_type,
                path.display()
            ));
        }

        // Get file metadata
        let metadata = fs::metadata(&canonical_path).map_err(|e| {
            format!(
                "Failed to read {} file metadata: {}",
                file_type, e
            )
        })?;

        // Check if it's a regular file (not directory or other special file)
        if !metadata.is_file() {
            return Err(format!(
                "{} path is not a regular file: {}",
                file_type,
                path.display()
            ));
        }

        // Check file size is reasonable
        let file_size = metadata.len();
        if file_size > max_size {
            return Err(format!(
                "{} file too large: {} bytes (max {} bytes)",
                file_type, file_size, max_size
            ));
        }

        if file_size == 0 {
            return Err(format!("{} file is empty: {}", file_type, path.display()));
        }

        // Check file permissions (must be readable)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = metadata.permissions();
            let mode = permissions.mode();

            // Check if file is readable by user (owner)
            if mode & 0o400 == 0 {
                return Err(format!(
                    "{} file is not readable: {}",
                    file_type,
                    path.display()
                ));
            }

            // Warn if file is world-readable with write permissions
            if mode & 0o002 != 0 {
                eprintln!(
                    "⚠️  Warning: {} file is world-writable: {}",
                    file_type,
                    path.display()
                );
            }
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
