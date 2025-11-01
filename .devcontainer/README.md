# Development Container Configuration

This directory contains the GitHub Codespaces / VS Code Dev Container configuration for the tmux-ui project.

## What's Included

- **Rust Development Environment**: Latest Rust toolchain with common development tools
- **tmux**: Installed and ready to use for testing the TUI
- **VS Code Extensions**:
  - rust-analyzer: Intelligent Rust language support
  - CodeLLDB: Debugging support
  - Even Better TOML: Better Cargo.toml editing
  - crates: Dependency version management
- **Pre-configured Settings**: Format on save, clippy integration

## Quick Start

### Using GitHub Codespaces

1. Click the "Code" button on the repository
2. Select "Codespaces" tab
3. Click "Create codespace on main" (or your branch)
4. Wait for the environment to build (first time takes ~2-3 minutes)
5. Once ready, run `cargo run` to start the TUI

### Using VS Code Locally

1. Install the "Dev Containers" extension in VS Code
2. Open the repository folder
3. Press `F1` and select "Dev Containers: Reopen in Container"
4. Wait for the container to build
5. Start coding!

## What Happens on First Launch

The `post-create.sh` script automatically:
- Installs tmux and system dependencies
- Installs Rust toolchain components (clippy, rustfmt, rust-src)
- Builds the project to cache dependencies
- Runs tests to verify everything works

## Manual Setup

If you need to run setup manually:

```bash
bash .devcontainer/post-create.sh
```

## Customization

Edit `devcontainer.json` to:
- Add more VS Code extensions
- Change container settings
- Add additional features
- Modify build commands

## Troubleshooting

**Container won't build**: Make sure Docker is running and you have internet access.

**tmux not found**: Run `sudo apt-get update && sudo apt-get install -y tmux` manually.

**Rust analyzer not working**: Try reloading the VS Code window (`F1` -> "Developer: Reload Window").
