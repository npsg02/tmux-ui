# tmux-ui

A terminal user interface (TUI) for managing tmux sessions, windows, and panes.

## Features

- 🖥️ Interactive Terminal User Interface (TUI) for tmux
- 📋 View and manage tmux sessions
- 🪟 Create and delete windows
- 🎯 Quick session switching
- 🔧 Command Line Interface (CLI) for scripting
- 🚀 Fast and lightweight

## Prerequisites

- tmux (version 2.0 or later)
- Rust 1.70 or later (for building from source)

## Installation

### Using GitHub Codespaces

The easiest way to try tmux-ui is with GitHub Codespaces:

1. Click the "Code" button on the repository
2. Select "Codespaces" tab
3. Click "Create codespace on main"
4. Wait for the environment to set up (~2-3 minutes)
5. Run `cargo run` to start the TUI

The development environment includes tmux and all necessary dependencies pre-installed. See [.devcontainer/README.md](.devcontainer/README.md) for more details.

### From Source

```bash
git clone https://github.com/npsg02/tmux-ui.git
cd tmux-ui
cargo build --release
```

The binary will be available at `./target/release/tmux-ui`

## Usage

### Interactive TUI (default)

Start the interactive terminal user interface:

```bash
tmux-ui
# or explicitly:
tmux-ui tui
```

#### TUI Keybindings:
- `h` - Show help
- `n` - Create new session
- `d` - Delete selected session
- `r` - Rename selected session
- `a` or `Enter` - Attach to selected session
- `x` - Detach from selected session
- `w` - Create new window in selected session
- `R` - Refresh session list
- `↑↓` - Navigate sessions
- `q` - Quit application

### Command Line Interface

```bash
# List all tmux sessions
tmux-ui list

# Create a new tmux session
tmux-ui new my-session

# Kill a tmux session
tmux-ui kill my-session

# Attach to a tmux session
tmux-ui attach my-session

# Show help
tmux-ui --help
```

## Project Structure

```
tmux-ui/
├── src/
│   ├── tmux.rs           # tmux client and data structures
│   ├── tui/              # Terminal UI implementation
│   ├── lib.rs            # Library root
│   └── main.rs           # CLI application
├── tests/                # Integration tests
└── examples/             # Usage examples
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Clippy (Linter)

```bash
cargo clippy -- -D warnings
```

### Formatting Code

```bash
cargo fmt
```

## How It Works

tmux-ui interacts with tmux through its command-line interface, parsing the output of commands like:
- `tmux list-sessions` - to get session information
- `tmux new-session` - to create new sessions
- `tmux kill-session` - to delete sessions
- And more tmux commands for window and pane management

The TUI is built using [ratatui](https://github.com/ratatui-org/ratatui), a modern terminal UI library for Rust.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
