// src/state/application_state.rs

use super::actions::{Action, MenuSelection};
use super::modes::{EditorMode, GameMode}; 
use super::world_state::WorldState; 

pub enum AppState {
    Menu,
    Editor(EditorMode),
    Game(GameMode),
    Exiting,
}

pub struct ApplicationState {
    pub state: AppState,
}

impl ApplicationState {
    pub fn new() -> Self {
        ApplicationState { state: AppState::Menu }
    }

    pub fn apply_action(&mut self, action: Action) {
        let mut next_state: Option<AppState> = None;

        match &mut self.state {
            // --- MAIN MENU ---
            AppState::Menu => {
                if let Action::MenuSelect(sel) = action {
                    match sel {
                        MenuSelection::EnterBuildMode => next_state = Some(AppState::Editor(EditorMode::new())),
                        MenuSelection::EnterPlayMode => {
                            match WorldState::load("standart.json") {
                                Ok(ws) => next_state = Some(AppState::Game(GameMode::new(ws))),
                                Err(_) => {}
                            }
                        },
                        MenuSelection::LoadLatest => {
                            match WorldState::load("map.json") {
                                Ok(ws) => next_state = Some(AppState::Game(GameMode::new(ws))),
                                Err(_) => {}
                            }
                        },
                    }
                } else if let Action::QuitApp = action {
                    next_state = Some(AppState::Exiting);
                }
            },
            
            // --- EDITOR MODE ---
            AppState::Editor(editor) => {
                match action {
                    Action::QuitApp => next_state = Some(AppState::Exiting),
                    Action::BackToMenu => next_state = Some(AppState::Menu),
                    
                    Action::CycleBuildTool => editor.cycle_tool(),
                    Action::SaveMap => editor.save_map(), 
                    Action::EditorClick { pos } => editor.handle_click(pos),
                    
                    Action::EditorMenuClick { screen_x, screen_y } => {
                        if let Some(reaction) = editor.handle_menu_click(screen_x, screen_y) {
                            if matches!(reaction, Action::BackToMenu) {
                                next_state = Some(AppState::Menu);
                            }
                        }
                    },
                    
                    Action::EditorType(c) => editor.handle_typing(c),
                    Action::EditorConfirm => editor.confirm_input(),
                    Action::EditorBackspace => editor.handle_backspace(),
                    Action::EditorCancel => editor.cancel_input(),

                    Action::EditorKeyPress(key) => {
                        if let Some(reaction) = editor.handle_keypress(key) {
                            if matches!(reaction, Action::BackToMenu) {
                                next_state = Some(AppState::Menu);
                            }
                        }
                    },
                    
                    _ => {}
                }
            },
            
            // --- GAME MODE ---
            AppState::Game(game) => {
                match action {
                    Action::QuitApp => next_state = Some(AppState::Exiting),
                    Action::BackToMenu => next_state = Some(AppState::Menu),
                    
                    Action::GameClick { pos } => game.handle_click(pos),
                    
                    Action::GameMenuClick { screen_x, screen_y } => { 
                        if let Some(reaction) = game.handle_menu_click(screen_x, screen_y) {
                            if matches!(reaction, Action::BackToMenu) {
                                next_state = Some(AppState::Menu);
                            }
                        }
                    },

                    Action::GameKeyPress(key) => {
                        if let Some(reaction) = game.handle_keypress(key) {
                            if matches!(reaction, Action::BackToMenu) {
                                next_state = Some(AppState::Menu);
                            }
                        }
                    },
                    
                    _ => {}
                }
            },
            _ => {}
        }

        if let Some(ns) = next_state {
            self.state = ns;
        }
    }
}