// src/state/modes/game/menu.rs
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color, Modifier};

use crate::tui::menu::{MenuState, MenuItem};
use crate::tui::utils::get_team_color;
use crate::state::actions::GameTool;

use super::game_mode::GameMode;

impl MenuState for GameMode {
    fn get_title(&self) -> String { " GAME ".to_string() }

    fn get_top_header(&self) -> Vec<Line<'_>> {
        let turn = self.world().current_team_turn;
        let color = get_team_color(turn);
        vec![
            Line::from(Span::styled(
                format!("--- TEAM {} TURN ---", turn), 
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            ))
        ]
    }

    fn get_tools(&self) -> Vec<MenuItem> {
        let t = self.current_tool();
        vec![
            MenuItem::new("1", "Select", t == GameTool::Select),
            MenuItem::new("2", "Move", t == GameTool::Move),
            MenuItem::new("3", "Attack", t == GameTool::Attack),
            MenuItem::new("4", "Skill", t == GameTool::Skill),
            MenuItem::spacer(),
            MenuItem::colored("T", "End Turn", Color::Yellow),
            MenuItem::spacer(),
            MenuItem::colored("Q", "Quit", Color::Red),
        ]
    }

    fn get_info_section(&self) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        
        if let Some(id) = self.selected_entity_id() {
            if let Some(e) = self.world().get_entity(id) {
                lines.push(Line::from(Span::styled(format!("UNIT: {}", e.display_name()), Style::default().fg(Color::Cyan))));
                lines.push(Line::from(format!("HP: {}/{}", e.health(), e.max_health())));
                lines.push(Line::from(format!("EN: {}/{}", e.energy(), e.max_energy())));
                
                let dmg_str = format!("DMG: {} | RNG: {}", e.damage(), e.attack_range());
                lines.push(Line::from(Span::styled(dmg_str, Style::default().fg(Color::Red))));
                
                let team_color = get_team_color(e.team());
                lines.push(Line::from(Span::styled(format!("TEAM: {}", e.team()), Style::default().fg(team_color))));
                
                if e.is_ai() {
                    lines.push(Line::from(Span::styled("(AI Controlled)", Style::default().fg(Color::DarkGray))));
                }
            }
        } else {
            lines.push(Line::from(Span::styled("No Unit Selected", Style::default().add_modifier(Modifier::DIM))));
        }
        lines
    }

    fn get_logs(&self) -> String {
        self.debug_message().to_string()
    }
}