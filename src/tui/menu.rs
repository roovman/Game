use ratatui::{text::Line, style::Color};

pub struct MenuItem {
    pub label: String,
    pub hotkey: String,
    pub is_active: bool,
    pub color: Option<Color>,
}

impl MenuItem {
    pub fn new(hotkey: &str, label: &str, is_active: bool) -> Self {
        Self { 
            hotkey: hotkey.to_string(), 
            label: label.to_string(), 
            is_active, 
            color: None 
        }
    }
    
    pub fn colored(hotkey: &str, label: &str, color: Color) -> Self {
        Self { 
            hotkey: hotkey.to_string(), 
            label: label.to_string(), 
            is_active: false, 
            color: Some(color) 
        }
    }

    /// Створює пустий елемент-розділювач для візуальних відступів
    pub fn spacer() -> Self {
        Self { 
            hotkey: String::new(), 
            label: String::new(), 
            is_active: false, 
            color: None 
        }
    }
}

/// Трейт, який перетворює будь-який ігровий режим на відображуване меню
pub trait MenuState {
    fn get_title(&self) -> String;
    fn get_top_header(&self) -> Vec<Line<'_>> { vec![] }
    fn get_tools(&self) -> Vec<MenuItem>;
    fn get_info_section(&self) -> Vec<Line<'_>>;
    fn get_logs(&self) -> String;
}