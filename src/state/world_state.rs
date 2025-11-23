use crate::map::{map::Map, position::MapPosition};
use crate::specials::{entity::{Entity, EntityID}, powerup::PowerupType};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct WorldState {
    pub map: Map, 
    pub entities: Vec<Entity>, 
    pub next_entity_id: EntityID,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
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

    pub fn spawn_entity(
        &mut self, 
        pos: MapPosition, 
        symbol: char, 
        name: String, 
        max_health: u32, 
        max_energy: u32, 
        damage: u32,
        attack_range: u32,
        team: u32,       
        is_ai: bool,     
    ) -> Option<EntityID> {
        
        // 1. Check if the tile is standable
        if !self.map.is_standable(&pos) { return None; }

        let id = self.next_entity_id;
        self.next_entity_id += 1;

        // 2. Use the new Entity::new constructor (already correct)
        let mut new_entity = Entity::new(
            id, 
            symbol, 
            name, 
            pos, 
            team, 
            max_health, 
            max_energy,
            damage,
            attack_range
        );

        if is_ai {
            new_entity.set_ai(true);
        }

        self.entities.push(new_entity);

        // 3. Update the map tile
        if let Some(tile) = self.map.get_tile_mut(&pos) {
            // FIX: Use set_entity_id mutator
            tile.set_entity_id(Some(id));
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
            tile.set_type(crate::map::tile::TileType::WalkableGeneric);
        }
    }

    fn clear_pos(&mut self, pos: MapPosition) {
    if let Some(tile) = self.map.get_tile(&pos) {
        // FIX: Use tile.entity_id() accessor
        if let Some(id_to_clear) = tile.entity_id() { 
            // CRITICAL FIX: The logic must REMOVE the entity with the matching ID.
            // If the closure returns TRUE, the element is KEPT.
            // We want to KEEP entities whose IDs are NOT the ID_TO_CLEAR.
            self.entities.retain(|e| e.id() != id_to_clear); // Use e.id() accessor
        }
    }
    if let Some(tile) = self.map.get_tile_mut(&pos) {
        // Correct, uses Tile mutators
        tile.set_entity_id(None);
        tile.set_powerup(PowerupType::None);
    }
}

pub fn get_entity(&self, id: EntityID) -> Option<&Entity> {
    // FIX: Use e.id() accessor
    self.entities.iter().find(|e| e.id() == id)
}

pub fn get_entity_mut(&mut self, id: EntityID) -> Option<&mut Entity> {
    // FIX: Use e.id() accessor
    self.entities.iter_mut().find(|e| e.id() == id)
}

pub fn get_entity_id_at(&self, pos: MapPosition) -> Option<EntityID> {
    // Correct, uses tile.entity_id() accessor
    self.map.get_tile(&pos).and_then(|t| t.entity_id()) 
}
}