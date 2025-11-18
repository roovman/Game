// src/lib/state/actions.rs

use glam::IVec2;

#[derive(Debug, Clone)]
pub enum Action {
    Move { unit_id: u32, target_pos: IVec2 },
    Attack { attacker_id: u32, target_id: u32 },
    EndTurn,
    Quit,
    Click { pos: IVec2 }, // Для обробки кліків
    BuildWall { target_pos: IVec2 },
}