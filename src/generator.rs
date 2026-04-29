use crate::MinesweeperGrid;

pub trait Generator {
    fn generate(&self, width: usize, height: usize, pressed_coords: (usize, usize)) -> MinesweeperGrid;
}
