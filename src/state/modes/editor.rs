// src/state/modes/editor.rs


// --- IMPORTS ---
use super::super::world_state::WorldState; 
use super::super::actions::BuildTool;
use crate::map::position::MapPosition;

// === EDITOR MODE ===
pub struct EditorMode {
    pub world_state: WorldState,
    pub current_tool: BuildTool,
    pub debug_message: String,
}

impl EditorMode {
    pub fn new() -> Self {
        EditorMode {
            world_state: WorldState::new(),
            current_tool: BuildTool::Wall,
            debug_message: String::from("Editor: Tab->Tool, S->Save, Q->Menu"),
        }
    }

    pub fn save_map(&mut self) {
        if let Err(e) = self.world_state.save("map.json") {
            self.debug_message = format!("Save Failed: {}", e);
        } else {
            self.debug_message = String::from("Saved to 'map.json'!");
        }
    }

    pub fn cycle_tool(&mut self) {
        self.current_tool = match self.current_tool {
            BuildTool::Wall => BuildTool::Floor,
            BuildTool::Floor => BuildTool::Unit,
            BuildTool::Unit => BuildTool::Wall,
        };
        self.debug_message = format!("Tool: {:?}", self.current_tool);
    }

    pub fn handle_click(&mut self, pos: MapPosition) {
        match self.current_tool {
            BuildTool::Wall => {
                if self.world_state.build_wall(pos) {
                    self.debug_message = format!("Built Wall at {:?}", pos);
                }
            }
            BuildTool::Floor => {
                self.world_state.build_floor(pos);
                self.debug_message = format!("Cleared at {:?}", pos);
            }
            BuildTool::Unit => {
                if let Some(id) = self.world_state.spawn_entity(
                    pos, 
                    'E', // Editor defaults to 'E' for enemy/entity
                    String::from("Editor Unit"),
                    50,  // Smaller health
                    3,
                    25,
                     5,   // Smaller energy
                    1,   // Team 2 (Enemy)
                    false // Not AI (for now, easy to change later)
                ) {
                    self.debug_message = format!("Spawned Unit #{} (Team 2)", id);
                } else {
                    self.debug_message = String::from("Blocked!");
                }
            }
        }
    }
    
    // --- NEW: Handle Mouse Clicks in the Sidebar Menu ---
    pub fn handle_menu_click(&mut self, screen_x: i32, screen_y: i32) {
        // We are relying on the draw.rs layout for Y coordinates:
        // Y=3: [W] Wall
        // Y=4: [F] Floor
        // Y=5: [U] Unit
        // Y=8: [S] Save Map (assuming the spacing from draw.rs)
        
        match screen_y {
            3 => {
                self.current_tool = BuildTool::Wall;
                self.debug_message = String::from("Tool: Wall (Clicked)");
            },
            4 => {
                self.current_tool = BuildTool::Floor;
                self.debug_message = String::from("Tool: Floor (Clicked)");
            },
            5 => {
                self.current_tool = BuildTool::Unit;
                self.debug_message = String::from("Tool: Unit (Clicked)");
            },
            8 => {
                self.save_map(); 
            },
            _ => {
                self.debug_message = format!("Clicked menu area at ({}, {}). No action.", screen_x, screen_y);
            }
        }
    }
}