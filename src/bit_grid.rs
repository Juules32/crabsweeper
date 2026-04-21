use core::fmt;
use crate::{Grid, CellContent, CELL_THEME};

pub type BitGrid = Grid<bool>;

impl BitGrid {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn count_bits(&self) -> usize {
        self.cells.iter().filter(|b| **b).count()
    }

    pub fn count_adjacent_bits(&self, x: usize, y: usize) -> usize {
        self.neighbors_coords(x, y)
            .iter()
            .filter(|&&(nx, ny)| *self.get(nx, ny))
            .count()
    }

    pub fn get_cell_content(&self, x: usize, y: usize) -> CellContent {
        if *self.get(x, y) {
            CellContent::Mine
        } else {
            match self.count_adjacent_bits(x, y) {
                0 => CellContent::Empty,
                n => CellContent::Number(n as u8),
            }
        }
    }
}

impl fmt::Display for BitGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}",
            self.draw_grid(|x, y| {
                if *self.get(x, y) {
                    CELL_THEME.mine.to_string()
                } else {
                    CELL_THEME.empty.to_string()
                }
            })
        )?;

        writeln!(f, "\nWidth: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Mines: {}", self.count_bits())?;
        Ok(())
    }
}
