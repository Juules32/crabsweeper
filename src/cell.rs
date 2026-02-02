#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellContent {
    Mine,
    Empty,
    Number(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum CellState {
    Covered,
    Revealed,
    Flagged,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub content: CellContent,
    pub state: CellState,
}

impl From<CellContent> for Cell {
    fn from(content: CellContent) -> Self {
        Self {
            content,
            state: CellState::Covered,
        }
    }
}
