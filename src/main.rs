pub mod draw;
pub mod geometry;

use draw::digital::{png, *};
use geometry::*;
use rand::{rngs::SmallRng, SeedableRng};

fn main() -> Result<(), cairo::IoError> {
    let paint_my_stuff = |width, height, context: &cairo::Context| -> Result<(), cairo::Error> {
        let mut rng: SmallRng = SmallRng::seed_from_u64(0);
        let points = poisson_disc(
            &mut rng,
            vec![Vec2::xy(10., 10.), Vec2::xy(width - 10., height - 10.)],
            10.,
            5,
        );
        context.set_source_rgb(0., 0., 0.);
        for point in points {
            context.scoped(|context| {
                context.sketch(Circle::new(point, 2.));
                context.fill()
            })?
        }
        Ok(())
    };
    png::write_file("out/output.png", 100, 100, paint_my_stuff)?;
    svg::write_file("out/output.svg", 100, 100, paint_my_stuff)?;
    Ok(())
}
