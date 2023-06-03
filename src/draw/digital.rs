use crate::geometry::*;
use cairo::Context;

pub trait Sketch {
    fn sketch(self, ctx: &Context) -> ();
}

impl Sketch for Line {
    fn sketch(self, ctx: &Context) -> () {
        let Line {
            start: Vec2 { x: x0, y: y0 },
            end: Vec2 { x: x1, y: y1 },
        } = self;
        ctx.move_to(x0, y0);
        ctx.line_to(x1, y1);
    }
}
