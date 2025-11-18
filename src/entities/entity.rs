// src/entities/entity.rs

// Використовуємо glam для позиції
use glam::IVec2; 
use super::Team;

// Entity: Щось, що рухається або має HP
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub id: u32,
    pub team: Team,
    pub pos: IVec2, // (x, y)
    pub health: u8,
    pub symbol: char,
    pub max_move: u8,
    pub attack_range: u8,
}

impl Entity {
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}