use crate::{MinesweeperGrid, Position};

pub trait Generator {
    fn generate(&self, width: usize, height: usize, pressed_position: Position) -> MinesweeperGrid;
}
