use crate::{BitGrid, Generator, MinesweeperGrid};

pub struct RandomGenerator;

impl Generator for RandomGenerator {
    fn generate(&self, width: usize, height: usize) -> MinesweeperGrid {
        let mut bit_grid = BitGrid::empty(width, height);
        bit_grid.set(0, 0, true);
        bit_grid.into()
    }
}
