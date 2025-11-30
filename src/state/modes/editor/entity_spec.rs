use crate::specials::entity::{Entity, EntityID};
use crate::map::position::MapPosition;

#[derive(Debug, Clone)]
pub struct EntitySpec {
    pub team: u32,
    pub hp: u32,
    pub energy: u32,
    pub damage: u32,
    pub range: u32,
    pub is_ai: bool,
    pub symbol: char,
}

impl Default for EntitySpec {
    fn default() -> Self {
        EntitySpec {
            team: 1,          
            hp: 50,
            energy: 50,
            damage: 5,
            range: 4,
            is_ai: true,       
            symbol: '@', 
        }
    }
}

impl EntitySpec {
    pub fn update_symbol(&mut self) {
        if self.team == 1 {
            self.symbol = '@'; 
        } else {
            let base = b'A'; 
            let offset = (self.team as u8).wrapping_add(0) % 26; 
            self.symbol = (base + offset) as char;
        }
        self.is_ai = self.team != 1;
    }

    pub fn toggle_ai(&mut self) {
        self.is_ai = !self.is_ai;
    }

    pub fn to_entity(&self, id: EntityID, pos: MapPosition) -> Entity {
        let name = if self.team == 1 { 
            "Player".to_string() 
        } else { 
            "Enemy".to_string() 
        };

        let mut entity = Entity::new(
            id,
            self.symbol,
            name,
            pos,
            self.team,
            self.hp,
            self.energy,
            self.damage,
            self.range
        );

        entity.set_ai(self.is_ai);
        
        entity
    }
}