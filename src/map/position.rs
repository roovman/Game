// src/map/position.rs (ОНОВЛЕНО: Додані методи руху)

use serde::{Serialize, Deserialize};
use glam::IVec2;
use std::ops::{Add, Sub};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub struct MapPosition(IVec2);

impl MapPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    pub fn x(&self) -> i32 { self.0.x }
    pub fn y(&self) -> i32 { self.0.y }

    // Скорочені методи руху 
    pub fn offset(&self, dx: i32, dy: i32) -> Self {
        Self(self.0 + IVec2::new(dx, dy))
    }
    
    pub fn up(&self, dist: i32) -> Self { self.offset(0, -dist) }
    pub fn down(&self, dist: i32) -> Self { self.offset(0, dist) }
    pub fn left(&self, dist: i32) -> Self { self.offset(-dist, 0) }
    pub fn right(&self, dist: i32) -> Self { self.offset(dist, 0) }

    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        (self.0.x - other.0.x).abs() as u32 + (self.0.y - other.0.y).abs() as u32
    }
    pub fn neighbors(&self) -> [Self; 4] {
        [
            self.up(1),
            self.down(1),
            self.left(1),
            self.right(1),
        ]
    }
}

impl From<&MapPosition> for MapPosition {
    fn from(pos: &MapPosition) -> Self {
        *pos 
    }
}
// Це дозволить нам кидати (x, y) туди, де очікується MapPosition
impl From<(i32, i32)> for MapPosition {
    fn from(tuple: (i32, i32)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl From<IVec2> for MapPosition {
    fn from(vec: IVec2) -> Self {
        Self(vec)
    }
}
// --- GENERICS MAGIC END ---

impl Add for MapPosition {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for MapPosition {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}