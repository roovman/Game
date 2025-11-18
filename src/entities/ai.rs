// src/entities/mod.rs

pub mod entity;
// Тут може бути ai.rs, який ми поки не наповнюємо
// pub mod ai; 

pub use entity::Entity;

// Team: Приналежність сутності
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    AI,
    Neutral,
}