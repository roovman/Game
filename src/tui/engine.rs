use std::io::{stdout};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use color_eyre::Result;

use crate::state::application_state::{ApplicationState, AppState};
// Make sure to import GameMode if not impli
use super::input::handle_input;
use super::draw::ui;

pub fn run() -> Result<()> {
    let mut app = ApplicationState::new();
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;

    while !matches!(app.state, AppState::Exiting) {
        
        // 1. INPUT PHASE
        if let Some(action) = handle_input(&app)? {
            app.apply_action(action);
        }

        // 2. UPDATE PHASE (
        if let AppState::Game(mode) = &mut app.state {
            mode.tick();
        }

        // 3. RENDER PHASE
        terminal.draw(|f| ui(f, &app))?;
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}