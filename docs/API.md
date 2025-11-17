# Eidos API Documentation

Complete API reference for using Eidos programmatically and extending its functionality.

## Table of Contents

1. [CLI API](#cli-api)
2. [Rust Library API](#rust-library-api)
3. [Configuration API](#configuration-api)
4. [Examples](#examples)

## CLI API

### eidos

Main command-line interface.

```bash
eidos [OPTIONS] <COMMAND>
```

**Options:**
- `-h, --help` - Print help information
- `-V, --version` - Print version information

**Commands:**
- `chat` - Chat with AI model
- `core` - Generate shell commands from natural language
- `translate` - Translate text between languages
- `help` - Print command help

---

### eidos chat

Start an AI chat session.

```bash
eidos chat <TEXT>
```

**Arguments:**
- `TEXT` - Input message for the chat

**Environment Variables:**
- `OPENAI_API_KEY` - OpenAI API key
- `OLLAMA_HOST` - Ollama server URL (default: http://localhost:11434)
- `LLM_API_URL` - Custom OpenAI-compatible API URL
- `LLM_API_KEY` - API key for custom endpoint

**Examples:**

```bash
# Chat with OpenAI
export OPENAI_API_KEY=sk-...
eidos chat "Hello, how are you?"

# Chat with local Ollama
export OLLAMA_HOST=http://localhost:11434
eidos chat "Explain quantum computing"

# Chat with custom API
export LLM_API_URL=https://api.example.com/v1
export LLM_API_KEY=your-key
eidos chat "What is Rust?"
```

**Output:**
```
Assistant: I'm doing well, thank you! How can I help you today?
```

**Error Handling:**
```
Chat Error: No API provider configured
Tip: Configure an API provider:
  - OpenAI: export OPENAI_API_KEY=your-key
  - Ollama: export OLLAMA_HOST=http://localhost:11434
  - Custom: export LLM_API_URL=... LLM_API_KEY=...
```

---

### eidos core

Generate shell commands from natural language.

```bash
eidos core <PROMPT>
```

**Arguments:**
- `PROMPT` - Natural language description of desired command

**Environment Variables:**
- `EIDOS_MODEL_PATH` - Path to ONNX/GGUF model
- `EIDOS_TOKENIZER_PATH` - Path to tokenizer.json

**Examples:**

```bash
# List files
eidos core "list all files"
# Output: ls -la

# Find Python files
eidos core "find all Python scripts"
# Output: find . -name '*.py'

# Show disk usage
eidos core "show disk space"
# Output: df -h
```

**Configuration:**

Create `~/.config/eidos/eidos.toml`:
```toml
model_path = "/path/to/model.onnx"
tokenizer_path = "/path/to/tokenizer.json"
```

Or set environment variables:
```bash
export EIDOS_MODEL_PATH=/path/to/model.onnx
export EIDOS_TOKENIZER_PATH=/path/to/tokenizer.json
```

**Error Handling:**
```
Configuration validation failed: Model file not found: model.onnx
Tip: Set EIDOS_MODEL_PATH and EIDOS_TOKENIZER_PATH environment variables
  or create an eidos.toml config file
```

---

### eidos translate

Translate text between languages.

```bash
eidos translate <TEXT>
```

**Arguments:**
- `TEXT` - Text to translate

**Features:**
- Auto-detects source language (75+ languages)
- Translates to English by default
- Uses LibreTranslate API or offline detection only

**Examples:**

```bash
# French to English
eidos translate "Bonjour le monde"
# Output:
# Detected language: fr
# Translated (en): Hello world

# Spanish to English
eidos translate "Hola, ¿cómo estás?"
# Output:
# Detected language: es
# Translated (en): Hello, how are you?

# Already English
eidos translate "This is English text"
# Output:
# Detected language: en
# Text is already in English
```

## Rust Library API

### lib_core

Command generation from AI models.

#### Core

```rust
pub struct Core {
    // Private fields
}

impl Core {
    /// Create new Core instance with ONNX model
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self>;

    /// Generate shell command from natural language prompt
    pub fn run(&self, prompt: &str) -> Result<String>;
}
```

**Example:**

```rust
use lib_core::Core;

fn main() -> anyhow::Result<()> {
    // Initialize model
    let core = Core::new("model.onnx", "tokenizer.json")?;

    // Generate command
    let command = core.run("list all files")?;
    println!("Generated: {}", command);

    Ok(())
}
```

#### QuantizedLlm

```rust
pub struct QuantizedLlm {
    // Private fields
}

impl QuantizedLlm {
    /// Create new QuantizedLlm with GGUF model
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self>;

    /// Generate text from prompt
    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String>;
}
```

**Example:**

```rust
use lib_core::QuantizedLlm;

fn main() -> anyhow::Result<()> {
    let mut llm = QuantizedLlm::new("model.gguf", "tokenizer.json")?;

    let response = llm.generate("list files", 50)?;
    println!("{}", response);

    Ok(())
}
```

---

### lib_chat

Chat and LLM API integration.

#### Chat

```rust
pub struct Chat {
    // Private fields
}

impl Chat {
    /// Create new Chat instance
    pub fn new() -> Self;

    /// Send message synchronously (blocks)
    pub fn run(&mut self, text: &str);

    /// Send message asynchronously
    pub async fn send_async(&mut self, text: &str) -> Result<String>;
}
```

**Example (Sync):**

```rust
use lib_chat::Chat;

fn main() {
    let mut chat = Chat::new();
    chat.run("Hello!");
    // Prints: Assistant: Hello! How can I help you?
}
```

**Example (Async):**

```rust
use lib_chat::Chat;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut chat = Chat::new();

    let response = chat.send_async("What is Rust?").await?;
    println!("Response: {}", response);

    Ok(())
}
```

#### ApiProvider

```rust
pub enum ApiProvider {
    OpenAI {
        api_key: String,
        model: String,
    },
    Ollama {
        base_url: String,
        model: String,
    },
    Custom {
        base_url: String,
        api_key: Option<String>,
        model: String,
    },
}

impl ApiProvider {
    /// Detect provider from environment variables
    pub fn from_env() -> Result<Self>;
}
```

**Example:**

```rust
use lib_chat::ApiProvider;
use std::env;

fn main() -> anyhow::Result<()> {
    // Set environment variable
    env::set_var("OPENAI_API_KEY", "sk-...");

    // Auto-detect provider
    let provider = ApiProvider::from_env()?;

    // Or create manually
    let provider = ApiProvider::OpenAI {
        api_key: "sk-...".to_string(),
        model: "gpt-3.5-turbo".to_string(),
    };

    Ok(())
}
```

#### ConversationHistory

```rust
pub struct ConversationHistory {
    // Private fields
}

pub enum Role {
    System,
    User,
    Assistant,
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

impl ConversationHistory {
    /// Create new conversation history
    pub fn new(max_messages: usize) -> Self;

    /// Add message to history
    pub fn add_message(&mut self, role: Role, content: String);

    /// Get all messages
    pub fn messages(&self) -> &[Message];

    /// Clear history
    pub fn clear(&mut self);
}
```

**Example:**

```rust
use lib_chat::{ConversationHistory, Role};

fn main() {
    let mut history = ConversationHistory::new(100);

    history.add_message(Role::User, "Hello".to_string());
    history.add_message(Role::Assistant, "Hi there!".to_string());

    println!("Messages: {}", history.messages().len());
}
```

---

### lib_translate

Language detection and translation.

#### Translate

```rust
pub struct Translate {
    // Private fields
}

impl Translate {
    /// Create new Translate instance
    pub fn new() -> Self;

    /// Detect language and translate to English (sync)
    pub fn run(&self, text: &str);

    /// Detect language and translate asynchronously
    pub async fn detect_and_translate_async(
        &self,
        text: &str,
        target_lang: &str
    ) -> Result<TranslationResult>;
}
```

**Example:**

```rust
use lib_translate::Translate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let translate = Translate::new();

    let result = translate
        .detect_and_translate_async("Bonjour", "en")
        .await?;

    println!("Detected: {}", result.detected_lang);
    println!("Translated: {}", result.translated);

    Ok(())
}
```

#### Language Detection

```rust
/// Detect language from text
pub fn detect_language(text: &str) -> Result<Language>;

/// Detect language and return ISO code
pub fn detect_language_code(text: &str) -> Result<String>;

/// Check if text is English
pub fn is_english(text: &str) -> bool;
```

**Example:**

```rust
use lib_translate::detector::{detect_language_code, is_english};

fn main() -> anyhow::Result<()> {
    let code = detect_language_code("Hola mundo")?;
    println!("Language: {}", code); // "es"

    let is_en = is_english("Hello world");
    println!("Is English: {}", is_en); // true

    Ok(())
}
```

#### Translation API

```rust
pub enum TranslatorProvider {
    LibreTranslate {
        url: String,
        api_key: Option<String>,
    },
    Mock,
}

pub struct Translator {
    // Private fields
}

impl Translator {
    /// Create new translator
    pub fn new(provider: TranslatorProvider) -> Self;

    /// Translate text asynchronously
    pub async fn translate_async(
        &self,
        text: &str,
        source: &str,
        target: &str
    ) -> Result<String>;
}
```

**Example:**

```rust
use lib_translate::translator::{Translator, TranslatorProvider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = TranslatorProvider::LibreTranslate {
        url: "https://libretranslate.com".to_string(),
        api_key: None,
    };

    let translator = Translator::new(provider);

    let result = translator
        .translate_async("Hello", "en", "es")
        .await?;

    println!("Translated: {}", result); // "Hola"

    Ok(())
}
```

---

### lib_bridge

Request routing system.

#### Bridge

```rust
pub struct Bridge {
    // Private fields
}

pub enum Request {
    Chat,
    Core,
    Translate,
}

pub type Handler = Box<dyn Fn(&str) -> Result<(), String>>;

impl Bridge {
    /// Create new bridge
    pub fn new() -> Self;

    /// Register request handler
    pub fn register(&mut self, request: Request, handler: Handler);

    /// Route request to handler
    pub fn route(&self, request: Request, input: &str) -> Result<(), String>;
}
```

**Example:**

```rust
use lib_bridge::{Bridge, Request};

fn main() -> Result<(), String> {
    let mut bridge = Bridge::new();

    // Register handlers
    bridge.register(
        Request::Chat,
        Box::new(|text: &str| {
            println!("Chat: {}", text);
            Ok(())
        }),
    );

    bridge.register(
        Request::Core,
        Box::new(|prompt: &str| {
            println!("Core: {}", prompt);
            Ok(())
        }),
    );

    // Route requests
    bridge.route(Request::Chat, "Hello")?;
    bridge.route(Request::Core, "list files")?;

    Ok(())
}
```

## Configuration API

### Config

```rust
pub struct Config {
    pub model_path: PathBuf,
    pub tokenizer_path: PathBuf,
}

impl Config {
    /// Load configuration with priority:
    /// 1. Environment variables
    /// 2. ./eidos.toml
    /// 3. ~/.config/eidos/eidos.toml
    /// 4. Defaults
    pub fn load() -> Result<Self, String>;

    /// Validate configuration (check file existence)
    pub fn validate(&self) -> Result<(), String>;

    /// Load from specific file
    pub fn from_file(path: &str) -> Result<Self, String>;

    /// Load from environment variables
    pub fn from_env() -> Result<Self, String>;

    /// Get default configuration
    pub fn default() -> Self;
}
```

**Example:**

```rust
use eidos::config::Config;

fn main() -> Result<(), String> {
    // Auto-load from all sources
    let config = Config::load()?;

    // Validate paths exist
    config.validate()?;

    println!("Model: {:?}", config.model_path);
    println!("Tokenizer: {:?}", config.tokenizer_path);

    Ok(())
}
```

**TOML Format:**

```toml
model_path = "/path/to/model.onnx"
tokenizer_path = "/path/to/tokenizer.json"
```

**Environment Variables:**

```bash
export EIDOS_MODEL_PATH=/path/to/model.onnx
export EIDOS_TOKENIZER_PATH=/path/to/tokenizer.json
```

## Examples

### Complete Chat Bot

```rust
use lib_chat::Chat;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut chat = Chat::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            break;
        }

        match chat.send_async(&input).await {
            Ok(response) => println!("Assistant: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
```

### Command Generation with Validation

```rust
use lib_core::Core;
use std::env;

fn main() -> anyhow::Result<()> {
    let model_path = env::var("EIDOS_MODEL_PATH")?;
    let tokenizer_path = env::var("EIDOS_TOKENIZER_PATH")?;

    let core = Core::new(&model_path, &tokenizer_path)?;

    let prompts = vec![
        "list all files",
        "show current directory",
        "find Python files",
    ];

    for prompt in prompts {
        match core.run(prompt) {
            Ok(command) => {
                println!("Prompt: {}", prompt);
                println!("Command: {}", command);
                println!();
            }
            Err(e) => eprintln!("Error for '{}': {}", prompt, e),
        }
    }

    Ok(())
}
```

### Custom Request Handler

```rust
use lib_bridge::{Bridge, Request};

fn main() -> Result<(), String> {
    let mut bridge = Bridge::new();

    // Custom handler with error handling
    bridge.register(
        Request::Core,
        Box::new(|prompt: &str| {
            if prompt.is_empty() {
                return Err("Prompt cannot be empty".to_string());
            }

            if prompt.len() > 1000 {
                return Err("Prompt too long".to_string());
            }

            // Process prompt
            println!("Processing: {}", prompt);
            Ok(())
        }),
    );

    // Test
    bridge.route(Request::Core, "test")?;
    bridge.route(Request::Core, "")?; // Error

    Ok(())
}
```

### Multi-Language Translation

```rust
use lib_translate::Translate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let translate = Translate::new();

    let texts = vec![
        "Bonjour",           // French
        "Hola",              // Spanish
        "Guten Tag",         // German
        "こんにちは",          // Japanese
        "Hello",             // English
    ];

    for text in texts {
        match translate.detect_and_translate_async(text, "en").await {
            Ok(result) => {
                println!("Text: {}", text);
                println!("Language: {}", result.detected_lang);
                if result.detected_lang != "en" {
                    println!("Translation: {}", result.translated);
                }
                println!();
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
```

## Error Handling

All library functions return `Result` types:

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

**Error Types:**

- `lib_core::CoreError` - Model loading, inference errors
- `lib_chat::ChatError` - API errors, network issues
- `lib_translate::TranslateError` - Detection, translation errors

**Example Error Handling:**

```rust
use lib_chat::Chat;

#[tokio::main]
async fn main() {
    let mut chat = Chat::new();

    match chat.send_async("Hello").await {
        Ok(response) => println!("{}", response),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Please check your API configuration");
        }
    }
}
```

## See Also

- [Architecture](ARCHITECTURE.md) - System design details
- [Model Guide](MODEL_GUIDE.md) - Training and deployment
- [Contributing](../CONTRIBUTING.md) - Development guidelines
