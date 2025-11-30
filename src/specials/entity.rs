// src/specials/entity.rs

use crate::map::position::MapPosition; 
use serde::{Serialize, Deserialize};

pub type EntityID = u32;

// --- CONSTANTS ---
pub const MOVE_COST: u32 = 1;
pub const ATTACK_COST: u32 = 2;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)] 
pub struct Entity {
    // Identity
    id: EntityID,
    symbol: char,
    name: String,
    team: u32,

    // Stats
    max_health: u32,
    health: u32,
    max_energy: u32,
    energy: u32,
    damage: u32,
    attack_range: u32,

    // State
    position: MapPosition,
    stunned_for_turns: u32,
    is_selected: bool,
    is_ai: bool,
}

impl Entity {
    // =========================================================================
    //                            CONSTRUCTOR
    // =========================================================================
    
    pub fn new(
        id: EntityID,
        symbol: char,
        name: String,
        position: MapPosition,
        team: u32, 
        max_health: u32, 
        max_energy: u32,
        damage: u32,          
        attack_range: u32
    ) -> Self {
        Entity { 
            id,
            symbol,
            name,
            team,
            
            max_health,
            health: max_health,
            max_energy,
            energy: max_energy, 
            damage,
            attack_range,

            position,
            stunned_for_turns: 0,
            is_selected: false,
            is_ai: false, 
        }
    }

    // =========================================================================
    //                            ACCESSORS (GETTERS)
    // =========================================================================
    
    pub fn id(&self) -> EntityID { self.id }
    pub fn symbol(&self) -> char { self.symbol }
    pub fn display_name(&self) -> &str { &self.name }
    pub fn team(&self) -> u32 { self.team }
    
    pub fn position(&self) -> MapPosition { self.position }
    
    // Stats
    pub fn health(&self) -> u32 { self.health }
    pub fn max_health(&self) -> u32 { self.max_health }
    pub fn energy(&self) -> u32 { self.energy }
    pub fn max_energy(&self) -> u32 { self.max_energy }
    pub fn damage(&self) -> u32 { self.damage }
    pub fn attack_range(&self) -> u32 { self.attack_range }

    // Flags
    pub fn is_selected(&self) -> bool { self.is_selected }
    pub fn is_ai(&self) -> bool { self.is_ai }
    pub fn is_active(&self) -> bool { self.stunned_for_turns == 0 }
    pub fn is_stunned(&self) -> bool { self.stunned_for_turns > 0 }
    pub fn is_dead(&self) -> bool { self.health == 0 }

    /// Перевіряє, чи вистачає енергії і чи юніт не застанений.
    pub fn can_act(&self, cost: u32) -> bool {
        self.is_active() && self.energy >= cost
    }

    // =========================================================================
    //                           STATE MODIFIERS (SETTERS)
    // =========================================================================
    
    pub fn set_position(&mut self, pos: MapPosition) {
        self.position = pos;
    }

    pub fn set_team(&mut self, team: u32) {
        self.team = team;
    }
    
    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }
    
    pub fn set_ai(&mut self, is_ai: bool){
        self.is_ai = is_ai;
    }

    // =========================================================================
    //                            GAMEPLAY LOGIC
    // =========================================================================

    pub fn reduce_stun(&mut self) {
        if self.stunned_for_turns > 0 {
            self.stunned_for_turns -= 1;
        }
    }
    
    /// Спроба виконати дію. Повертає true, якщо енергія була витрачена.
    pub fn consume_energy(&mut self, cost: u32) -> bool {
        if self.can_act(cost) {
            self.energy -= cost;
            true
        } else {
            false
        }
    }
    
    /// Відновлює енергію до максимуму (якщо не застанений).
    pub fn refill_energy(&mut self) {
        if self.is_active() {
            self.energy = self.max_energy; 
        }
    }
    
    /// Наносить шкоду. Повертає true, якщо сутність померла.
    pub fn take_damage(&mut self, amount: u32) -> bool {
        self.health = self.health.saturating_sub(amount);
        self.is_dead()
    }

    /// Лікує сутність, не перевищуючи максимум.
    pub fn heal(&mut self, amount: u32) {
        if self.is_dead() { return; }
        self.health = self.health.saturating_add(amount).min(self.max_health);
    }
}