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

    pub fn draw_grid<F: Fn(usize, usize) -> String>(&self, get_symbol: F) -> String {
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
                let sym = get_symbol(x, y);
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
