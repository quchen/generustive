use crate::geometry::core::*;
use rand::Rng;
use std::ops::{Index, IndexMut};
use std::vec;

/// Sample a number of points to yield a visually uniform point distribution.
pub fn poisson_disc<R: Rng, Region: HasBB>(
    rng: &mut R,
    region: Region,
    radius: f64,
    k: usize,
) -> Vec<Vec2> {
    let bb = region.bb();
    let initial_point = region.bb().center();
    let mut grid = Grid::new(radius / 2_f64.sqrt(), bb);
    let mut active_points = Vec::with_capacity((grid.size() as f64).sqrt() as usize);
    // ^ Not sure about a good initial size, but sqrt(grid size) should be a good starting point.
    grid.insert(initial_point);
    active_points.push(initial_point);
    let mut result = Vec::with_capacity(grid.size());

    while let Some(active_i) = random_index(rng, &active_points) {
        let active_sample = active_points[active_i];

        let new_point: Option<Vec2> =
            quality_candidates_around_sample(rng, k, radius, active_sample)
                .filter(|candidate| candidate.bb().is_inside(bb))
                .find(|candidate| {
                    let neighbours = grid.neighbouring_points(*candidate);
                    let too_close =
                        |neighbour| (*candidate - neighbour).norm_square() <= radius.powi(2);
                    !neighbours.into_iter().any(too_close)
                });

        match new_point {
            None => {
                // No new point found. Retire sample.
                active_points.swap_remove(active_i);
                result.push(active_sample);
            }
            Some(new) => {
                // New point found. Mark it as active, and insert its location into the grid for future collision detection.
                grid.insert(new);
                active_points.push(new);
            }
        }
    }
    result.shrink_to_fit();
    result
}

fn random_index<R: Rng, T>(rng: &mut R, vec: &Vec<T>) -> Option<usize> {
    if vec.is_empty() {
        None
    } else {
        Some(rng.gen_range(0..vec.len()))
    }
}

/// Sample new points in an annulus of (r,2r) around the center.
///
/// Yields good results with medium-sized k (e.g. 20).
fn quality_candidates_around_sample<R: Rng>(
    rng: &mut R,
    k: usize,
    radius: f64,
    center: Vec2,
) -> impl Iterator<Item = Vec2> + '_ {
    (0..k).map(move |_i| {
        let r = rng.gen_range(radius..(2. * radius));
        let phi = Angle::deg(rng.gen_range(0. ..360.));
        let offset = Vec2::polar(r, phi);
        center + offset
    })
}

struct Grid {
    cell_size: f64,
    size_x: usize,
    size_y: usize,
    vec: Vec<Option<Vec2>>,
}

impl Grid {
    pub fn new(cell_size: f64, bb: BB) -> Self {
        let size_x = (bb.max().x / cell_size).ceil() as usize;
        let size_y = (bb.max().y / cell_size).ceil() as usize;
        let size = size_x * size_y;
        Grid {
            cell_size,
            size_x,
            size_y,
            vec: vec![None; size],
        }
    }

    pub fn size(&self) -> usize {
        self.size_x * self.size_y
    }

    fn cell(&self, point: Vec2) -> (usize, usize) {
        (
            (point.x / self.cell_size).floor() as usize,
            (point.y / self.cell_size).floor() as usize,
        )
    }

    pub fn insert(&mut self, point: Vec2) {
        let index = self.cell(point);
        self[index] = Some(point);
    }

    fn neighbouring_points(&self, point: Vec2) -> Vec<Vec2> {
        let cell = self.cell(point);

        let center_ix = cell.0 as isize;
        let center_iy = cell.1 as isize;

        // 25 is a 5*5 square. We don’t actually need to look at the corners so strictly
        // speaking 21 would be enough, which I will implement later (AKA, not going to).
        let mut result = Vec::with_capacity(25);
        for ix_delta in -2..=2 {
            let ix = center_ix + ix_delta;
            let x_out_of_bounds = ix < 0 || ix > self.size_x as isize - 1;
            if x_out_of_bounds {
                continue;
            }
            for iy_delta in -2..=2 {
                let iy = center_iy + iy_delta;
                let y_out_of_bounds = iy < 0 || iy > self.size_y as isize - 1;
                if y_out_of_bounds {
                    continue;
                }

                // The corners of the 5*5 square are too far away to be reached
                // within a distance of radius r from the center tile, so we
                // exclude them here.
                let is_corner = (ix_delta * iy_delta).abs() == 4;
                if is_corner {
                    continue;
                }

                if let Some(p) = self[(ix as usize, iy as usize)] {
                    result.push(p)
                };
            }
        }
        result
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Option<Vec2>;

    fn index(&self, (ix, iy): (usize, usize)) -> &Self::Output {
        &self.vec[iy * self.size_y + ix]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (ix, iy): (usize, usize)) -> &mut Self::Output {
        &mut self.vec[iy * self.size_y + ix]
    }
}
