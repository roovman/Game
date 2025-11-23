// src/map/map.rs

use serde::{Serialize, Deserialize};

use super::tile::{Tile, TileType};
use super::position::MapPosition; 

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    width: i32, 
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    /// Створює нову карту заданої ширини та висоти, ініціалізуючи її прохідними клітинками.
    pub fn new(width: i32, height: i32) -> Self {
        let default_tile = Tile::new_walkable(); 
        let size = (width * height) as usize;
        let tiles = vec![default_tile; size];
        
        Map { width, height, tiles }
    }
    
    // =========================================================================
    //                            PRIVATE METHODS 
    // =========================================================================

    /// Приватна функція: Перетворення 2D-координат (i32) на 1D-індекс.
    fn get_index_i32(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            // Формула: index = y * width + x
            let index = (y * self.width + x) as usize;
            Some(index)
        } else {
            None 
        }
    }

    // =========================================================================
    //                            READ-ONLY ACCESSORS 
    // =========================================================================

    /// Повертає ширину карти.
    pub fn width(&self) -> i32 {
        self.width
    }
    
    /// Повертає висоту карти.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Отримує незмінну клітинку (Tile) за MapPosition.
    pub fn get_tile(&self, pos: &MapPosition) -> Option<&Tile> {
        self.get_tile_i32(pos.x(), pos.y())
    }

    /// Отримує змінну клітинку (Tile) за MapPosition.
    pub fn get_tile_mut(&mut self, pos: &MapPosition) -> Option<&mut Tile> {
        self.get_tile_mut_i32(pos.x(), pos.y())
    }
    
    /// Отримує незмінну клітинку за прямими i32 координатами.
    pub fn get_tile_i32(&self, x: i32, y: i32) -> Option<&Tile> {
        match self.get_index_i32(x, y) {
            Some(index) => self.tiles.get(index),
            None => None,
        }
    }

    /// Отримує змінну клітинку за прямими i32 координатами.
    pub fn get_tile_mut_i32(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        match self.get_index_i32(x, y) {
            Some(index) => self.tiles.get_mut(index),
            None => None,
        }
    }


    // =========================================================================
    //                            LOGIC METHODS 
    // =========================================================================

    /// Перевіряє, чи можна ходити на клітинку (без перешкод сутностями).
    pub fn is_walkable(&self, pos: &MapPosition) -> bool {
        if let Some(tile) = self.get_tile(pos) {
            tile.is_walkable()
        } else {
            false 
        }
    }

    /// Перевіряє, чи можна стати на клітинку (прохідна і не зайнята сутністю).
    pub fn is_standable(&self, pos: &MapPosition) -> bool {
        if let Some(tile) = self.get_tile(pos) {
            tile.is_standable()
        } else {
            false
        }
    }

    /// Будує стіну на заданій позиції, якщо клітинка прохідна.
    pub fn build_wall(&mut self, pos: &MapPosition) -> bool {
        if let Some(tile) = self.get_tile_mut(pos) {
            if tile.is_walkable() {
                // Використовуємо інкапсульований метод Tile::set_type
                tile.set_type(TileType::Wall);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}