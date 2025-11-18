// lib_chat/src/history.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }
}

#[derive(Debug, Clone)]
pub struct ConversationHistory {
    messages: Vec<Message>,
    max_messages: usize,
    max_bytes_total: usize,      // Max total memory for all messages
    max_bytes_per_message: usize, // Max size for a single message
}

impl ConversationHistory {
    pub fn new(max_messages: usize) -> Self {
        Self::new_with_limits(
            max_messages,
            10 * 1024 * 1024,  // 10MB total by default
            1 * 1024 * 1024,   // 1MB per message by default
        )
    }

    pub fn new_with_limits(
        max_messages: usize,
        max_bytes_total: usize,
        max_bytes_per_message: usize,
    ) -> Self {
        Self {
            messages: Vec::new(),
            max_messages,
            max_bytes_total,
            max_bytes_per_message,
        }
    }

    /// Calculate total byte size of all messages
    fn total_bytes(&self) -> usize {
        self.messages
            .iter()
            .map(|m| m.content.len())
            .sum()
    }

    pub fn add_message(&mut self, message: Message) -> Result<(), String> {
        // Check individual message size
        let message_bytes = message.content.len();
        if message_bytes > self.max_bytes_per_message {
            return Err(format!(
                "Message too large: {} bytes (max {} bytes)",
                message_bytes, self.max_bytes_per_message
            ));
        }

        self.messages.push(message);

        // Keep only the most recent messages by count
        if self.messages.len() > self.max_messages {
            let start = self.messages.len() - self.max_messages;
            self.messages.drain(0..start);
        }

        // Keep only the most recent messages by total size
        while self.total_bytes() > self.max_bytes_total && self.messages.len() > 1 {
            // Remove oldest message
            self.messages.remove(0);
        }

        Ok(())
    }

    pub fn add_user_message(&mut self, content: impl Into<String>) -> Result<(), String> {
        self.add_message(Message::user(content))
    }

    pub fn add_assistant_message(&mut self, content: impl Into<String>) -> Result<(), String> {
        self.add_message(Message::assistant(content))
    }

    pub fn add_system_message(&mut self, content: impl Into<String>) -> Result<(), String> {
        self.add_message(Message::system(content))
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

impl Default for ConversationHistory {
    fn default() -> Self {
        Self::new(50) // Default to keeping last 50 messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::user("Hello");
        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content, "Hello");

        let msg = Message::assistant("Hi there");
        assert_eq!(msg.role, Role::Assistant);
        assert_eq!(msg.content, "Hi there");
    }

    #[test]
    fn test_conversation_history() {
        let mut history = ConversationHistory::new(3);

        history.add_user_message("Message 1").unwrap();
        history.add_assistant_message("Response 1").unwrap();
        history.add_user_message("Message 2").unwrap();

        assert_eq!(history.len(), 3);

        // Adding more messages should drop oldest
        history.add_assistant_message("Response 2").unwrap();
        assert_eq!(history.len(), 3);
        assert_eq!(history.messages()[0].content, "Response 1");
    }

    #[test]
    fn test_clear_history() {
        let mut history = ConversationHistory::new(10);
        history.add_user_message("Test").unwrap();
        assert!(!history.is_empty());

        history.clear();
        assert!(history.is_empty());
    }

    #[test]
    fn test_message_size_limit() {
        let mut history = ConversationHistory::new_with_limits(10, 1000, 100);

        // Message within limit should succeed
        assert!(history.add_user_message("x".repeat(50)).is_ok());

        // Message exceeding limit should fail
        let result = history.add_user_message("x".repeat(150));
        assert!(result.is_err());
    }

    #[test]
    fn test_total_size_limit() {
        let mut history = ConversationHistory::new_with_limits(10, 200, 100);

        // Add messages that together exceed total limit
        history.add_user_message("x".repeat(80)).unwrap();
        history.add_user_message("x".repeat(80)).unwrap();
        history.add_user_message("x".repeat(80)).unwrap();

        // Should have dropped old messages to stay under total limit
        assert!(history.total_bytes() <= 200);
        assert!(history.len() < 3);
    }
}
