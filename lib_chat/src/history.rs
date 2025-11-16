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
}

impl ConversationHistory {
    pub fn new(max_messages: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_messages,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);

        // Keep only the most recent messages
        if self.messages.len() > self.max_messages {
            let start = self.messages.len() - self.max_messages;
            self.messages.drain(0..start);
        }
    }

    pub fn add_user_message(&mut self, content: impl Into<String>) {
        self.add_message(Message::user(content));
    }

    pub fn add_assistant_message(&mut self, content: impl Into<String>) {
        self.add_message(Message::assistant(content));
    }

    pub fn add_system_message(&mut self, content: impl Into<String>) {
        self.add_message(Message::system(content));
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

        history.add_user_message("Message 1");
        history.add_assistant_message("Response 1");
        history.add_user_message("Message 2");

        assert_eq!(history.len(), 3);

        // Adding more messages should drop oldest
        history.add_assistant_message("Response 2");
        assert_eq!(history.len(), 3);
        assert_eq!(history.messages()[0].content, "Response 1");
    }

    #[test]
    fn test_clear_history() {
        let mut history = ConversationHistory::new(10);
        history.add_user_message("Test");
        assert!(!history.is_empty());

        history.clear();
        assert!(history.is_empty());
    }
}
