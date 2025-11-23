// src/state/modes/game.rs

use super::super::world_state::WorldState; 
use crate::specials::entity::{EntityID, MOVE_COST, ATTACK_COST}; // Import costs
use crate::map::position::MapPosition;
use super::super::actions::GameTool;
pub struct GameMode {
    pub world_state: WorldState,
    pub selected_entity_id: Option<EntityID>,
    pub debug_message: String,
    pub current_tool: GameTool,
}

impl GameMode {
    pub fn new(world_state: WorldState) -> Self {
        GameMode {
            world_state,
            selected_entity_id: None,
            debug_message: String::from("Game Mode. Select Unit."),
            current_tool: GameTool::Select,
        }
    }

    pub fn handle_click(&mut self, pos: MapPosition) {
        
        // We need a selected unit to perform Move/Attack/Skill
        // If nothing is selected, any click just attempts to select something.
        if self.selected_entity_id.is_none() {
            let clicked_id = self.world_state.get_entity_id_at(pos);
            self.select_entity(clicked_id);
            // If we just selected someone, we don't perform an action immediately
            return; 
        }

        let selected_id = self.selected_entity_id.unwrap();

        // Match the TOOL, not the context!
        match self.current_tool {
            
            // Mode: SELECT (Safe Mode)
            // Clicking anywhere just changes selection or deselects
            GameTool::Select => {
                let clicked_id = self.world_state.get_entity_id_at(pos);
                if clicked_id != self.selected_entity_id {
                    self.select_entity(clicked_id);
                } else {
                    // Clicking self in select mode could show detailed info?
                    self.debug_message = String::from("Unit already selected.");
                }
            }

            // Mode: MOVE
            // Forces movement logic. If you click a wall or enemy, it errors.
            GameTool::Move => {
                 // If we clicked a unit (even an enemy), Move tool says "I want to walk here".
                 // Since we can't walk on units, this checks is_standable implicitly in try_move.
                 self.try_move_entity(selected_id, pos);
                 // Optional: Switch back to Select after action? 
                 // self.current_tool = GameTool::Select; 
            }

            // Mode: ATTACK
            // Forces attack logic. 
            GameTool::Attack => {
                if let Some(target_id) = self.world_state.get_entity_id_at(pos) {
                    // Cannot attack self
                    if target_id == selected_id {
                         self.debug_message = String::from("Cannot attack self!");
                    } else {
                         self.try_attack_entity(selected_id, target_id);
                         // Optional: Switch back to Select after action?
                    }
                } else {
                    self.debug_message = String::from("No target there!");
                }
            }

            // Mode: SKILL (Placeholder)
            GameTool::Skill => {
                self.debug_message = String::from("Super Ability not implemented yet!");
                // TODO: self.cast_super_ability(selected_id, pos);
            }
        }
    }

    // === SIDEBAR MENU HANDLER ===
    pub fn handle_menu_click(&mut self, screen_x: i32, screen_y: i32) {
        // Adjusting Y coordinates for Game Menu
        // Let's assume:
        // Y=3: [1] Select
        // Y=4: [2] Move
        // Y=5: [3] Attack
        // Y=6: [4] Skill
        // Y=8: [T] End Turn
        
        match screen_y {
            3 => {
                self.current_tool = GameTool::Select;
                self.debug_message = String::from("Tool: Select");
            },
            4 => {
                self.current_tool = GameTool::Move;
                self.debug_message = String::from("Tool: Move (Click empty tile)");
            },
            5 => {
                self.current_tool = GameTool::Attack;
                self.debug_message = String::from("Tool: Attack (Click enemy)");
            },
            6 => {
                self.current_tool = GameTool::Skill;
                self.debug_message = String::from("Tool: Skill (TODO)");
            },
            8 => {
                self.end_turn();
            },
            _ => {
                self.debug_message = format!("Menu Click ({}, {})", screen_x, screen_y);
            }
        }
    }

    fn select_entity(&mut self, id: Option<EntityID>) {
        if let Some(prev) = self.selected_entity_id {
            if let Some(e) = self.world_state.get_entity_mut(prev) { e.set_selected(false); }
        }
        self.selected_entity_id = id;
        if let Some(new) = id {
            if let Some(e) = self.world_state.get_entity_mut(new) { e.set_selected(true); }
            self.debug_message = format!("Selected Unit #{}", new);
        } else {
            self.debug_message = String::from("Selection cleared.");
        }
    }

    // --- ЛОГІКА РУХУ ---
    fn try_move_entity(&mut self, id: EntityID, target_pos: MapPosition) {
        // 1. Перевірка чи можна стати
        if !self.world_state.map.is_standable(&target_pos) {
            self.debug_message = String::from("Path blocked!");
            return;
        }

        let old_pos;
        let dist;
        let move_cost;

        // 2. Розрахунок вартості (Manhattan Distance * MOVE_COST)
        {
            let e = self.world_state.get_entity(id).unwrap();
            old_pos = e.position();
            dist = old_pos.manhattan_distance(&target_pos);
            move_cost = dist * MOVE_COST;

            // 3. Перевірка енергії (тільки перевірка, списання пізніше)
            if !e.can_act(move_cost) {
                self.debug_message = format!("Need {} Energy (Have {})", move_cost, e.energy());
                return;
            }
        }

        // 4. Виконання руху (Списання енергії відбувається тут через mut reference)
        // Ми беремо мутабельне посилання знову, бо scope попереднього закінчився
        if let Some(e) = self.world_state.get_entity_mut(id) {
            e.consume_energy(move_cost);
            e.set_position(target_pos);
        }

        // 5. Оновлення мапи (Tile IDs)
        self.world_state.map.get_tile_mut(&old_pos).unwrap().set_entity_id(None);
        
        let mut msg = format!("Moved (Cost: {})", move_cost);
        if let Some(tile) = self.world_state.map.get_tile_mut(&target_pos) {
            tile.set_entity_id(Some(id));
            if tile.has_powerup() {
                 let p = tile.take_powerup();
                 msg = format!("Moved & Got {:?}", p);
            }
        }
        self.debug_message = msg;
    }

    // --- ЛОГІКА АТАКИ ---
    fn try_attack_entity(&mut self, attacker_id: EntityID, target_id: EntityID) {
        
        let damage;
        let range;
        let attacker_pos;
        let target_pos;
        let target_name;

        // 1. Збір даних (Read-only)
        {
            let attacker = self.world_state.get_entity(attacker_id).unwrap();
            let target = self.world_state.get_entity(target_id).unwrap();
            
            attacker_pos = attacker.position();
            target_pos = target.position();
            target_name = target.display_name().to_string();
            
            damage = attacker.damage();
            range = attacker.attack_range();

            // 2. Перевірка можливості атаки
            if !attacker.can_act(ATTACK_COST) {
                self.debug_message = String::from("Not enough energy to attack!");
                return;
            }

            let dist = attacker_pos.manhattan_distance(&target_pos);
            if dist > range {
                self.debug_message = format!("Target out of range! (Dist: {}, Range: {})", dist, range);
                return;
            }
        } // Release immutable borrows

        // 3. Виконання атаки (Mutable borrows)
        // Крок А: Списання енергії у нападника
        if let Some(att) = self.world_state.get_entity_mut(attacker_id) {
            att.consume_energy(ATTACK_COST);
        }

        // Крок Б: Нанесення урону цілі
        let mut target_died = false;
        if let Some(tgt) = self.world_state.get_entity_mut(target_id) {
            target_died = tgt.take_damage(damage);
        }

        // 4. Результат
        if target_died {
            self.debug_message = format!("{} destroyed!", target_name);
            // Очистити труп з мапи
            self.world_state.map.get_tile_mut(&target_pos).unwrap().set_entity_id(None);
            
            // Можна видалити з вектора entities або залишити як "мертвий" об'єкт.
            // Поки що залишаємо, бо видалення з Vec зміщує індекси, якщо ми звертаємось по індексу.
            // Але у нас ID-система, тому просто помічаємо як мертвий.
        } else {
            self.debug_message = format!("Attacked {} for {} dmg!", target_name, damage);
        }
    }
    
    pub fn end_turn(&mut self) {
        for entity in self.world_state.entities.iter_mut() {
            if !entity.is_dead() {
                entity.reduce_stun();
                entity.refill_energy();
            }
        }
        self.debug_message = String::from("Next Turn. Energy Refilled.");
    }
}