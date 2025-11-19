use std::io::{stdout};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use color_eyre::Result;

use crate::state::application_state::{ApplicationState, AppState};
use super::input::handle_input;
use super::draw::app_ui;


pub fn run() -> Result<()> {
    let mut app = ApplicationState::new();
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;

    while !matches!(app.state, AppState::Exiting) {
        terminal.draw(|f| app_ui(f, &app))?;
        if let Some(action) = handle_input(&app)? {
            app.apply_action(action);
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}