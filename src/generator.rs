use crate::{MinesweeperGrid, Position};

pub trait Generator: Send {
    fn generate(&self, width: usize, height: usize, pressed_position: Position) -> MinesweeperGrid;
}
