use core::fmt;
use crate::{BitGrid, Cell, CellContent, CellState, Grid, CELL_THEME};

pub type MinesweeperGrid = Grid<Cell>;

impl MinesweeperGrid {
    fn count_mines(&self) -> usize {
        self.cells.iter().filter(|c| c.content == CellContent::Mine).count()
    }

    fn count_flags(&self) -> usize {
        self.cells.iter().filter(|c| c.state == CellState::Flagged).count()
    }

    fn try_reveal_single(&mut self, x: usize, y: usize) {
        let cell = self.get_mut(x, y);
        if cell.can_be_revealed() {
            cell.state = CellState::Revealed;
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        self.try_reveal_single(x, y);

        if !self.get(x, y).should_propagate() {
            return;
        }

        for (nx, ny) in self.neighbors_coords(x, y) {
            let neighbor_cell = self.get(nx, ny);
            if neighbor_cell.can_be_revealed() {
                self.reveal(nx, ny);
            }
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        let cell = self.get_mut(x, y);
        match cell.state {
            CellState::Covered => cell.state = CellState::Flagged,
            CellState::Revealed => (),
            CellState::Flagged => cell.state = CellState::Covered,
        }
    }
}

impl From<BitGrid> for MinesweeperGrid {
    fn from(bitmap: BitGrid) -> Self {
        let width = bitmap.width;
        let height = bitmap.height;
        let cells = (0..width * height)
            .map(|i| {
                let x = i % width;
                let y = i / width;
                bitmap.get_cell_content(x, y).into()
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }
}

impl fmt::Display for MinesweeperGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}",
            self.draw_grid(|x, y| {
                let Cell { content, state } = self.get(x, y);
                match state {
                    CellState::Covered => CELL_THEME.covered.to_string(),
                    CellState::Flagged => CELL_THEME.flag.to_string(),
                    CellState::Revealed => match content {
                        CellContent::Empty => CELL_THEME.empty.to_string(),
                        CellContent::Mine => CELL_THEME.mine.to_string(),
                        CellContent::Number(n) => n.to_string(),
                    },
                }
            })
        )?;

        writeln!(f, "\nWidth: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Mines: {}", self.count_mines())?;
        writeln!(f, "Flags: {}", self.count_flags())?;
        Ok(())
    }
}
