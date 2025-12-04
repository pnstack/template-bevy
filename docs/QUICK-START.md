# Quick Start Guide

Choose your preferred development method and get started in minutes!

## 🚀 Fastest Methods

### GitHub Codespaces (Zero Setup!)
1. Click "Code" → "Codespaces" → "Create codespace on main"
2. Wait ~2 minutes for setup
3. Start coding!

**Best for:** Trying the project, contributing without local setup

---

### Nix (Reproducible)
```bash
git clone https://github.com/pnstack/template-bevy.git
cd template-bevy
nix develop
cargo run --features dev
```

**Best for:** Guaranteed reproducible builds, NixOS users

---

### Local Development (Traditional)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/pnstack/template-bevy.git
cd template-bevy
cargo build --release
```

**Best for:** Full control, native performance

---

## 📋 Common Commands

### Using Make (Recommended)
```bash
make help          # Show all available commands
make build         # Build debug version
make build-dev     # Build with dynamic linking (faster)
make run           # Run the game
make test          # Run tests
make clippy        # Run linter
make fmt           # Format code
make all           # Format, lint, test, and build
```

### Using Cargo
```bash
cargo build                      # Debug build
cargo build --features dev       # Fast dev build with dynamic linking
cargo build --release            # Optimized build
cargo test                       # Run tests
cargo clippy -- -D warnings      # Lint
cargo fmt                        # Format
cargo run --features dev         # Run the game
```

---

## 🎯 First Steps After Setup

1. **Run the game:**
   ```bash
   cargo run --features dev
   ```

2. **Controls:**
   - **WASD** or **Arrow Keys** - Move player
   - **ESC** - Quit game

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Check the example:**
   ```bash
   cargo run --example basic_usage --features dev
   ```

---

## 🔍 Need More Details?

- **Full setup instructions:** [SETUP.md](../SETUP.md)
- **Usage examples:** [README.md](../README.md)
- **CI/CD info:** [CI-CD.md](CI-CD.md)

---

## ❓ Troubleshooting

### "Missing audio libraries" (Linux)
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev libudev-dev

# Fedora
sudo dnf install alsa-lib-devel systemd-devel

# Nix/Codespaces - Already included!
```

### "Slow first build"
This is normal! Bevy has many dependencies that need to compile on first build.
Subsequent builds are much faster (seconds instead of minutes).

**Tips for faster iteration:**
- Use `--features dev` for dynamic linking
- Use `cargo check` for quick syntax checks
- Use `cargo watch` for automatic rebuilds

---

## 🎓 Learning Resources

### Bevy
- [Bevy Book](https://bevyengine.org/learn/book/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Bevy Examples](https://bevyengine.org/examples/)

### Rust Basics
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Project-Specific
- Explore [src/](../src/) for code organization
- Check [tests/](../tests/) for testing examples
- Read [examples/](../examples/) for usage patterns

---

## 💡 Tips

1. **Use `--features dev`**: Dynamic linking speeds up compile times dramatically
2. **Use rust-analyzer**: Best IDE support for Rust
3. **Run clippy often**: Catches bugs early
4. **Format before commit**: `cargo fmt` or `make fmt`
5. **Test frequently**: `cargo test` or `make test`

---

## 🤝 Contributing

Ready to contribute?

1. Fork the repository
2. Create a branch: `git checkout -b feature/my-feature`
3. Make changes
4. Test: `make all`
5. Commit: `git commit -m "Add feature"`
6. Push: `git push origin feature/my-feature`
7. Open a Pull Request

---

## 📞 Getting Help

- 📖 Read the [full docs](../README.md)
- 🐛 [Open an issue](https://github.com/pnstack/template-bevy/issues)
- 💬 Check [existing issues](https://github.com/pnstack/template-bevy/issues?q=is%3Aissue)

---

**Happy game development! 🦀🎮**
