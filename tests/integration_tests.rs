use tmux_ui::tmux::{TmuxClient, TmuxSession};
use std::env;

#[test]
fn test_tmux_client_creation() {
    let _client = TmuxClient::new();
    // Just verify we can create a client without panicking
    // TmuxClient is a unit struct, so size might be 0
}

#[test]
fn test_session_struct() {
    let session = TmuxSession {
        name: "test-session".to_string(),
        windows: 2,
        attached: true,
        created: "1234567890".to_string(),
    };

    assert_eq!(session.name, "test-session");
    assert_eq!(session.windows, 2);
    assert!(session.attached);
    assert_eq!(session.created, "1234567890");
}

#[test]
fn test_list_sessions_when_none_exist() {
    // This test assumes tmux might not have sessions
    // It should not panic
    let client = TmuxClient::new();
    let result = client.list_sessions();
    // Should either return empty vec or sessions if they exist
    assert!(result.is_ok());
}

#[test]
fn test_is_inside_tmux() {
    let client = TmuxClient::new();
    
    // Save current TMUX env var
    let original = env::var("TMUX").ok();
    
    // Test when TMUX is not set
    env::remove_var("TMUX");
    assert!(!client.is_inside_tmux());
    
    // Test when TMUX is set
    env::set_var("TMUX", "/tmp/tmux-1000/default,1234,0");
    assert!(client.is_inside_tmux());
    
    // Restore original TMUX env var
    if let Some(val) = original {
        env::set_var("TMUX", val);
    } else {
        env::remove_var("TMUX");
    }
}

