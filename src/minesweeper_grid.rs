use core::fmt;
use std::collections::{HashSet, VecDeque};
use crate::{BitGrid, Cell, CellContent, CellState, Grid, Position, CELL_THEME};

pub type MinesweeperGrid = Grid<Cell>;

impl MinesweeperGrid {
    pub fn empty(width: usize, height: usize) -> Self {
        BitGrid::empty(width, height).into()
    }
    
    pub fn count_mines(&self) -> usize {
        self.iter().filter(|c| c.content == CellContent::Mine).count()
    }

    pub fn count_covered_or_flagged(&self) -> usize {
        self.iter().filter(|c| c.state == CellState::Covered || c.state == CellState::Flagged).count()
    }

    pub fn count_revealed_mines(&self) -> usize {
        self.iter().filter(|c| c.content == CellContent::Mine && c.state == CellState::Revealed).count()
    }

    fn count_flags(&self) -> usize {
        self.iter().filter(|c| c.state == CellState::Flagged).count()
    }

    pub fn count_remaining_flags(&self) -> isize {
        self.count_mines() as isize - self.count_flags() as isize
    }

    pub fn count_unflagged_mines(&self) -> usize {
        self.iter().filter(|c| c.content == CellContent::Mine && (c.state == CellState::Covered || c.state == CellState::Revealed)).count()
    }

    fn try_reveal_single(&mut self, position: Position) {
        let cell = self.get_mut(position);
        if cell.can_be_revealed() {
            cell.state = CellState::Revealed;
        }
    }

    pub fn reveal(&mut self, position: Position) {
        let mut stack = VecDeque::new();
        stack.push_back(position);
        let mut visited_cells = HashSet::new();

        while let Some(position) = stack.pop_front() {
            if visited_cells.contains(&position) {
                continue;
            }
            visited_cells.insert(position);
            
            self.try_reveal_single(position);

            if !self.get(position).should_propagate() {
                continue;
            }

            for np in self.neighbor_positions(position) {
                let neighbor_cell = self.get(np);
                if neighbor_cell.can_be_revealed() {
                    stack.push_back(np);
                }
            }
        }
    }

    pub fn chord(&mut self, position: Position) {
        let cell = self.get(position);
        let neighbor_positions = self.neighbor_positions(position);
        if let CellContent::Number(n) = cell.content {
            let num_flagged_neighbors = neighbor_positions
                .iter()
                .map(|&np| self.get(np))
                .filter(|c| c.state == CellState::Flagged)
                .count();

            if num_flagged_neighbors == n as usize {
                for np in neighbor_positions {
                    self.try_reveal_single(np);
                }
            }
        }
    }

    pub fn flag(&mut self, position: Position) {
        let cell = self.get_mut(position);
        match cell.state {
            CellState::Covered => cell.state = CellState::Flagged,
            CellState::Revealed => (),
            CellState::Flagged => cell.state = CellState::Covered,
        }
    }

    pub fn update_content(&mut self, minesweeper_grid: MinesweeperGrid) {
        self.set_cells(self
            .iter()
            .zip(minesweeper_grid.iter())
            .map(|(old, new)| {
                Cell {
                    content: new.content,
                    state: old.state,
                }
            })
            .collect()
        )
    }

    pub fn is_solved(&self) -> bool {
        self.count_covered_or_flagged() <= self.count_mines()
    }

    pub fn from_mine_positions(width: usize, height: usize, mine_positions: &HashSet<Position>) -> Self {
        let mut bit_grid = BitGrid::empty(width, height);
        for &mp in mine_positions {
            bit_grid.set(mp, true);
        }
        bit_grid.into()
    }

    pub fn reveal_mines(&mut self) {
        for cell in self.iter_mut() {
            if cell.content == CellContent::Mine {
                cell.reveal();
            }
        }
    }

    pub fn to_minesweeprs_string(&self) -> String {
        let mut res = String::new();
            for x in 0..self.width {
        for y in 0..self.height {
                let position = Position { x, y };
                let cell = self.get(position);
                res.push(cell.get_minesweeprs_char())
            }
            res.push('\n');
        }
        res
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
                bit_grid.get_cell_content(Position { x, y }).into()
            })
            .collect();

        Self::new(width, height, cells)
    }
}

impl fmt::Display for MinesweeperGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}",
            self.draw_grid(|position| {
                let Cell { content, state } = self.get(position);
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
