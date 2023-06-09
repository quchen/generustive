use super::angle::Angle;
use super::vec2::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
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

    pub fn subdivide_n(self, num_segments: usize) -> Vec<Vec2> {
        let mut result = Vec::with_capacity(num_segments + 1);
        result.push(self.start);
        for i in 1..num_segments - 1 {
            // ^ -1 because we explicitly add start/end outside of the loop, in
            // order to guarantee the end points remain identical.
            let frac: f64 = i as f64 / num_segments as f64;
            let v = self.vec2();
            result.push(v * frac);
        }
        result.push(self.end);
        result
    }

    pub fn subdivide_by_length(self, segment_length: f64) -> Vec<Vec2> {
        let segments = self.length() / segment_length.abs();
        self.subdivide_n(segments.ceil() as usize)
    }
}

/// Start, end
impl From<(Vec2, Vec2)> for Line {
    fn from(ab: (Vec2, Vec2)) -> Self {
        Line {
            start: ab.0,
            end: ab.1,
        }
    }
}

/// Start x, start y, end x, end y
impl From<(f64, f64, f64, f64)> for Line {
    fn from(abxy: (f64, f64, f64, f64)) -> Self {
        let start = Vec2::from((abxy.0, abxy.1));
        let end = Vec2::from((abxy.2, abxy.3));
        Line::from((start, end))
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::core::*;

    #[test]
    fn subdivide_n() {
        let line = Line::from_to(Vec2::xy(0., 0.), Vec2::xy(100., 0.));
        let segments = 10;
        let result = line.subdivide_n(segments);
        assert_eq!(result.len(), segments);
    }

    #[test]
    fn subdivide_by_length() {
        let line = Line::from_to(Vec2::xy(0., 0.), Vec2::xy(100., 0.));
        let segment_length = 10.;
        let result = line.subdivide_by_length(segment_length);
        assert_eq!(result.len(), 10);
    }
}
