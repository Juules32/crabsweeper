use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use crate::{Generator, MinesweeperGrid, RandomGenerator, Solver};

pub struct NaiveSolvableGenerator {
    pub num_mines: usize,
    pub seed: u64,
}

impl Generator for NaiveSolvableGenerator {
    fn generate(&self, width: usize, height: usize, pressed_coords: (usize, usize)) -> MinesweeperGrid {
        let solver = Solver;

        let mut rng = Pcg64::seed_from_u64(self.seed);

        for _ in 0..100 {
            let random_generator = RandomGenerator {
                num_mines: self.num_mines,
                seed: rng.next_u64(),
            };
            let mut grid = random_generator.generate(width, height, pressed_coords);
            grid.reveal(pressed_coords.0, pressed_coords.1);
            if solver.is_solvable(&grid) {
                return grid;
            }
        }

        panic!("Ran out of tries")
    }
}
