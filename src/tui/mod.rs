use crate::tmux::{TmuxClient, TmuxSession};
use crate::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use tokio::time::Duration;

/// Application state
pub struct App {
    client: TmuxClient,
    sessions: Vec<TmuxSession>,
    selected: ListState,
    input: String,
    input_mode: InputMode,
    status_message: String,
    attach_on_exit: Option<String>,
}

#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    CreatingSession,
    RenamingSession,
}

impl App {
    pub fn new(client: TmuxClient) -> Self {
        let mut selected = ListState::default();
        selected.select(Some(0));

        Self {
            client,
            sessions: Vec::new(),
            selected,
            input: String::new(),
            input_mode: InputMode::Normal,
            status_message: "Welcome to tmux-ui! Press 'h' for help.".to_string(),
            attach_on_exit: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        // If we need to attach to a session, do it after restoring terminal.
        // This is crucial because tmux attach needs to take over the terminal,
        // which requires that we've fully released our terminal handling first.
        // Attempting to attach while still in alternate screen or raw mode
        // would cause terminal corruption and keyboard input issues.
        if let Some(session_name) = &self.attach_on_exit {
            self.client.attach_session(session_name)?;
        }

        result
    }

    async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        self.refresh_sessions().await?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match self.input_mode {
                            InputMode::Normal => {
                                if self.handle_normal_input(key.code).await? {
                                    break;
                                }
                            }
                            InputMode::CreatingSession => {
                                if self.handle_creating_input(key.code).await? {
                                    break;
                                }
                            }
                            InputMode::RenamingSession => {
                                if self.handle_renaming_input(key.code).await? {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_normal_input(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('h') => {
                self.status_message = "Commands: q=quit, n=new session, d=delete session, a=attach, r=rename, w=new window, x=detach, R=refresh, ↑↓=navigate, Enter=attach".to_string();
            }
            KeyCode::Char('n') => {
                self.input_mode = InputMode::CreatingSession;
                self.input.clear();
                self.status_message =
                    "Enter session name (ESC to cancel, Enter to create):".to_string();
            }
            KeyCode::Char('r') => {
                if let Some(index) = self.selected.selected() {
                    if index < self.sessions.len() {
                        self.input_mode = InputMode::RenamingSession;
                        self.input.clear();
                        self.status_message =
                            "Enter new session name (ESC to cancel, Enter to rename):".to_string();
                    }
                }
            }
            KeyCode::Char('d') => {
                if let Some(index) = self.selected.selected() {
                    if index < self.sessions.len() {
                        let session = &self.sessions[index];
                        match self.client.kill_session(&session.name) {
                            Ok(_) => {
                                self.status_message =
                                    format!("Session '{}' deleted!", session.name);
                                self.refresh_sessions().await?;
                            }
                            Err(e) => {
                                self.status_message = format!("Error deleting session: {}", e);
                            }
                        }
                    }
                }
            }
            KeyCode::Char('a') | KeyCode::Enter => {
                if let Some(index) = self.selected.selected() {
                    if index < self.sessions.len() {
                        let session = &self.sessions[index];
                        
                        // Check if we're already inside a tmux session
                        if self.client.is_inside_tmux() {
                            // Use switch-client to change to the selected session
                            // This works within tmux and doesn't require exiting the TUI
                            match self.client.switch_client(&session.name) {
                                Ok(_) => {
                                    self.status_message = format!("Switched to session '{}'", session.name);
                                    self.refresh_sessions().await?;
                                }
                                Err(e) => {
                                    self.status_message = format!("Error switching to session: {}", e);
                                }
                            }
                        } else {
                            // Not inside tmux, use attach-session
                            // Store the session to attach to after TUI exits
                            self.attach_on_exit = Some(session.name.clone());
                            self.status_message = format!("Attaching to session '{}'...", session.name);
                            // Return true to exit TUI, then attach
                            return Ok(true);
                        }
                    }
                }
            }
            KeyCode::Char('x') => {
                if let Some(index) = self.selected.selected() {
                    if index < self.sessions.len() {
                        let session = &self.sessions[index];
                        match self.client.detach_session(&session.name) {
                            Ok(_) => {
                                self.status_message =
                                    format!("Detached from session '{}'", session.name);
                                self.refresh_sessions().await?;
                            }
                            Err(e) => {
                                self.status_message = format!("Error detaching: {}", e);
                            }
                        }
                    }
                }
            }
            KeyCode::Char('w') => {
                if let Some(index) = self.selected.selected() {
                    if index < self.sessions.len() {
                        let session = &self.sessions[index];
                        match self.client.create_window(&session.name, None) {
                            Ok(_) => {
                                self.status_message =
                                    format!("New window created in session '{}'", session.name);
                                self.refresh_sessions().await?;
                            }
                            Err(e) => {
                                self.status_message = format!("Error creating window: {}", e);
                            }
                        }
                    }
                }
            }
            KeyCode::Down => {
                let i = match self.selected.selected() {
                    Some(i) => {
                        if i >= self.sessions.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.selected.select(Some(i));
            }
            KeyCode::Up => {
                let i = match self.selected.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.sessions.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.selected.select(Some(i));
            }
            KeyCode::Char('R') => {
                self.refresh_sessions().await?;
                self.status_message = "Sessions refreshed!".to_string();
            }
            _ => {}
        }
        Ok(false)
    }

    async fn handle_creating_input(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    let session_name = self.input.trim().to_string();
                    match self.client.create_session(&session_name) {
                        Ok(_) => {
                            self.status_message = format!("Session '{}' created!", session_name);
                            self.input.clear();
                            self.input_mode = InputMode::Normal;
                            self.refresh_sessions().await?;
                        }
                        Err(e) => {
                            self.status_message = format!("Error creating session: {}", e);
                            self.input_mode = InputMode::Normal;
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Esc => {
                self.input.clear();
                self.input_mode = InputMode::Normal;
                self.status_message = "Cancelled".to_string();
            }
            _ => {}
        }
        Ok(false)
    }

    async fn handle_renaming_input(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    if let Some(index) = self.selected.selected() {
                        if index < self.sessions.len() {
                            let old_name = self.sessions[index].name.clone();
                            let new_name = self.input.trim().to_string();
                            match self.client.rename_session(&old_name, &new_name) {
                                Ok(_) => {
                                    self.status_message = format!(
                                        "Session renamed from '{}' to '{}'!",
                                        old_name, new_name
                                    );
                                    self.input.clear();
                                    self.input_mode = InputMode::Normal;
                                    self.refresh_sessions().await?;
                                }
                                Err(e) => {
                                    self.status_message = format!("Error renaming session: {}", e);
                                    self.input_mode = InputMode::Normal;
                                }
                            }
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Esc => {
                self.input.clear();
                self.input_mode = InputMode::Normal;
                self.status_message = "Cancelled".to_string();
            }
            _ => {}
        }
        Ok(false)
    }

    async fn refresh_sessions(&mut self) -> Result<()> {
        self.sessions = self.client.list_sessions()?;

        // Adjust selection if needed
        if self.sessions.is_empty() {
            self.selected.select(None);
        } else if let Some(selected) = self.selected.selected() {
            if selected >= self.sessions.len() {
                self.selected.select(Some(self.sessions.len() - 1));
            }
        } else {
            self.selected.select(Some(0));
        }

        Ok(())
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        // Title
        let title = Paragraph::new("🖥️  tmux-ui - Session Manager")
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Session list
        let sessions: Vec<ListItem> = self
            .sessions
            .iter()
            .map(|session| {
                let attached_indicator = if session.attached { "●" } else { "○" };
                let style = if session.attached {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let content = format!(
                    "{} {} ({} windows)",
                    attached_indicator, session.name, session.windows
                );
                ListItem::new(content).style(style)
            })
            .collect();

        let sessions_list = List::new(sessions)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("tmux Sessions ({})", self.sessions.len())),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(sessions_list, chunks[1], &mut self.selected);

        // Status/Input bar
        let status_text = match self.input_mode {
            InputMode::Normal => self.status_message.clone(),
            InputMode::CreatingSession => format!("New session name: {}", self.input),
            InputMode::RenamingSession => format!("Rename to: {}", self.input),
        };

        let status = Paragraph::new(status_text)
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                _ => Style::default().fg(Color::Yellow),
            })
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Status"));

        f.render_widget(status, chunks[2]);
    }
}
