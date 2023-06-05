use super::bb::*;
use super::line::*;
use super::vec2::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Polygon(Vec<Vec2>);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Orientation {
    Positive,
    Negative,
}

impl Polygon {
    pub fn from_points(points: Vec<Vec2>) -> Self {
        Self(points)
    }

    pub fn points(&self) -> &Vec<Vec2> {
        &self.0
    }

    pub fn edges(&self) -> impl Iterator<Item = Line> + '_ {
        let points = self.points();
        points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .map(|(p1, p2)| Line::from_to(*p1, *p2))
    }

    pub fn circumference(&self) -> f64 {
        self.edges().fold(0., |acc, line| acc + line.length())
    }

    pub fn area_signed(&self) -> f64 {
        self.edges()
            .map(|Line { start, end }| start.cross(end))
            .sum::<f64>()
            / 2.
    }

    pub fn orientation(&self) -> Orientation {
        if self.area_signed() > 0. {
            Orientation::Positive
        } else {
            Orientation::Negative
        }
    }

    pub fn area(&self) -> f64 {
        self.area_signed().abs()
    }

    /// Check whether the polygon is convex.
    ///
    /// To avoid numerically unstable near-straight edges, we consider
    /// almost-straight corners convex.
    pub fn is_convex(&self) -> bool {
        let points = self.points();

        // Idea: calculate the cross product of adjacent lines. The sign of a cross
        // product tells us whether the bend direction. If all edges bend
        // left/right, then all cross products have the same sign. Finally, convex
        // polygons have all angles in the same direction.
        let cross_signs = points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .zip(points.iter().cycle().skip(2))
            .map(|((p, q), r)| (*q - *p).cross(*r - *q))
            .filter(|x| x.abs() > 1e-10)
            .map(|x| x.partial_cmp(&0.).unwrap());

        let all_equal = |xs| {
            let mut first = None;
            for x in xs {
                match first {
                    None => first = Some(x),
                    Some(y) if y != x => return false,
                    _other => {}
                }
            }
            true
        };

        all_equal(cross_signs)
    }
}

impl HasBB for Polygon {
    fn bb(&self) -> BB {
        self.points().bb()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::core::*;

    #[test]
    fn number_of_edges() {
        let points = vec![
            Vec2::xy(0., 0.),
            Vec2::xy(100., 0.),
            Vec2::xy(100., 100.),
            Vec2::xy(0., 100.),
        ];
        let points_len = points.len();

        let polygon = Polygon::from_points(points);
        let edges: Vec<Line> = polygon.edges().collect();
        assert_eq!(points_len, edges.len());
    }

    #[test]
    fn is_convex_for_convex() {
        let polygon = Polygon::from_points(vec![
            Vec2::xy(10., 10.),
            Vec2::xy(10., 90.),
            Vec2::xy(90., 90.),
            Vec2::xy(90., 10.),
        ]);
        assert!(polygon.is_convex())
    }

    #[test]
    fn is_convex_for_concave() {
        let polygon = Polygon::from_points(vec![
            Vec2::xy(110., 10.),
            Vec2::xy(110., 90.),
            Vec2::xy(150., 50.),
            Vec2::xy(190., 90.),
            Vec2::xy(190., 10.),
        ]);
        assert!(!polygon.is_convex())
    }

    #[test]
    fn standard_square_has_positive_orientation() {
        let standard_square = Polygon::from_points(vec![
            Vec2::xy(0., 0.),
            Vec2::xy(100., 0.),
            Vec2::xy(100., 100.),
            Vec2::xy(0., 100.),
        ]);
        assert_eq!(standard_square.orientation(), Orientation::Positive);
    }
}
