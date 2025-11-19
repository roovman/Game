use super::game_state::GameState;
use super::actions::{BuildTool};
use crate::specials::entity::EntityID;
use crate::map::position::MapPosition;
use crate::specials::powerup::PowerupType;

// === EDITOR MODE ===
pub struct EditorMode {
    pub game_state: GameState,
    pub current_tool: BuildTool,
    pub debug_message: String,
}

impl EditorMode {
    pub fn new() -> Self {
        EditorMode {
            game_state: GameState::new(),
            current_tool: BuildTool::Wall,
            debug_message: String::from("Editor: Tab->Tool, S->Save, Q->Menu"),
        }
    }

    pub fn save_map(&mut self) {
        if let Err(e) = self.game_state.save("map.json") {
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
                if self.game_state.build_wall(pos) {
                    self.debug_message = format!("Built Wall at {:?}", pos);
                }
            }
            BuildTool::Floor => {
                self.game_state.build_floor(pos);
                self.debug_message = format!("Cleared at {:?}", pos);
            }
            BuildTool::Unit => {
                if let Some(id) = self.game_state.spawn_entity(pos, '@', 100) {
                    self.debug_message = format!("Spawned Unit #{}", id);
                } else {
                    self.debug_message = String::from("Blocked!");
                }
            }
        }
    }
}

// === GAME MODE ===
pub struct GameMode {
    pub game_state: GameState,
    pub selected_entity_id: Option<EntityID>,
    pub debug_message: String,
}

impl GameMode {
    pub fn new(game_state: GameState) -> Self {
        GameMode {
            game_state,
            selected_entity_id: None,
            debug_message: String::from("Game Mode. Click to Select/Move."),
        }
    }

    pub fn handle_click(&mut self, pos: MapPosition) {
        if let Some(selected_id) = self.selected_entity_id {
            if let Some(clicked_id) = self.game_state.get_entity_id_at(pos) {
                if clicked_id != selected_id {
                    self.select_entity(Some(clicked_id));
                    return;
                }
            }
            self.try_move_entity(selected_id, pos);
        } else {
            let clicked_id = self.game_state.get_entity_id_at(pos);
            self.select_entity(clicked_id);
        }
    }

    fn select_entity(&mut self, id: Option<EntityID>) {
        if let Some(prev) = self.selected_entity_id {
            if let Some(e) = self.game_state.get_entity_mut(prev) { e.is_selected = false; }
        }
        self.selected_entity_id = id;
        if let Some(new) = id {
            if let Some(e) = self.game_state.get_entity_mut(new) { e.is_selected = true; }
            self.debug_message = format!("Selected Unit #{}", new);
        } else {
            self.debug_message = String::from("Selection cleared.");
        }
    }

    fn try_move_entity(&mut self, id: EntityID, target_pos: MapPosition) {
        if !self.game_state.map.is_standable(&target_pos) {
            self.debug_message = String::from("Blocked path!");
            return;
        }

        let old_pos;
        {
            let e = self.game_state.get_entity(id).unwrap();
            old_pos = e.position;
        }

        self.game_state.map.get_tile_mut(&old_pos).unwrap().entity_id = None;
        
        let mut msg = format!("Unit moved.");
        if let Some(tile) = self.game_state.map.get_tile_mut(&target_pos) {
            tile.entity_id = Some(id);
            if tile.has_powerup() {
                 let p = tile.take_powerup();
                 msg = format!("Moved & Got {:?}", p);
            }
        }

        if let Some(e) = self.game_state.get_entity_mut(id) {
            e.position = target_pos;
        }
        self.debug_message = msg;
    }
}