// src/main.rs

use color_eyre::Result;

fn main() -> Result<()> {
    // Налаштування обробки помилок
    color_eyre::install()?;
    
    // Запускаємо TUI-двигун
    game::run()?;
    
    Ok(())
}