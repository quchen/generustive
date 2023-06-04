use crate::geometry::*;
use cairo::{Context, Error};

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

pub mod png {
    use cairo::{Context, Error, Format, ImageSurface, IoError};
    use std::{fs::File, path::Path};

    pub fn write_file<P: AsRef<Path>, R>(
        path: P,
        actions: impl FnOnce(&Context) -> Result<R, Error>,
    ) -> Result<(), IoError> {
        let surface = ImageSurface::create(Format::ARgb32, 600, 600)?;
        let context = Context::new(&surface)?;
        context.set_source_rgba(0.9, 0.9, 0.9, 0.);
        context.paint()?;

        actions(&context)?;

        let mut file = File::create(path)?;
        surface.write_to_png(&mut file)
    }
}
