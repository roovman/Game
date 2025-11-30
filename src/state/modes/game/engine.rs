use crate::state::WorldState;
use crate::specials::entity::{EntityID, MOVE_COST, ATTACK_COST};
use crate::map::position::MapPosition;

#[derive(Debug, Clone)]
pub enum TurnResult {
    TurnContinues,
    TurnChanged(u32),
}

#[derive(Debug, Clone)]
pub enum ActionResult {
    SuccessMove,
    SuccessAttack { damage: u32, target_died: bool },
    Fail(String),
}

pub struct GameEngine {
    world: WorldState, // Приватне поле
}

impl GameEngine {
    pub fn new(world: WorldState) -> Self {
        Self { world }
    }

    pub fn world(&self) -> &WorldState {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut WorldState {
        &mut self.world
    }

    // =========================================================================
    //                            PUBLIC API
    // =========================================================================

    pub fn move_entity(&mut self, id: EntityID, target_pos: MapPosition) -> ActionResult {
        if let Err(e) = self.validate_actor(id, MOVE_COST) { return e; }

        if !self.world.map.is_standable(&target_pos) {
            return ActionResult::Fail("Position blocked".to_string());
        }

        let old_pos = self.world.get_entity(id).unwrap().position();
        let cost = old_pos.manhattan_distance(&target_pos) * MOVE_COST;

        if let Some(e) = self.world.get_entity(id) {
            if !e.can_act(cost) { return ActionResult::Fail("Not enough energy for distance".to_string()); }
        }

        self.apply_energy_cost(id, cost);
        
        if let Some(e) = self.world.get_entity_mut(id) {
            e.set_position(target_pos);
        }

        self.update_map_placement(old_pos, target_pos, id);

        ActionResult::SuccessMove
    }

    pub fn attack_entity(&mut self, attacker_id: EntityID, target_id: EntityID) -> ActionResult {
        if let Err(e) = self.validate_actor(attacker_id, ATTACK_COST) { return e; }
        
        if attacker_id == target_id {
            return ActionResult::Fail("Cannot attack self".to_string());
        }

        let (damage, range, attacker_pos, target_pos) = {
            let att = self.world.get_entity(attacker_id).unwrap();
            let tgt = match self.world.get_entity(target_id) {
                Some(t) => t,
                None => return ActionResult::Fail("Target lost".to_string()),
            };
            (att.damage(), att.attack_range(), att.position(), tgt.position())
        };

        if attacker_pos.manhattan_distance(&target_pos) > range {
            return ActionResult::Fail("Target out of range".to_string());
        }

        self.apply_energy_cost(attacker_id, ATTACK_COST);

        let mut target_died = false;
        if let Some(tgt) = self.world.get_entity_mut(target_id) {
            target_died = tgt.take_damage(damage);
        }

        if target_died {
            self.clear_map_tile(target_pos);
        }

        ActionResult::SuccessAttack { damage, target_died }
    }

    pub fn end_turn(&mut self) -> TurnResult {
        let mut active_teams: Vec<u32> = self.world.entities.iter()
            .filter(|e| !e.is_dead())
            .map(|e| e.team())
            .collect();
        active_teams.sort();
        active_teams.dedup();

        if active_teams.is_empty() { return TurnResult::TurnContinues; }

        let current = self.world.current_team_turn;
        let next_team = if let Some(pos) = active_teams.iter().position(|&t| t == current) {
            active_teams[(pos + 1) % active_teams.len()]
        } else {
            active_teams[0]
        };

        self.world.current_team_turn = next_team;
        
        for e in self.world.entities.iter_mut() {
            if e.team() == next_team {
                e.reduce_stun();
                e.refill_energy();
            }
        }
        TurnResult::TurnChanged(next_team)
    }

    pub fn current_team(&self) -> u32 {
        self.world.current_team_turn
    }

    // =========================================================================
    //                            PRIVATE HELPERS
    // =========================================================================

    fn validate_actor(&self, id: EntityID, base_cost: u32) -> Result<(), ActionResult> {
        let ent = self.world.get_entity(id)
            .ok_or_else(|| ActionResult::Fail("Entity not found".to_string()))?;

        if ent.team() != self.world.current_team_turn {
            return Err(ActionResult::Fail("Not your turn!".to_string()));
        }

        if !ent.can_act(base_cost) {
            return Err(ActionResult::Fail("Not enough energy".to_string()));
        }

        Ok(())
    }

    fn apply_energy_cost(&mut self, id: EntityID, cost: u32) {
        if let Some(e) = self.world.get_entity_mut(id) {
            e.consume_energy(cost);
        }
    }

    fn clear_map_tile(&mut self, pos: MapPosition) {
        if let Some(tile) = self.world.map.get_tile_mut(&pos) {
            tile.set_entity(None);
        }
    }

    fn update_map_placement(&mut self, old_pos: MapPosition, new_pos: MapPosition, id: EntityID) {
        self.clear_map_tile(old_pos);

        if let Some(new_tile) = self.world.map.get_tile_mut(&new_pos) {
            new_tile.set_entity(Some(id));
            new_tile.take_powerup();
        }
    }
}