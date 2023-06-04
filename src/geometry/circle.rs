use super::vec2::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Vec2, radius: f64) -> Circle {
        Self { center, radius }
    }
}
