use super::actions::{Action, MenuSelection};
// Use the new folder structure for modes (assuming you commit the mod.rs files)
use super::modes::{EditorMode, GameMode}; 
// Use the new WorldState name
use super::world_state::WorldState; 

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
                            let mut ws = WorldState::new();
                            // Updated spawn call for the player
                            ws.spawn_entity(
                                crate::map::position::MapPosition::new(10,10), 
                                '@', 
                                String::from("Player"),
                                100, 
                                5,
                                25,
                                5,     
                                1,  
                                false 
                            );
                            self.state = AppState::Simulation(GameMode::new(ws));
                        },
                        MenuSelection::LoadLatest => {
                            match WorldState::load("map.json") {
                                Ok(ws) => self.state = AppState::Simulation(GameMode::new(ws)),
                                Err(_) => { /* TODO: Handle error UI */ }
                            }
                        },
                    }
                } else if let Action::QuitApp = action {
                    self.state = AppState::Exiting;
                }
            },
            AppState::Editor(editor) => {
                match action {
                    Action::BackToMenu => self.state = AppState::Menu,
                    Action::CycleBuildTool => editor.cycle_tool(),
                    Action::SaveMap => editor.save_map(), 
                    Action::EditorClick { pos } => editor.handle_click(pos),
                    // NEW: Route menu clicks to the mode's menu handler
                    Action::EditorMenuClick { screen_x, screen_y } => editor.handle_menu_click(screen_x, screen_y),
                    _ => {}
                }
            },
            AppState::Simulation(game) => {
                match action {
                    Action::BackToMenu => self.state = AppState::Menu,
                    Action::GameClick { pos } => game.handle_click(pos),
                    // NEW: Route menu clicks to the mode's menu handler
                    Action::GameMenuClick { screen_x, screen_y } => { 
                        game.handle_menu_click(screen_x, screen_y);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}