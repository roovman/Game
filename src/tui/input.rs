use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use std::time::Duration;
use crate::state::actions::{Action, MenuSelection};
use crate::state::application_state::{ApplicationState, AppState};
use crate::map::position::MapPosition;
use color_eyre::Result;

pub fn handle_input(app: &ApplicationState) -> Result<Option<Action>> {
    if event::poll(Duration::from_millis(16))? {
        let event = event::read()?;
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    match app.state {
                        AppState::Menu => return Ok(Some(Action::QuitApp)),
                        _ => return Ok(Some(Action::BackToMenu)),
                    }
                },
                // Menu
                KeyCode::Char('1') if matches!(app.state, AppState::Menu) => 
                    return Ok(Some(Action::MenuSelect(MenuSelection::EnterBuildMode))),
                KeyCode::Char('2') if matches!(app.state, AppState::Menu) => 
                    return Ok(Some(Action::MenuSelect(MenuSelection::EnterPlayMode))),
                KeyCode::Char('3') if matches!(app.state, AppState::Menu) => 
                    return Ok(Some(Action::MenuSelect(MenuSelection::LoadLatest))), // ⭐️ Load

                // Editor
                KeyCode::Tab if matches!(app.state, AppState::Editor(_)) => 
                    return Ok(Some(Action::CycleBuildTool)),
                KeyCode::Char('s') if matches!(app.state, AppState::Editor(_)) => 
                    return Ok(Some(Action::SaveMap)), // ⭐️ Save

                _ => {}
            },
            
            Event::Mouse(mouse) => {
                let x = mouse.column as i32;
                let y = mouse.row as i32;
                let map_pos = MapPosition::new(x - 1, y - 1); 
                
                if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                    match app.state {
                        AppState::Editor(_) => return Ok(Some(Action::EditorClick { pos: map_pos })),
                        AppState::Simulation(_) => return Ok(Some(Action::GameClick { pos: map_pos })),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}