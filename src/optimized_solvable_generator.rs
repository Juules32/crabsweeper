use std::collections::HashSet;
use rand::{Rng, SeedableRng};
use rand::seq::{IteratorRandom, SliceRandom};
use rand_pcg::Pcg64;
use crate::{BitGrid, Generator, MinesweeperGrid, RandomGenerator, Solver};

pub struct OptimizedSolvableGenerator {
    pub num_mines: usize,
    pub seed: u64,
}

impl Generator for OptimizedSolvableGenerator {
    fn generate(&self, width: usize, height: usize, pressed_coords: (usize, usize)) -> MinesweeperGrid {
        let (pressed_x, pressed_y) = pressed_coords;
        let solver = Solver;
        let mut rng = Pcg64::seed_from_u64(self.seed);

        let mut grid = MinesweeperGrid::from(BitGrid::empty(width, height));
        let eligible_mine_coords: HashSet<(usize, usize)> = grid.generate_eligible_mine_coords(pressed_x, pressed_y).into_iter().collect();
        let mut current_eligible_mine_coords = eligible_mine_coords.clone();
        let mut mines: HashSet<(usize, usize)> = HashSet::new();
        let mut rejected_mines: HashSet<(usize, usize)> = HashSet::new();
        let mut num_failed_tries = 0;

        while mines.len() < self.num_mines {
            if let Some(&eligible_mine_coord) = current_eligible_mine_coords.iter().choose(&mut rng) {
                mines.insert(eligible_mine_coord);
                current_eligible_mine_coords.remove(&eligible_mine_coord);
                grid = MinesweeperGrid::from_mine_coords(width, height, &mines);
                grid.reveal(pressed_x, pressed_y);
                println!("{grid}");
                if solver.is_solvable(&grid) {
                    current_eligible_mine_coords.extend(&rejected_mines);
                    rejected_mines.clear();
                } else {
                    mines.remove(&eligible_mine_coord);
                    rejected_mines.insert(eligible_mine_coord);
                }
            } else {
                mines.clear();
                rejected_mines.clear();
                current_eligible_mine_coords = eligible_mine_coords.clone();
                num_failed_tries += 1;
                if num_failed_tries >= 100 {
                    panic!("Too many tries")
                }
            }
        }

        grid
    }
}
