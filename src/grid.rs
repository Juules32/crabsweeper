use std::collections::HashSet;
use unicode_width::UnicodeWidthStr;

pub struct CellTheme {
    pub width: usize,
    pub mine: &'static str,
    pub covered: &'static str,
    pub flag: &'static str,
    pub empty: &'static str,
}

#[cfg(feature = "emoji_grid")]
pub const CELL_THEME: CellTheme = CellTheme {
    width: 2,
    mine: "🦀",
    covered: "🟦",
    flag: "🚩",
    empty: " ",
};

#[cfg(not(feature = "emoji_grid"))]
pub const CELL_THEME: CellTheme = CellTheme {
    width: 1,
    mine: "M",
    covered: "#",
    flag: "F",
    empty: " ",
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, cells: Vec<T>) -> Grid<T> {
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn set_cells(&mut self, cells: Vec<T>) {
        self.cells = cells;
    }

    pub fn index(&self, position: Position) -> usize {
        debug_assert!(position.x < self.width && position.y < self.height);
        position.y * self.width + position.x
    }

    pub fn set(&mut self, position: Position, value: T) {
        let index = self.index(position);
        self.cells[index] = value;
    }

    pub fn get(&self, position: Position) -> &T {
        &self.cells[self.index(position)]
    }

    pub fn get_mut(&mut self, position: Position) -> &mut T {
        let index = self.index(position);
        &mut self.cells[index]
    }

    pub fn neighbor_positions(&self, position: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = position.x as isize + dx;
                let ny = position.y as isize + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    neighbors.push(Position { x: nx as usize, y: ny as usize });
                }
            }
        }
        neighbors
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.cells.iter_mut()
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Position> {
        (0..self.cells.len()).map(|i| Position {
            x: i % self.width,
            y: i / self.width,
        })
    }

    pub fn draw_grid<F: Fn(Position) -> String>(&self, get_symbol: F) -> String {
        let mut out = String::new();

        let top = horizontal_line(self.width, '┌', '┬', '┐');
        let mid = horizontal_line(self.width, '├', '┼', '┤');
        let bot = horizontal_line(self.width, '└', '┴', '┘');

        out.push_str(&top);
        out.push('\n');

        for y in 0..self.height {
            let mut row = String::new();
            row.push('│');
            for x in 0..self.width {
                let position = Position { x, y };
                let sym = get_symbol(position);
                row.push(' ');
                row.push_str(&format_cell(&sym));
                row.push(' ');
                row.push('│');
            }
            out.push_str(&row);
            out.push('\n');

            if y + 1 < self.height {
                out.push_str(&mid);
                out.push('\n');
            }
        }

        out.push_str(&bot);
        out
    }
    
    pub fn generate_eligible_generator_positions(&self, pressed_position: Position) -> Vec<Position> {
        let mut surrounding_positions = self.neighbor_positions(pressed_position);
        surrounding_positions.push(pressed_position);

        let forbidden: HashSet<_> = surrounding_positions.into_iter().collect();

        self
            .iter_positions()
            .filter(|pos| !forbidden.contains(pos))
            .collect()
    }
}

fn horizontal_line(width: usize, left: char, mid: char, right: char) -> String {
    let mut line = String::new();
    line.push(left);
    for x in 0..width {
        line.push_str(&"─".repeat(CELL_THEME.width + 2));
        if x + 1 < width {
            line.push(mid);
        }
    }
    line.push(right);
    line
}

fn format_cell(s: &str) -> String {
    let w = UnicodeWidthStr::width(s);
    if w >= CELL_THEME.width {
        s.to_string()
    } else {
        let pad = CELL_THEME.width - w;
        format!("{}{}", " ".repeat(pad), s)
    }
}
