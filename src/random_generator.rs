use std::collections::HashSet;
use rand_pcg::Pcg64;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use crate::{BitGrid, Generator, MinesweeperGrid};

pub struct RandomGenerator {
    pub num_mines: usize,
    pub seed: u64,
}

impl Generator for RandomGenerator {
    fn generate(&self, width: usize, height: usize, pressed_coords: (usize, usize)) -> MinesweeperGrid {
        let (pressed_x, pressed_y) = pressed_coords;

        let mut bit_grid = BitGrid::empty(width, height);
        let mut rng = Pcg64::seed_from_u64(self.seed);
        let mut pressed_surrounding_coords = bit_grid.neighbor_coords(pressed_x, pressed_y);
        pressed_surrounding_coords.push((pressed_x, pressed_y));

        let forbidden: HashSet<_> = pressed_surrounding_coords.into_iter().collect();

        let mut positions: Vec<_> = bit_grid
            .iter_xy()
            .map(|(x, y, _)| (x, y))
            .filter(|pos| !forbidden.contains(pos))
            .collect();

        positions.shuffle(&mut rng);

        for &(x, y) in positions.iter().take(self.num_mines) {
            bit_grid.set(x, y, true);
        }

        bit_grid.into()
    }
}
