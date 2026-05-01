use rand_pcg::Pcg64;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use crate::{BitGrid, Generator, MinesweeperGrid, Position};

pub struct RandomGenerator {
    pub num_mines: usize,
    pub seed: u64,
}

impl Generator for RandomGenerator {
    fn generate(&self, width: usize, height: usize, pressed_position: Position) -> MinesweeperGrid {
        let mut bit_grid = BitGrid::empty(width, height);
        let mut rng = Pcg64::seed_from_u64(self.seed);

        let mut eligible_generator_positions = bit_grid.generate_eligible_generator_positions(pressed_position);
        eligible_generator_positions.shuffle(&mut rng);

        for &position in eligible_generator_positions.iter().take(self.num_mines) {
            bit_grid.set(position, true);
        }

        bit_grid.into()
    }
}
