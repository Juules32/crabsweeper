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
        matches!(self.state, CellState::Covered)
    }

    pub fn should_propagate(&self) -> bool {
        matches!(self.state, CellState::Revealed) && matches!(self.content, CellContent::Empty)
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
