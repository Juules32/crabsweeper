use core::fmt;
use unicode_width::UnicodeWidthStr;
use crate::{Cell, CellState, CellContent};

#[cfg(feature = "emoji_grid")]
const CELL_WIDTH: usize = 2;
#[cfg(not(feature = "emoji_grid"))]
const CELL_WIDTH: usize = 1;

#[cfg(feature = "emoji_grid")]
const MINE_CELL: &str = "🦀";
#[cfg(not(feature = "emoji_grid"))]
const MINE_CELL: &str = "M";

#[cfg(feature = "emoji_grid")]
const COVERED_CELL: &str = "🟦";
#[cfg(not(feature = "emoji_grid"))]
const COVERED_CELL: &str = "#";

#[cfg(feature = "emoji_grid")]
const FLAG_CELL: &str = "🚩";
#[cfg(not(feature = "emoji_grid"))]
const FLAG_CELL: &str = "F";

const EMPTY_CELL: &str = " ";

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width && y < self.height);
        y * self.width + x
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let index = self.index(x, y);
        self.cells[index] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.cells[self.index(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = self.index(x, y);
        &mut self.cells[index]
    }

    pub fn neighbors_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for dx in [-1isize, 0, 1] {
            for dy in [-1isize, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    result.push((nx as usize, ny as usize));
                }
            }
        }
        result
    }

    pub fn iter_xy(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.cells.iter().enumerate().map(|(i, cell)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, cell)
        })
    }
}

pub type BitGrid = Grid<bool>;

impl BitGrid {
    pub fn new(width: usize, height: usize) -> Self {
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

fn horizontal_line(width: usize, left: char, mid: char, right: char) -> String {
    let mut line = String::new();
    line.push(left);
    for x in 0..width {
        line.push_str(&"─".repeat(CELL_WIDTH + 2));
        if x + 1 < width {
            line.push(mid);
        }
    }
    line.push(right);
    line
}

fn format_cell(s: &str) -> String {
    let w = UnicodeWidthStr::width(s);
    if w >= CELL_WIDTH {
        s.to_string()
    } else {
        let pad = CELL_WIDTH - w;
        format!("{}{}", " ".repeat(pad), s)
    }
}

fn draw_grid<F: Fn(usize, usize) -> String>(width: usize, height: usize, get_symbol: F) -> String {
    let mut out = String::new();

    let top = horizontal_line(width, '┌', '┬', '┐');
    let mid = horizontal_line(width, '├', '┼', '┤');
    let bot = horizontal_line(width, '└', '┴', '┘');

    out.push_str(&top);
    out.push('\n');

    for y in 0..height {
        let mut row = String::new();
        row.push('│');
        for x in 0..width {
            let sym = get_symbol(x, y);
            row.push(' ');
            row.push_str(&format_cell(&sym));
            row.push(' ');
            row.push('│');
        }
        out.push_str(&row);
        out.push('\n');

        if y + 1 < height {
            out.push_str(&mid);
            out.push('\n');
        }
    }

    out.push_str(&bot);
    out
}

impl fmt::Display for BitGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}",
            draw_grid(self.width, self.height, |x, y| {
                if *self.get(x, y) {
                    MINE_CELL.to_string()
                } else {
                    EMPTY_CELL.to_string()
                }
            })
        )?;

        writeln!(f, "\nWidth: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Mines: {}", self.count_bits())?;
        Ok(())
    }
}

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
            draw_grid(self.width, self.height, |x, y| {
                let Cell { content, state } = self.get(x, y);
                match state {
                    CellState::Covered => COVERED_CELL.to_string(),
                    CellState::Flagged => FLAG_CELL.to_string(),
                    CellState::Revealed => match content {
                        CellContent::Empty => EMPTY_CELL.to_string(),
                        CellContent::Mine => MINE_CELL.to_string(),
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
