// src/tui/draw.rs

use ratatui::{
    Frame, 
    widgets::{Paragraph, Block, Borders, Wrap},
    style::{Style, Color, Modifier}, 
    layout::{Constraint, Layout, Direction, Rect},
    prelude::Alignment,
    text::{Span, Line}
};
use crate::state::application_state::{ApplicationState, AppState};
use crate::state::world_state::WorldState;
use crate::map::tile::TileType;
use crate::state::modes::{EditorMode, GameMode}; 
use crate::state::actions::{GameTool, BuildTool}; // Import Tools for comparison

// --- HELPERS ---

fn draw_map_tiles(f: &mut Frame, world_state: &WorldState, inner_area: Rect) {
    // Only draw what is visible in the viewport (simple culling)
    let view_width = inner_area.width as i32;
    let view_height = inner_area.height as i32;

    for y in 0..view_height {
        for x in 0..view_width {
            // Check bounds against map size
            if x >= world_state.map.width() || y >= world_state.map.height() {
                continue;
            }

            let tile = world_state.map.get_tile_i32(x, y);
            if let Some(tile) = tile {
                
                let (mut symbol, mut style) = match tile.tile_type() { 
                    TileType::WalkableGeneric => (tile.symbol(), Style::default().fg(Color::DarkGray)),
                    TileType::Wall => (tile.symbol(), Style::default().fg(Color::Rgb(255, 165, 0))),
                };

                if let Some(id) = tile.entity_id() { 
                    if let Some(e) = world_state.get_entity(id) {
                        symbol = e.symbol(); 
                        if e.is_selected() { 
                            style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
                        } else if e.team() != 1 { // Simple enemy check (Team 1 is player)
                             style = Style::default().fg(Color::Red);
                        } else {
                            style = Style::default().fg(Color::Cyan);
                        }
                    }
                }
                
                // Draw to buffer
                f.buffer_mut()
                    .get_mut(inner_area.x + x as u16, inner_area.y + y as u16)
                    .set_symbol(&symbol.to_string())
                    .set_style(style);
            }
        }
    }
}

// --- MENU RENDERING ---

fn draw_editor_menu(f: &mut Frame, mode: &EditorMode, area: Rect) {
    let mut lines = Vec::new();
    
    lines.push(Line::from(Span::styled("--- TOOLS ---", Style::default().add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));

    // Helper to style active tools
    let get_style = |tool| {
        if mode.current_tool == tool {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };

    lines.push(Line::from(Span::styled("[W] Wall", get_style(BuildTool::Wall))));
    lines.push(Line::from(Span::styled("[F] Floor", get_style(BuildTool::Floor))));
    lines.push(Line::from(Span::styled("[U] Unit", get_style(BuildTool::Unit))));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("[S] Save Map", Style::default().fg(Color::Blue))));
    
    // Logs at bottom
    lines.push(Line::from("")); 
    lines.push(Line::from("")); 
    lines.push(Line::from(Span::styled("--- LOGS ---", Style::default().add_modifier(Modifier::DIM))));
    lines.push(Line::from(Span::raw(&mode.debug_message)));

    let p = Paragraph::new(lines)
        .wrap(Wrap { trim: false }) 
        .block(Block::default().title(" EDITOR ").borders(Borders::ALL));
    
    f.render_widget(p, area);
}

fn draw_game_menu(f: &mut Frame, mode: &GameMode, area: Rect) {
    let mut lines = Vec::new();

    lines.push(Line::from(Span::styled("--- COMMANDS ---", Style::default().add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));

    // Helper to style active tools
    let get_style = |tool| {
        if mode.current_tool == tool {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD) // Active = Green
        } else {
            Style::default().fg(Color::Gray) // Inactive = Gray
        }
    };

    // Corresponds to Y=3, 4, 5, 6 in Handle Input
    lines.push(Line::from(Span::styled("[1] Select", get_style(GameTool::Select))));
    lines.push(Line::from(Span::styled("[2] Move", get_style(GameTool::Move))));
    lines.push(Line::from(Span::styled("[3] Attack", get_style(GameTool::Attack))));
    lines.push(Line::from(Span::styled("[4] Skill", get_style(GameTool::Skill))));
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("[T] End Turn", Style::default().fg(Color::Yellow))));
    lines.push(Line::from(""));

    // Unit Info Section
    if let Some(id) = mode.selected_entity_id {
        if let Some(e) = mode.world_state.get_entity(id) {
            lines.push(Line::from(Span::styled(format!("UNIT: {}", e.display_name()), Style::default().fg(Color::Cyan))));
            
            // Health Bar logic (simple text for now)
            lines.push(Line::from(format!("HP: {}/{}", e.health(), e.max_health())));
            lines.push(Line::from(format!("EN: {}/{}", e.energy(), e.max_energy())));
            
            let dmg_str = format!("DMG: {}", e.damage());
            lines.push(Line::from(Span::styled(dmg_str, Style::default().fg(Color::Red))));
        }
    } else {
        lines.push(Line::from(Span::styled("No Unit Selected", Style::default().add_modifier(Modifier::DIM))));
    }

    // Logs
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("--- LOGS ---", Style::default().add_modifier(Modifier::DIM))));
    lines.push(Line::from(Span::raw(&mode.debug_message)));

    let p = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(Block::default().title(" GAME ").borders(Borders::ALL));
    f.render_widget(p, area);
}

pub fn get_menu_rect(area: Rect) -> Rect {
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(10), // The fixed height of your menu
            Constraint::Min(0),
        ])
        .split(area);

    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(v_chunks[1]);

    h_chunks[1]
}
fn draw_greeting_menu(f: &mut Frame){
    // Use the shared helper!
    let menu_area = get_menu_rect(f.area());

    let menu_lines = vec![
        // Index 0
        Line::from(Span::styled("--- RUST TUI CORE ---", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD))),
        // Index 1
        Line::from(""),
        // Index 2 (Button 1)
        Line::from(Span::styled("1. START EDITOR (BUILD MODE)", Style::default().fg(Color::Green))),
        // Index 3 (Button 2)
        Line::from(Span::styled("2. START NEW GAME (PLAY MODE)", Style::default().fg(Color::Yellow))),
        // Index 4 (Button 3)
        Line::from(Span::styled("3. LOAD LAST SAVED MAP", Style::default().fg(Color::Blue))),
        // Index 5
        Line::from(""),
        // Index 6 (Button Q)
        Line::from(Span::styled("Q. QUIT APPLICATION", Style::default().fg(Color::Red))),
    ];

    let p = Paragraph::new(menu_lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(p, menu_area); 
}

fn draw_game_wrapper(f: &mut Frame, world_state: &WorldState, mode_specific_drawer: impl FnOnce(&mut Frame, Rect)) {
    // 75% Map / 25% Sidebar split
    let chunks = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]).split(f.area());
    
    let map_block = Block::default().title(" Map ").borders(Borders::ALL);
    f.render_widget(map_block.clone(), chunks[0]);
    draw_map_tiles(f, world_state, map_block.inner(chunks[0]));
    
    mode_specific_drawer(f, chunks[1]);
}

pub fn app_ui(f: &mut Frame, app: &ApplicationState) {
    match &app.state {
        AppState::Menu => draw_greeting_menu(f),
        AppState::Editor(mode) => draw_game_wrapper(f, &mode.world_state, |f, area| draw_editor_menu(f, mode, area)),
        AppState::Simulation(mode) => draw_game_wrapper(f, &mode.world_state, |f, area| draw_game_menu(f, mode, area)),
        _ => {}
    }
}