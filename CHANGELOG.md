# Changelog

All notable changes to the Eidos project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-beta] - 2025-11-17

### Added
- User config file support (`~/.config/eidos/eidos.toml`)
- Comprehensive command validation module with 7 test suites
- HTTP client timeouts (30s request, 10s connection) to prevent hanging
- Shared tokio runtime in lib_translate for better performance
- Error propagation throughout the application for proper exit codes
- Enhanced documentation and code quality improvements

### Changed
- Config loading priority: env vars > local config > user config > defaults
- Chat and Translate `run()` methods now return `Result` types
- Improved model caching with better Arc usage (no unwrap)
- Extracted validation logic to dedicated module (eliminated duplication)

### Removed
- Dangerous `execute_command()` method from Core (security improvement)
- Duplicate validation tests
- Unimplemented test stubs

### Fixed
- Version number consistency across all files (tests, Dockerfile, docs)
- Config validation now properly returns errors instead of swallowing them
- RwLock usage with proper pattern matching (no unwrap calls)
- Double-check pattern in model cache simplified
- Translation runtime inefficiency (was creating new runtime per request)

### Security
- Removed command execution capability - now display-only
- Enhanced validation prevents shell injection attempts
- Blocks 60+ dangerous command patterns
- Path traversal protection
- IFS manipulation detection

### Performance
- Model caching saves ~2-4 seconds per subsequent request
- Shared runtime saves ~10-50ms per async operation
- Minimal tokio features reduce binary size

## [0.1.0] - 2024

### Added
- Initial release
- Natural language to shell command translation
- AI chat integration (OpenAI, Ollama, custom providers)
- Language detection and translation (75+ languages)
- Docker deployment support
- Comprehensive test suite (38 tests)
- Full documentation

### Security
- Whitelist-based command validation
- Shell injection prevention
- No automatic command execution
