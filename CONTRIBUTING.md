# Contributing to RTasks

Thank you for your interest in contributing to RTasks! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a new branch for your feature or bugfix
4. Make your changes
5. Test your changes thoroughly
6. Submit a pull request

## Development Setup

1. Make sure you have Rust installed (https://rustup.rs/)
2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rtasks.git
   cd rtasks
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the project:
   ```bash
   cargo run
   ```

## Building

- **Debug build**: `cargo build`
- **Release build**: `cargo build --release`
- **Run**: `cargo run`
- **Test**: `cargo test`

## Code Style

- Follow standard Rust formatting guidelines
- Use `cargo fmt` to format your code
- Use `cargo clippy` to check for common issues
- Write clear, descriptive commit messages

## Submitting Changes

1. Make sure your code builds without warnings
2. Test your changes with both interactive and command-line modes
3. Update documentation if needed
4. Update the README.md if you add new features
5. Submit a pull request with a clear description of your changes

## Bug Reports

When reporting bugs, please include:
- Your operating system
- Rust version (`rustc --version`)
- Steps to reproduce the issue
- Expected vs actual behavior

## Feature Requests

We welcome feature requests! Please:
- Check if the feature already exists or is planned
- Describe the use case clearly
- Provide examples of how it would work

## Questions

If you have questions about the codebase or need help getting started, feel free to open an issue with the "question" label.

Thank you for contributing! 🦀
