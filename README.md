# Eidos: AI-Powered CLI for Linux

[![CI](https://github.com/Ru1vly/Eidos/workflows/CI/badge.svg)](https://github.com/Ru1vly/Eidos/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

Eidos is an AI-powered command-line interface that brings natural language processing to your Linux terminal. Built with Rust, Eidos leverages large language models to translate natural language into safe shell commands, provide intelligent chat assistance, and offer multi-language translation.

🚀 **Project Status**: **Beta** - Core functionality complete with comprehensive testing and documentation. Performance optimizations implemented (model caching, shared runtime).

⚠️ **Important**: Eidos requires trained ONNX or GGUF models to function. See [Model Training Guide](docs/MODEL_GUIDE.md) for instructions on training your own models, or wait for pre-trained model releases.

## ✨ Features

### 🤖 Natural Language Command Generation
- Translate English descriptions into shell commands
- Support for ONNX models (T5, BART, GPT-2) via tract
- Support for quantized models (LLaMA, Mistral) via candle/GGUF
- Intelligent command validation with 60+ dangerous pattern detection

### 💬 AI Chat Integration
- Multi-provider support: OpenAI, Ollama, custom APIs
- Conversation history with auto-pruning
- Async/sync runtime support
- Configurable via environment variables

### 🌍 Language Translation
- Auto-detect 75+ languages with lingua
- Translate to/from any supported language
- LibreTranslate API integration
- Offline language detection

### 🔒 Security-First Design
- Whitelist-based command validation
- Shell injection prevention
- Path traversal protection
- No automatic command execution
- Comprehensive security testing (7 dedicated tests)

### 📦 Distribution Ready
- Docker support with multi-stage builds
- Interactive installation script
- Makefile for common tasks
- Pre-built binary support
- Systemd and Kubernetes deployment examples

## 📊 Quick Start

### One-Line Install

```bash
curl -sSf https://raw.githubusercontent.com/Ru1vly/eidos/main/install.sh | bash
```

### Docker

```bash
docker pull eidos:latest
docker run --rm eidos chat "Hello, world!"
```

### From Source

```bash
git clone https://github.com/Ru1vly/eidos
cd eidos
make build-release
make install
```

## 🎯 Usage

### Core - Command Generation

```bash
# Set up model paths
export EIDOS_MODEL_PATH=/path/to/model.onnx
export EIDOS_TOKENIZER_PATH=/path/to/tokenizer.json

# Generate commands from natural language
eidos core "list all files"
# Output: ls -la

eidos core "find Python files in current directory"
# Output: find . -name '*.py'

eidos core "show disk usage"
# Output: df -h
```

### Chat - AI Assistant

```bash
# Configure API
export OPENAI_API_KEY=sk-...
# or
export OLLAMA_HOST=http://localhost:11434

# Start chatting
eidos chat "Explain how grep works"
eidos chat "What is the difference between cat and less?"
```

### Translate - Multi-Language

```bash
eidos translate "Bonjour le monde"
# Detected language: fr
# Translated (en): Hello world

eidos translate "Hola, ¿cómo estás?"
# Detected language: es
# Translated (en): Hello, how are you?
```

## 🏗️ Architecture

Eidos follows a modular design with clear separation of concerns:

```
┌─────────────────────────────────────┐
│          CLI Interface              │
│         (src/main.rs)               │
└────────────────┬────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│         Request Router              │
│         (lib_bridge)                │
└──┬──────────┬──────────┬────────────┘
   │          │          │
   ▼          ▼          ▼
┌────────┐ ┌────────┐ ┌──────────┐
│lib_core│ │lib_chat│ │lib_      │
│        │ │        │ │translate │
│ONNX/   │ │OpenAI/ │ │Lingua/   │
│GGUF    │ │Ollama  │ │Libre     │
└────────┘ └────────┘ └──────────┘
```

### Components

- **`lib_core`**: Command generation with ONNX/GGUF model support
- **`lib_chat`**: Multi-provider LLM API integration
- **`lib_translate`**: Language detection and translation
- **`lib_bridge`**: Dynamic request routing system
- **`src/`**: CLI interface, configuration, error handling

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed design documentation.

## 📚 Documentation

- **[Installation & Deployment](docs/DEPLOYMENT.md)** - Complete deployment guide
- **[Architecture](docs/ARCHITECTURE.md)** - System design and components
- **[Model Training](docs/MODEL_GUIDE.md)** - Train and deploy custom models
- **[API Reference](docs/API.md)** - Programmatic usage
- **[Contributing](CONTRIBUTING.md)** - Development guidelines

## 🔧 Configuration

### Priority (Highest to Lowest)

1. **Environment Variables**
   ```bash
   export EIDOS_MODEL_PATH=/path/to/model.onnx
   export EIDOS_TOKENIZER_PATH=/path/to/tokenizer.json
   export OPENAI_API_KEY=sk-...
   ```

2. **Local Config** (`./eidos.toml`)
   ```toml
   model_path = "model.onnx"
   tokenizer_path = "tokenizer.json"
   ```

3. **User Config** (`~/.config/eidos/eidos.toml`)

4. **Built-in Defaults**

## 🧪 Testing

Comprehensive test suite with **38 tests passing**:

```bash
# Run all tests
cargo test --all

# Run specific test suite
cargo test -p lib_core
cargo test --test integration_tests

# Run with coverage
make test

# Run benchmarks
cargo bench
```

### Test Coverage

- **Unit Tests (29)**: Core logic, routing, API integration
- **Integration Tests (9)**: End-to-end CLI workflows
- **Security Tests (7)**: Command validation and injection prevention
- **Benchmark Suite**: Performance testing for inference

## 🔐 Security

Eidos implements defense-in-depth security:

### Layer 1: Input Validation
- Length limits and encoding checks
- Empty input rejection

### Layer 2: Command Validation
- 60+ dangerous pattern detection
- Shell metacharacter blocking
- Path traversal prevention

### Layer 3: Execution Prevention
- **Never executes commands automatically**
- Display-only mode
- User reviews all output

### Blocked Patterns
```
rm -rf, dd if=, mkfs, chmod 777, curl | sh,
>, |, &, ;, $( ), ` `, ../,
~/.ssh/, /dev/, /proc/, fork bombs, etc.
```

See [lib_core/tests/command_validation_tests.rs](lib_core/tests/command_validation_tests.rs) for complete test suite.

## 🎓 Training Models

Eidos supports custom model training:

### Quick Start

```bash
# 1. Prepare training data (JSONL format)
cat > training_data.jsonl <<EOF
{"prompt": "list all files", "command": "ls -la"}
{"prompt": "show current directory", "command": "pwd"}
EOF

# 2. Train model
./scripts/train_model.py training_data.jsonl -o ./my-model

# 3. Validate
./scripts/validate_model.py ./my-model/final_model test_data.jsonl

# 4. Convert to ONNX
./scripts/convert_to_onnx.py ./my-model/final_model -o model.onnx

# 5. Use with Eidos
eidos core "list files"
```

See [docs/MODEL_GUIDE.md](docs/MODEL_GUIDE.md) for comprehensive training guide.

### Example Dataset

100+ example command pairs provided in [datasets/example_commands.jsonl](datasets/example_commands.jsonl):

```json
{"prompt": "list all files", "command": "ls -la"}
{"prompt": "find Python files", "command": "find . -name '*.py'"}
{"prompt": "count lines in file.txt", "command": "wc -l file.txt"}
```

## 🐳 Docker Deployment

### Basic Usage

```bash
# Build image
docker build -t eidos:latest .

# Run command
docker run --rm \
  -v $(pwd)/models:/home/eidos/models:ro \
  eidos core "list files"
```

### Docker Compose

```bash
# Start services
docker-compose up -d

# Run command
docker-compose run eidos chat "Hello"

# With Ollama
docker-compose --profile with-ollama up -d
```

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for production deployment guides.

## 🛠️ Development

### Setup

```bash
# Clone and enter directory
git clone https://github.com/Ru1vly/eidos
cd eidos

# Install development tools
make dev-setup

# Build
make build

# Run tests
make test

# Format and lint
make check-all
```

### Project Structure

```
eidos/
├── src/              # Main binary (CLI, config, errors)
├── lib_core/         # Command generation (ONNX/GGUF)
├── lib_chat/         # Chat API integration
├── lib_translate/    # Translation service
├── lib_bridge/       # Request routing
├── tests/            # Integration tests
├── benches/          # Performance benchmarks
├── docs/             # Documentation
├── scripts/          # Training/validation scripts
└── datasets/         # Example training data
```

### Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Good First Issues:**
- Check issues labeled `good first issue`
- Add more training examples
- Improve documentation
- Add test coverage

## 📈 Roadmap

### Completed ✅
- [x] Core command generation (ONNX/GGUF)
- [x] Multi-provider chat integration
- [x] Language detection and translation
- [x] Comprehensive security validation
- [x] Full test suite (38 tests)
- [x] Docker deployment
- [x] Installation scripts
- [x] Complete documentation

### Planned 🚧
- [ ] Streaming responses
- [ ] Plugin system for custom handlers
- [ ] Conversation history persistence
- [ ] Web interface (optional GUI)
- [ ] Pre-trained model releases
- [ ] Multi-architecture binaries

## 📊 Benchmarks

Performance characteristics (T5-small on CPU):

- **Inference**: ~100-500ms per command
- **Memory**: ~200MB (model) + ~50MB (runtime)
- **Startup**: <100ms
- **Language Detection**: ~10-50ms

Run benchmarks:
```bash
cargo bench
```

## 🤝 Community

- **Issues**: [GitHub Issues](https://github.com/Ru1vly/eidos/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Ru1vly/eidos/discussions)
- **Contributions**: See [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 License

Eidos is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- [tract](https://github.com/sonos/tract) - ONNX runtime
- [candle](https://github.com/huggingface/candle) - Rust ML framework
- [lingua-rs](https://github.com/pemistahl/lingua-rs) - Language detection
- [Hugging Face](https://huggingface.co/) - Model ecosystem

## 📞 Support

- **Documentation**: [docs/](docs/)
- **Bug Reports**: [Create an issue](https://github.com/Ru1vly/eidos/issues/new)
- **Feature Requests**: [Start a discussion](https://github.com/Ru1vly/eidos/discussions/new)

---

Built with ❤️ using Rust
