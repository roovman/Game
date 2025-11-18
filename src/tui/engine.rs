// src/lib/tui/engine.rs
use std::io::{stdout, Stdout};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use color_eyre::Result;

use crate::state::GameState;
use super::input::handle_events;
use super::draw::ui;

type TuiBackend = CrosstermBackend<Stdout>;

fn setup_tui(terminal: &mut Terminal<TuiBackend>) -> Result<()> {
    enable_raw_mode()?;
    // Вмикаємо alternate screen та захоплення миші (важливо!)
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        crossterm::event::EnableMouseCapture,
    )?;
    Ok(())
}

fn restore_tui(terminal: &mut Terminal<TuiBackend>) -> Result<()> {
    // Відновлюємо термінал та вимикаємо захоплення миші
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture,
    )?;
    disable_raw_mode()?;
    Ok(())
}

pub fn run() -> Result<()> {
    let mut game_state = GameState::new();
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    setup_tui(&mut terminal)?;

    while game_state.is_running {
        // 1. Малювання
        terminal.draw(|f| ui::<TuiBackend>(f, &game_state))?;
        
        // 2. Обробка подій та оновлення стану
        if let Some(action) = handle_events(&game_state)? {
            game_state.apply_action(action);
        }
    }

    restore_tui(&mut terminal)?;
    Ok(())
}