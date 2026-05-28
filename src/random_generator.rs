use rand_pcg::Pcg64;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use crate::{BitGrid, Generator, MinesweeperGrid, Position};

pub struct RandomGenerator;

impl Generator for RandomGenerator {
    fn generate(&self, width: usize, height: usize, pressed_position: Position, num_mines: usize, seed: u64) -> MinesweeperGrid {
        let mut bit_grid = BitGrid::empty(width, height);
        let mut rng = Pcg64::seed_from_u64(seed);

        let mut eligible_generator_positions = bit_grid.generate_eligible_generator_positions(pressed_position);
        eligible_generator_positions.shuffle(&mut rng);

        for &position in eligible_generator_positions.iter().take(num_mines) {
            bit_grid.set(position, true);
        }

        bit_grid.into()
    }

    fn name(&self) -> &'static str {
        "Random Generator"
    }
}
