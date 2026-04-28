use rand_pcg::Pcg64;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use crate::{BitGrid, Generator};

pub struct RandomGenerator {
    pub seed: u64,
    pub num_mines: usize,
}

impl Generator for RandomGenerator {
    fn generate(&self, width: usize, height: usize, pressed_x: usize, pressed_y: usize) -> BitGrid {
        let mut bit_grid = BitGrid::empty(width, height);
        let mut rng = Pcg64::seed_from_u64(self.seed);
        let mut pressed_surrounding_coords = bit_grid.neighbor_coords(pressed_x, pressed_y);
        pressed_surrounding_coords.push((pressed_x, pressed_y));
        let mut positions: Vec<(usize, usize)> = bit_grid
            .iter_xy()
            .map(|(x, y, _)| (x, y))
            .filter(|&(x, y)| !pressed_surrounding_coords.contains(&(x, y)))
            .collect();
        positions.shuffle(&mut rng);

        for &(x, y) in positions.iter().take(self.num_mines) {
            bit_grid.set(x, y, true);
        }

        bit_grid
    }
}
