use super::engine::{GameEngine, ActionResult};
use crate::specials::entity::{EntityID, MOVE_COST}; 
use crate::map::position::MapPosition;
use super::pathfinding::Pathfinding;

enum AiDecision {
    Attack(EntityID),
    Move(MapPosition),
    Wait,
}

pub struct AiSystem;

impl AiSystem {
    pub fn perform_turn(engine: &mut GameEngine, team_id: u32) -> Vec<String> {
        let mut turn_logs = Vec::new();

        let ai_ids: Vec<EntityID> = engine.world().entities.iter()
            .filter(|e| e.team() == team_id && e.is_ai() && !e.is_dead())
            .map(|e| e.id())
            .collect();

        for id in ai_ids {
            let unit_logs = Self::process_entity_loop(engine, id);
            turn_logs.extend(unit_logs);
        }
        
        turn_logs
    }

    fn process_entity_loop(engine: &mut GameEngine, entity_id: EntityID) -> Vec<String> {
        let mut logs = Vec::new();

        for _ in 0..20 {
            let decision = Self::decide_next_action(engine, entity_id);

            match decision {
                AiDecision::Attack(target_id) => {
                    logs.push(format!("AI {} attacks {}!", entity_id, target_id));
                    engine.attack_entity(entity_id, target_id);
                    break;
                },
                AiDecision::Move(pos) => {
                    if let ActionResult::Fail(_) = engine.move_entity(entity_id, pos) {
                        break; 
                    }
                },
                AiDecision::Wait => {
                    break; 
                }
            }
        }
        logs
    }

    fn decide_next_action(engine: &GameEngine, entity_id: EntityID) -> AiDecision {
        let state = engine.world();
        
        let me = match state.get_entity(entity_id) {
            Some(e) if !e.is_dead() => e,
            _ => return AiDecision::Wait,
        };

        if !me.can_act(MOVE_COST) {
            return AiDecision::Wait;
        }

        let my_pos = me.position();
        let my_range = me.attack_range();

        let target = state.entities.iter()
            .filter(|e| e.team() != me.team() && !e.is_dead())
            .map(|e| (e.id(), e.position(), e.position().manhattan_distance(&my_pos)))
            .min_by_key(|(_, _, dist)| *dist);

        if let Some((target_id, target_pos, dist)) = target {
            if dist <= my_range {
                return AiDecision::Attack(target_id);
            } 
            
            if let Some(path) = Pathfinding::find_path(&state.map, my_pos, target_pos) {
                if let Some(&next_step) = path.first() {
                    if state.map.is_standable(next_step) {
                        return AiDecision::Move(next_step);
                    }
                }
            }
        }

        AiDecision::Wait
    }
}