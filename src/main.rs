pub mod geometry;

use geometry::*;

fn main() {
    let v1 = Vec2::xy(1., 2.);
    let v2 = Vec2::xy(3., 4.);
    let l = Line::from_to(v1, v2);
    println!("Vector: {:?}", l)
}
