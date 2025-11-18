// src/entities/mod.rs (ПОВИННО БУТИ ТАК)

pub mod entity; // <--- Оголошує, що існує файл entity.rs

// Припускаємо, що Team визначено тут
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Player,
    AI,
    Neutral,
}

// Експортуємо Entity для зручності:
pub use entity::Entity; 