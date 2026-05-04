#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellContent {
    Empty,
    Number(u8),
    Mine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Covered,
    Revealed,
    Flagged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub content: CellContent,
    pub state: CellState,
}

impl Cell {
    pub fn can_be_revealed(&self) -> bool {
        self.state == CellState::Covered
    }

    pub fn should_propagate(&self) -> bool {
        self.state == CellState::Revealed && self.content == CellContent::Empty
    }
    
    pub fn is_flagged(&self) -> bool {
        self.state == CellState::Flagged
    }

    pub fn reveal(&mut self) {
        if self.can_be_revealed() {
            self.state = CellState::Revealed;
        }
    }

    pub fn get_minesweeprs_char(&self) -> char {
        match self.state {
            CellState::Flagged => '*',
            CellState::Covered => 'x',
            CellState::Revealed => {
                match self.content {
                    CellContent::Empty => '0',
                    CellContent::Number(n) => char::from_digit(n as u32, 10).unwrap(),
                    CellContent::Mine => '*' // Should never happen
                }
            }
        }
    }
}

impl From<CellContent> for Cell {
    fn from(content: CellContent) -> Self {
        Self {
            content,
            state: CellState::Covered,
        }
    }
}
