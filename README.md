# Eidos: The AI-Powered Command Line for Linux

[![CI](https://github.com/Ru1vly/Eidos/workflows/CI/badge.svg)](https://github.com/Ru1vly/Eidos/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Eidos is an AI-powered command-line interface (CLI) that brings natural language processing to your Linux terminal. Built with Rust, Eidos leverages large language models (LLMs) to translate natural language requests into safe shell commands.

⚠️ **Project Status**: Early Development - The project is functional but requires trained models to operate.

## Features

*   **Natural Language to Command Translation:** Translates natural language requests into Linux shell commands
*   **Robust Command Validation:** Whitelist-based security system blocks dangerous commands and shell injection attempts
*   **Modular Architecture:** Clean separation of concerns with dedicated libraries for core functionality, chat, translation, and routing
*   **Flexible Configuration:** Supports both config files and environment variables for model paths
*   **Comprehensive Testing:** Security-critical code is thoroughly tested

## Architecture

Eidos follows a modular library structure:

*   **`lib_core`**: Core LLM inference and command execution
    - Supports ONNX models via tract for command generation
    - Implements comprehensive command validation and security
    - Blocks dangerous commands, shell metacharacters, and injection attempts
*   **`lib_chat`**: Chat and search API integration (stub implementation)
*   **`lib_translate`**: Language detection and translation (stub implementation)
*   **`lib_bridge`**: Request routing between different modules
*   **`src/main.rs`**: CLI interface using clap for command-line parsing
*   **`src/config.rs`**: Configuration management with TOML and environment variable support

## Getting Started

### Prerequisites

To build and run Eidos, you will need the following:

*   **Rust:** Eidos is built with Rust. You can install it from the official website: https://www.rust-lang.org/tools/install
*   **Cargo:** Cargo is the Rust package manager. It is included with the Rust installation.
*   **Git:** You will need Git to clone the Eidos repository.

### Building

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/Ru1vly/Eidos.git
    cd eidos
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

3.  **Run tests:**

    ```bash
    cargo test
    ```

### Configuration

Eidos requires an ONNX model and tokenizer to function. You can configure these paths in three ways:

**Option 1: Configuration File** (Recommended)

Create `eidos.toml` in the project root:

```toml
model_path = "path/to/your/model.onnx"
tokenizer_path = "path/to/your/tokenizer.json"
```

**Option 2: Environment Variables**

```bash
export EIDOS_MODEL_PATH="/path/to/model.onnx"
export EIDOS_TOKENIZER_PATH="/path/to/tokenizer.json"
```

**Option 3: Default Paths**

Place `model.onnx` and `tokenizer.json` in the project root directory.

### Training Your Own Model

See `CORE.md` for detailed instructions on training a model on Linux MAN pages using the provided Python scripts in `scripts/`.

## Usage

Eidos provides three main commands:

```bash
# Natural language to command translation
eidos core "list all files in the current directory"

# Chat functionality (stub - not yet implemented)
eidos chat "Hello"

# Translation functionality (stub - not yet implemented)
eidos translate "Bonjour"
```

### Security

Eidos implements multiple layers of security:

- **Whitelist approach**: Only allows safe commands (ls, cat, grep, etc.)
- **Dangerous command blocking**: Blocks rm, sudo, chmod, curl, wget, ssh, etc.
- **Shell injection prevention**: Blocks pipes, redirects, command substitution, and other shell metacharacters
- **Path traversal protection**: Blocks `../`, `/dev/`, `/proc/`, `/sys/` access
- **No arbitrary code execution**: Commands are validated before execution

**Note**: Even with these protections, only use Eidos with trusted models and in controlled environments.

## Contributing

We welcome contributions from the community! If you're interested in helping us improve Eidos, please check out our contributing guidelines.

We're excited to have you on board!
