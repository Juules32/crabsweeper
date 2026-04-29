use core::fmt;
use std::collections::{HashSet, VecDeque};
use crate::{BitGrid, Cell, CellContent, CellState, Grid, CELL_THEME};

pub type MinesweeperGrid = Grid<Cell>;

impl MinesweeperGrid {
    pub fn empty(width: usize, height: usize) -> Self {
        BitGrid::empty(width, height).into()
    }
    
    pub fn count_mines(&self) -> usize {
        self.cells.iter().filter(|c| c.content == CellContent::Mine).count()
    }

    pub fn count_covered_or_flagged(&self) -> usize {
        self.cells.iter().filter(|c| c.state == CellState::Covered || c.state == CellState::Flagged).count()
    }

    pub fn count_revealed_mines(&self) -> usize {
        self.cells.iter().filter(|c| c.content == CellContent::Mine && c.state == CellState::Revealed).count()
    }

    fn count_flags(&self) -> usize {
        self.cells.iter().filter(|c| c.state == CellState::Flagged).count()
    }

    pub fn count_remaining_flags(&self) -> isize {
        self.count_mines() as isize - self.count_flags() as isize
    }

    pub fn count_unflagged_mines(&self) -> usize {
        self.cells.iter().filter(|c| c.content == CellContent::Mine && (c.state == CellState::Covered || c.state == CellState::Revealed)).count()
    }

    fn try_reveal_single(&mut self, x: usize, y: usize) {
        let cell = self.get_mut(x, y);
        if cell.can_be_revealed() {
            cell.state = CellState::Revealed;
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        let mut stack = VecDeque::new();
        stack.push_back((x, y));
        let mut visited_cells = HashSet::new();

        while let Some((x, y)) = stack.pop_front() {
            if visited_cells.contains(&(x, y)) {
                continue;
            }
            visited_cells.insert((x, y));
            
            self.try_reveal_single(x, y);

            if !self.get(x, y).should_propagate() {
                continue;
            }

            for (nx, ny) in self.neighbor_coords(x, y) {
                let neighbor_cell = self.get(nx, ny);
                if neighbor_cell.can_be_revealed() {
                    stack.push_back((nx, ny));
                }
            }
        }
    }

    pub fn chord(&mut self, x: usize, y: usize) {
        let cell = self.get(x, y);
        let neighbors_coords = self.neighbor_coords(x, y);
        if let CellContent::Number(n) = cell.content {
            let num_flagged_neighbors = neighbors_coords
                .iter()
                .map(|&(nx, ny)| self.get(nx, ny))
                .filter(|c| c.state == CellState::Flagged)
                .count();

            if num_flagged_neighbors == n as usize {
                for (nx, ny) in neighbors_coords {
                    self.try_reveal_single(nx, ny);
                }
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

    pub fn update_content(&mut self, minesweeper_grid: MinesweeperGrid) {
        self.cells = self.cells
            .iter()
            .zip(minesweeper_grid.cells.iter())
            .map(|(old, new)| {
                Cell {
                    content: new.content,
                    state: old.state,
                }
            })
            .collect()
    }

    pub fn is_solved(&self) -> bool {
        self.count_covered_or_flagged() <= self.count_mines()
    }
}

impl From<BitGrid> for MinesweeperGrid {
    fn from(bit_grid: BitGrid) -> Self {
        let width = bit_grid.width;
        let height = bit_grid.height;
        let cells = (0..width * height)
            .map(|i| {
                let x = i % width;
                let y = i / width;
                bit_grid.get_cell_content(x, y).into()
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
