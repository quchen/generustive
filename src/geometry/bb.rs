use super::line::*;
use super::vec2::*;
use std::ops::Add;

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

pub trait HasBB {
    fn bb(self) -> BB;
}

impl HasBB for Vec2 {
    fn bb(self) -> BB {
        BB {
            min: self,
            max: self,
        }
    }
}

impl HasBB for Line {
    fn bb(self) -> BB {
        self.start.bb() + self.end.bb()
    }
}
