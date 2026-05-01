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

        let mut eligible_mine_coords = bit_grid.generate_eligible_mine_coords(pressed_x, pressed_y);
        eligible_mine_coords.shuffle(&mut rng);

        for &(x, y) in eligible_mine_coords.iter().take(self.num_mines) {
            bit_grid.set(x, y, true);
        }

        bit_grid.into()
    }
}
