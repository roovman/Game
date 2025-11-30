// src/state/world_state.rs

use crate::map::map::Map;
use crate::map::position::MapPosition;
use crate::specials::entity::{Entity, EntityID};
use crate::specials::powerup::PowerupType;
use crate::map::tile::TileType; // Не забудь цей імпорт!
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct WorldState {
    pub map: Map, 
    pub entities: Vec<Entity>, 
    pub next_entity_id: EntityID,
    pub current_team_turn: u32,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
            map: Map::new(100, 25),
            entities: Vec::new(),
            next_entity_id: 0,
            current_team_turn: 1,
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

    /// Додає готову сутність у світ і прив'язує її до карти.
    pub fn add_entity(&mut self, entity: Entity) -> EntityID {
        // 1. Запам'ятовуємо дані перед переміщенням (Move)
        let id = entity.id();
        let pos = entity.position();
        
        // 2. Додаємо у сховище
        self.entities.push(entity);
        
        // 3. Оновлюємо карту (щоб тайл знав, що на ньому хтось є)
        if let Some(tile) = self.map.get_tile_mut(pos) {
            tile.set_entity(Some(id));
        }
        
        id
    }

    pub fn next_id(&mut self) -> EntityID {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    // Зручний хелпер для спавну (використовується в меню)
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
        
        if !self.map.is_standable(pos) { return None; }

        let id = self.next_id();

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

        Some(self.add_entity(new_entity))
    }

    pub fn build_wall(&mut self, pos: MapPosition) -> bool {
        self.clear_pos(pos);
        self.map.build_wall(pos)
    }

    pub fn build_floor(&mut self, pos: MapPosition) {
        self.clear_pos(pos);
        if let Some(tile) = self.map.get_tile_mut(pos) {
            tile.transform(TileType::WalkableGeneric);
        }
    }

    fn clear_pos(&mut self, pos: MapPosition) {
        // Спочатку знаходимо ID того, кого треба видалити
        let id_to_remove = self.map.get_tile(pos).and_then(|t| t.entity_id());

        // Якщо там хтось був -> видаляємо з вектора entities
        if let Some(id) = id_to_remove {
            self.entities.retain(|e| e.id() != id);
        }

        // Потім очищаємо сам тайл (включаючи паверапи)
        if let Some(tile) = self.map.get_tile_mut(pos) {
            tile.set_entity(None);
            tile.set_powerup(PowerupType::None);
        }
    }

    pub fn get_entity(&self, id: EntityID) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id() == id)
    }

    pub fn get_entity_mut(&mut self, id: EntityID) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id() == id)
    }

    pub fn get_entity_id_at(&self, pos: MapPosition) -> Option<EntityID> {
        self.map.get_tile(pos).and_then(|t| t.entity_id()) 
    }
}