use core::fmt;
use crate::{Grid, CellContent, CELL_THEME, Position};

pub type BitGrid = Grid<bool>;

impl BitGrid {
    pub fn empty(width: usize, height: usize) -> Self {
        Grid::new(width, height, vec![false; width * height])
    }

    pub fn count_bits(&self) -> usize {
        self.iter().filter(|&&b| b).count()
    }

    pub fn count_adjacent_bits(&self, position: Position) -> usize {
        self.neighbor_positions(position)
            .iter()
            .filter(|&&np| *self.get(np))
            .count()
    }

    pub fn get_cell_content(&self, position: Position) -> CellContent {
        if *self.get(position) {
            CellContent::Mine
        } else {
            match self.count_adjacent_bits(position) {
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
            self.draw_grid(|position| {
                if *self.get(position) {
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
