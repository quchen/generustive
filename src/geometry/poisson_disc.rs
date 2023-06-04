use crate::geometry::core::*;
use rand::Rng;
use std::cmp::{max, min};
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
    let mut result = vec![];

    while let Some(active_i) = random_index(rng, &active_points) {
        let active_sample = active_points[active_i];

        let candidates = candidates_around_sample(rng, k, radius, &active_sample);

        let new_point: Option<Vec2> = candidates
            .into_iter()
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
    result
}

fn random_index<R: Rng, T>(rng: &mut R, vec: &Vec<T>) -> Option<usize> {
    if vec.is_empty() {
        None
    } else {
        Some(rng.gen_range(0..vec.len()))
    }
}

fn candidates_around_sample<R: Rng>(rng: &mut R, k: usize, radius: f64, point: &Vec2) -> Vec<Vec2> {
    let pi = std::f64::consts::PI;
    let phi0 = rng.gen_range(0. ..2. * pi);
    let delta_phi = 2. * pi / (k as f64);
    let radius_plus_epsilon = radius + 1e-6;
    let result = (0..k)
        .map(|i| {
            let offset = Vec2::polar(
                radius_plus_epsilon,
                Angle::rad(phi0 + delta_phi * (i as f64)),
            );
            *point + offset
        })
        .collect();
    shuffle(rng, result)
}

/// Random in-place permutation.
fn shuffle<R: Rng, T>(rng: &mut R, mut vec: Vec<T>) -> Vec<T> {
    // Fisher/Yates shuffle
    let n = vec.len();
    for i in 0..n - 1 {
        let j = rng.gen_range(i..n);
        vec.swap(i, j);
    }
    vec
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
        let ix_range = max(0, center_ix - 2)..=min(self.size_x as isize - 1, center_ix + 2);

        let center_iy = cell.1 as isize;
        let iy_range = max(0, center_iy - 2)..=min(self.size_y as isize - 1, center_iy + 2);

        // 25 is a 5*5 square. We don’t actually need to look at the corners so strictly
        // speaking 21 would be enough, which I will implement later (AKA, not going to).
        let mut result = Vec::with_capacity(25);
        ix_range.for_each(|ix| {
            iy_range.clone().for_each(|iy| {
                if let Some(p) = self[(ix as usize, iy as usize)] {
                    result.push(p)
                };
            });
        });
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
