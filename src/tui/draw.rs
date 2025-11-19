use ratatui::{
    Frame, 
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color, Modifier}, 
    layout::{Constraint, Layout, Direction, Rect},
    prelude::Alignment,
};
use crate::state::application_state::{ApplicationState, AppState};
use crate::state::game_state::GameState;
use crate::map::tile::TileType;

// --- HELPERS ---
fn draw_map_tiles(f: &mut Frame, game_state: &GameState, inner_area: Rect) {
    for y in 0..game_state.map.height {
        for x in 0..game_state.map.width {
            let tile = game_state.map.get_tile_i32(x, y);
            if let Some(tile) = tile {
                let (mut symbol, mut style) = match tile.tile_type {
                    TileType::WalkableGeneric => (tile.symbol, Style::default().fg(Color::DarkGray)),
                    TileType::Wall => (tile.symbol, Style::default().fg(Color::Rgb(255, 165, 0))),
                };

                if let Some(id) = tile.entity_id {
                    if let Some(e) = game_state.get_entity(id) {
                        symbol = e.symbol;
                        if e.is_selected {
                            style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
                        } else {
                            style = Style::default().fg(Color::White);
                        }
                    }
                }

                if (x as u16) < inner_area.width && (y as u16) < inner_area.height {
                    f.buffer_mut()
                        .get_mut(inner_area.x + x as u16, inner_area.y + y as u16)
                        .set_symbol(&symbol.to_string())
                        .set_style(style);
                }
            }
        }
    }
}

fn draw_status(f: &mut Frame, msg: &str, title: &str, area: Rect) {
    let p = Paragraph::new(msg)
        .block(Block::default().title(title).borders(Borders::ALL).style(Style::default().fg(Color::Green)));
    f.render_widget(p, area);
}

fn draw_game_wrapper(f: &mut Frame, gs: &GameState, status_msg: &str, status_title: &str) {
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)]).split(f.area());
    
    let map_block = Block::default().title(" Map ").borders(Borders::ALL);
    f.render_widget(map_block.clone(), chunks[0]);
    draw_map_tiles(f, gs, map_block.inner(chunks[0]));
    
    draw_status(f, status_msg, status_title, chunks[1]);
}

// --- MAIN DISPATCHER ---
pub fn app_ui(f: &mut Frame, app: &ApplicationState) {
    match &app.state {
        AppState::Menu => {
            // Updated Menu
            let p = Paragraph::new("1. BUILD MODE\n2. PLAY NEW\n3. LOAD 'map.json'\nQ. QUIT")
                .alignment(Alignment::Center)
                .block(Block::default().title(" MENU ").borders(Borders::ALL));
            f.render_widget(p, f.area()); 
        },
        AppState::Editor(mode) => {
            let info = format!("Tool: {:?} (Tab) | Save (S)\n{}", mode.current_tool, mode.debug_message);
            draw_game_wrapper(f, &mode.game_state, &info, " EDITOR ");
        },
        AppState::Simulation(mode) => {
            draw_game_wrapper(f, &mode.game_state, &mode.debug_message, " GAME ");
        },
        _ => {}
    }
}