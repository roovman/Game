// src/map/position.rs (ОНОВЛЕНО: Додані методи руху)

use serde::{Serialize, Deserialize};
use glam::IVec2;
use std::ops::{Add, Sub};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)] 
pub struct MapPosition(IVec2);

impl MapPosition {
    /// Створює нову позицію карти з цілочисельних координат.
    pub fn new(x: i32, y: i32) -> Self {
        MapPosition(IVec2::new(x, y))
    }

    /// Отримує X-координату.
    pub fn x(&self) -> i32 {
        self.0.x
    }

    /// Отримує Y-координату.
    pub fn y(&self) -> i32 {
        self.0.y
    }
    
    /// Повертає нову позицію, зміщену вгору на вказану відстань.
    pub fn up(&self, distance: u32) -> Self {
        MapPosition(self.0 - IVec2::new(0, distance as i32))
    }

    /// Повертає нову позицію, зміщену вниз на вказану відстань.
    pub fn down(&self, distance: u32) -> Self {
        MapPosition(self.0 + IVec2::new(0, distance as i32))
    }
    
    /// Повертає нову позицію, зміщену вліво на вказану відстань.
    pub fn left(&self, distance: u32) -> Self {
        MapPosition(self.0 - IVec2::new(distance as i32, 0))
    }
    
    /// Повертає нову позицію, зміщену вправо на вказану відстань.
    pub fn right(&self, distance: u32) -> Self {
        MapPosition(self.0 + IVec2::new(distance as i32, 0))
    }
    
    /// Перетворення назад на IVec2 для сумісності з glam.
    pub fn to_ivec2(&self) -> IVec2 {
        self.0
    }
    
    /// Конвертер для u16 (якщо потрібно для малювання в TUI).
    pub fn as_u16(&self) -> Option<(u16, u16)> {
        if self.0.x >= 0 && self.0.y >= 0 {
            Some((self.0.x as u16, self.0.y as u16))
        } else {
            None
        }
    }
}

// Реалізація трейтів для зручності:
// Дозволяє додавати дві MapPosition
impl Add for MapPosition {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        MapPosition(self.0 + other.0)
    }
}

// Дозволяє віднімати дві MapPosition
impl Sub for MapPosition {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        MapPosition(self.0 - other.0)
    }
}