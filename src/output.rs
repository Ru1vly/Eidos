// Output formatting module
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "text" | "plain" => Some(Self::Text),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CommandResult {
    pub prompt: String,
    pub command: String,
    pub safety_level: String,
    pub is_safe: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<String>>,
}

impl CommandResult {
    pub fn new(prompt: impl Into<String>, command: impl Into<String>, is_safe: bool) -> Self {
        let is_safe = is_safe;
        Self {
            prompt: prompt.into(),
            command: command.into(),
            safety_level: if is_safe { "SAFE".to_string() } else { "UNSAFE".to_string() },
            is_safe,
            explanation: None,
            alternatives: None,
        }
    }

    pub fn with_explanation(mut self, explanation: impl Into<String>) -> Self {
        self.explanation = Some(explanation.into());
        self
    }

    pub fn with_alternatives(mut self, alternatives: Vec<String>) -> Self {
        self.alternatives = Some(alternatives);
        self
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_text(&self) -> String {
        let mut output = String::new();

        if self.is_safe {
            output.push_str(&format!("✅ {}\n", self.command));
        } else {
            output.push_str(&format!("❌ {} (UNSAFE)\n", self.command));
        }

        if let Some(ref explanation) = self.explanation {
            output.push_str(&format!("\nExplanation: {}\n", explanation));
        }

        if let Some(ref alternatives) = self.alternatives {
            if !alternatives.is_empty() {
                output.push_str("\nAlternatives:\n");
                for (i, alt) in alternatives.iter().enumerate() {
                    output.push_str(&format!("  {}. {}\n", i + 1, alt));
                }
            }
        }

        output
    }
}

#[derive(Debug, Serialize)]
pub struct ChatResult {
    pub user_message: String,
    pub assistant_message: String,
}

impl ChatResult {
    pub fn new(user_message: impl Into<String>, assistant_message: impl Into<String>) -> Self {
        Self {
            user_message: user_message.into(),
            assistant_message: assistant_message.into(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_text(&self) -> String {
        format!("Assistant: {}", self.assistant_message)
    }
}

#[derive(Debug, Serialize)]
pub struct TranslationResultOutput {
    pub detected_language: String,
    pub target_language: String,
    pub original_text: String,
    pub translated_text: String,
    pub was_translated: bool,
}

impl TranslationResultOutput {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_text(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Detected language: {}\n", self.detected_language));

        if self.was_translated {
            output.push_str(&format!("Original ({}): {}\n", self.detected_language, self.original_text));
            output.push_str(&format!("Translated ({}): {}\n", self.target_language, self.translated_text));
        } else {
            output.push_str(&format!("Text is already in {}\n", self.target_language));
            output.push_str(&format!("Text: {}\n", self.original_text));
        }

        output
    }
}
