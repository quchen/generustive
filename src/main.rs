pub mod draw;
pub mod geometry;

use cairo::Context;
use draw::{png, *};
use geometry::*;
use rand_seeder::{Seeder, SipRng};

fn main() -> Result<(), cairo::IoError> {
    let paint_my_stuff = |width, height, context: &Context| {
        let mut rng: SipRng = Seeder::from("stripy zebra").make_rng();
        let points = poisson_disc(
            &mut rng,
            vec![Vec2::xy(10., 10.), Vec2::xy(width - 10., height - 10.)],
            20.,
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
