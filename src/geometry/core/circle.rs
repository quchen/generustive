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

/// Center, radius
impl From<(Vec2, f64)> for Circle {
    fn from(value: (Vec2, f64)) -> Self {
        Circle::new(value.0, value.1)
    }
}
