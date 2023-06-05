use super::{angle::Angle, Polygon};
use std::cmp::Ordering::Greater;
use std::fmt::Display;
use std::ops::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn xy(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn polar(r: f64, angle: Angle) -> Self {
        Self::xy(r * angle.as_rad().cos(), r * angle.as_rad().sin())
    }

    pub fn dot(self, p2: Self) -> f64 {
        self.x * p2.x + self.y * p2.y
    }

    pub fn norm_square(self) -> f64 {
        self.dot(self)
    }

    pub fn norm(self) -> f64 {
        self.norm_square().sqrt()
    }

    pub fn cross(self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn angle(self) -> Angle {
        Angle::rad(self.y.atan2(self.x))
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
    fn add_assign(&mut self, other: Self) {
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
    fn sub_assign(&mut self, other: Self) {
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
    fn mul_assign(&mut self, other: f64) {
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
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

pub fn convex_hull(points: &[Vec2]) -> Polygon {
    // Andrew's monotone chain convex hull algorithm

    let mut sorted = points.to_owned();
    sorted.sort_by(|p, q| {
        let compare_x = p.x.partial_cmp(&q.x).unwrap_or(Greater);
        let compare_y = p.y.partial_cmp(&q.y).unwrap_or(Greater);
        compare_x.then(compare_y)
    });

    let mut result = Vec::new();
    {
        for &p2 in sorted.iter() {
            while result.len() >= 2 {
                let p0: Vec2 = result[result.len() - 1]; // Not sure why this needs a type annotation.
                let p1 = result[result.len() - 2];
                let left_right = (p1 - p0).cross(p2 - p1);
                if left_right > 0. {
                    result.pop();
                } else {
                    break;
                }
            }
            result.push(p2)
        }
        result.pop();
    }

    {
        let mut back = Vec::with_capacity(result.len());
        for &p2 in sorted.iter().rev() {
            while back.len() >= 2 {
                let p0: Vec2 = back[back.len() - 1];
                let p1 = back[back.len() - 2];
                let left_right = (p1 - p0).cross(p2 - p1);
                if left_right > 0. {
                    back.pop();
                } else {
                    break;
                }
            }
            back.push(p2)
        }
        back.pop();

        result.append(&mut back);
    }

    Polygon::from_points(result)
}

#[cfg(test)]
mod tests {
    use crate::geometry::core::*;

    #[test]
    fn convex_hull_tests() {
        let points = vec![
            Vec2::xy(0., 0.),
            Vec2::xy(100., 0.),
            Vec2::xy(50., 50.),
            Vec2::xy(100., 100.),
            Vec2::xy(0., 100.),
        ];
        let hull = convex_hull(&points);
        let expected = Polygon::from_points(vec![
            Vec2 { x: 0., y: 0. },
            Vec2 { x: 100., y: 0. },
            Vec2 { x: 100., y: 100. },
            Vec2 { x: 0., y: 100. },
        ]);
        assert!(hull.is_convex());
        assert_eq!(hull, expected);
        assert_eq!(hull.orientation(), Orientation::Positive);
    }
}
