use num::One;

use super::*;
use std::ops::{Mul, MulAssign};

/// 2D affine transformation.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Transformation {
    // / m11 m12 | b1 \
    // | m21 m22 | b2 |
    // \ 0   0   | 1  /
    m11: f64,
    m12: f64,
    m21: f64,
    m22: f64,

    b1: f64,
    b2: f64,
}

impl Transformation {
    pub fn id() -> Self {
        Transformation::one()
    }

    pub fn inverse(self) -> Self {
        let Transformation {
            m11: a,
            m12: b,
            m21: d,
            m22: e,

            b1: c,
            b2: f,
        } = self;
        let x = 1. / (a * e - b * d);
        Transformation {
            m11: x * e,
            m12: x * (-b),
            m21: x * (-d),
            m22: x * a,

            b1: x * (-e * c + b * f),
            b2: x * (d * c - a * f),
        }
    }

    pub fn translate(delta: Vec2) -> Transformation {
        Transformation {
            m11: 1.,
            m12: 0.,
            m21: 0.,
            m22: 1.,
            b1: delta.x,
            b2: delta.y,
        }
    }

    pub fn rotate(angle: Angle) -> Transformation {
        let a = angle.as_rad();
        Transformation {
            m11: a.cos(),
            m12: -a.sin(),
            m21: a.sin(),
            m22: a.cos(),
            b1: 0.,
            b2: 0.,
        }
    }

    pub fn scale(factor: f64) -> Transformation {
        Transformation::scale_xy(factor, factor)
    }

    pub fn scale_xy(factor_x: f64, factor_y: f64) -> Transformation {
        Transformation {
            m11: factor_x,
            m12: 0.,
            m21: 0.,
            m22: factor_y,
            b1: 0.,
            b2: 0.,
        }
    }
}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Transformation {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,

            b1: self.m11 * rhs.b1 + self.m12 * rhs.b2 + self.b1,
            b2: self.m21 * rhs.b1 + self.m22 * rhs.b2 + self.b2,
        }
    }
}

impl One for Transformation {
    fn one() -> Self {
        Transformation {
            m11: 1.,
            m12: 0.,
            m21: 0.,
            m22: 1.,
            b1: 0.,
            b2: 0.,
        }
    }
}

impl MulAssign for Transformation {
    fn mul_assign(&mut self, rhs: Self) {
        let m11 = self.m11 * rhs.m11 + self.m12 * rhs.m21;
        let m12 = self.m11 * rhs.m12 + self.m12 * rhs.m22;
        let m21 = self.m21 * rhs.m11 + self.m22 * rhs.m21;
        let m22 = self.m21 * rhs.m12 + self.m22 * rhs.m22;

        let b1 = self.m11 * rhs.b1 + self.m12 * rhs.b2 + self.b1;
        let b2 = self.m21 * rhs.b1 + self.m22 * rhs.b2 + self.b2;

        self.m11 = m11;
        self.m12 = m12;
        self.m21 = m21;
        self.m22 = m22;

        self.b1 = b1;
        self.b2 = b2;
    }
}

trait Transform {
    fn transform(&self, t: Transformation) -> Self;
    fn transform_mut(&mut self, t: Transformation);
}

impl Transform for Vec2 {
    fn transform(&self, t: Transformation) -> Self {
        Vec2 {
            x: t.m11 * self.x + t.m12 * self.y + t.b1,
            y: t.m21 * self.x + t.m22 * self.y + t.b2,
        }
    }

    fn transform_mut(&mut self, t: Transformation) {
        self.x = t.m11 * self.x + t.m12 * self.y + t.b1;
        self.y = t.m21 * self.x + t.m22 * self.y + t.b2;
    }
}

impl Transform for Vec<Vec2> {
    fn transform(&self, t: Transformation) -> Self {
        self.iter().map(|p| p.transform(t)).collect()
    }

    fn transform_mut(&mut self, t: Transformation) {
        self.iter_mut().for_each(|p| p.transform_mut(t));
    }
}

impl Transform for Line {
    fn transform(&self, t: Transformation) -> Self {
        Line {
            start: self.start.transform(t),
            end: self.end.transform(t),
        }
    }

    fn transform_mut(&mut self, t: Transformation) {
        self.start.transform_mut(t);
        self.end.transform_mut(t);
    }
}

#[cfg(test)]
mod transform_line_test {
    use super::Transformation;
    use crate::geometry::{core::transformation::Transform, Line, Vec2};

    #[test]
    fn inplace() {
        let mut line = Line {
            start: Vec2::xy(0., 0.),
            end: Vec2::xy(100., 0.),
        };
        let t = Transformation::translate(Vec2::xy(0., 10.));
        line.transform_mut(t);
        let expected = Line {
            start: Vec2::xy(0., 10.),
            end: Vec2::xy(100., 10.),
        };

        assert_eq!(line, expected);
    }
}

impl Transform for Polygon {
    fn transform(&self, t: Transformation) -> Self {
        Polygon::from_points(self.points().transform(t))
    }

    fn transform_mut(&mut self, t: Transformation) {
        self.points_mut().transform_mut(t);
    }
}

#[cfg(test)]
mod transform_polygon_test {
    use super::Transformation;
    use crate::geometry::{core::transformation::Transform, Polygon, Vec2};

    #[test]
    fn inplace() {
        let mut polygon = Polygon::from_points(vec![
            Vec2::xy(0., 0.),
            Vec2::xy(100., 0.),
            Vec2::xy(50., 50.),
            Vec2::xy(100., 100.),
            Vec2::xy(0., 100.),
        ]);
        let t = Transformation::translate(Vec2::xy(0., 10.));
        polygon.transform_mut(t);
        let expected = Polygon::from_points(vec![
            Vec2::xy(0., 10.),
            Vec2::xy(100., 10.),
            Vec2::xy(50., 60.),
            Vec2::xy(100., 110.),
            Vec2::xy(0., 110.),
        ]);

        assert_eq!(polygon, expected);
    }
}
