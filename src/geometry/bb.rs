use super::line::*;
use super::vec2::*;
use impl_trait_for_tuples::impl_for_tuples;
use std::f64::INFINITY;
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BB {
    min: Vec2,
    max: Vec2,
}

static ZERO_BB: BB = BB {
    min: Vec2 {
        x: INFINITY,
        y: INFINITY,
    },
    max: Vec2 {
        x: -INFINITY,
        y: -INFINITY,
    },
};

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

impl Sum for BB {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ZERO_BB, |acc, bb| acc + bb)
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

/// Neutral element with respect to +.
impl HasBB for () {
    fn bb(&self) -> BB {
        ZERO_BB
    }
}

#[impl_for_tuples(1, 3)]
/// The bounding box of a tuple is the union of the elements’ bounding boxes.
impl HasBB for Tuple {
    fn bb(&self) -> BB {
        for_tuples!( ( #( Tuple.bb())+* ) );
    }
}
