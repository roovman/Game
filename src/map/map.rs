// src/map/map.rs

use super::tile::{Tile, TileType};

pub struct Map {
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<Vec<Tile>>,
    // Додайте координати для TUI-рендерингу, якщо потрібно:
    // pub render_offset: glam::IVec2,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Self {
        // Створюємо сітку з WalkableGeneric клітинок
        let default_tile = Tile::new(TileType::WalkableGeneric, '.');
        
        let tiles = vec![
            vec![default_tile; width as usize]; 
            height as usize
        ];
        
        Map { width, height, tiles }
    }
    pub fn is_walkable(&self, x: u8, y: u8) -> bool {
        if x < self.width && y < self.height {
            // Припускаємо, що TileType::Wall має бути Unwalkable
            !matches!(self.tiles[y as usize][x as usize].tile_type, super::tile::TileType::Wall)
        } else {
            false
        }
    }
}