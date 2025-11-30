use crate::state::world_state::WorldState;
use crate::specials::entity::EntityID;
use crate::map::position::MapPosition;
use crate::state::actions::{GameTool, Action};

use super::engine::{GameEngine, ActionResult, TurnResult};
use super::ai::AiSystem;

pub struct GameMode {
    engine: GameEngine,
    selected_entity_id: Option<EntityID>,
    debug_message: String,
    current_tool: GameTool,
}

impl GameMode {
    pub fn new(world_state: WorldState) -> Self {
        GameMode {
            engine: GameEngine::new(world_state),
            selected_entity_id: None,
            debug_message: String::from("Game Start. Select a unit."),
            current_tool: GameTool::Select,
        }
    }

    // =========================================================================
    //                             GETTERS (API)
    // =========================================================================

    pub fn world(&self) -> &WorldState {
        self.engine.world()
    }

    pub fn current_tool(&self) -> GameTool {
        self.current_tool
    }

    pub fn selected_entity_id(&self) -> Option<EntityID> {
        self.selected_entity_id
    }

    pub fn debug_message(&self) -> &str {
        &self.debug_message
    }

    // =========================================================================
    //                            MAIN LOOP
    // =========================================================================
    
    pub fn tick(&mut self) {
        // let current_team = self.engine.current_team();

        // if current_team != 1 {
        //     let ai_logs = AiSystem::perform_turn(&mut self.engine, current_team);
            
        //     if !ai_logs.is_empty() {
        //         self.debug_message = ai_logs.last().unwrap().clone();
        //     }
            
        //     self.end_turn_logic();
        // }
    }

    // =========================================================================
    //                            INPUT HANDLERS
    // =========================================================================

    pub fn handle_keypress(&mut self, key: char) -> Option<Action> {
        
        match key {
            '1' => self.switch_tool(GameTool::Select),
            '2' => self.switch_tool(GameTool::Move),
            '3' => self.switch_tool(GameTool::Attack),
            '4' => self.switch_tool(GameTool::Skill),
            't' | 'T' => self.end_turn_logic(),
            'q' | 'Q' => return Some(Action::BackToMenu),
            _ => {}
        }
        
        None
    }

    pub fn handle_menu_click(&mut self, _screen_x: i32, screen_y: i32) -> Option<Action> {
    
        match screen_y {
            3 => self.switch_tool(GameTool::Select),
            4 => self.switch_tool(GameTool::Move),
            5 => self.switch_tool(GameTool::Attack),
            6 => self.switch_tool(GameTool::Skill),
            8 => self.end_turn_logic(),
            10 => return Some(Action::BackToMenu),
            _ => {}
        }
        None
    }

    pub fn handle_click(&mut self, pos: MapPosition) {

        match self.current_tool {
            GameTool::Select => self.do_select(pos),
            GameTool::Move => self.do_move(pos),
            GameTool::Attack => self.do_attack(pos),
            GameTool::Skill => self.debug_message = "Skills not implemented yet".to_string(),
        }
    }

    // =========================================================================
    //                            PRIVATE LOGIC
    // =========================================================================

    fn switch_tool(&mut self, tool: GameTool) {
        self.current_tool = tool;
        self.debug_message = format!("Tool: {:?}", tool);
    }

    fn end_turn_logic(&mut self) {
        // 1. Спочатку ходять БОТИ поточної команди (якщо вони є)
        // Це реалізує логіку: Гравець походив -> Натиснув T -> Боти доробили роботу
        let current_team = self.engine.current_team();
        let ai_logs = AiSystem::perform_turn(&mut self.engine, current_team);
        
        if !ai_logs.is_empty() {
            // Показуємо, що зробили боти
            self.debug_message = ai_logs.last().unwrap().clone();
        }

        // 2. Тільки після цього передаємо хід наступній команді
        match self.engine.end_turn() {
            TurnResult::TurnChanged(new_team) => {
                let name = if new_team == 1 { "Player" } else { "Enemy" };
                // Додаємо інформацію до попереднього повідомлення або перезаписуємо
                self.debug_message = format!("Turn: {} (Team {})", name, new_team);
                
                if new_team == 1 {
                    self.current_tool = GameTool::Select;
                }
            }
            TurnResult::TurnContinues => {
                self.debug_message = "Waiting for others...".to_string();
            }
        }
    }

    fn do_select(&mut self, pos: MapPosition) {
        let clicked_id = self.engine.world().get_entity_id_at(pos);
        
        if let Some(old_id) = self.selected_entity_id {
            if let Some(e) = self.engine.world_mut().get_entity_mut(old_id) { 
                e.set_selected(false); 
            }
        }
        
        self.selected_entity_id = clicked_id;
        
        if let Some(new_id) = clicked_id {
            let name = if let Some(e) = self.engine.world_mut().get_entity_mut(new_id) { 
                e.set_selected(true);
                e.display_name().to_string()
            } else {
                "Unknown".to_string()
            };
            self.debug_message = format!("Selected: {}", name);
        } else {
             self.debug_message = "Selection cleared".to_string();
        }
    }

    fn do_move(&mut self, target_pos: MapPosition) {
        let id = match self.selected_entity_id {
            Some(id) => id,
            None => { self.debug_message = "Select a unit first!".to_string(); return; }
        };

        match self.engine.move_entity(id, target_pos) {
            ActionResult::SuccessMove => self.debug_message = "Moved successfully.".to_string(),
            ActionResult::Fail(reason) => self.debug_message = format!("Move failed: {}", reason),
            _ => {}
        }
    }

    fn do_attack(&mut self, pos: MapPosition) {
        let attacker_id = match self.selected_entity_id {
            Some(id) => id,
            None => { self.debug_message = "Select a unit first!".to_string(); return; }
        };

        let target_id = match self.engine.world().get_entity_id_at(pos) {
            Some(id) => id,
            None => { self.debug_message = "Click on an enemy!".to_string(); return; }
        };

        match self.engine.attack_entity(attacker_id, target_id) {
            ActionResult::SuccessAttack { damage, target_died } => {
                if target_died {
                    self.debug_message = format!("FATAL HIT! -{} HP", damage);
                } else {
                    self.debug_message = format!("Hit! -{} HP", damage);
                }
            },
            ActionResult::Fail(reason) => self.debug_message = format!("Attack failed: {}", reason),
            _ => {}
        }
    }
}