use std::ops::Deref;

use crate::geometry::*;
use cairo::{Context, Error};
use rand_distr::BinomialError;

pub trait Sketch<T> {
    fn sketch(&self, object: T) -> ();
}

impl Sketch<Line> for Context {
    fn sketch(&self, line: Line) -> () {
        let Line { start, end } = line;

        let Vec2 { x: x0, y: y0 } = start;
        self.move_to(x0, y0);

        let Vec2 { x: x1, y: y1 } = end;
        self.line_to(x1, y1);
    }
}

impl Sketch<Circle> for Context {
    fn sketch(&self, circle: Circle) -> () {
        let Circle { center, radius } = circle;

        self.arc(center.x, center.y, radius, 0., 2. * std::f64::consts::PI)
    }
}

pub trait Scoping {
    fn scoped<R>(&self, body: impl FnOnce(&Context) -> Result<R, Error>) -> Result<R, Error>;
}

impl Scoping for Context {
    fn scoped<R>(&self, body: impl FnOnce(&Context) -> Result<R, Error>) -> Result<R, Error> {
        self.save()?;
        let result = body(&self)?;
        self.restore()?;
        Ok(result)
    }
}
