use super::angle::Angle;
use crate::geometry::vec2::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    pub fn from_to(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn angled(start: Vec2, length: f64, angle: Angle) -> Self {
        Self::from_to(start, start + Vec2::polar(length, angle))
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

    pub fn angle(self) -> Angle {
        self.vec2().angle()
    }
}
