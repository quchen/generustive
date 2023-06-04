pub mod draw;
pub mod geometry;

use cairo::{Context, Format, ImageSurface};
use draw::*;
use geometry::*;
use std::fs::File;

fn main() {
    let surface =
        ImageSurface::create(Format::ARgb32, 600, 600).expect("Couldn't create a surface");
    let context = Context::new(&surface).expect("Couldn't create a context");
    context.set_source_rgb(0.9, 0.9, 0.9);
    context.paint().expect("Couldn't paint");

    let mut rng = rand::thread_rng();
    let points = poisson_disc(
        &mut rng,
        vec![Vec2::xy(10., 10.), Vec2::xy(590., 590.)],
        20.,
        5,
    );
    context.set_source_rgb(0., 0., 0.);
    for point in points {
        context.sketch(Circle::new(point, 3.));
        context.fill().expect("Cannot fill");
    }

    let mut file = File::create("output.png").expect("Couldn't create file.");
    surface
        .write_to_png(&mut file)
        .expect("Couldn't write to png");
}
