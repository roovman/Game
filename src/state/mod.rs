// src/state/mod.rs (Зміни)
pub mod actions;

use crate::map::map::Map;
use crate::entities::{Entity, Team}; // <--- ВИПРАВЛЕНО: Імпортуємо напряму з кореня 'entities'
use crate::map::tile::{Tile, TileType}; 
use actions::Action;
use glam::IVec2;
// Додаємо SpecialAbility, щоб виправити помилку E0433 (SpecialAbility)

// Єдине джерело істини
pub struct GameState {
    pub map: Map,
    pub entities: Vec<Entity>,
    pub current_team: Team,
    pub is_running: bool,
    pub debug_message: String,
}

impl GameState {
    pub fn new() -> Self {
        let mut map = Map::new(20, 10);
        
        map.tiles[5][5] = Tile::new(TileType::Wall, '█'); // '█' - символ блоку

        // Додаємо лінію стін для візуалізації
        for x in 2..15 {
            if (x as usize) < map.width as usize {
                map.tiles[2][x as usize] = Tile::new(TileType::Wall, '█');
            }
        }

        // Вертикальна стіна
        for y in 3..8 {
            if (y as usize) < map.height as usize {
                map.tiles[y as usize][14] = Tile::new(TileType::Wall, '█');
            }
        }
        GameState {
            map,
            entities: vec![
                Entity {
                    id: 1, team: Team::Player, pos: IVec2::new(1, 1), health: 10, 
                    max_move: 3, attack_range: 1, symbol: 'P'
                }
            ],
            current_team: Team::Player,
            is_running: true,
            // FIX E0063: Додаємо відсутнє поле
            debug_message: String::from("Натисніть 'q' для виходу або клікніть."), 
        }
    }

    /// Обробляє дію, мутуючи GameState. Це серце вашого рушія.
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.is_running = false;
            },
            // Action::Move { unit_id, target_pos } => {
            //     if let Some(unit) = self.entities.iter_mut().find(|e| e.id == unit_id) {
            //         // ТУТ БУДЕ СКЛАДНА ЛОГІКА ПЕРЕВІРКИ РУХУ
            //         // if self.map.is_walkable(special_ability.x as u8, target_pos.y as u8) {
            //         //      // Перевірка, чи в межах max_move, і чи не перетинає стіни
            //         //     unit.pos = target_pos;
            //         // }
            //     }
            // },
            Action::EndTurn => {
                self.current_team = match self.current_team {
                    Team::Player => Team::AI,
                    Team::AI => Team::Player,
                    _ => Team::Player,
                };
            }
            Action::BuildWall { target_pos } => {
                let target_x = target_pos.x as u8;
                let target_y = target_pos.y as u8;

                // Перевіряємо, чи позиція знаходиться в межах мапи
                if target_x < self.map.width && target_y < self.map.height {
                    // Перевіряємо, чи немає там вже сутності
                    if self.entities.iter().all(|e| e.pos != target_pos) {
                        
                        // Створюємо нову плитку-стіну
                        let new_wall = Tile::new(TileType::Wall, '█');
                        
                        // Оновлюємо мапу!
                        self.map.tiles[target_y as usize][target_x as usize] = new_wall;
                        
                        self.debug_message = format!("Стіна збудована на ({}, {}).", target_pos.x, target_pos.y);
                    } else {
                        self.debug_message = format!("Неможливо збудувати стіну на клітинці ({}, {}), там вже є юніт!", target_pos.x, target_pos.y);
                    }
                } else {
                    self.debug_message = "Неможливо збудувати стіну за межами карти.".to_string();
                }
            }
            _ => {}
        }
    }
}