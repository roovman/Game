// src/tui/input.rs

use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use crossterm::terminal; // NEW IMPORT: Required to get terminal size
use std::time::Duration;
use crate::state::actions::{Action, MenuSelection};
use crate::state::application_state::{ApplicationState, AppState};
use crate::map::position::MapPosition;
use color_eyre::Result;
use ratatui::layout::Rect; 
use crate::tui::draw::get_menu_rect;

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
                    return Ok(Some(Action::MenuSelect(MenuSelection::LoadLatest))),

                // Editor
                KeyCode::Tab if matches!(app.state, AppState::Editor(_)) => 
                    return Ok(Some(Action::CycleBuildTool)),
                KeyCode::Char('s') if matches!(app.state, AppState::Editor(_)) => 
                    return Ok(Some(Action::SaveMap)), 

                _ => {}
            },
            
            Event::Mouse(mouse) => {
                let x = mouse.column as i32;
                let y = mouse.row as i32;
                let (term_width, term_height) = terminal::size()?;

                // Handle Menu clicks first, as they are full-width
                if matches!(app.state, AppState::Menu) {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        
                        // 1. Reconstruct the full screen area
                        let screen_area = Rect::new(0, 0, term_width, term_height);
                        
                        // 2. Get the EXACT Rect where the menu was drawn
                        let menu_rect = get_menu_rect(screen_area);
                        
                        // 3. Check if click is inside the menu box horizontally
                        // (Optional, but good for precision)
                        if x < menu_rect.x as i32 || x > (menu_rect.x + menu_rect.width) as i32 {
                            return Ok(None);
                        }

                        
                        let relative_y = y - menu_rect.y as i32;

                        match relative_y {
                            3 => return Ok(Some(Action::MenuSelect(MenuSelection::EnterBuildMode))),
                            4 => return Ok(Some(Action::MenuSelect(MenuSelection::EnterPlayMode))),
                            5 => return Ok(Some(Action::MenuSelect(MenuSelection::LoadLatest))),
                            7 => return Ok(Some(Action::QuitApp)), // Note: Index 6 + 1 for border
                            _ => {}
                        }
                    }
                    return Ok(None); 
                }

                let sidebar_x_start = (terminal::size()?.0 as f32 * 0.75).round() as i32; 
                let is_in_sidebar = x >= sidebar_x_start; 
                let map_pos = MapPosition::new(x - 1, y - 1); 

                // --- 2. Route Click Event (Left Button Down) for Editor/Simulation ---
                if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                    match app.state {
                        AppState::Editor(_) => {
                            if is_in_sidebar {
                                return Ok(Some(Action::EditorMenuClick { screen_x: x, screen_y: y })); 
                            } else if x > 0 && y > 0 && x < sidebar_x_start { 
                                return Ok(Some(Action::EditorClick { pos: map_pos }));
                            }
                        }
                        AppState::Simulation(_) => {
                            if is_in_sidebar {
                                return Ok(Some(Action::GameMenuClick { screen_x: x, screen_y: y })); 
                            } else if x > 0 && y > 0 && x < sidebar_x_start { 
                                return Ok(Some(Action::GameClick { pos: map_pos }));
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}