use std::collections::HashSet;
use rand::SeedableRng;
use rand::seq::IteratorRandom;
use rand_pcg::Pcg64;
use crate::{BitGrid, Generator, MinesweeperGrid, Position, Solver};

pub struct OptimizedSolvableGenerator {
    pub num_mines: usize,
    pub seed: u64,
}

impl Generator for OptimizedSolvableGenerator {
    fn generate(&self, width: usize, height: usize, pressed_position: Position) -> MinesweeperGrid {
        let mut rng = Pcg64::seed_from_u64(self.seed);
        let mut grid = MinesweeperGrid::from(BitGrid::empty(width, height));
        let eligible_generator_positions: HashSet<Position> = grid.generate_eligible_generator_positions(pressed_position).into_iter().collect();
        let mut current_eligible_generator_positions = eligible_generator_positions.clone();
        let mut mines: HashSet<Position> = HashSet::new();
        let mut rejected_mines: HashSet<Position> = HashSet::new();

        while mines.len() < self.num_mines {
            if let Some(&eligible_mine_position) = current_eligible_generator_positions.iter().choose(&mut rng) {
                mines.insert(eligible_mine_position);
                current_eligible_generator_positions.remove(&eligible_mine_position);
                grid = MinesweeperGrid::from_mine_positions(width, height, &mines);
                grid.reveal(pressed_position);
                if Solver::is_solvable(&grid) {
                    current_eligible_generator_positions.extend(&rejected_mines);
                    rejected_mines.clear();
                } else {
                    mines.remove(&eligible_mine_position);
                    rejected_mines.insert(eligible_mine_position);
                }
            } else {
                mines.clear();
                rejected_mines.clear();
                current_eligible_generator_positions = eligible_generator_positions.clone();
            }
        }

        grid
    }
}
