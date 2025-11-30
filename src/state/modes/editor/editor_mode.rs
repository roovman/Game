use crate::state::world_state::WorldState; 
use crate::state::actions::{BuildTool, Action}; 
use crate::map::position::MapPosition;

// Локальні модулі
use super::input::{EditorInput, InputTarget};
use super::entity_spec::EntitySpec;

pub struct EditorMode {
    world_state: WorldState,       
    current_tool: BuildTool,       
    debug_message: String,         
    entity_spec: EntitySpec,       
    input_manager: EditorInput,    
}

impl EditorMode {
    pub fn new() -> Self {
        EditorMode {
            world_state: WorldState::new(),
            current_tool: BuildTool::Wall,
            debug_message: String::from("Editor: Use W/F/U keys or click menu"),
            entity_spec: EntitySpec::default(),
            input_manager: EditorInput::new(),
        }
    }

    // =========================================================================
    //                             GETTERS (API)
    // =========================================================================
    
    pub fn world(&self) -> &WorldState { &self.world_state }
    
    pub fn current_tool(&self) -> BuildTool { self.current_tool }
    pub fn debug_message(&self) -> &str { &self.debug_message }
    pub fn entity_spec(&self) -> &EntitySpec { &self.entity_spec }
    pub fn input_target(&self) -> InputTarget { self.input_manager.input_target }
    pub fn input_buffer(&self) -> &str { &self.input_manager.input_buffer }

    // =========================================================================
    //                            LOGIC & ACTIONS
    // =========================================================================

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

    fn set_tool(&mut self, tool: BuildTool) {
        self.current_tool = tool;
        self.debug_message = format!("Tool: {:?}", self.current_tool);
    }
    
    // --- INPUT HANDLERS ---

    pub fn handle_click(&mut self, pos: MapPosition) {
        if self.input_manager.input_target != InputTarget::None {
            self.confirm_input();
            return;
        }

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
                let id = self.world_state.next_id();
                let new_entity = self.entity_spec.to_entity(id, pos);
                let name = new_entity.display_name().to_string(); 
                
                self.world_state.add_entity(new_entity);
                self.debug_message = format!("Spawned {} #{}", name, id);
            }
        }
    }

    pub fn handle_keypress(&mut self, key: char) -> Option<Action> {
        if self.input_manager.input_target != InputTarget::None {
            return None; 
        }

        match key {
            'w' | 'W' => self.set_tool(BuildTool::Wall),
            'f' | 'F' => self.set_tool(BuildTool::Floor),
            'u' | 'U' => self.set_tool(BuildTool::Unit),
            's' | 'S' => self.save_map(),
            'q' | 'Q' => return Some(Action::BackToMenu),
            
            't' | 'T' => self.start_input(InputTarget::Team),
            'h' | 'H' => self.start_input(InputTarget::Hp),
            'e' | 'E' => self.start_input(InputTarget::Energy),
            'd' | 'D' => self.start_input(InputTarget::Damage),
            'a' | 'A' => {
                self.entity_spec.toggle_ai();
                self.debug_message = format!("AI toggled: {}", self.entity_spec.is_ai);
            }
            _ => {}
        }
        None
    }

    pub fn handle_menu_click(&mut self, _screen_x: i32, screen_y: i32) -> Option<Action> {
        if self.input_manager.input_target != InputTarget::None {
            self.confirm_input();
        }

        match screen_y {
            3 => self.set_tool(BuildTool::Wall),
            4 => self.set_tool(BuildTool::Floor),
            5 => self.set_tool(BuildTool::Unit),
            
            7 => self.save_map(),
            
            9 => return Some(Action::BackToMenu),

            12 => self.start_input(InputTarget::Team),
            13 => self.start_input(InputTarget::Hp),
            14 => self.start_input(InputTarget::Energy),
            15 => self.start_input(InputTarget::Damage),
            16 => {
                self.entity_spec.toggle_ai();
                self.debug_message = format!("AI set to {}", self.entity_spec.is_ai);
            },

            _ => {}
        }
        None
    }
    
    // --- INPUT DELEGATION ---

    pub fn handle_typing(&mut self, c: char) { self.input_manager.handle_typing(c); }
    pub fn handle_backspace(&mut self) { self.input_manager.handle_backspace(); }
    
    pub fn confirm_input(&mut self) {
        if let Some(msg) = self.input_manager.confirm(&mut self.entity_spec) {
            self.debug_message = msg;
        }
    }

    pub fn cancel_input(&mut self) {
        self.input_manager.cancel();
        self.debug_message = "Edit Cancelled".to_string();
    }

    fn start_input(&mut self, target: InputTarget) {
        self.input_manager.cancel();
        self.input_manager.input_target = target;
        self.debug_message = format!("Edit {:?}: Type number + Enter", target);
    }
}