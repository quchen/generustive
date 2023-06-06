pub mod draw;
pub mod geometry;
pub mod numerics;

use draw::{
    color::{mma97::mma97, Rgb},
    digital::{png, *},
};

use geometry::*;
use rand::{rngs::SmallRng, SeedableRng};

fn main() -> Result<(), cairo::IoError> {
    let paint_my_stuff = |width, height, context: &cairo::Context| -> Result<(), cairo::Error> {
        let mut rng: SmallRng = SmallRng::seed_from_u64(0);
        let points = poisson_disc(
            &mut rng,
            vec![Vec2::xy(10., 10.), Vec2::xy(width - 10., height - 10.)],
            10.,
            50,
        );
        for (i, &point) in points.iter().enumerate() {
            context.scoped(|context| {
                let Rgb(r, g, b): Rgb = mma97(i);
                context.set_source_rgb(r, g, b);
                context.sketch(Circle::new(point, 2.));
                context.fill()?;
                // context.sketch(Circle::new(point, 10.));
                // context.stroke()
                Ok(())
            })?
        }
        Ok(())
    };
    png::write_file("out/output.png", 500, 500, paint_my_stuff)?;
    svg::write_file("out/output.svg", 500, 500, paint_my_stuff)?;
    Ok(())
}
