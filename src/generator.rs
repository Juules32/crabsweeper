use crate::MinesweeperGrid;

pub trait Generator: Send {
    fn generate(&self, width: usize, height: usize, pressed_coords: (usize, usize)) -> MinesweeperGrid;
}
