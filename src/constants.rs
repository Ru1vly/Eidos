// Global constants for Eidos CLI
// Centralizes magic numbers and configuration values for easier maintenance

/// Input validation limits
pub const MAX_CHAT_INPUT_LENGTH: usize = 10_000;
pub const MAX_CORE_PROMPT_LENGTH: usize = 1_000;
pub const MAX_TRANSLATE_INPUT_LENGTH: usize = 5_000;

/// HTTP client timeouts
pub const API_REQUEST_TIMEOUT_SECS: u64 = 30;
pub const API_CONNECT_TIMEOUT_SECS: u64 = 10;

/// Chat history configuration
pub const DEFAULT_MAX_CONVERSATION_MESSAGES: usize = 50;

/// Language detection configuration
pub const LANGUAGE_DETECTION_CONFIDENCE_THRESHOLD: f64 = 0.25;

/// Model inference configuration
pub const SEED_FOR_REPRODUCIBILITY: u64 = 299792458; // Speed of light in m/s

/// Application metadata
pub const APP_VERSION: &str = "0.2.0-beta";
pub const APP_NAME: &str = "Eidos";
pub const APP_DESCRIPTION: &str = "AI-powered CLI for Linux - Natural language to shell commands";

/// Cache configuration (for future use)
pub const DEFAULT_CACHE_SIZE: usize = 1000;
pub const DEFAULT_CACHE_TTL_HOURS: u64 = 24;

/// Performance tuning
pub const VALIDATION_PATTERNS_CAPACITY: usize = 64;
pub const HISTORY_BUFFER_CAPACITY: usize = 100;
