pub mod api;
pub mod error;
pub mod history;

use crate::api::{ApiClient, ApiProvider};
use crate::error::Result;
use crate::history::{ConversationHistory, Message};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

/// Global shared tokio runtime for synchronous chat operations
///
/// Creating a new Runtime on every request is expensive (~10-50ms overhead).
/// This static runtime is created once and reused for all chat operations.
static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Runtime::new().expect("Failed to create tokio runtime")
});

pub struct Chat {
    client: Option<ApiClient>,
    history: ConversationHistory,
}

impl Chat {
    /// Create a new Chat instance with API client from environment
    pub fn new() -> Self {
        let client = ApiClient::from_env().ok();
        if client.is_none() {
            eprintln!("Warning: No API provider configured. Set OPENAI_API_KEY, OLLAMA_HOST, or LLM_API_URL");
        }
        Self {
            client,
            history: ConversationHistory::default(),
        }
    }

    /// Create a Chat instance with a specific provider
    pub fn with_provider(provider: ApiProvider) -> Self {
        Self {
            client: Some(ApiClient::new(provider)),
            history: ConversationHistory::default(),
        }
    }

    /// Send a message and get a response (async)
    pub async fn send_async(&mut self, message: &str) -> Result<String> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| error::ChatError::NoProviderError)?;

        // Add user message to history
        self.history.add_user_message(message);

        // Send to API with full conversation history
        let response = client
            .send_message(self.history.messages(), Some(0.7), Some(1000))
            .await?;

        // Add assistant response to history
        self.history.add_assistant_message(&response);

        Ok(response)
    }

    /// Synchronous wrapper that blocks on async send
    /// This is the method called from main.rs
    ///
    /// Uses a shared global runtime to avoid the overhead of creating
    /// a new runtime on every chat request (~10-50ms saved per call).
    pub fn run(&mut self, text: &str) -> Result<String> {
        let response = RUNTIME.block_on(self.send_async(text))?;
        Ok(response)
    }

    /// Add a system message to guide the conversation
    pub fn set_system_prompt(&mut self, prompt: &str) {
        self.history.add_system_message(prompt);
    }

    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get conversation history
    pub fn history(&self) -> &[Message] {
        self.history.messages()
    }

    /// Check if API client is configured
    pub fn is_configured(&self) -> bool {
        self.client.is_some()
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export commonly used types for convenience
pub use error::ChatError;
