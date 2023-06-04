use super::line::*;
use super::vec2::*;
use std::f64::INFINITY;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BB {
    min: Vec2,
    max: Vec2,
}

impl Add for BB {
    type Output = Self;

    fn add(self, other: BB) -> BB {
        BB {
            min: Vec2 {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
            },
            max: Vec2 {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
            },
        }
    }
}

impl BB {
    pub fn center(self) -> Vec2 {
        (self.min + self.max) / 2.
    }

    pub fn is_inside(self, other: BB) -> bool {
        self + other == other
    }

    pub fn max(self) -> Vec2 {
        self.max
    }

    pub fn min(self) -> Vec2 {
        self.min
    }
}

impl std::iter::Sum for BB {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let zero_bb = BB {
            min: Vec2 {
                x: INFINITY,
                y: INFINITY,
            },
            max: Vec2 {
                x: -INFINITY,
                y: -INFINITY,
            },
        };
        iter.fold(zero_bb, |acc, bb| acc + bb)
    }
}

pub trait HasBB {
    fn bb(&self) -> BB;
}

impl HasBB for Vec2 {
    fn bb(&self) -> BB {
        BB {
            min: *self,
            max: *self,
        }
    }
}

impl<T: HasBB> HasBB for Vec<T> {
    fn bb(&self) -> BB {
        self.iter().map(|p| p.bb()).sum()
    }
}

impl HasBB for Line {
    fn bb(&self) -> BB {
        self.start.bb() + self.end.bb()
    }
}
