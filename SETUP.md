# Setup Guide

This guide will help you set up the development environment for template-bevy using various methods.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Setup Methods](#setup-methods)
  - [Local Development](#local-development)
  - [Docker](#docker)
  - [Nix](#nix)
  - [GitHub Codespaces / Devcontainer](#github-codespaces--devcontainer)
- [Building the Project](#building-the-project)
- [Running Tests](#running-tests)
- [Common Issues](#common-issues)

## Prerequisites

Choose one of the following setup methods based on your preference:

- **Local Development**: Rust 1.70+, platform-specific dependencies
- **Docker**: Docker 20.10+ and Docker Compose (optional)
- **Nix**: Nix package manager with flakes enabled
- **Codespaces**: GitHub account (no local setup required)

## Quick Start

The fastest way to get started depends on your environment:

```bash
# Local development (with dynamic linking for faster builds)
cargo run --features dev

# Docker
docker compose up

# Nix (with flakes)
nix develop
cargo run

# GitHub Codespaces
# Just open the repository in Codespaces - everything is preconfigured!
```

## Setup Methods

### Local Development

#### Installation

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Install platform-specific dependencies**:
   
   - **Ubuntu/Debian**:
     ```bash
     sudo apt-get update
     sudo apt-get install pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
     ```
   
   - **Fedora**:
     ```bash
     sudo dnf install pkgconfig alsa-lib-devel systemd-devel
     ```
   
   - **macOS**:
     No additional dependencies needed.
   
   - **Windows**:
     No additional dependencies needed.

3. **Clone and build**:
   ```bash
   git clone https://github.com/pnstack/template-bevy.git
   cd template-bevy
   cargo build --release
   ```

4. **Run the game**:
   ```bash
   # Development build (faster compilation)
   cargo run --features dev
   
   # Release build (optimized)
   cargo run --release
   ```

#### Development Tools

Install additional development tools:

```bash
# Format checker
rustup component add rustfmt

# Linter
rustup component add clippy

# IDE support
rustup component add rust-analyzer
```

### Docker

#### Building and Running

1. **Build the Docker image**:
   ```bash
   docker build -t template-bevy:latest .
   ```

2. **Run the container**:
   ```bash
   docker run --rm template-bevy:latest --help
   ```

### Nix

Nix provides reproducible development environments across different systems.

#### Using Nix Flakes (Recommended)

1. **Enable flakes** (if not already enabled):
   ```bash
   mkdir -p ~/.config/nix
   echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
   ```

2. **Enter development environment**:
   ```bash
   nix develop
   ```

   This provides:
   - Rust toolchain with rust-analyzer
   - All build dependencies
   - Development tools

3. **Build the project**:
   ```bash
   nix build
   ```

4. **Run the game**:
   ```bash
   nix run
   ```

#### Using Traditional Nix Shell

If you prefer not to use flakes:

```bash
nix-shell
```

This provides the same development environment using `shell.nix`.

### GitHub Codespaces / Devcontainer

The easiest way to get started with zero local setup.

#### Using GitHub Codespaces

1. **Open in Codespaces**:
   - Navigate to the repository on GitHub
   - Click "Code" → "Codespaces" → "Create codespace on main"
   - Wait for the environment to build (first time only)

2. **Start developing**:
   - All tools are pre-installed
   - VS Code with Rust extensions ready
   - Project automatically builds on creation

#### Using Local Devcontainer

If you have Docker and VS Code locally:

1. **Install prerequisites**:
   - Docker Desktop
   - VS Code
   - "Dev Containers" extension for VS Code

2. **Open in container**:
   - Open the repository in VS Code
   - Press `F1` → "Dev Containers: Reopen in Container"
   - Wait for container to build

3. **Start developing**:
   - Integrated terminal has all tools
   - Extensions auto-installed
   - Same environment as Codespaces

## Building the Project

### Development Build (Faster Compilation)

```bash
cargo build --features dev
```

### Release Build (Optimized)

```bash
cargo build --release
```

The binary will be in `target/release/template-bevy`.

### Checking Code Without Building

```bash
cargo check
```

## Running Tests

### All Tests

```bash
cargo test
```

### Specific Test

```bash
cargo test test_name
```

### With Output

```bash
cargo test -- --nocapture
```

### Integration Tests Only

```bash
cargo test --test integration_tests
```

## Code Quality

### Format Code

```bash
cargo fmt
```

### Check Formatting

```bash
cargo fmt --check
```

### Run Linter

```bash
cargo clippy
```

### Run Linter (Strict)

```bash
cargo clippy -- -D warnings
```

## Common Issues

### Issue: Missing audio libraries (Linux)

**Solution**: Install ALSA development libraries

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel
```

### Issue: Missing udev libraries (Linux)

**Solution**: Install udev development libraries

```bash
# Ubuntu/Debian
sudo apt-get install libudev-dev

# Fedora
sudo dnf install systemd-devel
```

### Issue: Slow compilation times

**Solution**: Use dynamic linking during development

```bash
cargo run --features dev
```

### Issue: Bevy takes a long time to compile

**Solution**: This is normal for the first build - Bevy has many dependencies. Subsequent builds are much faster due to incremental compilation.

Tips for faster iteration:
- Use `cargo check` instead of `cargo build` when just checking for errors
- Use `--features dev` for dynamic linking
- Consider using `cargo watch` for automatic rebuilds

## Environment Variables

The game supports the following environment variables:

- `RUST_BACKTRACE`: Set to `1` or `full` for detailed error traces
- `RUST_LOG`: Set log level (e.g., `debug`, `info`, `warn`, `error`)

Example:

```bash
export RUST_BACKTRACE=1
export RUST_LOG=info
cargo run
```

## Next Steps

After setting up your environment:

1. Read the [README.md](README.md) for usage instructions
2. Explore the [examples/](examples/) directory
3. Check the [tests/](tests/) directory for test examples
4. Review the [src/](src/) directory structure for the game architecture

## Getting Help

If you encounter issues:

1. Check this guide's [Common Issues](#common-issues) section
2. Search existing [GitHub Issues](https://github.com/pnstack/template-bevy/issues)
3. Open a new issue with:
   - Your setup method (Local/Docker/Nix/Codespaces)
   - Operating system and version
   - Rust version (`rustc --version`)
   - Complete error message
   - Steps to reproduce

## Contributing

See [README.md](README.md#contributing) for contribution guidelines.
