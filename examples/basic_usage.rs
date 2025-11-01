use tmux_ui::tmux::TmuxClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tmux client
    let client = TmuxClient::new();

    // Create some test sessions
    println!("Creating test sessions...");
    client.create_session("example-1")?;
    client.create_session("example-2")?;
    client.create_session("example-3")?;

    // List all sessions
    println!("\nAll tmux sessions:");
    let sessions = client.list_sessions()?;
    for session in &sessions {
        let attached = if session.attached { "●" } else { "○" };
        println!(
            "  {} {} - {} window(s)",
            attached, session.name, session.windows
        );
    }

    // Create a window in the first session
    println!("\nCreating a new window in example-1...");
    client.create_window("example-1", Some("test-window"))?;

    // List sessions again to see the window count increase
    println!("\nSessions after creating a window:");
    let updated_sessions = client.list_sessions()?;
    for session in &updated_sessions {
        let attached = if session.attached { "●" } else { "○" };
        println!(
            "  {} {} - {} window(s)",
            attached, session.name, session.windows
        );
    }

    // Clean up - kill the test sessions
    println!("\nCleaning up test sessions...");
    client.kill_session("example-1")?;
    client.kill_session("example-2")?;
    client.kill_session("example-3")?;

    println!("Done!");

    Ok(())
}

