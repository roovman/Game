// src/tui/input.rs

use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use crossterm::terminal; 
use std::time::Duration;
use ratatui::layout::Rect; 
use color_eyre::Result;

use crate::state::actions::{Action, MenuSelection};
use crate::state::application_state::{ApplicationState, AppState};
use crate::state::modes::editor::input::InputTarget; 
use crate::map::position::MapPosition;

// Імпортуємо наш Single Source of Truth
use crate::tui::layout::{get_main_layout, get_centered_rect, is_point_in_rect};

pub fn handle_input(app: &ApplicationState) -> Result<Option<Action>> {
    if event::poll(Duration::from_millis(16))? {
        let event = event::read()?;
        
        match event {
            // --- KEYBOARD ---
            Event::Key(key) => {
                // ... (Твій код клавіатури ідеальний, залишаємо без змін) ...
                // 1. Editor Typing
                if let AppState::Editor(mode) = &app.state {
                    if mode.input_target() != InputTarget::None { // Using getter now!
                        match key.code {
                            KeyCode::Char(c) if c.is_digit(10) => return Ok(Some(Action::EditorType(c))),
                            KeyCode::Backspace => return Ok(Some(Action::EditorBackspace)),
                            KeyCode::Enter => return Ok(Some(Action::EditorConfirm)),
                            KeyCode::Esc => return Ok(Some(Action::EditorCancel)),
                            _ => return Ok(None), 
                        }
                    }
                }

                // 2. Shortcuts
                match app.state {
                    AppState::Menu => {
                        match key.code {
                            KeyCode::Char('1') => return Ok(Some(Action::MenuSelect(MenuSelection::EnterBuildMode))),
                            KeyCode::Char('2') => return Ok(Some(Action::MenuSelect(MenuSelection::EnterPlayMode))),
                            KeyCode::Char('3') => return Ok(Some(Action::MenuSelect(MenuSelection::LoadLatest))),
                            KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(Some(Action::QuitApp)),
                            _ => {}
                        }
                    },
                    AppState::Editor(_) => {
                        match key.code {
                            KeyCode::Tab => return Ok(Some(Action::CycleBuildTool)),
                            KeyCode::Char(c) => return Ok(Some(Action::EditorKeyPress(c))),
                            _ => {}
                        }
                    },
                    AppState::Game(_) => {
                        match key.code {
                            KeyCode::Char(c) => return Ok(Some(Action::GameKeyPress(c))),
                            _ => {}
                        }
                    },
                    _ => {},
                }
            },
            
            // --- MOUSE ---
            Event::Mouse(mouse) => {
                if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                    let x = mouse.column as i32;
                    let y = mouse.row as i32;
                    let (term_width, term_height) = terminal::size()?; 
                    
                    // Створюємо Rect екрану, щоб розрахувати лейаут
                    let screen_area = Rect::new(0, 0, term_width, term_height);

                    match &app.state {
                        // 1. MAIN MENU
                        AppState::Menu => {
                            let menu_rect = get_centered_rect(40, 40, screen_area);
                            
                            if is_point_in_rect(x, y, menu_rect) {
                                let relative_y = y - menu_rect.y as i32;
                                match relative_y {
                                    5 => return Ok(Some(Action::MenuSelect(MenuSelection::EnterBuildMode))),
                                    6 => return Ok(Some(Action::MenuSelect(MenuSelection::EnterPlayMode))),
                                    7 => return Ok(Some(Action::MenuSelect(MenuSelection::LoadLatest))),
                                    9 => return Ok(Some(Action::QuitApp)),
                                    _ => {}
                                }
                            }
                        },

                        // 2. EDITOR & GAME (Unified Logic)
                        AppState::Editor(_) | AppState::Game(_) => {
                            // Отримуємо ті самі прямокутники, що і Draw!
                            let (map_rect, menu_rect) = get_main_layout(screen_area);

                            // А) Клік по МЕНЮ (Сайдбар)
                            if is_point_in_rect(x, y, menu_rect) {
                                // Рахуємо Y відносно початку меню (верхня рамка = 0)
                                let relative_y = y - menu_rect.y as i32; 
                                
                                match app.state {
                                    AppState::Editor(_) => return Ok(Some(Action::EditorMenuClick { screen_x: x, screen_y: relative_y })),
                                    AppState::Game(_) => return Ok(Some(Action::GameMenuClick { screen_x: x, screen_y: relative_y })),
                                    _ => unreachable!(),
                                }
                            }
                            
                            // Б) Клік по КАРТІ
                            else if is_point_in_rect(x, y, map_rect) {
                                // Рахуємо координати відносно карти (якщо карта не з 0,0)
                                let map_x = x - map_rect.x as i32;
                                let map_y = y - map_rect.y as i32;
                                let pos = MapPosition::new(map_x, map_y);

                                match app.state {
                                    AppState::Editor(_) => return Ok(Some(Action::EditorClick { pos })),
                                    AppState::Game(_) => return Ok(Some(Action::GameClick { pos })),
                                    _ => unreachable!(),
                                }
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