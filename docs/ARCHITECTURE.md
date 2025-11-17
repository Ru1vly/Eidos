# Eidos Architecture

This document describes the high-level architecture and design decisions of Eidos.

## Table of Contents

1. [Overview](#overview)
2. [System Architecture](#system-architecture)
3. [Components](#components)
4. [Data Flow](#data-flow)
5. [Security Model](#security-model)
6. [Design Decisions](#design-decisions)
7. [Extension Points](#extension-points)

## Overview

Eidos is a modular AI-powered CLI for Linux that translates natural language into shell commands and provides chat/translation capabilities.

### Core Principles

1. **Modularity**: Separate concerns into distinct crates
2. **Safety**: Validate all generated commands before execution
3. **Flexibility**: Support multiple model types and API providers
4. **Performance**: Optimized for fast inference and minimal resource usage

### Technology Stack

- **Language**: Rust 2021 edition
- **Inference**: tract (ONNX) and candle (GGUF)
- **CLI**: clap 4.x
- **Async**: tokio runtime
- **HTTP**: reqwest with rustls
- **Serialization**: serde with JSON/TOML support

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     CLI Interface                        │
│                    (src/main.rs)                        │
└───────────────────┬─────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────┐
│                   Request Router                         │
│                   (lib_bridge)                          │
└───┬──────────────┬──────────────┬──────────────────────┘
    │              │              │
    ▼              ▼              ▼
┌─────────┐  ┌──────────┐  ┌────────────┐
│lib_core │  │lib_chat  │  │lib_translate│
│         │  │          │  │            │
│ ONNX/   │  │ OpenAI   │  │ Lingua +   │
│ GGUF    │  │ Ollama   │  │LibreTranslate
│ Models  │  │ Custom   │  │            │
└─────────┘  └──────────┘  └────────────┘
```

### Component Diagram

```
┌────────────────────────────────────────────────────────┐
│ eidos (main binary)                                    │
│ ┌────────────┐  ┌─────────────┐  ┌─────────────┐    │
│ │   CLI      │  │   Config    │  │   Error     │    │
│ │  Parser    │  │  Manager    │  │  Handling   │    │
│ └────────────┘  └─────────────┘  └─────────────┘    │
└───────────────────────┬────────────────────────────────┘
                        │
        ┌───────────────┴───────────────┐
        ▼                               ▼
┌───────────────┐              ┌────────────────┐
│  lib_bridge   │              │   Utilities    │
│  ┌──────────┐ │              │   (shared)     │
│  │ Router   │ │              └────────────────┘
│  │ Registry │ │
│  └──────────┘ │
└───┬───┬───┬───┘
    │   │   │
    ▼   ▼   ▼
┌────────┐ ┌─────────┐ ┌──────────────┐
│lib_core│ │lib_chat │ │lib_translate │
└────────┘ └─────────┘ └──────────────┘
```

## Components

### 1. Main Binary (`src/`)

**Responsibilities:**
- CLI argument parsing with clap
- Configuration loading (TOML/env)
- Bridge setup and request routing
- Error handling and user feedback

**Key Files:**
- `main.rs`: Entry point, CLI setup, bridge initialization
- `config.rs`: Configuration management (TOML, env vars, defaults)
- `error.rs`: Application-level error types

**Flow:**
```rust
fn main() -> Result<()> {
    // 1. Parse CLI arguments
    let cli = Cli::parse();

    // 2. Initialize bridge with handlers
    let bridge = setup_bridge();

    // 3. Route request
    bridge.route(request, input)?;

    Ok(())
}
```

### 2. lib_bridge (Request Routing)

**Purpose**: Dynamic request routing to appropriate handlers

**Architecture:**
```rust
pub struct Bridge {
    router: HashMap<Request, Handler>,
}

pub type Handler = Box<dyn Fn(&str) -> Result<(), String>>;

impl Bridge {
    pub fn register(&mut self, request: Request, handler: Handler);
    pub fn route(&self, request: Request, input: &str) -> Result<(), String>;
}
```

**Design Pattern**: Strategy Pattern
- Decouples request types from implementations
- Allows runtime handler registration
- Enables easy testing and mocking

### 3. lib_core (Command Generation)

**Purpose**: AI model inference for command generation

**Architecture:**
```
lib_core/
├── lib.rs           # Public API
├── error.rs         # CoreError types
├── tract_llm.rs     # ONNX model inference
└── quantized_llm.rs # GGUF model inference
```

**Components:**

**Core Struct:**
```rust
pub struct Core {
    model: tract_onnx::model::TypedModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl Core {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self>;
    pub fn run(&self, prompt: &str) -> Result<String>;
}
```

**Security Validation:**
```rust
fn is_safe_command(command: &str) -> bool {
    // 1. Whitelist check (allow known safe commands)
    // 2. Blacklist check (block dangerous patterns)
    // 3. Shell injection detection
    // 4. Path traversal prevention
}
```

**Supported Models:**
- **ONNX** via tract: T5, BART, GPT-2
- **GGUF** via candle: LLaMA, Mistral (quantized)

### 4. lib_chat (Chat Functionality)

**Purpose**: LLM API integration for conversational AI

**Architecture:**
```
lib_chat/
├── lib.rs        # Public API
├── error.rs      # ChatError types
├── history.rs    # Conversation management
└── api.rs        # API providers
```

**Provider Pattern:**
```rust
pub enum ApiProvider {
    OpenAI { api_key: String, model: String },
    Ollama { base_url: String, model: String },
    Custom { base_url: String, api_key: Option<String>, model: String },
}

impl ApiProvider {
    pub fn from_env() -> Result<Self>;
}
```

**Conversation Management:**
```rust
pub struct ConversationHistory {
    messages: Vec<Message>,
    max_messages: usize,  // Auto-prune
}

pub struct Message {
    pub role: Role,       // System, User, Assistant
    pub content: String,
}
```

**Async Runtime:**
- Uses tokio for async HTTP requests
- Blocking wrapper for CLI usage
- Configurable timeouts

### 5. lib_translate (Translation Service)

**Purpose**: Language detection and translation

**Architecture:**
```
lib_translate/
├── lib.rs         # Public API
├── error.rs       # TranslateError types
├── detector.rs    # Language detection (lingua)
└── translator.rs  # Translation API (LibreTranslate)
```

**Two-Stage Process:**

**1. Detection (Offline):**
```rust
pub fn detect_language(text: &str) -> Result<Language> {
    let detector = get_detector();  // Cached, supports 75+ languages
    detector.detect_language_of(text)
}
```

**2. Translation (Online):**
```rust
pub enum TranslatorProvider {
    LibreTranslate { url: String, api_key: Option<String> },
    Mock,  // For testing
}

pub async fn translate_async(
    &self,
    text: &str,
    source: &str,
    target: &str
) -> Result<String>;
```

## Data Flow

### Command Generation Flow

```
User Input
    │
    ▼
┌─────────────┐
│ CLI Parser  │ Parse: "list all files"
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Bridge    │ Route to Request::Core
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Config    │ Load model paths
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  lib_core   │ Initialize model
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Tokenizer  │ Encode: "list all files" → [4234, 832, 9012]
└──────┬──────┘
       │
       ▼
┌─────────────┐
│Model Inference│ Generate: [4234, 832, 9012] → [5123, 823]
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Tokenizer  │ Decode: [5123, 823] → "ls -la"
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Validation  │ Check safety: is_safe_command("ls -la")
└──────┬──────┘
       │
       ▼
Output: "ls -la"
```

### Chat Flow

```
User: "Hello"
    │
    ▼
┌──────────────┐
│  lib_chat    │ Chat::send_async("Hello")
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   History    │ Add user message
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ API Provider │ Detect from env (OpenAI/Ollama/Custom)
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ HTTP Request │ POST to API with messages
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Response   │ Extract assistant reply
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   History    │ Add assistant message
└──────┬───────┘
       │
       ▼
Output: "Hello! How can I help?"
```

### Translation Flow

```
Input: "Bonjour le monde"
    │
    ▼
┌────────────────┐
│lib_translate   │
└────────┬───────┘
         │
         ▼
┌────────────────┐
│   Detector     │ lingua: detect language
└────────┬───────┘
         │
         ▼
Result: "fr" (French)
         │
         ▼
┌────────────────┐
│  is_english?   │ Check if already English
└────────┬───────┘
         │ No
         ▼
┌────────────────┐
│  Translator    │ LibreTranslate API
└────────┬───────┘
         │
         ▼
┌────────────────┐
│  API Request   │ POST {"q": "Bonjour", "source": "fr", "target": "en"}
└────────┬───────┘
         │
         ▼
Output: "Hello world"
```

## Security Model

### Defense in Depth

**Layer 1: Input Validation**
- Command length limits
- Character encoding validation
- Empty input rejection

**Layer 2: Command Validation**
```rust
const DANGEROUS_PATTERNS: &[&str] = &[
    "rm -rf", "dd if=", "mkfs", "chmod 777",
    ">(", "|", "&", ";", "$(",  // Shell injection
    "../", "~/.ssh/",            // Path traversal
    // ... 60+ patterns
];

fn is_safe_command(cmd: &str) -> bool {
    // Check whitelist first (performance)
    if is_whitelisted(cmd) {
        return true;
    }

    // Check for dangerous patterns
    for pattern in DANGEROUS_PATTERNS {
        if cmd.contains(pattern) {
            return false;
        }
    }

    true
}
```

**Layer 3: Execution Prevention**
- Eidos NEVER executes commands
- User reviews output before execution
- Display-only mode

**Layer 4: API Key Protection**
- Environment variables only
- Never logged or displayed
- Optional (not required for core functionality)

### Threat Model

**In Scope:**
- Malicious prompt injection
- Shell command injection
- Path traversal attacks
- Denial of service (resource exhaustion)

**Out of Scope:**
- Model poisoning (user-trained models)
- Side-channel attacks
- Physical access

## Design Decisions

### 1. Why Rust?

**Pros:**
- Memory safety without garbage collection
- Zero-cost abstractions
- Excellent async support (tokio)
- Fast compilation and execution
- Strong type system prevents bugs

**Trade-offs:**
- Steeper learning curve
- Longer compile times than interpreted languages
- Smaller ecosystem than Python/JavaScript

### 2. Why Modular Crates?

**Benefits:**
- Clear separation of concerns
- Independent testing and versioning
- Reusable components
- Parallel compilation

**Structure:**
```
eidos (binary)
├── lib_core (inference)
├── lib_chat (API integration)
├── lib_translate (language services)
└── lib_bridge (routing)
```

### 3. Why tract and candle?

**tract (ONNX):**
- Mature, production-ready
- CPU-optimized
- No external dependencies
- Good T5/BART support

**candle (GGUF):**
- Rust-native
- Quantized model support
- LLaMA/Mistral compatibility
- Memory efficient

**Alternative Considered:**
- ONNX Runtime (C++ dependency)
- llama.cpp (FFI complexity)

### 4. Configuration Hierarchy

**Priority (highest to lowest):**
1. Environment variables (runtime flexibility)
2. `./eidos.toml` (project-specific)
3. `~/.config/eidos/eidos.toml` (user default)
4. Built-in defaults (fallback)

**Rationale:**
- Follows Unix philosophy (env > config > defaults)
- Enables Docker/CI deployment
- User-friendly for local development

### 5. Sync vs Async

**Decision:**
- Core inference: Sync (blocking operations)
- Chat/Translate APIs: Async (I/O bound)
- CLI wrapper: Tokio runtime for async calls

**Code Pattern:**
```rust
pub fn run(&mut self, text: &str) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    match runtime.block_on(self.send_async(text)) {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 6. Error Handling Strategy

**Use anyhow for applications, thiserror for libraries:**

```rust
// Library (lib_chat/src/error.rs)
#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    #[error("API request failed: {0}")]
    RequestError(String),
}

// Application (src/main.rs)
use anyhow::{Result, Context};

fn main() -> Result<()> {
    load_config()
        .context("Failed to load configuration")?;
    Ok(())
}
```

## Extension Points

### Adding New Commands

1. **Define Request Type**
```rust
// lib_bridge/src/lib.rs
pub enum Request {
    Chat,
    Core,
    Translate,
    NewCommand,  // Add here
}
```

2. **Implement Handler**
```rust
// src/main.rs
fn setup_bridge() -> Bridge {
    let mut bridge = Bridge::new();

    bridge.register(
        Request::NewCommand,
        Box::new(|input: &str| {
            // Implementation
            Ok(())
        }),
    );

    bridge
}
```

3. **Add CLI Subcommand**
```rust
#[derive(Subcommand, Debug)]
enum Commands {
    NewCommand { input: String },
    // ...
}
```

### Adding Model Support

**ONNX Model:**
```rust
// lib_core/src/new_model.rs
pub struct NewModel {
    model: TypedModel,
    tokenizer: Tokenizer,
}

impl NewModel {
    pub fn load(path: &str) -> Result<Self> {
        // Load model
    }

    pub fn infer(&self, input: &str) -> Result<String> {
        // Run inference
    }
}
```

### Adding API Providers

```rust
// lib_chat/src/api.rs
pub enum ApiProvider {
    NewProvider {
        endpoint: String,
        api_key: String,
    },
    // ...
}

impl ApiProvider {
    async fn send_request(&self, messages: &[Message]) -> Result<String> {
        match self {
            Self::NewProvider { endpoint, api_key } => {
                // Implementation
            }
            // ...
        }
    }
}
```

## Performance Characteristics

### Resource Usage

**Memory:**
- Base binary: ~5MB
- ONNX model (T5-small): ~200MB
- GGUF model (LLaMA-7B Q4): ~4GB
- Runtime overhead: ~50MB

**CPU:**
- Inference (T5-small): ~100-500ms per request
- Inference (LLaMA-7B Q4): ~1-5s per request
- Lingua detection: ~10-50ms

**Disk:**
- Binary: ~5MB (stripped)
- Models: 100MB-10GB (user-provided)
- Config: <1KB

### Optimization Strategies

1. **Model Caching**: Models loaded once, reused
2. **Lazy Initialization**: Components created on-demand
3. **Streaming**: Future support for streaming responses
4. **Quantization**: Q4/Q8 models for memory efficiency

## Testing Strategy

### Test Pyramid

```
        /\
       /  \        E2E Tests (9)
      /────\       Integration Tests
     /──────\      Unit Tests (29)
    /────────\
```

**Unit Tests (29):**
- lib_core: Command validation (7)
- lib_chat: History management (3)
- lib_translate: Detection/translation (7)
- lib_bridge: Routing logic (10)
- Config: Loading hierarchy (2)

**Integration Tests (9):**
- CLI commands end-to-end
- Error handling
- Configuration integration

**Benchmarks:**
- Inference performance
- Command validation speed

## Future Enhancements

### Planned Features

1. **Streaming Responses**: Real-time chat output
2. **Plugin System**: External command handlers
3. **Web Interface**: Optional GUI
4. **History Persistence**: Save conversations
5. **Multi-model Support**: Model switching at runtime

### Scalability Considerations

- **Horizontal**: Run multiple instances (stateless)
- **Vertical**: Larger models with more RAM
- **Caching**: Cache frequent commands
- **Distribution**: Pre-compiled binaries for all platforms

---

For implementation details, see source code and inline documentation.
