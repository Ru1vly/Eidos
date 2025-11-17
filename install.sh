#!/bin/bash
# Eidos Installation Script for Linux
# This script installs Eidos and its dependencies

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
CONFIG_DIR="${CONFIG_DIR:-$HOME/.config/eidos}"
VERSION="${VERSION:-0.1.0}"
REPO_URL="https://github.com/Ru1vly/eidos"

# Print colored message
print_message() {
    local color=$1
    shift
    echo -e "${color}$@${NC}"
}

print_success() {
    print_message "$GREEN" "✓ $@"
}

print_error() {
    print_message "$RED" "✗ $@"
}

print_info() {
    print_message "$YELLOW" "→ $@"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    else
        echo "unknown"
    fi
}

# Check prerequisites
check_prerequisites() {
    print_info "Checking prerequisites..."

    # Check for Rust/Cargo
    if ! command_exists cargo; then
        print_error "Cargo not found. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    print_success "Rust/Cargo installed"

    # Check for Git
    if ! command_exists git; then
        print_error "Git not found. Please install git first."
        exit 1
    fi
    print_success "Git installed"
}

# Install from source
install_from_source() {
    print_info "Installing Eidos from source..."

    # Create temporary directory
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"

    # Clone repository
    print_info "Cloning repository..."
    git clone "$REPO_URL" eidos
    cd eidos

    # Build release binary
    print_info "Building Eidos (this may take a few minutes)..."
    cargo build --release

    # Create installation directory
    mkdir -p "$INSTALL_DIR"

    # Copy binary
    print_info "Installing binary to $INSTALL_DIR..."
    cp target/release/eidos "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/eidos"

    # Copy example config
    mkdir -p "$CONFIG_DIR"
    if [ -f "eidos.toml.example" ]; then
        cp eidos.toml.example "$CONFIG_DIR/eidos.toml.example"
    fi

    # Cleanup
    cd /
    rm -rf "$temp_dir"

    print_success "Eidos installed to $INSTALL_DIR/eidos"
}

# Install from pre-built binary (if available)
install_from_binary() {
    print_info "Downloading pre-built binary..."

    local os=$(detect_os)
    local arch=$(uname -m)
    local binary_url="${REPO_URL}/releases/download/v${VERSION}/eidos-${os}-${arch}"

    # Create installation directory
    mkdir -p "$INSTALL_DIR"

    # Download binary
    if command_exists curl; then
        curl -L -o "$INSTALL_DIR/eidos" "$binary_url"
    elif command_exists wget; then
        wget -O "$INSTALL_DIR/eidos" "$binary_url"
    else
        print_error "Neither curl nor wget found. Cannot download binary."
        return 1
    fi

    chmod +x "$INSTALL_DIR/eidos"
    print_success "Eidos installed to $INSTALL_DIR/eidos"
}

# Setup PATH
setup_path() {
    print_info "Checking PATH configuration..."

    # Check if INSTALL_DIR is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_info "Adding $INSTALL_DIR to PATH..."

        # Detect shell
        local shell_rc=""
        if [ -n "$BASH_VERSION" ]; then
            shell_rc="$HOME/.bashrc"
        elif [ -n "$ZSH_VERSION" ]; then
            shell_rc="$HOME/.zshrc"
        else
            shell_rc="$HOME/.profile"
        fi

        # Add to shell rc if not already present
        if ! grep -q "export PATH.*$INSTALL_DIR" "$shell_rc" 2>/dev/null; then
            echo "" >> "$shell_rc"
            echo "# Eidos" >> "$shell_rc"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$shell_rc"
            print_success "Added $INSTALL_DIR to PATH in $shell_rc"
            print_info "Run 'source $shell_rc' or restart your shell to apply changes"
        fi
    else
        print_success "PATH already configured"
    fi
}

# Post-installation setup
post_install() {
    print_info "Setting up configuration..."

    # Create config directory
    mkdir -p "$CONFIG_DIR"

    # Print next steps
    cat << EOF

${GREEN}========================================
Eidos Installation Complete!
========================================${NC}

${YELLOW}Next Steps:${NC}

1. Configure Eidos:
   ${GREEN}vim ~/.config/eidos/eidos.toml${NC}

   Or set environment variables:
   ${GREEN}export EIDOS_MODEL_PATH=/path/to/model.onnx
   export EIDOS_TOKENIZER_PATH=/path/to/tokenizer.json${NC}

2. (Optional) Configure Chat API:
   ${GREEN}export OPENAI_API_KEY=your-key${NC}
   or
   ${GREEN}export OLLAMA_HOST=http://localhost:11434${NC}

3. Test installation:
   ${GREEN}eidos --version
   eidos --help${NC}

4. Try commands:
   ${GREEN}eidos chat "Hello, world!"
   eidos translate "Bonjour"${NC}

${YELLOW}Documentation:${NC}
   README: $REPO_URL
   Models: See docs/MODEL_GUIDE.md

${YELLOW}Installed to:${NC} $INSTALL_DIR/eidos
${YELLOW}Config dir:${NC} $CONFIG_DIR

EOF
}

# Main installation flow
main() {
    cat << "EOF"
 _____ _     _
|  ___(_) __| | ___  ___
| |_  | |/ _` |/ _ \/ __|
|  _| | | (_| | (_) \__ \
|_|   |_|\__,_|\___/|___/

AI-Powered CLI for Linux
EOF

    print_info "Starting installation..."

    # Check prerequisites
    check_prerequisites

    # Ask installation method
    print_info "Installation method:"
    echo "  1) Install from source (recommended)"
    echo "  2) Install from pre-built binary (if available)"
    read -p "Choose [1-2]: " choice

    case $choice in
        1)
            install_from_source
            ;;
        2)
            if ! install_from_binary; then
                print_info "Binary installation failed, falling back to source..."
                install_from_source
            fi
            ;;
        *)
            install_from_source
            ;;
    esac

    # Setup PATH
    setup_path

    # Post-installation
    post_install

    print_success "Installation complete!"
}

# Run main installation
main "$@"
