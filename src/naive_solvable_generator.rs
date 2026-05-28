use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use crate::{Generator, MinesweeperGrid, Position, RandomGenerator, Solver};

pub struct NaiveSolvableGenerator;

impl Generator for NaiveSolvableGenerator {
    fn generate(&self, width: usize, height: usize, pressed_position: Position, num_mines: usize, seed: u64) -> MinesweeperGrid {
        let mut rng = Pcg64::seed_from_u64(seed);
        loop {
            let mut grid = RandomGenerator.generate(width, height, pressed_position, num_mines, rng.next_u64());
            grid.reveal(pressed_position);
            if Solver::is_solvable(&grid) {
                return grid;
            }
        }
    }

    fn name(&self) -> &'static str {
        "Naïve Solvable Generator"
    }
}
