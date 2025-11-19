use crate::map::{map::Map, position::MapPosition};
use crate::specials::{entity::{Entity, EntityID}, powerup::PowerupType};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub map: Map, 
    pub entities: Vec<Entity>, 
    pub next_entity_id: EntityID,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            map: Map::new(100, 25),
            entities: Vec::new(),
            next_entity_id: 0,
        }
    }

    // --- Persistence ---
    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub fn load(filename: &str) -> std::io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let state = serde_json::from_reader(reader)?;
        Ok(state)
    }

    // --- Logic Helpers ---

    pub fn spawn_entity(&mut self, pos: MapPosition, symbol: char, health: u32) -> Option<EntityID> {
        if !self.map.is_standable(&pos) { return None; }

        let id = self.next_entity_id;
        self.next_entity_id += 1;

        self.entities.push(Entity::new(id, pos, symbol));

        if let Some(tile) = self.map.get_tile_mut(&pos) {
            tile.entity_id = Some(id);
        }
        Some(id)
    }

    pub fn build_wall(&mut self, pos: MapPosition) -> bool {
        self.clear_pos(pos);
        self.map.build_wall(&pos)
    }

    pub fn build_floor(&mut self, pos: MapPosition) {
        self.clear_pos(pos);
        if let Some(tile) = self.map.get_tile_mut(&pos) {
            *tile = crate::map::tile::Tile::new_walkable();
        }
    }

    fn clear_pos(&mut self, pos: MapPosition) {
        if let Some(tile) = self.map.get_tile(&pos) {
            if let Some(id) = tile.entity_id {
                self.entities.retain(|e| e.id != id);
            }
        }
        if let Some(tile) = self.map.get_tile_mut(&pos) {
            tile.entity_id = None;
            tile.powerup = PowerupType::None;
        }
    }

    pub fn get_entity(&self, id: EntityID) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }
    
    pub fn get_entity_mut(&mut self, id: EntityID) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn get_entity_id_at(&self, pos: MapPosition) -> Option<EntityID> {
        self.map.get_tile(&pos).and_then(|t| t.entity_id)
    }
}