pub mod draw;
pub mod geometry;

use draw::{png::write_file, *};
use geometry::*;

fn main() -> Result<(), cairo::IoError> {
    write_file("output.png", |context| {
        let mut rng = rand::thread_rng();
        let points = poisson_disc(
            &mut rng,
            vec![Vec2::xy(10., 10.), Vec2::xy(590., 590.)],
            20.,
            5,
        );
        context.set_source_rgb(0., 0., 0.);
        for point in points {
            context.scoped(|context| {
                context.sketch(Circle::new(point, 3.));
                context.fill()
            })?
        }
        Ok(())
    })
}
