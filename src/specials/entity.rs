// src/specials/entity.rs

use crate::map::position::MapPosition; 
use serde::{Serialize, Deserialize};
pub type EntityID = u32;
// --- CONSTANTS ---
pub const MOVE_COST: u32 = 1;
pub const ATTACK_COST: u32 = 2;
// --- EntityStatus (Private, Unchanged) ---
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
struct EntityStatus{
    position: MapPosition, 
    health: u32,      
    energy: u32,
    damage: u32,       
    attack_range: u32,    
    team: u32,         
    stunned_for_turns: u32, 
    is_selected: bool, 
    is_ai: bool
}

// --- Entity: ALL FIELDS ARE NOW PRIVATE ---
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)] 
pub struct Entity {
    id: EntityID,       // PRIVATE
    symbol: char,       // PRIVATE
    name: [u8; 32],     // PRIVATE
    max_health: u32,    // PRIVATE
    max_energy: u32,    // PRIVATE
    status: EntityStatus, // PRIVATE
}

impl Entity {
    // =========================================================================
    //                            CONSTRUCTORS/HELPERS
    // =========================================================================

    fn string_to_name_array(s: String) -> [u8; 32] {
        let mut array = [0u8; 32];
        let bytes = s.as_bytes();
        let len = bytes.len().min(32);
        array[..len].copy_from_slice(&bytes[..len]);
        array
    }

    fn new_status(
        position: MapPosition, 
        team: u32, 
        max_health: u32, 
        max_energy: u32,
        damage: u32,      
        attack_range: u32
    ) -> EntityStatus {
        EntityStatus { 
            position,
            health: max_health,  
            energy: max_energy,
            damage,
            attack_range,  
            team, 
            stunned_for_turns: 0, 
            is_selected: false,
            is_ai: false
        }
    }
    
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
            name: Self::string_to_name_array(name),
            max_health,
            max_energy, 
            status: Entity::new_status(
                position,
                team,
                max_health,
                max_energy,
                damage,
                attack_range
            ),
        }
    }

    // =========================================================================
    //                            READ-ONLY ACCESSORS (New and Updated)
    // =========================================================================
    
    /// Returns the entity's unique ID.
    pub fn id(&self) -> EntityID {
        self.id
    }
    
    /// Returns the entity's TUI symbol.
    pub fn symbol(&self) -> char {
        self.symbol
    }
    
    /// Returns the entity's immutable max health.
    pub fn max_health(&self) -> u32 {
        self.max_health
    }
    
    /// Returns the entity's immutable max energy.
    pub fn max_energy(&self) -> u32 {
        self.max_energy
    }

    /// Returns the entity's name as a displayable string slice.
    pub fn display_name(&self) -> &str {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        std::str::from_utf8(&self.name[..len]).unwrap_or("[Name Error]")
    }

    /// Returns the entity's current map position.
    pub fn position(&self) -> MapPosition {
        self.status.position
    }

    /// Returns the entity's current health.
    pub fn health(&self) -> u32 {
        self.status.health
    }
    
    /// Returns the entity's current energy.
    pub fn energy(&self) -> u32 {
        self.status.energy
    }

    /// Returns the entity's team ID.
    pub fn team(&self) -> u32 {
        self.status.team
    }

    /// Checks if the entity is currently selected in the UI.
    pub fn is_selected(&self) -> bool {
        self.status.is_selected
    }

    /// Checks if the entity is controlled by AI.
    pub fn is_ai(&self) -> bool{
        self.status.is_ai
    }
    
    /// Checks if the entity can take any action (i.e., not stunned).
    pub fn is_active(&self) -> bool {
        self.status.stunned_for_turns == 0
    }

    /// Checks if the entity is currently stunned.
    pub fn is_stunned(&self) -> bool {
        self.status.stunned_for_turns > 0
    }
    
    /// Checks if the entity's health is zero or less.
    pub fn is_dead(&self) -> bool {
        self.status.health == 0
    }
  
    pub fn damage(&self) -> u32 { self.status.damage }

    pub fn attack_range(&self) -> u32 { self.status.attack_range }

    /// Checks if the entity has enough energy to perform an action cost.
    pub fn can_act(&self, cost: u32) -> bool {
        self.is_active() && self.status.energy >= cost
    }

    // =========================================================================
    //                           MUTABLE OPERATIONS
    // =========================================================================
    
    /// Sets the entity's current map position (used by movement logic).
    pub fn set_position(&mut self, pos: MapPosition) {
        self.status.position = pos;
    }

    /// Sets the entity's team ID.
    pub fn set_team(&mut self, team: u32) {
        self.status.team = team;
    }
    
    /// Sets the selection state for the UI.
    pub fn set_selected(&mut self, b: bool) {
        self.status.is_selected = b;
    }
    
    /// Sets the AI control state.
    pub fn set_ai(&mut self, b: bool){
        self.status.is_ai = b;
    }

    /// Decreases the stun counter by 1.
    pub fn reduce_stun(&mut self) {
        if self.status.stunned_for_turns > 0 {
            self.status.stunned_for_turns -= 1;
        }
    }
    
    
    /// Attempts to consume energy for an action. Returns true if successful.
    pub fn consume_energy(&mut self, cost: u32) -> bool {
        if self.can_act(cost) {
            self.status.energy -= cost;
            true
        } else {
            false
        }
    }
    
    /// Refills energy back to max, but only if not stunned.
    pub fn refill_energy(&mut self) {
        if self.is_active() {
            self.status.energy = self.max_energy; 
        }
    }
    
    /// Applies damage and checks if the entity is dead.
    pub fn take_damage(&mut self, amount: u32) -> bool {
        self.status.health = self.status.health.saturating_sub(amount);
        self.status.health == 0
    }

    /// Heals the entity, capping health at max_health.
    pub fn heal(&mut self, amount: u32) {
        if self.is_dead() { return; }
        self.status.health = self.status.health.saturating_add(amount).min(self.max_health);
    }

    /// Heals the entity, returning the excess amount if max_health is exceeded.
    pub fn overheal(&mut self, amount: u32) -> u32 {
        if self.is_dead() { return 0; }

        let new_health = self.status.health.saturating_add(amount);
        let overheal_amount = new_health.saturating_sub(self.max_health);
        
        self.status.health = new_health.min(self.max_health);
        overheal_amount 
    }
}