use ratatui::layout::{Constraint, Direction, Layout, Rect};

// Константи пропорцій (75% карта, 25% меню)
pub const MAP_PERCENTAGE: u16 = 75;
pub const MENU_PERCENTAGE: u16 = 25;

/// Розраховує головний лейаут: Карта (ліворуч) і Меню (праворуч).
pub fn get_main_layout(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(MAP_PERCENTAGE), 
            Constraint::Percentage(MENU_PERCENTAGE)
        ])
        .split(area);
    (chunks[0], chunks[1])
}

/// Центрує прямокутник (для головного меню або модальних вікон).
pub fn get_centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Перевіряє, чи точка знаходиться всередині Rect (для обробки миші).
pub fn is_point_in_rect(x: i32, y: i32, rect: Rect) -> bool {
    x >= rect.x as i32 && 
    x < (rect.x + rect.width) as i32 &&
    y >= rect.y as i32 && 
    y < (rect.y + rect.height) as i32
}