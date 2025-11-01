use anyhow::{Context, Result};
use std::env;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct TmuxSession {
    pub name: String,
    pub windows: usize,
    pub attached: bool,
    pub created: String,
}

#[derive(Debug, Clone)]
pub struct TmuxWindow {
    pub id: String,
    pub name: String,
    pub panes: usize,
    pub active: bool,
}

pub struct TmuxClient;

impl TmuxClient {
    pub fn new() -> Self {
        Self
    }

    /// List all tmux sessions
    pub fn list_sessions(&self) -> Result<Vec<TmuxSession>> {
        let output = Command::new("tmux")
            .args([
                "list-sessions",
                "-F",
                "#{session_name}|#{session_windows}|#{session_attached}|#{session_created}",
            ])
            .output()
            .context("Failed to execute tmux list-sessions")?;

        if !output.status.success() {
            // No sessions running
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut sessions = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                // Parse window count, defaulting to 1 if parsing fails
                // This maintains backwards compatibility if tmux format changes
                let windows = parts[1].parse().unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to parse window count '{}': {}", parts[1], e);
                    1
                });
                
                sessions.push(TmuxSession {
                    name: parts[0].to_string(),
                    windows,
                    attached: parts[2] != "0",
                    created: parts[3].to_string(),
                });
            }
        }

        Ok(sessions)
    }

    /// Create a new tmux session
    pub fn create_session(&self, name: &str) -> Result<()> {
        let status = Command::new("tmux")
            .args(["new-session", "-d", "-s", name])
            .status()
            .context("Failed to create tmux session")?;

        if !status.success() {
            anyhow::bail!("Failed to create session: {}", name);
        }

        Ok(())
    }

    /// Kill a tmux session
    pub fn kill_session(&self, name: &str) -> Result<()> {
        let status = Command::new("tmux")
            .args(["kill-session", "-t", name])
            .status()
            .context("Failed to kill tmux session")?;

        if !status.success() {
            anyhow::bail!("Failed to kill session: {}", name);
        }

        Ok(())
    }

    /// Check if currently running inside a tmux session
    pub fn is_inside_tmux(&self) -> bool {
        env::var("TMUX").is_ok()
    }

    /// Switch to a different tmux session (when already inside tmux)
    pub fn switch_client(&self, name: &str) -> Result<()> {
        let status = Command::new("tmux")
            .args(["switch-client", "-t", name])
            .status()
            .context("Failed to switch tmux client")?;

        if !status.success() {
            anyhow::bail!("Failed to switch to session: {}", name);
        }

        Ok(())
    }

    /// Attach to a tmux session
    pub fn attach_session(&self, name: &str) -> Result<()> {
        let status = Command::new("tmux")
            .args(["attach-session", "-t", name])
            .status()
            .context("Failed to attach to tmux session")?;

        if !status.success() {
            anyhow::bail!("Failed to attach to session: {}", name);
        }

        Ok(())
    }

    /// List windows in a session
    pub fn list_windows(&self, session: &str) -> Result<Vec<TmuxWindow>> {
        let output = Command::new("tmux")
            .args([
                "list-windows",
                "-t",
                session,
                "-F",
                "#{window_id}|#{window_name}|#{window_panes}|#{window_active}",
            ])
            .output()
            .context("Failed to execute tmux list-windows")?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut windows = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                // Parse pane count, defaulting to 1 if parsing fails
                // This maintains backwards compatibility if tmux format changes
                let panes = parts[2].parse().unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to parse pane count '{}': {}", parts[2], e);
                    1
                });
                
                windows.push(TmuxWindow {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    panes,
                    active: parts[3] == "1",
                });
            }
        }

        Ok(windows)
    }

    /// Create a new window in a session
    pub fn create_window(&self, session: &str, name: Option<&str>) -> Result<()> {
        let mut args = vec!["new-window", "-t", session];
        if let Some(n) = name {
            args.push("-n");
            args.push(n);
        }

        let status = Command::new("tmux")
            .args(&args)
            .status()
            .context("Failed to create tmux window")?;

        if !status.success() {
            anyhow::bail!("Failed to create window in session: {}", session);
        }

        Ok(())
    }

    /// Kill a window
    pub fn kill_window(&self, target: &str) -> Result<()> {
        let status = Command::new("tmux")
            .args(["kill-window", "-t", target])
            .status()
            .context("Failed to kill tmux window")?;

        if !status.success() {
            anyhow::bail!("Failed to kill window: {}", target);
        }

        Ok(())
    }

    /// Rename a session
    pub fn rename_session(&self, old_name: &str, new_name: &str) -> Result<()> {
        let status = Command::new("tmux")
            .args(["rename-session", "-t", old_name, new_name])
            .status()
            .context("Failed to rename tmux session")?;

        if !status.success() {
            anyhow::bail!("Failed to rename session from {} to {}", old_name, new_name);
        }

        Ok(())
    }

    /// Detach all clients from a session
    pub fn detach_session(&self, name: &str) -> Result<()> {
        // Detach all clients from the session
        // This may fail if no clients are attached, which is not an error
        let _result = Command::new("tmux")
            .args(["detach-client", "-s", name])
            .status();

        // Always return Ok since detaching from a session with no attached clients
        // is not an error condition
        Ok(())
    }
}

impl Default for TmuxClient {
    fn default() -> Self {
        Self::new()
    }
}
