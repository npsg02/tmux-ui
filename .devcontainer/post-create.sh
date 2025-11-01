#!/bin/bash
set -e

echo "🚀 Setting up tmux-ui development environment..."

# Install tmux
echo "📦 Installing tmux..."
sudo apt-get update
sudo apt-get install -y tmux

# Install Rust tools
echo "🦀 Installing Rust development tools..."
rustup component add clippy rustfmt rust-src

# Build the project to cache dependencies
echo "🔨 Building project and caching dependencies..."
cargo build

# Run tests to verify setup
echo "✅ Running tests to verify setup..."
cargo test

echo "✨ Development environment ready!"
echo ""
echo "Quick start commands:"
echo "  cargo run           - Run the TUI"
echo "  cargo test          - Run tests"
echo "  cargo clippy        - Run linter"
echo "  cargo fmt           - Format code"
echo ""
