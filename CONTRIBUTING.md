# Contributing to bevy-ui-builders

Thank you for your interest in contributing to bevy-ui-builders! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork:
```bash
`git clone https://github.com/yourusername/bevy-ui-builders.git`
```
4. Create a new branch:
```bash
`git checkout -b feature/your-feature-name`
```

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/noahsabaj/bevy-ui-builders.git
cd bevy-ui-builders

# Build the project
cargo build --all-features

# Run tests
cargo test --all-features

# Run clippy for linting
cargo clippy --all-features
```

## Coding Standards

### General Rules
- Follow Rust standard naming conventions
- Use `cargo fmt` before committing
- Ensure no clippy warnings with `cargo clippy --all-features`
- Add documentation for all public APIs
- Write tests for new functionality

### Architecture Guidelines

#### Gateway Pattern
Every module MUST follow the gateway pattern:
- `mod.rs` contains ONLY exports, no implementation
- All submodules are private (use `mod` not `pub mod`)
- Access between submodules via `super::`

#### Builder Pattern
When creating new builders:
- Follow existing builder patterns (see ButtonBuilder)
- Return `Entity` from `build()` methods
- Use `ChildSpawnerCommands` not `ChildBuilder` (Bevy 0.16+)
- Provide sensible defaults
- Support method chaining

### Commit Messages
- Use present tense ("Add feature" not "Added feature")
- Keep first line under 72 characters
- Reference issues when applicable (#123)

## Testing

### Running Tests
```bash
# Run all tests
cargo test --all-features

# Test specific feature
cargo test --features button

# Run with verbose output
cargo test --all-features -- --nocapture
```

### Writing Tests
- Add tests in the same file as the code being tested
- Use descriptive test names
- Test edge cases and error conditions

## Adding a New Builder

1. Create the module structure:
```
src/my_builder/
├── mod.rs      # Gateway only
├── builder.rs  # Builder implementation
├── types.rs    # Component types
├── plugin.rs   # Plugin (if needed)
└── systems.rs  # Systems (if needed)
```

2. Follow the gateway pattern in `mod.rs`
3. Add feature flag in `Cargo.toml`
4. Export from `lib.rs` with feature gate
5. Add documentation and examples
6. Write tests

## Pull Request Process

1. Ensure all tests pass: `cargo test --all-features`
2. Run clippy: `cargo clippy --all-features`
3. Format code: `cargo fmt`
4. Update documentation if needed
5. Add your changes to CHANGELOG.md (if it exists)
6. Submit PR with clear description

### PR Requirements
- Clear description of changes
- Tests for new functionality
- No breaking changes without discussion
- Documentation updates where needed
- Clean commit history (squash if needed)

## Code Review

All submissions require review. We aim to:
- Respond within 48 hours
- Provide constructive feedback
- Merge PRs that meet our standards

## Questions?

Feel free to:
- Open an issue for discussion
- Ask questions in PR comments
- Reach out on [Bevy Discord](https://discord.gg/bevy)

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0.
