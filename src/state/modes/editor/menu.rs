use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color, Modifier};

use crate::tui::menu::{MenuState, MenuItem};
use crate::tui::utils::get_team_color;
use crate::state::actions::BuildTool;

use super::editor_mode::EditorMode; 
use super::input::InputTarget; 

impl MenuState for EditorMode {
    fn get_title(&self) -> String { " EDITOR ".to_string() }

    fn get_tools(&self) -> Vec<MenuItem> {
        let t = self.current_tool(); 
        
        vec![
            MenuItem::spacer(), // Y=1
            MenuItem::spacer(), // Y=2
            MenuItem::new("W", "Wall", t == BuildTool::Wall), // Y=3
            MenuItem::new("F", "Floor", t == BuildTool::Floor), // Y=4
            MenuItem::new("U", "Unit", t == BuildTool::Unit), // Y=5
            MenuItem::spacer(), // Y=6
            MenuItem::colored("S", "Save Map", Color::Blue), // Y=7
            MenuItem::spacer(), // Y=8
            MenuItem::colored("Q", "Quit", Color::Red), // Y=9
        ]
    }

    fn get_info_section(&self) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        // Y=11
        lines.push(Line::from(Span::styled("--- ENTITY CFG ---", Style::default().add_modifier(Modifier::BOLD))));

        let spec = self.entity_spec();
        let current_target = self.input_target();
        let buffer = self.input_buffer();

        // Closure з явним лайфтаймом <'_>
        let format_input = |label: &str, is_target: bool, val: String| -> Line<'_> {
            if is_target {
                let txt = format!("{}: [ {}_ ]", label, buffer);
                Line::from(Span::styled(txt, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
            } else {
                Line::from(format!("{}: [{}]", label, val))
            }
        };

        // Y=12
        if current_target == InputTarget::Team {
             lines.push(format_input("Team", true, "".into()));
        } else {
             let color = get_team_color(spec.team);
             let txt = format!("Team: [{}] ({})", spec.team, spec.symbol);
             lines.push(Line::from(Span::styled(txt, Style::default().fg(color))));
        }

        // Y=13, 14, 15
        lines.push(format_input("HP ", current_target == InputTarget::Hp, spec.hp.to_string()));
        lines.push(format_input("Eng", current_target == InputTarget::Energy, spec.energy.to_string()));
        lines.push(format_input("Dmg", current_target == InputTarget::Damage, spec.damage.to_string()));

        // Y=16
        let ai_str = if spec.is_ai { "AI:   [ON]" } else { "AI:   [OFF]" };
        lines.push(Line::from(Span::styled(ai_str, Style::default().fg(Color::Yellow))));

        lines
    }

    fn get_logs(&self) -> String {
        self.debug_message().to_string()
    }
}