use core::fmt;
use crate::{Cell, CellContent, CellState};
use unicode_width::UnicodeWidthStr;

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
pub struct Bitmap {
    width: usize,
    height: usize,
    bits: Vec<bool>,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        Self::from_bits(width, height, vec![false; width * height])
    }

    pub fn from_bits(width: usize, height: usize, bits: Vec<bool>) -> Self {
        debug_assert_eq!(bits.len(), width * height);
        Self { width, height, bits }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width && y < self.height);
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.bits[self.index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let index = self.index(x, y);
        self.bits[index] = value;
    }

    fn get_adjacent_bits(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacent_bits = Vec::new();
        if x > 0 {
            adjacent_bits.push((x- 1, y));
            if y > 0 {
                adjacent_bits.push((x - 1, y - 1));
            }
            if y < self.height - 1 {
                adjacent_bits.push((x - 1, y + 1));
            }
        }
        if x < self.width - 1 {
            adjacent_bits.push((x + 1, y));
            if y > 0 {
                adjacent_bits.push((x + 1, y - 1));
            }
            if y < self.height - 1 {
                adjacent_bits.push((x + 1, y + 1));
            }
        }
        if y > 0 {
            adjacent_bits.push((x, y - 1));
        }
        if y < self.height - 1 {
            adjacent_bits.push((x, y + 1));
        }

        adjacent_bits
    }

    fn count_bits(&self) -> usize {
        self.bits.iter().filter(|b| **b).count()
    }

    fn count_adjacent_bits(&self, x: usize, y: usize) -> usize {
        self.get_adjacent_bits(x, y).iter().filter(|(x, y)| self.get(*x, *y)).count()
    }

    pub fn get_cell_content(&self, x: usize, y: usize) -> CellContent {
        if self.get(x, y) {
            CellContent::Mine
        } else {
            match self.count_adjacent_bits(x, y) {
                0 => CellContent::Empty,
                x => CellContent::Number(x as u8),
            }
        }
    }
}

fn horizontal_line(
    width: usize,
    left: char,
    mid: char,
    right: char,
) -> String {
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

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", draw_grid(self.width, self.height, |x, y| {
            if self.get(x, y) {
                MINE_CELL.to_string()
            } else {
                EMPTY_CELL.to_string()
            }
        }))?;
        writeln!(f, "\nWidth: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Mines: {}", self.count_bits())?;

        Ok(())
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    fn index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width && y < self.height);
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[self.index(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let index = self.index(x, y);
        &mut self.cells[index]
    }

    fn count_mines(&self) -> usize {
        self.cells.iter().filter(|c| c.content == CellContent::Mine).count()
    }

    fn count_flags(&self) -> usize {
        self.cells.iter().filter(|c| c.state == CellState::Flagged).count()
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        let cell = self.get_mut(x, y);
        cell.state = CellState::Revealed;
        // Propagate if cell is empty
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        let cell = self.get_mut(x, y);
        cell.state = CellState::Flagged;
    }
}

impl From<Bitmap> for Grid {
    fn from(bitmap: Bitmap) -> Self {
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", draw_grid(self.width, self.height, |x, y| {
            let Cell { content, state } = self.get(x, y);
            match state {
                CellState::Covered => COVERED_CELL.to_string(),
                CellState::Flagged => FLAG_CELL.to_string(),
                CellState::Revealed => {
                    match content {
                        CellContent::Empty => EMPTY_CELL.to_string(),
                        CellContent::Mine => MINE_CELL.to_string(),
                        CellContent::Number(n) => n.to_string(),
                    }
                }
            }
        }))?;

        writeln!(f, "\nWidth: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Mines: {}", self.count_mines())?;
        writeln!(f, "Flags: {}", self.count_flags())?;

        Ok(())
    }
}
