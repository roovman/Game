// src/tui/draw.rs (ВИПРАВЛЕНО)
// src/tui/draw.rs

use ratatui::{
    backend::Backend, 
    Frame, 
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color}, 
    layout::{Constraint, Layout}, 
};

use crate::state::GameState;
use crate::map::tile::TileType; 
// ...
// ВИПРАВЛЕНО E0107: Frame без дженерика <B>
pub fn ui<B: Backend>(f: &mut Frame, game_state: &GameState) { 
    // ВИПРАВЛЕНО Warning: Використовуємо .area() замість .size()
    let size = f.area(); 
    
    // Розділяємо екран
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(size);

    // ... (Map rendering)
    let map_area = layout[0];

    // Створюємо блок для карти, щоб вона мала рамку і заголовок
    let map_block = Block::default()
        .title("MAP")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    // Малюємо блок на першій області
    f.render_widget(&map_block, map_area);

    // Отримуємо внутрішню область, щоб малювати всередині рамки
    let inner_area = map_block.inner(map_area);

    // === ЛОГІКА МАРУВАННЯ КЛІТИНОК ===
    for y in 0..game_state.map.height {
        for x in 0..game_state.map.width {
            let tile = &game_state.map.tiles[y as usize][x as usize];
            
            // 1. Визначаємо символ і колір
            let (symbol, color) = match tile.tile_type {
                TileType::WalkableGeneric => (tile.symbol, Color::White), // '.' та сірий
                TileType::Wall => (tile.symbol, Color::Rgb(255, 165, 0)), // '█' та помаранчевий
                // Додайте інші типи пізніше
                _ => ('?', Color::White),
            };

            // Перевіряємо, чи поміщається клітинка в межі inner_area
            if (x as u16) < inner_area.width && (y as u16) < inner_area.height {
                // 2. Встановлюємо символ і стиль у буфері
                // Inner_area.x і y використовуються як зміщення
                f.buffer_mut()
                    .get_mut(inner_area.x + x as u16, inner_area.y + y as u16)
                    .set_symbol(&symbol.to_string())
                    .set_style(Style::default().fg(color));
            }
        }
    }
    // === КІНЕЦЬ ЛОГІКИ МАРУВАННЯ КЛІТИНОК ===
    // 2. Відображення Статусу/Дебагу
    let status_block = Block::default()
        .title("STATUS / DEBUG")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));
        
    // ВИПРАВЛЕНО E0609: debug_message вже існує у GameState (виправлено в mod.rs)
    let debug_info = Paragraph::new(game_state.debug_message.clone())
        .block(status_block);
        
    f.render_widget(debug_info, layout[1]);
}