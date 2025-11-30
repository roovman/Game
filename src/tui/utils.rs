use ratatui::style::Color;

pub fn get_team_color(team: u32) -> Color {
    match team {
        0 => Color::White,      // Gaia / Neutral
        1 => Color::Cyan,       // Player
        2 => Color::Red,        // Enemy
        3 => Color::Green,      // Ally
        _ => {
            // Процедурна генерація кольору для інших команд
            let r = (team.wrapping_mul(157) % 255) as u8;
            let g = (team.wrapping_mul(73) % 255) as u8;
            let b = (team.wrapping_mul(233) % 255) as u8;
            Color::Rgb(r, g, b)
        }
    }
}