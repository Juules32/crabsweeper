use crate::BitGrid;

pub trait Generator {
    fn generate(&self, width: usize, height: usize, pressed_x: usize, pressed_y: usize) -> BitGrid;
}
