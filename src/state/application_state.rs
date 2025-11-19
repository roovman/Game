use super::actions::{Action, MenuSelection};
use super::modes::{EditorMode, GameMode};
use super::game_state::GameState;

pub enum AppState {
    Menu,
    Editor(EditorMode),
    Simulation(GameMode),
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
        match &mut self.state {
            AppState::Menu => {
                if let Action::MenuSelect(sel) = action {
                    match sel {
                        MenuSelection::EnterBuildMode => self.state = AppState::Editor(EditorMode::new()),
                        MenuSelection::EnterPlayMode => {
                            let mut gs = GameState::new();
                            gs.spawn_entity(crate::map::position::MapPosition::new(10,10), '@', 100);
                            self.state = AppState::Simulation(GameMode::new(gs));
                        },
                        MenuSelection::LoadLatest => {
                            match GameState::load("map.json") {
                                Ok(gs) => self.state = AppState::Simulation(GameMode::new(gs)),
                                Err(_) => { /* TODO: Handle error UI */ }
                            }
                        },
                        _ => {}
                    }
                } else if let Action::QuitApp = action {
                    self.state = AppState::Exiting;
                }
            },
            AppState::Editor(editor) => {
                match action {
                    Action::BackToMenu => self.state = AppState::Menu,
                    Action::CycleBuildTool => editor.cycle_tool(),
                    Action::SaveMap => editor.save_map(), // ⭐️ Save
                    Action::EditorClick { pos } => editor.handle_click(pos),
                    _ => {}
                }
            },
            AppState::Simulation(game) => {
                match action {
                    Action::BackToMenu => self.state = AppState::Menu,
                    Action::GameClick { pos } => game.handle_click(pos),
                    _ => {}
                }
            },
            _ => {}
        }
    }
}