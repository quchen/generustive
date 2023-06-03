use super::angle::Angle;
use std::ops::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn xy(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn polar(r: f64, angle: Angle) -> Self {
        Self::xy(r * angle.as_rad().cos(), r * angle.as_rad().sin())
    }

    pub fn dot(self, p2: Self) -> f64 {
        return self.x * p2.x + self.y * p2.y;
    }

    pub fn norm_square(self) -> f64 {
        return self.dot(self);
    }

    pub fn norm(self) -> f64 {
        return self.norm_square().sqrt();
    }

    pub fn cross(self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn angle(self) -> Angle {
        Angle::rad(self.y.atan2(self.x))
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) -> () {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) -> () {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = f64;

    fn mul(self, other: Vec2) -> Self::Output {
        self.dot(other)
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, other: f64) -> () {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
