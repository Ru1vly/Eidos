# Contributing to Eidos

Thank you for your interest in contributing to Eidos! This document provides guidelines and instructions for contributing.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [How to Contribute](#how-to-contribute)
5. [Coding Standards](#coding-standards)
6. [Testing Guidelines](#testing-guidelines)
7. [Commit Messages](#commit-messages)
8. [Pull Request Process](#pull-request-process)
9. [Issue Guidelines](#issue-guidelines)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors, regardless of background or identity.

### Expected Behavior

- Be respectful and considerate
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards others

### Unacceptable Behavior

- Harassment, discriminatory language, or personal attacks
- Trolling or deliberately inflammatory comments
- Publishing others' private information
- Unethical or illegal conduct

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- Basic understanding of Rust and CLI development

### Quick Start

1. **Fork the repository**
   ```bash
   # Click "Fork" on GitHub, then:
   git clone https://github.com/YOUR_USERNAME/eidos
   cd eidos
   ```

2. **Set up development environment**
   ```bash
   make dev-setup
   ```

3. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make changes and test**
   ```bash
   cargo test --all
   cargo clippy --all-targets
   ```

5. **Submit pull request**

## Development Setup

### Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev
```

**macOS:**
```bash
brew install openssl pkg-config
```

**Fedora/RHEL:**
```bash
sudo dnf install -y gcc openssl-devel pkg-config
```

### Build Project

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test --all

# Run specific crate tests
cargo test -p lib_core
```

### Development Tools

```bash
# Install development tools
make dev-setup

# Auto-rebuild on changes
make watch

# Format code
make format

# Run linter
make lint

# Run all checks
make check-all
```

### Running Eidos

```bash
# Run from source
cargo run -- chat "Hello, world!"
cargo run -- translate "Bonjour"
cargo run -- core "list files"

# With specific features
cargo run --release -- core "show directory"
```

## How to Contribute

### Types of Contributions

We welcome various types of contributions:

1. **Bug Reports** - Help us identify issues
2. **Feature Requests** - Suggest new functionality
3. **Code Contributions** - Fix bugs or implement features
4. **Documentation** - Improve or expand documentation
5. **Testing** - Add test coverage
6. **Performance** - Optimize existing code

### Finding Work

- Check [Issues](https://github.com/yourusername/eidos/issues) labeled `good first issue`
- Look for `help wanted` labels
- Review open pull requests
- Propose new features in discussions

## Coding Standards

### Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/):

- Use `rustfmt` for formatting (automated with `make format`)
- Follow naming conventions:
  - `snake_case` for functions and variables
  - `CamelCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants
- Keep lines under 100 characters
- Use meaningful variable names

### Code Organization

```rust
// 1. Imports
use std::collections::HashMap;
use anyhow::Result;

// 2. Constants
const MAX_RETRIES: u32 = 3;

// 3. Type definitions
pub struct MyStruct {
    field: String,
}

// 4. Implementations
impl MyStruct {
    pub fn new() -> Self {
        // ...
    }
}

// 5. Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // ...
    }
}
```

### Error Handling

Use `anyhow::Result` for error propagation:

```rust
use anyhow::{Result, Context};

pub fn do_something() -> Result<String> {
    let data = read_file("config.toml")
        .context("Failed to read config file")?;

    Ok(data)
}
```

### Documentation

Document public APIs:

```rust
/// Processes a natural language prompt into a shell command.
///
/// # Arguments
///
/// * `prompt` - Natural language description of desired command
///
/// # Returns
///
/// Returns the generated shell command as a String
///
/// # Errors
///
/// Returns an error if:
/// - Model inference fails
/// - Prompt is empty or too long
///
/// # Examples
///
/// ```
/// let command = process_prompt("list all files")?;
/// assert_eq!(command, "ls -la");
/// ```
pub fn process_prompt(prompt: &str) -> Result<String> {
    // Implementation
}
```

### Performance Considerations

- Avoid unnecessary allocations
- Use `&str` instead of `String` when possible
- Prefer iterators over loops for collections
- Profile before optimizing (use `cargo bench`)

## Testing Guidelines

### Test Coverage

Aim for >80% code coverage:

```bash
# Run all tests
cargo test --all

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored
```

### Writing Tests

**Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = my_function("input");
        assert_eq!(result, "expected");
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_error_handling() {
        my_function("");
    }
}
```

**Integration Tests:**
```rust
// tests/integration_test.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_command() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("chat").arg("test");
    cmd.assert().success();
}
```

### Test Requirements

- All new features must include tests
- Bug fixes should include regression tests
- Tests should be deterministic (no random failures)
- Use descriptive test names: `test_<what>_<condition>_<expected>`

## Commit Messages

### Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

### Examples

```bash
# Feature
feat(chat): add streaming response support

Implements streaming for real-time chat responses.
Reduces perceived latency for long responses.

Closes #123

# Bug fix
fix(core): handle empty prompt validation

Previously crashed on empty input.
Now returns user-friendly error message.

Fixes #456

# Documentation
docs(readme): update installation instructions

Add Docker installation method.
Clarify model setup steps.
```

### Guidelines

- Use imperative mood ("add" not "added")
- Keep subject under 72 characters
- Separate subject from body with blank line
- Explain *what* and *why*, not *how*
- Reference issues and PRs

## Pull Request Process

### Before Submitting

1. **Update your branch**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run checks**
   ```bash
   make check-all
   ```

3. **Update documentation**
   - Add/update docstrings
   - Update README if needed
   - Add example usage

4. **Test thoroughly**
   ```bash
   cargo test --all
   cargo test --release
   ```

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Tests added/updated and passing
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] Branch is up-to-date with main
- [ ] No merge conflicts
- [ ] CI/CD passes

### PR Template

```markdown
## Description
Brief description of changes

## Motivation
Why is this change needed?

## Changes
- List of changes
- Another change

## Testing
How was this tested?

## Screenshots (if applicable)

## Related Issues
Closes #123
```

### Review Process

1. Maintainer reviews code
2. Automated checks run (CI/CD)
3. Feedback addressed
4. Approved and merged

### After Merge

- Delete feature branch
- Update local main branch
- Close related issues

## Issue Guidelines

### Bug Reports

Use the bug report template:

```markdown
**Describe the bug**
Clear description of the issue

**To Reproduce**
Steps to reproduce:
1. Run command '...'
2. See error

**Expected behavior**
What should happen

**Actual behavior**
What actually happens

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Eidos version: [e.g., 0.1.0]
- Rust version: [e.g., 1.75.0]

**Additional context**
Any other relevant information
```

### Feature Requests

```markdown
**Problem**
What problem does this solve?

**Proposed Solution**
How should it work?

**Alternatives**
Other approaches considered

**Additional Context**
Examples, mockups, etc.
```

### Questions

Use GitHub Discussions for:
- How-to questions
- Design discussions
- General feedback

## Development Workflow

### Typical Workflow

```bash
# 1. Create branch
git checkout -b feat/my-feature

# 2. Make changes
vim src/main.rs

# 3. Test
cargo test

# 4. Format and lint
make check-all

# 5. Commit
git add .
git commit -m "feat: add my feature"

# 6. Push
git push origin feat/my-feature

# 7. Create PR on GitHub
```

### Branch Naming

- `feat/feature-name` - New features
- `fix/bug-description` - Bug fixes
- `docs/what-changed` - Documentation
- `refactor/what-changed` - Refactoring
- `test/what-tested` - Test additions

### Working on Issues

1. Comment on issue to claim it
2. Reference issue in commits/PR
3. Link PR to issue

## Project Structure

```
eidos/
├── src/              # Main binary
├── lib_core/         # Core inference engine
├── lib_chat/         # Chat functionality
├── lib_translate/    # Translation service
├── lib_bridge/       # Request routing
├── tests/            # Integration tests
├── benches/          # Benchmarks
├── docs/             # Documentation
├── scripts/          # Utility scripts
└── datasets/         # Example training data
```

## Getting Help

- **Documentation**: See [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/yourusername/eidos/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/eidos/discussions)
- **Chat**: (if available)

## Recognition

Contributors are recognized in:
- [Contributors](https://github.com/yourusername/eidos/graphs/contributors) page
- Release notes
- Special thanks in README

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see [LICENSE](LICENSE)).

---

Thank you for contributing to Eidos! 🎉
