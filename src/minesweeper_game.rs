use crate::{CellContent, CellState, Generator, MinesweeperGrid};

#[derive(PartialEq)]
pub enum State {
    JustCreated,
    Playing,
    GameOver,
    YouWon,
}

pub struct MinesweeperGame {
    pub state: State,
    pub grid: MinesweeperGrid,
    generator: Box<dyn Generator>,
}

impl MinesweeperGame {
    pub fn new(width: usize, height: usize, generator: Box<dyn Generator>) -> Self {
        Self {
            state: State::JustCreated,
            grid: MinesweeperGrid::empty(width, height),
            generator,
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        if let State::JustCreated = self.state {
            self.state = State::Playing;
            self.grid.update_content(self.generator.generate(self.grid.width, self.grid.height, (x, y)));
        }
        if let State::Playing = self.state {
            let cell = self.grid.get(x, y);
            if cell.state == CellState::Covered {
            self.grid.reveal(x, y);
            } else {
                match cell.content {
                    CellContent::Number(_) => { self.grid.chord(x, y) },
                    _ => {}
                }
            }
            if self.grid.count_revealed_mines() > 0 {
                self.state = State::GameOver;
            } else if self.grid.is_solved() {
                self.state = State::YouWon;
            }
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        if self.state == State::JustCreated || self.state == State::Playing {
        self.grid.flag(x, y);
        }
    }
}
