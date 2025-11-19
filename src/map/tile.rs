// src/map/tile.rs (ОЧИЩЕНО)

use crate::specials::PowerupType;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    /// Звичайна прохідна клітинка
    WalkableGeneric, 
    /// Стіна (непрохідна, тверда)
    Wall,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub tile_type: TileType,
    pub symbol: char,
    pub entity_id: Option<u32>,
    pub powerup: PowerupType,
}

impl Tile {
    /// Створює новий Tile. Entity ID та Powerup за замовчуванням None.
    pub fn new(tile_type: TileType, symbol: char) -> Self {
        Tile {
            tile_type,
            symbol,
            entity_id: None,
            powerup: PowerupType::None,
        }
    }

    /// Допоміжний конструктор для прохідної клітинки.
    pub fn new_walkable() -> Self {
        Self::new(TileType::WalkableGeneric, '.')
    }

    /// Допоміжний конструктор для стіни.
    pub fn new_wall() -> Self {
        Self::new(TileType::Wall, '█')
    }

    /// Перевіряє, чи можна пересуватися на цю клітинку.
    pub fn is_walkable(&self) -> bool {
        matches!(self.tile_type, TileType::WalkableGeneric)
    }

    /// Перевіряє, чи є клітинка фізично твердою.
    pub fn is_solid(&self) -> bool {
        matches!(self.tile_type, TileType::Wall)
    }

    /// Перевіряє, чи зайнята клітинка сутністю.
    pub fn is_entity_occupied(&self) -> bool {
        self.entity_id.is_some()
    }

    /// Перевіряє, чи клітинка придатна для того, щоб на ній стояти.
    pub fn is_standable(&self) -> bool {
        self.is_walkable() && !self.is_entity_occupied()
    }

    /// Перевіряє, чи містить клітинка Powerup.
    pub fn has_powerup(&self) -> bool {
        self.powerup.is_some()
    }

    /// Змінює базовий тип клітинки, оновлюючи символ та очищаючи динамічний вміст.
    pub fn set_type(&mut self, new_type: TileType) {
        self.tile_type = new_type;
        self.symbol = match new_type {
            TileType::WalkableGeneric => '.',
            TileType::Wall => '█',
        };
        
        if self.is_solid() {
            self.entity_id = None;
            self.powerup = PowerupType::None; 
        }
    }
    
    /// Бере Powerup, замінюючи його на PowerupType::None.
    pub fn take_powerup(&mut self) -> PowerupType {
        use std::mem;
        mem::replace(&mut self.powerup, PowerupType::None)
    }
}