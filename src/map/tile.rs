// src/map/tile.rs
use crate::specials::{PowerupType};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    WalkableGeneric,
    Wall,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    tile_type: TileType,
    symbol: char,
    entity_id: Option<u32>,
    powerup: PowerupType,
}

impl Tile {
    pub fn new(tile_type: TileType, symbol: char) -> Self {
        Self {
            tile_type,
            symbol,
            entity_id: None,
            powerup: PowerupType::None,
        }
    }

    pub fn walkable() -> Self { Self::new(TileType::WalkableGeneric, '.') }
    pub fn wall() -> Self { Self::new(TileType::Wall, '█') }

    pub fn tile_type(&self) -> TileType { self.tile_type }
    pub fn symbol(&self) -> char { self.symbol }
    pub fn entity_id(&self) -> Option<u32> { self.entity_id }

    pub fn is_walkable(&self) -> bool { matches!(self.tile_type, TileType::WalkableGeneric) }
    pub fn is_solid(&self) -> bool { matches!(self.tile_type, TileType::Wall) }
    pub fn is_occupied(&self) -> bool { self.entity_id.is_some() }
    
    pub fn can_stand(&self) -> bool { self.is_walkable() && !self.is_occupied() }
    
    pub fn set_entity(&mut self, id: Option<u32>) { self.entity_id = id; }
    pub fn set_powerup(&mut self, powerup: PowerupType) { self.powerup = powerup; }

    pub fn take_powerup(&mut self) -> PowerupType{ 
        let powerup = self.powerup;
        self.powerup = PowerupType::None;   
        powerup
    }

    /// Змінює тип тайла і скидає його стан
    pub fn transform(&mut self, new_type: TileType) {
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
}