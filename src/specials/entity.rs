use crate::map::position::MapPosition; 
use serde::{Serialize, Deserialize};
pub type EntityID = u32;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entity {
    pub id: EntityID,
    pub position: MapPosition,
    pub symbol: char,
    pub health: u32,
    pub is_selected: bool, 
}

impl Entity {
    pub fn new(id: EntityID, position: MapPosition, symbol: char) -> Self {
        Entity { 
            id, 
            position, 
            symbol, 
            health: 100,
            is_selected: false,
        }
    }
}