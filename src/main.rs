use clap::{Parser, Subcommand};
use tmux_ui::{tmux::TmuxClient, tui::App};

/// A terminal user interface for managing tmux sessions
#[derive(Parser)]
#[command(name = "tmux-ui")]
#[command(about = "A TUI for managing tmux sessions, windows, and panes")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the interactive TUI (default)
    Tui,
    /// List all tmux sessions
    List,
    /// Create a new tmux session
    New {
        /// Session name
        name: String,
    },
    /// Kill a tmux session
    Kill {
        /// Session name
        name: String,
    },
    /// Attach to a tmux session
    Attach {
        /// Session name
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = TmuxClient::new();

    match cli.command {
        Some(Commands::Tui) | None => {
            // Default to TUI mode
            let mut app = App::new(client);
            app.run().await?;
        }
        Some(Commands::List) => {
            let sessions = client.list_sessions()?;
            if sessions.is_empty() {
                println!("No tmux sessions found.");
            } else {
                println!("tmux sessions:");
                for session in sessions {
                    let attached = if session.attached { "●" } else { "○" };
                    println!(
                        "  {} {} - {} window(s)",
                        attached, session.name, session.windows
                    );
                }
            }
        }
        Some(Commands::New { name }) => {
            client.create_session(&name)?;
            println!("Session '{}' created.", name);
        }
        Some(Commands::Kill { name }) => {
            client.kill_session(&name)?;
            println!("Session '{}' killed.", name);
        }
        Some(Commands::Attach { name }) => {
            client.attach_session(&name)?;
        }
    }

    Ok(())
}
