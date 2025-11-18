// src/tui/draw.rs (ОНОВЛЕНА ВЕРСІЯ)

use ratatui::{
    backend::Backend, 
    Frame, 
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color}, 
    layout::{Constraint, Layout}, 
};

use crate::state::GameState;
use crate::map::tile::TileType; 
use crate::specials::powerup::PowerupType; // ⭐️ Новий імпорт

// Допоміжна функція: Визначає символ Powerup
fn get_powerup_visuals(powerup_type: PowerupType) -> Option<(char, Color)> {
    match powerup_type {
        PowerupType::HealingPotion => Some(('+', Color::Red)),
        PowerupType::SpeedBoost => Some(('s', Color::Cyan)),
        PowerupType::WallBreaker => Some(('w', Color::LightRed)),
        PowerupType::None => None,
    }
}

pub fn ui<B: Backend>(f: &mut Frame, game_state: &GameState) { 
    let size = f.area(); 
    
    // Розділяємо екран
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(size);

    let map_area = layout[0];

    // Створюємо блок для карти
    let map_block = Block::default()
        .title("MAP")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(&map_block, map_area);
    let inner_area = map_block.inner(map_area);

    // === ЛОГІКА МАРУВАННЯ КЛІТИНОК ===
    for y in 0..game_state.map.height {
        for x in 0..game_state.map.width {
            
            // 💡 Використовуємо Zero-Cost Access та перевіряємо, чи клітинка в межах
            let tile = game_state.map.get_tile_i32(x, y);
            
            if let Some(tile) = tile {
                
                // 1. Визначаємо базовий символ і колір (Ваша градієнтна логіка)
                let (mut symbol, mut color) = match tile.tile_type {
                    TileType::WalkableGeneric => (tile.symbol, Color::Rgb(((2*x)%255) as u8, 165, ((5*y)%255) as u8)),
                    TileType::Wall => (tile.symbol, Color::Rgb(((2*x)%255) as u8, 165, ((5*y)%255) as u8)),
                };

                // 2. ⚡️ ПЕРЕКРИТТЯ: Powerup
                if tile.has_powerup() {
                    if let Some((p_sym, p_color)) = get_powerup_visuals(tile.powerup) {
                        symbol = p_sym;
                        color = p_color;
                    }
                }
                
                // 3. 👤 ПЕРЕКРИТТЯ: Сутність (Гравець)
                // Шукаємо сутність, використовуючи ID з клітинки
                if let Some(entity_id) = tile.entity_id {
                    let entity_id_u32 = entity_id; // Ваше рішення використовувати u32 для ID

                    // Знаходимо сутність у векторі GameState::entities
                    if let Some(entity) = game_state.entities.iter().find(|e| e.id == entity_id_u32) {
                        symbol = entity.symbol;
                        color = Color::White; // Виділяємо сутність
                    }
                }

                // 4. Встановлюємо символ і стиль у буфері
                if (x as u16) < inner_area.width && (y as u16) < inner_area.height {
                    f.buffer_mut()
                        .get_mut(inner_area.x + x as u16, inner_area.y + y as u16)
                        .set_symbol(&symbol.to_string())
                        .set_style(Style::default().fg(color));
                }
            }
        }
    }
    // === КІНЕЦЬ ЛОГІКИ МАРУВАННЯ КЛІТИНОК ===
    
    // 2. Відображення Статусу/Дебагу
    let status_block = Block::default()
        .title("STATUS / DEBUG")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));

    let mode_text = format!("Режим: {:?} | ", game_state.current_mode);
    let debug_info = Paragraph::new(format!("{}{}", mode_text, game_state.debug_message.clone()))
        .block(status_block);
        
    f.render_widget(debug_info, layout[1]);
}