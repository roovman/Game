use ratatui::{
    Frame, 
    widgets::{Paragraph, Block, Borders, Wrap},
    style::{Style, Color, Modifier}, 
    layout::Rect,
    prelude::Alignment,
    text::{Span, Line}
};

use crate::state::application_state::{ApplicationState, AppState};
use crate::state::world_state::WorldState;
use crate::map::tile::TileType;

// Імпорти з наших нових чистих модулів
use crate::tui::layout::{get_main_layout, get_centered_rect};
use crate::tui::utils::get_team_color;
use crate::tui::menu::MenuState; 

// =========================================================================
//                            MAP RENDERING
// =========================================================================

fn draw_map_tiles(f: &mut Frame, world_state: &WorldState, inner_area: Rect) {
    let map = &world_state.map;
    
    // Оптимізація: Малюємо тільки те, що влазить у вюпорт
    let view_width = inner_area.width as i32;
    let view_height = inner_area.height as i32;
    
    let buffer = f.buffer_mut();

    for y in 0..view_height {
        for x in 0..view_width {
            // Використовуємо наш safe accessor
            let tile = match map.get_tile((x, y)) {
                Some(t) => t,
                None => continue,
            };

            let (mut symbol, mut style) = match tile.tile_type() { 
                TileType::WalkableGeneric => (tile.symbol(), Style::default().fg(Color::DarkGray)),
                TileType::Wall => (tile.symbol(), Style::default().fg(Color::Rgb(255, 165, 0))),
            };

            if let Some(id) = tile.entity_id() { 
                if let Some(e) = world_state.get_entity(id) {
                    symbol = e.symbol(); 
                    if e.is_selected() { 
                        style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
                    } else {
                        let color = get_team_color(e.team());
                        style = Style::default().fg(color);
                    }
                }
            }

            // Безпечний запис у буфер
            let bx = inner_area.x + x as u16;
            let by = inner_area.y + y as u16;
            
            if bx < buffer.area.width && by < buffer.area.height {
                if let Some(cell) = buffer.cell_mut((bx, by)) {
                    cell.set_symbol(&symbol.to_string())
                        .set_style(style);
                }
            }
        }
    }
}

// =========================================================================
//                        GENERIC MENU RENDERER
// =========================================================================

/// Малює будь-який об'єкт, що реалізує MenuState (Editor, Game тощо)
fn draw_generic_menu<T: MenuState>(f: &mut Frame, state: &T, area: Rect) {
    let mut lines = Vec::new();

    // 1. Header
    let header = state.get_top_header();
    if !header.is_empty() {
        lines.extend(header);
        lines.push(Line::from(""));
    }

    // 2. Tools & Spacers
    for item in state.get_tools() {
        // Якщо це spacer (порожній хоткей і лейбл)
        if item.hotkey.is_empty() && item.label.is_empty() {
            lines.push(Line::from(""));
            continue;
        }

        let style = if item.is_active {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else if let Some(c) = item.color {
            Style::default().fg(c)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let span = Span::styled(
            format!("[{}] {}", item.hotkey, item.label), 
            style
        );
        lines.push(Line::from(span));
    }

    lines.push(Line::from(""));
    
    // 3. Info Section
    lines.extend(state.get_info_section());

    // 4. Logs
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("--- LOGS ---", Style::default().add_modifier(Modifier::DIM))));
    lines.push(Line::from(Span::raw(state.get_logs())));

    let p = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(Block::default().title(state.get_title()).borders(Borders::ALL));
    
    f.render_widget(p, area);
}

// =========================================================================
//                            MAIN ENTRY POINTS
// =========================================================================

fn draw_greeting_menu(f: &mut Frame){
    let menu_area = get_centered_rect(40, 40, f.area());
    
    let menu_lines = vec![
        Line::from(Span::styled("--- RUST TUI ROGUELIKE ---", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))), 
        Line::from(""), 
        Line::from("Controls: Mouse to click, Keys shortcuts supported"), 
        Line::from(""), 
        Line::from(Span::styled("[1] EDITOR MODE", Style::default().fg(Color::Cyan))), 
        Line::from("[2] NEW GAME"), 
        Line::from("[3] LOAD LATEST"), 
        Line::from(""), 
        Line::from(Span::styled("[Q] QUIT", Style::default().fg(Color::Red))), 
    ];
    let p = Paragraph::new(menu_lines)
        .alignment(Alignment::Center)
        .block(Block::default().title(" Main Menu ").borders(Borders::ALL));
    f.render_widget(p, menu_area);
}

pub fn ui(f: &mut Frame, app_state: &ApplicationState) {
     let size = f.area();
     
     match &app_state.state {
        AppState::Editor(mode) => {
            let (map_area, menu_area) = get_main_layout(size);
            draw_map_tiles(f, &mode.world(), map_area); 
            draw_generic_menu(f, mode, menu_area);
        },
        AppState::Game(mode) => {
            let (map_area, menu_area) = get_main_layout(size);
            draw_map_tiles(f, mode.world(), map_area); 
            draw_generic_menu(f, mode, menu_area);
        },
        AppState::Menu => {
             draw_greeting_menu(f);
        }
        _ =>{}
     }
}