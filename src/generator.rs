use crate::{MinesweeperGrid, Position};

pub trait Generator {
    fn generate(
        &self,
        width: usize,
        height: usize,
        pressed_position: Position,
        num_mines: usize,
        seed: u64
    ) -> MinesweeperGrid;

    fn name(&self) -> &'static str;
}
