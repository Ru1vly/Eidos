# Deployment Guide

This guide covers various ways to deploy and distribute Eidos.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Installation Methods](#installation-methods)
3. [Docker Deployment](#docker-deployment)
4. [Building from Source](#building-from-source)
5. [Configuration](#configuration)
6. [Production Deployment](#production-deployment)

## Quick Start

### One-Line Install

```bash
curl -sSf https://raw.githubusercontent.com/yourusername/eidos/main/install.sh | bash
```

Or download and inspect first:

```bash
curl -sSf https://raw.githubusercontent.com/yourusername/eidos/main/install.sh -o install.sh
chmod +x install.sh
./install.sh
```

## Installation Methods

### Method 1: Pre-built Binary

Download from releases page:

```bash
# Linux x86_64
wget https://github.com/yourusername/eidos/releases/download/v0.1.0/eidos-linux-x86_64.tar.gz
tar -xzf eidos-linux-x86_64.tar.gz
sudo mv eidos-linux-x86_64 /usr/local/bin/eidos
```

### Method 2: Cargo Install

```bash
cargo install --git https://github.com/yourusername/eidos
```

### Method 3: Build from Source

```bash
git clone https://github.com/yourusername/eidos
cd eidos
make build-release
sudo make install
```

### Method 4: Docker

```bash
docker pull eidos:latest
docker run --rm eidos --help
```

## Docker Deployment

### Basic Usage

```bash
# Build image
docker build -t eidos:latest .

# Run command
docker run --rm eidos chat "Hello, world!"
```

### With Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  eidos:
    image: eidos:latest
    volumes:
      - ./models:/home/eidos/models:ro
    environment:
      - EIDOS_MODEL_PATH=/home/eidos/models/model.onnx
      - EIDOS_TOKENIZER_PATH=/home/eidos/models/tokenizer.json
```

Run:

```bash
docker-compose run eidos core "list files"
```

### Persistent Configuration

```bash
# Create volume for config
docker volume create eidos-config

# Run with persistent config
docker run --rm \
  -v eidos-config:/home/eidos/.config \
  -v $(pwd)/models:/home/eidos/models:ro \
  eidos core "show files"
```

### Development with Docker

```bash
# Build and run with live code
docker-compose up -d

# Exec into container
docker-compose exec eidos bash

# View logs
docker-compose logs -f eidos
```

## Building from Source

### Prerequisites

- Rust 1.70+ (`rustup`)
- Git
- OpenSSL development libraries

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev git
```

**Fedora/RHEL:**
```bash
sudo dnf install -y gcc pkg-config openssl-devel git
```

**Arch Linux:**
```bash
sudo pacman -S base-devel openssl git
```

### Build Steps

```bash
# Clone repository
git clone https://github.com/yourusername/eidos
cd eidos

# Build release binary
cargo build --release

# Binary location
ls -lh target/release/eidos

# Install to ~/.local/bin
mkdir -p ~/.local/bin
cp target/release/eidos ~/.local/bin/
```

### Using Makefile

```bash
# Build optimized binary
make build-release

# Install to ~/.local/bin
make install

# Run tests
make test

# Format and lint
make check-all

# Create release package
make release
```

## Configuration

### Config File

Create `~/.config/eidos/eidos.toml`:

```toml
model_path = "/path/to/model.onnx"
tokenizer_path = "/path/to/tokenizer.json"

[model]
max_length = 64
temperature = 0.7
```

### Environment Variables

```bash
# Model paths
export EIDOS_MODEL_PATH=/path/to/model.onnx
export EIDOS_TOKENIZER_PATH=/path/to/tokenizer.json

# Chat API (optional)
export OPENAI_API_KEY=sk-...
export OLLAMA_HOST=http://localhost:11434
```

### Priority

Configuration priority (highest to lowest):
1. Environment variables
2. `./eidos.toml` (current directory)
3. `~/.config/eidos/eidos.toml`
4. Built-in defaults

## Production Deployment

### Systemd Service

Create `/etc/systemd/system/eidos.service`:

```ini
[Unit]
Description=Eidos AI CLI Service
After=network.target

[Service]
Type=simple
User=eidos
Group=eidos
WorkingDirectory=/opt/eidos
Environment="EIDOS_MODEL_PATH=/opt/eidos/models/model.onnx"
Environment="EIDOS_TOKENIZER_PATH=/opt/eidos/models/tokenizer.json"
ExecStart=/usr/local/bin/eidos core "$PROMPT"
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable eidos
sudo systemctl start eidos
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: eidos
spec:
  replicas: 3
  selector:
    matchLabels:
      app: eidos
  template:
    metadata:
      labels:
        app: eidos
    spec:
      containers:
      - name: eidos
        image: eidos:latest
        env:
        - name: EIDOS_MODEL_PATH
          value: /models/model.onnx
        - name: EIDOS_TOKENIZER_PATH
          value: /models/tokenizer.json
        volumeMounts:
        - name: models
          mountPath: /models
          readOnly: true
        resources:
          limits:
            memory: "2Gi"
            cpu: "1000m"
          requests:
            memory: "1Gi"
            cpu: "500m"
      volumes:
      - name: models
        persistentVolumeClaim:
          claimName: eidos-models
```

### Nginx Reverse Proxy

If running as a service with HTTP interface:

```nginx
server {
    listen 80;
    server_name eidos.example.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Resource Requirements

**Minimum:**
- CPU: 1 core
- RAM: 1GB
- Disk: 500MB

**Recommended:**
- CPU: 2 cores
- RAM: 2GB
- Disk: 2GB

**With Quantized Models:**
- RAM: 4-8GB (depending on model size)

### Monitoring

#### Health Checks

```bash
#!/bin/bash
# health-check.sh

if eidos --version &>/dev/null; then
    echo "OK"
    exit 0
else
    echo "FAIL"
    exit 1
fi
```

#### Prometheus Metrics

Add metrics endpoint (future enhancement):

```rust
// Example metrics
eidos_requests_total
eidos_request_duration_seconds
eidos_errors_total
```

### Security Considerations

1. **Run as non-root user**
   ```bash
   sudo useradd -r -s /bin/false eidos
   ```

2. **Restrict file permissions**
   ```bash
   chmod 755 /usr/local/bin/eidos
   chmod 644 /etc/eidos/eidos.toml
   ```

3. **Use secrets management**
   ```bash
   # Never commit API keys
   # Use environment variables or secret vaults
   export OPENAI_API_KEY=$(vault read -field=key secret/eidos/openai)
   ```

4. **Enable command validation**
   - Eidos includes built-in command validation
   - Blocks 60+ dangerous patterns
   - Whitelist mode available in config

5. **Network isolation**
   ```bash
   # Docker: use custom network
   docker network create --driver bridge eidos-net
   ```

## Updating

### Manual Update

```bash
# Pull latest
git pull origin main

# Rebuild
cargo build --release

# Reinstall
sudo make install
```

### Docker Update

```bash
# Pull latest image
docker pull eidos:latest

# Restart container
docker-compose down
docker-compose up -d
```

### Automated Updates

Create update script:

```bash
#!/bin/bash
# update-eidos.sh

set -e

cd /opt/eidos
git pull
cargo build --release
sudo systemctl restart eidos
```

Add to cron:

```cron
0 2 * * * /opt/eidos/update-eidos.sh
```

## Troubleshooting

### Binary not found

```bash
# Add to PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Permission denied

```bash
chmod +x ~/.local/bin/eidos
```

### Model not found

```bash
# Check config
eidos --version

# Set environment variable
export EIDOS_MODEL_PATH=/path/to/model.onnx
```

### Docker: Permission denied

```bash
# Run as current user
docker run --rm -u $(id -u):$(id -g) eidos chat "test"
```

## Uninstallation

### Remove binary

```bash
rm ~/.local/bin/eidos
# or
sudo rm /usr/local/bin/eidos
```

### Remove config

```bash
rm -rf ~/.config/eidos
```

### Remove Docker images

```bash
docker rmi eidos:latest
docker volume rm eidos-config
```

## See Also

- [Installation Script](../install.sh) - Automated installation
- [Makefile](../Makefile) - Build targets
- [Docker Compose](../docker-compose.yml) - Container orchestration
- [Model Guide](MODEL_GUIDE.md) - Model setup
