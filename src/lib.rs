//! tmux-ui - Terminal User Interface for tmux
//!
//! This is a TUI application for managing tmux sessions, windows, and panes.

pub mod tmux;
pub mod tui;

pub use tmux::*;

/// Application result type
pub type Result<T> = anyhow::Result<T>;
