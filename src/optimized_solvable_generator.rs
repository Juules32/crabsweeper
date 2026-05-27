use rand::{RngExt, SeedableRng};
use rand_pcg::Pcg64;
use std::collections::HashSet;
use crate::{BitGrid, Generator, MinesweeperGrid, Position, Solver};

pub struct OptimizedSolvableGenerator {
    pub num_mines: usize,
    pub seed: u64,
}

impl Generator for OptimizedSolvableGenerator {
    fn generate(&self, width: usize, height: usize, pressed_position: Position) -> MinesweeperGrid {
        let mut rng = Pcg64::seed_from_u64(self.seed);
        let mut grid = MinesweeperGrid::from(BitGrid::empty(width, height));

        let eligible_generator_positions: Vec<Position> =
            grid.generate_eligible_generator_positions(pressed_position);

        let mut current_positions: Vec<Position> = eligible_generator_positions.clone();

        let mut rejected_positions: Vec<Position> = Vec::new();

        let mut mines: HashSet<Position> = HashSet::new();

        while mines.len() < self.num_mines {
            if !current_positions.is_empty() {
                let idx = rng.random_range(0..current_positions.len());
                let pos = current_positions.swap_remove(idx);

                mines.insert(pos);

                grid = MinesweeperGrid::from_mine_positions(width, height, &mines);
                grid.reveal(pressed_position);

                if Solver::is_solvable(&grid) {
                    current_positions.append(&mut rejected_positions);
                } else {
                    mines.remove(&pos);
                    rejected_positions.push(pos);
                }
            } else {
                mines.clear();
                rejected_positions.clear();
                current_positions = eligible_generator_positions.clone();
            }
        }

        grid
    }
}
