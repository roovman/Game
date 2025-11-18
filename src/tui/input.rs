// src/lib/tui/input.rs

use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use std::time::Duration;
use glam::IVec2;

use crate::state::actions::Action;
use crate::state::GameState;
use color_eyre::Result;

pub fn handle_events(game_state: &GameState) -> Result<Option<Action>> {
    // Чекаємо подію (Poll)
    if event::poll(Duration::from_millis(16))? {
        let event = event::read()?;
        
        match event {
            // Клавіатура
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => return Ok(Some(Action::Quit)),
                    _ => {}
                }
            }
            // Миша
            Event::Mouse(mouse) => {
        let x = mouse.column as i32;
        let y = mouse.row as i32;
        let map_x = x - 1; // Координата X на мапі (зміщення рамки)
        let map_y = y - 1; // Координата Y на мапі (зміщення рамки)
        let target_pos = IVec2::new(map_x, map_y);

        // 1. Обробка ЛІВОГО кліка (Рух)
        if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
            // ... (Ваша існуюча логіка руху тут)
            
            // Якщо координати валідні для карти
            if map_x >= 0 && map_y >= 0 && map_x < game_state.map.width as i32 && map_y < game_state.map.height as i32 {
                // ... (повернення Action::Move)
            }
        }
        
        // !!! 2. Обробка ПРАВОГО кліка (Будівництво стіни) !!!
        if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
            if map_x >= 0 && map_y >= 0 && map_x < game_state.map.width as i32 && map_y < game_state.map.height as i32 {
                
                return Ok(Some(Action::BuildWall {
                    target_pos,
                }));
            }
        }
    }
            _ => {}
        }
    }
    // Подій не було
    Ok(None)
}