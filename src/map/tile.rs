// src/map/tile.rs

// TileType: Навіть якщо поки не використовується, це забезпечує цілісність
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    WalkableGeneric,
    Wall,
}

// Tile: Що знаходиться в кожній клітинці
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub symbol: char,
}

impl Tile {
    pub fn new(tile_type: TileType, symbol: char) -> Self {
        Tile { tile_type, symbol }
    }
}