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
    /// Створює нову карту заданої ширини та висоти, заповнену землею.
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Map { 
            width, 
            height, 
            tiles: vec![Tile::walkable(); size] // Tile має derive(Clone), тому це працює швидко
        }
    }
    
    // =========================================================================
    //                            ACCESSORS (READ)
    // =========================================================================

    /// Універсальний метод отримання тайла.
    /// Приймає MapPosition, &MapPosition або (x, y).
    pub fn get_tile<P: Into<MapPosition>>(&self, pos: P) -> Option<&Tile> {
        let idx = self.pos_to_index(pos.into())?;
        self.tiles.get(idx)
    }

    /// Повертає ширину карти.
    pub fn width(&self) -> i32 { self.width }
    
    /// Повертає висоту карти.
    pub fn height(&self) -> i32 { self.height }

    /// Швидкий доступ до сирого масиву тайлів (для рендерингу).
    pub fn raw_tiles(&self) -> &[Tile] {
        &self.tiles
    }

    /// Перевіряє, чи знаходиться позиція в межах карти.
    pub fn in_bounds<P: Into<MapPosition>>(&self, pos: P) -> bool {
        let p = pos.into();
        p.x() >= 0 && p.x() < self.width && p.y() >= 0 && p.y() < self.height
    }

    // =========================================================================
    //                            ACCESSORS (WRITE)
    // =========================================================================

    /// Отримує змінне посилання на тайл.
    pub fn get_tile_mut<P: Into<MapPosition>>(&mut self, pos: P) -> Option<&mut Tile> {
        let idx = self.pos_to_index(pos.into())?;
        self.tiles.get_mut(idx)
    }

    // =========================================================================
    //                            LOGIC & UTILS
    // =========================================================================

    /// Конвертує 2D координати в 1D індекс вектора з перевіркою меж.
    fn pos_to_index(&self, pos: MapPosition) -> Option<usize> {
        if self.in_bounds(pos) {
            Some((pos.y() * self.width + pos.x()) as usize)
        } else {
            None
        }
    }

    /// Чи можна ходити (логіка тайла).
    pub fn is_walkable<P: Into<MapPosition>>(&self, pos: P) -> bool {
        self.get_tile(pos).map_or(false, |t| t.is_walkable())
    }

    /// Чи можна стояти (логіка тайла + відсутність інших сутностей).
    pub fn is_standable<P: Into<MapPosition>>(&self, pos: P) -> bool {
        self.get_tile(pos).map_or(false, |t| t.can_stand())
    }

    /// Будує стіну, якщо це можливо.
    pub fn build_wall<P: Into<MapPosition>>(&mut self, pos: P) -> bool {
        if let Some(tile) = self.get_tile_mut(pos) {
            if tile.is_walkable() {
                tile.transform(TileType::Wall);
                return true;
            }
        }
        false
    }
}