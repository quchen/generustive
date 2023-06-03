pub mod draw;
pub mod geometry;

use cairo::{Context, Format, ImageSurface};
use draw::*;
use geometry::*;
use std::fs::File;

fn main() {
    let v1 = Vec2::xy(10., 10.);
    let v2 = Vec2::xy(590., 590.);
    let l = Line::from_to(v1, v2);

    let surface =
        ImageSurface::create(Format::ARgb32, 600, 600).expect("Couldn't create a surface");
    let context = Context::new(&surface).expect("Couldn't create a context");
    context.set_source_rgb(0.9, 0.9, 0.9);
    context.paint().expect("Couldn't paint");

    context.set_source_rgb(0., 0., 0.);
    l.sketch(&context);
    context.stroke().expect("could not stroke");

    let mut file = File::create("output.png").expect("Couldn't create file.");
    surface
        .write_to_png(&mut file)
        .expect("Couldn't write to png");
}
