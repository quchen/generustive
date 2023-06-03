use crate::geometry::vec2::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn vec2(&self) -> Vec2 {
        self.end - self.start
    }

    pub fn length(&self) -> f64 {
        self.vec2().norm()
    }

    pub fn reverse(self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }
}
