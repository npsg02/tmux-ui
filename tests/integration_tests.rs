use tmux_ui::tmux::{TmuxClient, TmuxSession};

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


